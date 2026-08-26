#!/usr/bin/env python3
"""Estimate human-equivalent effort for a measured HAMR experiment (Wave 1).

Consumes project-measures.json plus the experiment manifest, applies the
counterfactual category assignments and the linear SLOC effort model (all rates
UNCALIBRATED placeholders), normalizes session metrics (schema v2 and legacy
v1), and writes effort-estimate.json. Ratios are computed only for the
within-experiment modeled-vs-observed comparison under exact scope alignment;
cross-experiment ratios are never produced here.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import sys
from pathlib import Path
from typing import Any


SCRIPT_DIR = Path(__file__).resolve().parent
CONFIG_DIR = SCRIPT_DIR.parent / "config"
DEFAULT_MODEL_CONFIG = CONFIG_DIR / "effort-model-v1.json"
DEFAULT_COUNTERFACTUAL = CONFIG_DIR / "counterfactual-v1.json"

UNCALIBRATED_LABEL = "UNCALIBRATED MODEL"


# --------------------------------------------------------------------------
# Session-metrics normalization
# Ported from the h-wf-compare skill's scripts/normalize_experiments.py
# (normalize_metrics / normalize_friction_v1 / normalize_friction_v2 / totals),
# copied with this provenance comment so skills stay self-contained.
# --------------------------------------------------------------------------

def totals(models: dict[str, Any]) -> dict[str, int]:
    result = {"input": 0, "output": 0, "cache_read": 0, "cache_write": 0, "reasoning_output": 0}
    for usage in models.values():
        if not isinstance(usage, dict):
            continue
        for key in result:
            value = usage.get(key)
            if isinstance(value, (int, float)):
                result[key] += int(value)
    return result


def normalize_friction_v2(value: dict[str, Any]) -> dict[str, int]:
    def length(key: str) -> int:
        item = value.get(key)
        return len(item) if isinstance(item, list) else int(item or 0)
    return {
        "escalation_requests": length("escalation_requests"),
        "dynamic_shell_commands": length("dynamic_shell_commands"),
        "sandbox_failures": length("sandbox_failures"),
        "observable_approval_outcomes": length("observable_approval_outcomes"),
        "parser_coverage_warnings": length("parser_coverage_warnings"),
    }


def normalize_friction_v1(value: Any) -> dict[str, int]:
    value = value if isinstance(value, dict) else {}
    return {
        "escalation_requests": int(value.get("sandbox_escape_calls", 0) or 0),
        "dynamic_shell_commands": int(value.get("dynamic_command_calls", 0) or 0),
        "sandbox_failures": int(value.get("sandbox_failure_results", 0) or 0),
        "observable_approval_outcomes": 0,
        "parser_coverage_warnings": 0,
    }


def normalize_metrics(metrics: dict[str, Any] | None) -> tuple[dict[str, Any], list[str]]:
    caveats: list[str] = []
    if not metrics:
        return {
            "schema_version": None,
            "session_id": None,
            "harness": None,
            "harness_version": None,
            "adapter_version": None,
            "models": [],
            "tokens": totals({}),
            "api_list_price_equivalent_usd": None,
            "actual_cost_usd": None,
            "active_seconds": None,
            "wall_clock_seconds": None,
            "friction": normalize_friction_v2({}),
        }, ["missing or invalid session-metrics.json"]
    if metrics.get("schema_version") == 2:
        harness = metrics.get("harness") if isinstance(metrics.get("harness"), dict) else {}
        session = metrics.get("session") if isinstance(metrics.get("session"), dict) else {}
        span = metrics.get("span") if isinstance(metrics.get("span"), dict) else {}
        models = metrics.get("models") if isinstance(metrics.get("models"), dict) else {}
        costs = metrics.get("cost_usd") if isinstance(metrics.get("cost_usd"), dict) else {}
        equivalent = costs.get("api_list_price_equivalent") if isinstance(costs.get("api_list_price_equivalent"), dict) else {}
        friction_value = metrics.get("friction") if isinstance(metrics.get("friction"), dict) else {}
        if costs.get("actual") is None:
            caveats.append("actual billed cost unavailable")
        if metrics.get("scope", {}).get("subagent_activity") == "excluded":
            caveats.append("subagents excluded")
        warnings = friction_value.get("parser_coverage_warnings")
        if isinstance(warnings, list) and warnings:
            caveats.append(f"{len(warnings)} parser coverage warning(s); friction counts undercounted")
        return {
            "schema_version": 2,
            "session_id": session.get("id"),
            "harness": harness.get("name"),
            "harness_version": harness.get("version"),
            "adapter_version": harness.get("adapter_version"),
            "models": sorted(models),
            "tokens": totals(models),
            "api_list_price_equivalent_usd": equivalent.get("total"),
            "actual_cost_usd": costs.get("actual"),
            "active_seconds": span.get("active_seconds"),
            "wall_clock_seconds": span.get("wall_clock_seconds"),
            "friction": normalize_friction_v2(friction_value),
        }, caveats

    caveats.extend(["legacy schema-v1 metrics adapted", "actual billed cost unavailable", "subagent inclusion unknown or excluded"])
    models = metrics.get("models") if isinstance(metrics.get("models"), dict) else {}
    span = metrics.get("span") if isinstance(metrics.get("span"), dict) else {}
    cost = metrics.get("cost_usd") if isinstance(metrics.get("cost_usd"), dict) else {}
    return {
        "schema_version": 1,
        "session_id": metrics.get("session_id"),
        "harness": "claude-code",
        "harness_version": metrics.get("version"),
        "adapter_version": "legacy-adapter-v1",
        "models": sorted(models),
        "tokens": totals(models),
        "api_list_price_equivalent_usd": cost.get("total"),
        "actual_cost_usd": None,
        "active_seconds": span.get("active_seconds"),
        "wall_clock_seconds": span.get("wall_clock_seconds"),
        "friction": normalize_friction_v1(metrics.get("friction")),
    }, caveats


# --------------------------------------------------------------------------
# Counterfactual category assignment
# --------------------------------------------------------------------------

def category_for(bucket: dict, counterfactual: dict) -> str:
    rules = counterfactual["assignment_rules"]
    provenance = bucket["provenance"]
    by_provenance = rules["by_provenance"]
    if provenance in by_provenance:
        return by_provenance[provenance]
    if provenance == "developer_authored":
        by_type = rules["developer_authored_by_artifact_type"]
        return by_type.get(bucket["artifact_type"], by_type.get("_default", "hamr_path_authored"))
    return "unknown_provenance"


# --------------------------------------------------------------------------
# Estimator interface and registry (§9.1)
# --------------------------------------------------------------------------

class LinearSlocModel:
    """hours = sloc / sloc_per_hour[type] * multiplier(profile, type) [* equivalence].

    A future COCOMO-II-family model reuses the pipeline and schema family but
    not necessarily this call signature (nonlinear aggregation does not
    decompose per bucket).
    """

    def __init__(self, config: dict):
        self.config = config

    def hours_for(self, artifact_type: str, measures: dict, profile_name: str, equivalence: float = 1.0) -> float:
        rates = self.config["rates"]
        profile = self.config["profiles"][profile_name]
        rate_entry = rates.get(artifact_type)
        if rate_entry is None:
            rate_entry = {"sloc_per_hour": 10}
        if rate_entry.get("fixed_hours_override") is not None:
            fixed = rate_entry["fixed_hours_override"] * measures.get("bucket_count", 1)
            return fixed * profile["default_multiplier"]
        sloc = measures.get("sloc", 0)
        multiplier = profile["type_multipliers"].get(artifact_type, profile["default_multiplier"])
        return (sloc / rate_entry["sloc_per_hour"]) * multiplier * equivalence

    def estimate(self, by_type: dict[str, dict], profile_name: str, equivalence: float = 1.0) -> dict:
        profile = self.config["profiles"][profile_name]
        breakdown = {}
        total_hours = 0.0
        for artifact_type, measures in sorted(by_type.items()):
            hours = self.hours_for(artifact_type, measures, profile_name, equivalence)
            breakdown[artifact_type] = {"sloc": measures.get("sloc", 0), "hours": round(hours, 2)}
            total_hours += hours
        return {
            "hours": round(total_hours, 2),
            "cost_usd": round(total_hours * profile["hourly_cost_usd"], 2),
            "by_type": breakdown,
        }


MODELS = {"linear_sloc_v1": LinearSlocModel}


# --------------------------------------------------------------------------
# Workflow-status lookups (mirrors the measurer's lightweight matching)
# --------------------------------------------------------------------------

def _expand_row_keys(row: dict) -> list[str]:
    keys = [row["key"]]
    if row.get("alias"):
        keys.append(row["alias"])
    expanded: list[str] = []
    for key in keys:
        expanded.append(key)
        range_match = re.match(r"^(.*)\.([A-Za-z0-9]+)-([A-Za-z0-9]+)$", key)
        if range_match:
            base, first, second = range_match.groups()
            expanded.extend([f"{base}.{first}", f"{base}.{second}"])
    return expanded


def build_status_index(rows: list[dict], components: list[str]) -> dict[str, dict]:
    index: dict[str, dict] = {}
    for row in rows:
        for key in _expand_row_keys(row):
            wildcard = re.match(r"^([A-Za-z]+)\(\*\)(.*)$", key)
            if wildcard and components:
                for component in components:
                    index.setdefault(f"{wildcard.group(1)}({component}){wildcard.group(2)}".lower(), row)
            else:
                index.setdefault(key.lower(), row)
    return index


def status_for(index: dict[str, dict], key: str) -> str | None:
    row = index.get(key.lower())
    if row is None:
        return None
    return row["status"].split("/")[0].strip()


# --------------------------------------------------------------------------
# Step ordering for stable display
# --------------------------------------------------------------------------

STEP_ORDER = [
    r"^SysPlanAndReq\.1$", r"^SysPlanAndReq\.2$", r"^SysPlanAndReq\.3$", r"^SysPlanAndReq\.4$",
    r"^SysModeling", r"^CompGUMBOSpec", r"^SysGUMBOIntegrationCheck",
    r"^CodeGen\.2$", r"^CodeGen\.4$", r"^CompDev\([^)]*\)\.2$", r"^CompDev\([^)]*\)\.3$",
    r"^CompDev\([^)]*\)\.4$", r"^CompDev\([^)]*\)\.6$", r"^SysSchedDef",
    r"^SysGUMBOSysSpecCheck$", r"^SysGUMBOSysSpecCheck\.4$", r"^SysGUMBOSysSpecCheck\.5$",
]


def step_sort_key(step_key: str) -> tuple[int, str]:
    for rank, pattern in enumerate(STEP_ORDER):
        if re.match(pattern, step_key):
            return (rank, step_key)
    if step_key == "unattributed":
        return (len(STEP_ORDER) + 1, step_key)
    return (len(STEP_ORDER), step_key)


# --------------------------------------------------------------------------
# Estimation driver
# --------------------------------------------------------------------------

def sha256_of(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def aggregate_buckets(measures: dict, counterfactual: dict) -> dict:
    """Aggregate file buckets into category -> artifact_type -> measures, plus
    per-step category/type aggregations."""
    by_category: dict[str, dict[str, dict]] = {}
    per_step: dict[str, dict] = {}
    for entry in measures["files"]:
        for bucket in entry["buckets"]:
            category = category_for(bucket, counterfactual)
            slot = by_category.setdefault(category, {}).setdefault(
                bucket["artifact_type"], {"sloc": 0, "bucket_count": 0}
            )
            slot["sloc"] += bucket["sloc"]
            slot["bucket_count"] += 1
            step_key = bucket["workflow"]["key"]
            step = per_step.setdefault(step_key, {
                "granularity": bucket["workflow"]["granularity"],
                "sloc_total": 0,
                "by_provenance": {},
                "authored_by_type": {},
                "generated_by_type": {},
                "unknown_sloc": 0,
                "artifact_ids": set(),
            })
            step["sloc_total"] += bucket["sloc"]
            step["by_provenance"][bucket["provenance"]] = step["by_provenance"].get(bucket["provenance"], 0) + bucket["sloc"]
            if bucket.get("artifact_id"):
                step["artifact_ids"].add(bucket["artifact_id"])
            if category in ("common_both_paths", "hamr_path_authored"):
                type_slot = step["authored_by_type"].setdefault(bucket["artifact_type"], {"sloc": 0, "bucket_count": 0})
                type_slot["sloc"] += bucket["sloc"]
                type_slot["bucket_count"] += 1
            elif category == "hamr_generated_manual_replacement":
                type_slot = step["generated_by_type"].setdefault(bucket["artifact_type"], {"sloc": 0, "bucket_count": 0})
                type_slot["sloc"] += bucket["sloc"]
                type_slot["bucket_count"] += 1
            elif category == "unknown_provenance":
                step["unknown_sloc"] += bucket["sloc"]
    return {"by_category": by_category, "per_step": per_step}


def expand_overhead_steps(model_config: dict, components: list[str]) -> list[dict]:
    expanded = []
    for entry in model_config.get("activity_overheads", {}).get("steps", []):
        pattern = entry["step_pattern"]
        if "<component>" in pattern:
            for component in components:
                expanded.append({**entry, "step_key": pattern.replace("<component>", component)})
        else:
            expanded.append({**entry, "step_key": pattern})
    return expanded


def step_assurance(step_key: str, assurance: dict) -> dict | None:
    """Attach the relevant §8 assurance observation to its workflow row."""
    comp_match = re.match(r"^CompDev\(([^)]*)\)\.(\d)$", step_key)
    if comp_match:
        component, sub = comp_match.groups()
        if sub == "3":
            value = assurance["tests"]["defined_static"].get(component)
            return {"tests_defined_static": value} if value else None
        if sub == "4":
            runtime = assurance["tests"]["runtime"].get(component)
            cover = assurance["coverage"].get(component)
            return {"tests_runtime": runtime, "coverage": cover}
        if sub == "6":
            value = assurance["component_verus"].get(component)
            return {"component_verus": value} if value else None
        return None
    if step_key == "SysGUMBOIntegrationCheck.1":
        return {"integration_check": assurance["integration_check"]}
    if step_key == "CodeGen.2":
        return {"generated_resources": assurance["generated_resources"]}
    if step_key == "SysGUMBOSysSpecCheck.4":
        structure = assurance["system_proof"].get("structure")
        return {"system_proof_structure": structure} if structure else None
    if step_key == "SysGUMBOSysSpecCheck.5":
        return {"system_proof_results": assurance["system_proof"]["results"]}
    return None


def estimate(measures: dict, manifest: dict, model_config_path: Path, counterfactual_path: Path,
             session_metrics_path: Path | None, now: str,
             measures_path: Path | None = None) -> dict:
    model_config = json.loads(model_config_path.read_text(encoding="utf-8"))
    counterfactual = json.loads(counterfactual_path.read_text(encoding="utf-8"))
    model = MODELS[model_config["model"]](model_config)
    profiles = sorted(model_config["profiles"])
    scenarios = [
        {"name": name, "factor": value["factor"], "note": value["note"]}
        for name, value in model_config["equivalence_scenarios"]["scenarios"].items()
    ]

    caveats: list[str] = list(measures.get("caveats", []))
    caveats.append(f"all effort estimates are {UNCALIBRATED_LABEL} placeholders (calibration_status={model_config['calibration_status']})")

    aggregated = aggregate_buckets(measures, counterfactual)
    by_category = aggregated["by_category"]
    per_step = aggregated["per_step"]

    components = measures.get("workflow_status", {}).get("components", [])
    rows = measures.get("workflow_status", {}).get("rows", [])
    index = build_status_index(rows, components)

    # ---- per-step rows (SLOC steps + activity-overhead steps)
    overheads = expand_overhead_steps(model_config, components)
    overhead_by_step: dict[str, dict] = {entry["step_key"]: entry for entry in overheads}
    step_keys = sorted(set(per_step) | set(overhead_by_step), key=step_sort_key)

    steps = []
    for step_key in step_keys:
        data = per_step.get(step_key)
        status = status_for(index, step_key)
        if status is None:
            base = re.match(r"^([A-Za-z]+(\([^)]*\))?)", step_key)
            if base:
                status = status_for(index, base.group(1))
        overhead = overhead_by_step.get(step_key)
        overhead_applies = overhead is not None and status in ("done", "waived")
        human_estimates = {}
        for profile_name in profiles:
            profile = model_config["profiles"][profile_name]
            estimate_value = model.estimate(data["authored_by_type"], profile_name) if data else {"hours": 0.0, "cost_usd": 0.0, "by_type": {}}
            overhead_hours = round(overhead["hours"] * profile["default_multiplier"], 2) if overhead_applies else 0.0
            hours = round(estimate_value["hours"] + overhead_hours, 2)
            human_estimates[profile_name] = {
                "hours": hours,
                "cost_usd": round(hours * profile["hourly_cost_usd"], 2),
                "by_type": estimate_value["by_type"],
                "overhead_hours": overhead_hours,
            }
        steps.append({
            "step_key": step_key,
            "granularity": data["granularity"] if data else "exact_step",
            "status": status,
            "measures": {
                "sloc_total": data["sloc_total"] if data else 0,
                "by_provenance": data["by_provenance"] if data else {},
                "unknown_sloc": data["unknown_sloc"] if data else 0,
                "artifact_ids": sorted(data["artifact_ids"]) if data else [],
                "activity": overhead["activity"] if overhead else None,
            },
            "assurance": step_assurance(step_key, measures["assurance"]),
            "human_estimates": human_estimates,
            "agent_actuals": None,
        })

    # ---- headline aggregations
    generated_types = by_category.get("hamr_generated_manual_replacement", {})
    generated_sloc = sum(entry["sloc"] for entry in generated_types.values())
    generated_estimates: dict[str, dict] = {}
    for profile_name in profiles:
        generated_estimates[profile_name] = {}
        for scenario in scenarios:
            generated_estimates[profile_name][scenario["name"]] = model.estimate(
                generated_types, profile_name, equivalence=scenario["factor"]
            )

    common_types = by_category.get("common_both_paths", {})
    common_estimates = {profile: model.estimate(common_types, profile) for profile in profiles}
    authored_types = by_category.get("hamr_path_authored", {})
    authored_estimates = {profile: model.estimate(authored_types, profile) for profile in profiles}

    unknown_types = by_category.get("unknown_provenance", {})
    unknown_sloc = sum(entry["sloc"] for entry in unknown_types.values())

    template_totals = {
        "files": len(measures.get("template_accounting", [])),
        "supplied": sum(entry["supplied"] for entry in measures.get("template_accounting", [])),
        "retained": sum(entry["retained"] for entry in measures.get("template_accounting", [])),
        "replaced": sum(entry["replaced"] for entry in measures.get("template_accounting", [])),
    }

    # ---- session totals (once per experiment; never repeated into step rows)
    raw_metrics = None
    metrics_source = None
    if session_metrics_path and session_metrics_path.is_file():
        metrics_source = str(session_metrics_path)
        try:
            raw_metrics = json.loads(session_metrics_path.read_text(encoding="utf-8"))
        except (OSError, json.JSONDecodeError):
            raw_metrics = None
    normalized, metric_caveats = normalize_metrics(raw_metrics)
    if normalized["schema_version"] == 1:
        cost_kind = "list_price_equivalent"
        metric_caveats.append("legacy cost is approximate main-session list price")
    elif normalized["actual_cost_usd"] is not None:
        cost_kind = "actual"
    else:
        cost_kind = "list_price_equivalent"
    if manifest.get("harness") == "claude-code":
        metric_caveats.append("Claude Code metrics exclude subagent activity by design")
    session_totals = {
        "source": metrics_source,
        "schema_or_adapter": ("schema-v2" if normalized["schema_version"] == 2 else normalized.get("adapter_version")),
        "harness": normalized["harness"],
        "harness_version": normalized["harness_version"],
        "models": normalized["models"],
        "tokens": normalized["tokens"],
        "cost_usd": {
            "kind": cost_kind if (normalized["api_list_price_equivalent_usd"] is not None or normalized["actual_cost_usd"] is not None) else None,
            "total": normalized["actual_cost_usd"] if cost_kind == "actual" else normalized["api_list_price_equivalent_usd"],
        },
        "active_seconds": normalized["active_seconds"],
        "wall_clock_seconds": normalized["wall_clock_seconds"],
        "friction": normalized["friction"],
        "caveats": sorted(set(metric_caveats)),
    }

    # ---- within-experiment comparison (§6.2, §6.3)
    total_overhead = {
        profile: round(sum(step["human_estimates"][profile]["overhead_hours"] for step in steps), 2)
        for profile in profiles
    }
    modeled_human = {}
    for profile_name in profiles:
        profile = model_config["profiles"][profile_name]
        hours = round(
            common_estimates[profile_name]["hours"]
            + authored_estimates[profile_name]["hours"]
            + total_overhead[profile_name],
            2,
        )
        modeled_human[profile_name] = {
            "hours": hours,
            "cost_usd": round(hours * profile["hourly_cost_usd"], 2),
            "composition": "common_both_paths + hamr_path_authored + activity overheads",
        }

    scope_exact = bool(
        manifest.get("lifecycle") == "greenfield"
        and manifest.get("metrics_cover_full_run")
        and measures["inputs"]["codegen_report"]["report_status"] != "absent"
    )
    scope_alignment = "exact" if scope_exact else "partial"
    active_seconds = session_totals["active_seconds"]
    agent_cost = session_totals["cost_usd"]["total"]
    ratio = None
    if scope_alignment == "exact" and active_seconds:
        agent_hours = active_seconds / 3600.0
        ratio = {"label": UNCALIBRATED_LABEL, "by_profile": {}}
        for profile_name in profiles:
            entry: dict[str, Any] = {
                "time_ratio": round(modeled_human[profile_name]["hours"] / agent_hours, 1),
            }
            if agent_cost:
                entry["cost_ratio"] = round(modeled_human[profile_name]["cost_usd"] / agent_cost, 1)
            ratio["by_profile"][profile_name] = entry
    comparison_caveats = list(session_totals["caveats"])
    comparison_caveats.append(
        f"{UNCALIBRATED_LABEL}: modeled human hours use uncalibrated placeholder rates under a named profile"
    )
    if not scope_exact:
        comparison_caveats.append("scope alignment is not exact; ratio suppressed")
    comparisons = [{
        "kind": "within_experiment_modeled_human_vs_observed_agent",
        "scope_alignment": scope_alignment,
        "values": {
            "modeled_human": modeled_human,
            "observed_agent": {
                "active_seconds": active_seconds,
                "wall_clock_seconds": session_totals["wall_clock_seconds"],
                "cost_usd": session_totals["cost_usd"],
                "tokens": session_totals["tokens"],
            },
        },
        "ratio": ratio,
        "caveats": comparison_caveats,
    }]

    # ---- attribution accounting (100% including unattributed)
    total_sloc = measures["rollups"]["total_sloc"]
    by_granularity: dict[str, int] = {}
    unattributed_sloc = 0
    for step in steps:
        gran = step["granularity"]
        by_granularity[gran] = by_granularity.get(gran, 0) + step["measures"]["sloc_total"]
        if step["step_key"] == "unattributed":
            unattributed_sloc += step["measures"]["sloc_total"]
    coverage = round((total_sloc - unattributed_sloc) / total_sloc, 4) if total_sloc else 1.0

    assignments_summary = {
        category: {
            "sloc": sum(entry["sloc"] for entry in types.values()),
            "by_artifact_type": {name: entry["sloc"] for name, entry in sorted(types.items())},
        }
        for category, types in sorted(by_category.items())
    }

    return {
        "schema_version": 1,
        "generated": now,
        "project": measures["project"],
        "manifest": manifest,
        "measures_ref": str(measures_path.resolve()) if measures_path else "",
        "estimator": {
            "model": model_config["model"],
            "config_sha256": sha256_of(model_config_path),
            "calibration_status": model_config["calibration_status"],
            "scope_note": model_config["scope_note"],
        },
        "counterfactual": {
            "config_sha256": sha256_of(counterfactual_path),
            "assignments": assignments_summary,
        },
        "scenarios": scenarios,
        "steps": steps,
        "hamr_generated_value": {
            "sloc": generated_sloc,
            "by_artifact_type": {name: entry["sloc"] for name, entry in sorted(generated_types.items())},
            "template_accounting": template_totals,
            "estimates": generated_estimates,
        },
        "common_mode": {
            "sloc": sum(entry["sloc"] for entry in common_types.values()),
            "by_artifact_type": {name: entry["sloc"] for name, entry in sorted(common_types.items())},
            "estimates": common_estimates,
        },
        "authored_hamr_path": {
            "sloc": sum(entry["sloc"] for entry in authored_types.values()),
            "by_artifact_type": {name: entry["sloc"] for name, entry in sorted(authored_types.items())},
            "estimates": authored_estimates,
        },
        "unknown_provenance": {
            "sloc": unknown_sloc,
            "by_artifact_type": {name: entry["sloc"] for name, entry in sorted(unknown_types.items())},
            "note": "origin unresolved (no compatible template baseline); counted in neither headline story",
        },
        "unmodeled_manual": counterfactual["manual_only_unmodeled"],
        "workflow_completion": {
            "workflows": measures.get("workflow_status", {}).get("workflows", {}),
            "completed": measures.get("workflow_status", {}).get("completed", []),
            "components": components,
            "profile": measures.get("workflow_status", {}).get("profile"),
        },
        "session_totals": session_totals,
        "comparisons": comparisons,
        "attribution_summary": {"coverage": coverage, "by_granularity": by_granularity},
        "caveats": caveats,
    }


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--measures", required=True, help="project-measures.json path")
    parser.add_argument("--manifest", required=True, help="effort manifest path")
    parser.add_argument("--config", default=str(DEFAULT_MODEL_CONFIG))
    parser.add_argument("--counterfactual", default=str(DEFAULT_COUNTERFACTUAL))
    parser.add_argument("--session-metrics", help="session-metrics.json path (default: manifest session_metrics relative to project)")
    parser.add_argument("--now", help="ISO timestamp for the generated field (reproducibility)")
    parser.add_argument("--out", help="output path (default alongside measures as effort-estimate.json)")
    args = parser.parse_args(argv)

    measures_path = Path(args.measures)
    manifest_path = Path(args.manifest)
    for path in (measures_path, manifest_path):
        if not path.is_file():
            print(f"error: not found: {path}", file=sys.stderr)
            return 2
    measures = json.loads(measures_path.read_text(encoding="utf-8"))
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    if args.session_metrics:
        session_metrics_path = Path(args.session_metrics)
    else:
        project = Path(measures["project"]["path"])
        session_metrics_path = project / manifest.get("session_metrics", "experiment-reports/session-metrics.json")
    now = args.now or __import__("datetime").datetime.now().astimezone().isoformat(timespec="seconds")
    result = estimate(measures, manifest, Path(args.config), Path(args.counterfactual), session_metrics_path, now,
                      measures_path=measures_path)
    out = Path(args.out) if args.out else measures_path.parent / "effort-estimate.json"
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(result, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(f"wrote {out.resolve()}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
