#!/usr/bin/env python3
"""Render stakeholder-viewable effort reports (Markdown + self-contained HTML).

Per-experiment mode consumes effort-estimate.json (+ project-measures.json for
assurance/template detail) and writes EFFORT-<project>-<date>.md/.html. The
--compare mode renders a multi-experiment page over the commonly completed
workflow subset with no cross-experiment ratios.

HTML output is self-contained: inline CSS, CSS-bar charts, zero JS and zero
external dependencies. Every chart's values also appear in an adjacent table.
"""

from __future__ import annotations

import argparse
import hashlib
import html
import json
import sys
from pathlib import Path
from string import Template


SCRIPT_DIR = Path(__file__).resolve().parent
TEMPLATE_PATH = SCRIPT_DIR.parent / "template" / "effort-report.md"
DEFAULT_MODEL_CONFIG = SCRIPT_DIR.parent / "config" / "effort-model-v1.json"

UNCALIBRATED_LABEL = "UNCALIBRATED MODEL"

# Validated categorical palette (dataviz reference slots 1-4, fixed order,
# color follows the provenance class everywhere) + neutral for the unknown fold.
PROVENANCE_COLORS = [
    ("hamr_generated", "HAMR generated", "#2a78d6"),
    ("hamr_woven", "HAMR woven", "#008300"),
    ("hamr_template_retained", "HAMR template retained", "#e87ba4"),
    ("developer_authored", "developer/agent authored", "#eda100"),
    ("__unknown__", "unknown origin", "#9b9a97"),
]
UNKNOWN_FOLD = {"preserved_origin_unknown", "unknown", "library"}
BAR_HUE = "#2a78d6"


def fmt_hours(value: float) -> str:
    return f"{value:,.0f} h" if value >= 10 else f"{value:.1f} h"


def fmt_money(value: float) -> str:
    return f"${value:,.0f}"


def fmt_int(value) -> str:
    return f"{value:,}" if isinstance(value, (int, float)) and value is not None else "—"


def md_table(headers: list[str], rows: list[list[str]]) -> str:
    lines = ["| " + " | ".join(headers) + " |", "|" + "|".join(["---"] * len(headers)) + "|"]
    lines.extend("| " + " | ".join(str(cell) for cell in row) + " |" for row in rows)
    return "\n".join(lines)


def provenance_totals(estimate: dict) -> dict[str, int]:
    totals: dict[str, int] = {}
    for step in estimate["steps"]:
        for provenance, sloc in step["measures"]["by_provenance"].items():
            totals[provenance] = totals.get(provenance, 0) + sloc
    return totals


def folded_provenance(totals: dict[str, int]) -> list[tuple[str, str, str, int]]:
    result = []
    for key, label, color in PROVENANCE_COLORS:
        if key == "__unknown__":
            value = sum(totals.get(name, 0) for name in UNKNOWN_FOLD)
        else:
            value = totals.get(key, 0)
        result.append((key, label, color, value))
    return result


def replacement_range(estimate: dict) -> tuple[float, float, float, float]:
    hours = []
    costs = []
    for profile_values in estimate["hamr_generated_value"]["estimates"].values():
        for scenario_values in profile_values.values():
            hours.append(scenario_values["hours"])
            costs.append(scenario_values["cost_usd"])
    if not hours:
        return (0.0, 0.0, 0.0, 0.0)
    return (min(hours), max(hours), min(costs), max(costs))


def modeled_human_range(estimate: dict) -> tuple[float, float, float, float]:
    modeled = estimate["comparisons"][0]["values"]["modeled_human"]
    hours = [entry["hours"] for entry in modeled.values()]
    costs = [entry["cost_usd"] for entry in modeled.values()]
    return (min(hours), max(hours), min(costs), max(costs))


def summarize_step_assurance(assurance: dict | None) -> str:
    if not assurance:
        return ""
    parts = []
    if "tests_defined_static" in assurance:
        parts.append(f"{assurance['tests_defined_static']['total']} tests defined (static)")
    if "tests_runtime" in assurance and assurance["tests_runtime"]:
        value = assurance["tests_runtime"]["value"]
        if value.get("passed") is not None:
            parts.append(f"{value['passed']} passed, {value.get('failed') or 0} failed (notes)")
    if "coverage" in assurance and assurance["coverage"]:
        pct = assurance["coverage"]["value"].get("entrypoint_lines_percent")
        if pct is not None:
            parts.append(f"entrypoint lines {pct}% (notes)")
    if "component_verus" in assurance:
        value = assurance["component_verus"]["value"]
        if value.get("verified") is not None:
            parts.append(f"Verus {value['verified']}/{value.get('errors', 0)} (notes)")
    if "integration_check" in assurance:
        value = assurance["integration_check"]["value"]
        vacuous = " — vacuous by design" if value.get("vacuous") else ""
        if value.get("expected_handshakes") is not None:
            parts.append(f"handshakes N={value['expected_handshakes']}{vacuous} (notes)")
    if "generated_resources" in assurance and "value" in assurance["generated_resources"]:
        value = assurance["generated_resources"]["value"]
        parts.append(f"{value['deduped']} resources ({value['overwritten_true']} regen / {value['overwritten_false']} once) (report)")
    if "system_proof_structure" in assurance:
        crates = assurance["system_proof_structure"]["value"]
        for crate in crates:
            parts.append(f"proof crate {crate['crate']}: {crate['property_count']} properties, {crate['rs_files']} files (structure)")
    if "system_proof_results" in assurance:
        value = assurance["system_proof_results"]["value"]
        if value.get("verified") is not None:
            parts.append(f"system VCs {value['verified']}/{value.get('errors', 0)} (notes)")
    return "; ".join(parts)


GRANULARITY_MARKS = {"workflow_only": " †", "inferred_step": " ‡", "unattributed": " §"}


# --------------------------------------------------------------------------
# Markdown sections (per-experiment)
# --------------------------------------------------------------------------

def render_executive_summary(estimate: dict) -> str:
    generated = estimate["hamr_generated_value"]
    h_lo, h_hi, c_lo, c_hi = replacement_range(estimate)
    profiles = sorted(estimate["hamr_generated_value"]["estimates"])
    scenario_names = [scenario["name"] for scenario in estimate["scenarios"]]
    comparison = estimate["comparisons"][0]
    session = estimate["session_totals"]
    mh_lo, mh_hi, mc_lo, mc_hi = modeled_human_range(estimate)
    cost_kind = session["cost_usd"]["kind"]
    cost_note = "list-price equivalent" if cost_kind == "list_price_equivalent" else (cost_kind or "unavailable")
    active_h = (session["active_seconds"] or 0) / 3600.0
    authored = estimate["authored_hamr_path"]
    common = estimate["common_mode"]

    lines = ["## 1. Executive summary", ""]
    lines.append(
        f"**HAMRvsManual (Modeled).** HAMR generated **{fmt_int(generated['sloc'])} SLOC** of "
        f"infrastructure, configuration, and assurance artifacts. Hand-building a manual equivalent is "
        f"modeled at **{fmt_hours(h_lo)}–{fmt_hours(h_hi)}** (**{fmt_money(c_lo)}–{fmt_money(c_hi)}**) across "
        f"profiles ({', '.join(profiles)}) and equivalence scenarios ({', '.join(scenario_names)}). "
        f"This is a *modeled manual-equivalent replacement value*, never a measured net saving."
    )
    lines.append("")
    ratio_text = ""
    if comparison["ratio"]:
        parts = []
        for profile, values in comparison["ratio"]["by_profile"].items():
            fragment = f"~{values['time_ratio']}× time"
            if "cost_ratio" in values:
                fragment += f" / ~{values['cost_ratio']}× cost"
            parts.append(f"{fragment} ({profile})")
        ratio_text = (
            f" Within-experiment ratio (scope `{comparison['scope_alignment']}`, {UNCALIBRATED_LABEL}): "
            + "; ".join(parts) + "."
        )
    elif comparison["scope_alignment"] != "exact":
        ratio_text = f" Scope alignment is `{comparison['scope_alignment']}`; no ratio is rendered."
    total_cost = session["cost_usd"]["total"]
    lines.append(
        f"**HAMR-AGENTvsHAMR (Compared).** Observed agent session: "
        f"**{fmt_money(total_cost) if total_cost is not None else '—'}** ({cost_note}), "
        f"**{active_h:.2f} active hours** ({fmt_int(session['tokens']['output'])} output tokens). "
        f"Modeled human effort for the same authored artifacts plus tool-running overheads: "
        f"**{fmt_hours(mh_lo)}–{fmt_hours(mh_hi)}** (**{fmt_money(mc_lo)}–{fmt_money(mc_hi)}**) by profile."
        + ratio_text
    )
    lines.append("")
    lines.append(
        f"> **{UNCALIBRATED_LABEL}.** Every modeled number above uses uncalibrated placeholder "
        f"productivity rates under a named profile, scenario, and counterfactual (§7). Observed values "
        f"are measured; modeled values are estimates; no calibrated prediction accuracy is claimed."
    )
    lines.append("")
    rows = []
    for artifact_type in sorted(set(authored["by_artifact_type"]) | set(common["by_artifact_type"])):
        source = "common (both paths)" if artifact_type in common["by_artifact_type"] else "HAMR-path authored"
        sloc = common["by_artifact_type"].get(artifact_type, authored["by_artifact_type"].get(artifact_type, 0))
        rows.append([artifact_type, source, fmt_int(sloc)])
    lines.append("Authored-artifact composition (Observed SLOC; modeled per profile in §2/§4):")
    lines.append("")
    lines.append(md_table(["Artifact type", "Counterfactual category", "SLOC"], rows))
    return "\n".join(lines)


def render_step_table(estimate: dict) -> str:
    profiles = sorted(next(iter(estimate["steps"]), {}).get("human_estimates", {"experienced_sel4": None, "new_to_sel4": None}))
    rows = []
    for step in estimate["steps"]:
        measures = step["measures"]
        prov = measures["by_provenance"]
        generated = sum(prov.get(name, 0) for name in ("hamr_generated", "hamr_woven", "hamr_template_retained"))
        authored = prov.get("developer_authored", 0)
        unknown = sum(prov.get(name, 0) for name in UNKNOWN_FOLD)
        sloc_cell = f"gen {fmt_int(generated)} · auth {fmt_int(authored)} · unk {fmt_int(unknown)}" if measures["sloc_total"] else "—"
        hours_cell = " / ".join(fmt_hours(step["human_estimates"][profile]["hours"]) for profile in profiles)
        artifacts = ", ".join(measures["artifact_ids"]) or (measures.get("activity") or "—")
        rows.append([
            step["step_key"] + GRANULARITY_MARKS.get(step["granularity"], ""),
            step["status"] or "—",
            artifacts,
            sloc_cell,
            summarize_step_assurance(step.get("assurance")) or "—",
            hours_cell,
            "—",
        ])
    table = md_table(
        ["Step", "Status", "Artifacts", "SLOC by provenance", "Assurance (Observed)",
         f"Modeled hours ({' / '.join(profiles)})", "Agent actuals"],
        rows,
    )
    legend = (
        "\n\n† artifact produced across several steps of this workflow (workflow-only granularity) · "
        "‡ inferred step (file generated earlier, edited here) · § unattributed accounting bucket. "
        "Modeled hours are " + UNCALIBRATED_LABEL + " placeholders."
    )
    return table + legend


def render_generation_detail(estimate: dict, measures: dict | None) -> str:
    generated = estimate["hamr_generated_value"]
    profiles = sorted(generated["estimates"])
    scenarios = [scenario["name"] for scenario in estimate["scenarios"]]
    lines = [
        f"Generated SLOC (Observed): **{fmt_int(generated['sloc'])}** across "
        f"{len(generated['by_artifact_type'])} artifact types. Modeled replacement estimates below are "
        f"{UNCALIBRATED_LABEL} placeholders; the conservative scenario halves the line-equivalence."
    , ""]
    header = ["Artifact type", "SLOC"]
    for profile in profiles:
        for scenario in scenarios:
            header.append(f"{profile} · {scenario}")
    rows = []
    for artifact_type, sloc in sorted(generated["by_artifact_type"].items()):
        row = [artifact_type, fmt_int(sloc)]
        for profile in profiles:
            for scenario in scenarios:
                by_type = generated["estimates"][profile][scenario]["by_type"]
                row.append(fmt_hours(by_type.get(artifact_type, {}).get("hours", 0.0)))
        rows.append(row)
    total_row = ["**Total**", f"**{fmt_int(generated['sloc'])}**"]
    for profile in profiles:
        for scenario in scenarios:
            entry = generated["estimates"][profile][scenario]
            total_row.append(f"**{fmt_hours(entry['hours'])} ({fmt_money(entry['cost_usd'])})**")
    rows.append(total_row)
    lines.append(md_table(header, rows))
    lines.append("")
    accounting = generated["template_accounting"]
    lines.append(
        f"Generate-once templates: HAMR supplied **{fmt_int(accounting['supplied'])}** template SLOC in "
        f"{accounting['files']} files; **{fmt_int(accounting['retained'])}** retained in the final system "
        f"(counted above), **{fmt_int(accounting['replaced'])}** replaced by the developer/agent "
        f"(counted conservatively as authored work, not as HAMR value)."
    )
    if measures and measures.get("template_accounting"):
        lines.append("")
        lines.append(md_table(
            ["Generate-once file", "Template supplied", "Retained", "Replaced"],
            [[entry["file"], fmt_int(entry["supplied"]), fmt_int(entry["retained"]), fmt_int(entry["replaced"])]
             for entry in measures["template_accounting"]],
        ))
    unknown = estimate["unknown_provenance"]
    lines.append("")
    lines.append(
        f"Unknown-provenance bucket: **{fmt_int(unknown['sloc'])} SLOC** — {unknown['note']}."
    )
    return "\n".join(lines)


def render_common_mode(estimate: dict) -> str:
    common = estimate["common_mode"]
    profiles = sorted(common["estimates"])
    rows = []
    for artifact_type, sloc in sorted(common["by_artifact_type"].items()):
        row = [artifact_type, fmt_int(sloc)]
        for profile in profiles:
            row.append(fmt_hours(common["estimates"][profile]["by_type"].get(artifact_type, {}).get("hours", 0.0)))
        rows.append(row)
    total = ["**Total**", f"**{fmt_int(common['sloc'])}**"]
    for profile in profiles:
        entry = common["estimates"][profile]
        total.append(f"**{fmt_hours(entry['hours'])} ({fmt_money(entry['cost_usd'])})**")
    rows.append(total)
    return md_table(["Artifact type", "SLOC", *[f"Modeled ({p})" for p in profiles]], rows)


def _sourced_cell(value: dict) -> str:
    return f"{value.get('source', '—')} · {value.get('classification_basis', '—')} · {value.get('confidence', '—')}"


def render_assurance_detail(measures: dict | None) -> str:
    if not measures:
        return "_project-measures.json not supplied to the renderer; see effort-estimate.json step rows._"
    assurance = measures["assurance"]
    lines = []
    rows = []
    for component, static in sorted(assurance["tests"]["defined_static"].items()):
        runtime = assurance["tests"]["runtime"].get(component, {})
        runtime_value = runtime.get("value", {})
        reconciliation = assurance["tests"]["reconciliation"].get(component, {})
        consistent = reconciliation.get("consistent")
        rows.append([
            component,
            f"{static['total']} ({static['tests_rs']} tests.rs + {static['cb_apis_rs']} cb_apis.rs)",
            f"{runtime_value.get('passed', '—')} passed / {runtime_value.get('failed', '—')} failed",
            {True: "yes", False: "**no**", None: "—"}[consistent],
        ])
    lines.append("**Tests** (static counts are exact; runtime results derive from workflow-status Notes):")
    lines.append("")
    lines.append(md_table(["Component", "Defined (static)", "Run (notes)", "Static = runtime?"], rows))
    lines.append("")
    rows = []
    for component, cover in sorted(assurance["coverage"].items()):
        value = cover["value"]
        rows.append([component, value.get("entrypoint_lines_percent") or "—", cover["confidence"]])
    lines.append(md_table(["Component", "Entrypoint line coverage %", "Confidence"], rows))
    lines.append("")
    rows = []
    for component, verus in sorted(assurance["component_verus"].items()):
        value = verus["value"]
        rows.append([component, value.get("verified", "—"), value.get("errors", "—"), verus["confidence"]])
    lines.append(md_table(["Component", "Verus obligations verified", "Errors", "Confidence"], rows))
    lines.append("")
    proof = assurance["system_proof"]
    if proof.get("structure"):
        for crate in proof["structure"]["value"]:
            lines.append(
                f"**System proof** (structure, exact): crate `{crate['crate']}` — "
                f"{crate['property_count']} properties ({', '.join(crate['properties'])}), "
                f"{crate['rs_files']} Rust files, {fmt_int(crate['total_lines'])} lines."
            )
    results = proof["results"]["value"]
    if results.get("verified") is not None:
        lines.append(f"System VCs: **{results['verified']} verified, {results['errors']} errors** (workflow-status Notes, derived).")
    else:
        lines.append("System VCs: — (" + (proof["results"].get("note") or "not recorded") + ").")
    lines.append("")
    integration = assurance["integration_check"]
    value = integration["value"]
    vacuous = " — **vacuous by design** (zero receiver-side integration assumes; hamr-sysml-patterns.md §2)" if value.get("vacuous") else ""
    lines.append(f"Integration check: expected handshakes N={value.get('expected_handshakes', '—')}{vacuous} (notes, derived).")
    lines.append("")
    audit = assurance["audit_findings"]
    if audit.get("present"):
        severities = ", ".join(f"{count} {name}" for name, count in sorted(audit["value"]["by_severity"].items()))
        lines.append(f"Experiment assessment findings: {audit['value']['total']} ({severities}) — assessment-summary.json, exact.")
    else:
        lines.append(f"Experiment assessment findings: not available ({audit.get('note', 'absent')}).")
    contract_rows = []
    for component, entry in sorted(assurance.get("contract_audit_findings", {}).items()):
        contract_rows.append([component, entry["value"].get("findings", "—"), entry["confidence"]])
    if contract_rows:
        lines.append("")
        lines.append(md_table(["Component", "Contract-audit findings (CompGUMBOSpec.3)", "Confidence"], contract_rows))
    return "\n".join(lines)


def render_session_totals(estimate: dict) -> str:
    session = estimate["session_totals"]
    tokens = session["tokens"]
    rows = [
        ["Harness", f"{session['harness'] or '—'} {session['harness_version'] or ''}".strip()],
        ["Models", ", ".join(session["models"]) or "—"],
        ["Metrics schema / adapter", session["schema_or_adapter"] or "—"],
        ["Tokens (in / out / cache r / cache w)",
         f"{fmt_int(tokens['input'])} / {fmt_int(tokens['output'])} / {fmt_int(tokens['cache_read'])} / {fmt_int(tokens['cache_write'])}"],
        ["Cost", (fmt_money(session['cost_usd']['total']) if session['cost_usd']['total'] is not None else "—")
         + f" ({session['cost_usd']['kind'] or 'unavailable'})"],
        ["Active / wall-clock time",
         f"{fmt_int(session['active_seconds'])} s / {fmt_int(session['wall_clock_seconds'])} s"],
        ["Friction (escalations / dynamic shell / sandbox failures)",
         f"{session['friction']['escalation_requests']} / {session['friction']['dynamic_shell_commands']} / {session['friction']['sandbox_failures']}"],
    ]
    caveats = "\n".join(f"- {caveat}" for caveat in session["caveats"])
    return md_table(["Session metric (Observed)", "Value"], rows) + "\n\nCaveats:\n\n" + caveats


def render_assumptions(estimate: dict, model_config: dict | None, measures: dict | None) -> str:
    lines = []
    lines.append(f"Estimator: `{estimate['estimator']['model']}` — calibration status "
                 f"**{estimate['estimator']['calibration_status']}**. Scope: {estimate['estimator']['scope_note']}")
    lines.append("")
    if model_config:
        rows = []
        for artifact_type, entry in sorted(model_config["rates"].items()):
            rate = entry["sloc_per_hour"]
            override = entry.get("fixed_hours_override")
            rate_cell = f"{rate}" + (f" (fixed {override} h override)" if override is not None else "")
            rows.append([artifact_type, rate_cell, entry["rationale"]])
        lines.append(md_table(["Artifact type", "SLOC/hour (placeholder)", "Rationale"], rows))
        lines.append("")
        rows = []
        for name, profile in sorted(model_config["profiles"].items()):
            multipliers = ", ".join(f"{k} ×{v}" for k, v in sorted(profile["type_multipliers"].items())) or "—"
            rows.append([name, fmt_money(profile["hourly_cost_usd"]) + "/h", f"×{profile['default_multiplier']}", multipliers])
        lines.append(md_table(["Profile", "Rate", "Default multiplier", "Type multipliers"], rows))
        lines.append("")
    for scenario in estimate["scenarios"]:
        lines.append(f"- Equivalence scenario **{scenario['name']}** (factor {scenario['factor']}): {scenario['note']}")
    lines.append("")
    lines.append("Counterfactual category assignments (Observed SLOC per category):")
    lines.append("")
    rows = []
    for category, entry in estimate["counterfactual"]["assignments"].items():
        types = ", ".join(f"{name} {fmt_int(sloc)}" for name, sloc in entry["by_artifact_type"].items())
        rows.append([category, fmt_int(entry["sloc"]), types])
    lines.append(md_table(["Category", "SLOC", "By artifact type"], rows))
    lines.append("")
    lines.append("Manual-path effort deliberately **not** modeled (the honest asymmetry — these costs exist but are speculative):")
    lines.extend(f"- {item}" for item in estimate["unmodeled_manual"])
    lines.append("")
    coverage = estimate["attribution_summary"]
    lines.append(
        f"Attribution accounting: **{coverage['coverage']:.1%}** of measured SLOC attributed to workflow "
        f"steps; granularity mix: "
        + ", ".join(f"{name} {fmt_int(sloc)}" for name, sloc in sorted(coverage["by_granularity"].items()))
        + ". 100% of measured SLOC sits in exactly one accounting bucket including `unattributed`."
    )
    lines.append("")
    lines.append("Data-quality caveats:")
    lines.extend(f"- {caveat}" for caveat in estimate["caveats"])
    lines.append("")
    lines.append("Reproducibility:")
    lines.append(f"- effort-model config SHA-256 `{estimate['estimator']['config_sha256']}`")
    lines.append(f"- counterfactual config SHA-256 `{estimate['counterfactual']['config_sha256']}`")
    lines.append(f"- session-metrics source `{estimate['session_totals']['source']}` "
                 f"({estimate['session_totals']['schema_or_adapter']})")
    if measures:
        baseline = measures["inputs"]["generation_baseline"]
        if baseline and baseline.get("present"):
            provenance = baseline.get("provenance") or {}
            agreement = baseline.get("agreement") or {}
            lines.append(
                f"- generation baseline: {provenance.get('sireum_version', 'unknown tool version')}, "
                f"regeneration agreement {agreement.get('fraction', '—')} "
                f"({agreement.get('identical', '—')}/{agreement.get('overwrite_true_compared', '—')} regenerated files identical)"
            )
        else:
            lines.append("- generation baseline: none (generate-once provenance falls back to preserved_origin_unknown)")
        report = measures["inputs"]["codegen_report"]
        lines.append(
            f"- codegen report status **{report['report_status']}** "
            f"({fmt_int(report['resource_count_raw'])} raw / {fmt_int(report['resource_count_deduped'])} deduped resources; "
            f"classification mode {measures['inputs']['classification_mode']})"
        )
    lines.append(f"- generated timestamp {estimate['generated']} (injectable via --now)")
    return "\n".join(lines)


def render_markdown(estimate: dict, measures: dict | None, model_config: dict | None) -> str:
    template = Template(TEMPLATE_PATH.read_text(encoding="utf-8"))
    banner = (
        f"**{UNCALIBRATED_LABEL}** — modeled values use uncalibrated placeholder rates; every headline is "
        f"tagged Observed (measured), Modeled (estimated under a named profile/scenario/counterfactual), "
        f"or Compared (scope-gated). Experiment `{estimate['project']['name']}`, workflow "
        f"{estimate['manifest'].get('workflow')} ({estimate['manifest'].get('profile')} profile), harness "
        f"{estimate['manifest'].get('harness')}."
    )
    return template.substitute(
        experiment=estimate["project"]["name"],
        date=estimate["generated"][:10],
        label_banner=banner,
        executive_summary=render_executive_summary(estimate),
        step_table=render_step_table(estimate),
        generation_detail=render_generation_detail(estimate, measures),
        common_mode=render_common_mode(estimate),
        assurance_detail=render_assurance_detail(measures),
        session_totals=render_session_totals(estimate),
        assumptions=render_assumptions(estimate, model_config, measures),
    )


# --------------------------------------------------------------------------
# HTML rendering (self-contained, light-committed, zero JS)
# --------------------------------------------------------------------------

CSS = """
:root { color-scheme: light; }
* { box-sizing: border-box; }
body { margin: 2rem auto; max-width: 72rem; padding: 0 1.5rem; background: #fcfcfb; color: #0b0b0b;
       font: 15px/1.55 system-ui, -apple-system, "Segoe UI", sans-serif; }
h1 { font-size: 1.6rem; margin: 0 0 .25rem; }
h2 { font-size: 1.15rem; margin: 2.2rem 0 .6rem; border-bottom: 1px solid #e4e3df; padding-bottom: .3rem; }
h3 { font-size: 1rem; margin: 1.4rem 0 .4rem; }
p, li { max-width: 62rem; }
.meta { color: #52514e; margin-bottom: 1.2rem; }
.badge { display: inline-block; background: #fdf3d8; border: 1px solid #eda100; color: #6b4a00;
         border-radius: 4px; padding: .05rem .45rem; font-size: .78rem; font-weight: 600; letter-spacing: .02em; }
.tag { display: inline-block; border-radius: 4px; padding: 0 .35rem; font-size: .72rem; font-weight: 600;
       border: 1px solid #d8d7d2; color: #52514e; margin-right: .3rem; }
.box { border: 1px solid #d8d7d2; border-radius: 8px; padding: 1rem 1.2rem; margin: .8rem 0; background: #ffffff; }
.box h3 { margin-top: 0; }
table { border-collapse: collapse; margin: .8rem 0; font-size: .88rem; width: 100%; }
th, td { border: 1px solid #e4e3df; padding: .3rem .55rem; text-align: left; vertical-align: top; }
th { background: #f4f3f0; font-weight: 600; }
td.num, th.num { text-align: right; font-variant-numeric: tabular-nums; }
.scroll { overflow-x: auto; }
.bar-track { display: flex; height: 16px; border-radius: 4px; overflow: hidden; background: #efeeea; margin: .3rem 0; }
.bar-track span { display: block; height: 100%; border-right: 2px solid #fcfcfb; }
.bar-track span:last-child { border-right: none; }
.hbar { display: flex; align-items: center; gap: .6rem; margin: .25rem 0; }
.hbar .lbl { flex: 0 0 17rem; font-size: .85rem; color: #52514e; text-align: right; }
.hbar .trk { flex: 1; background: #efeeea; border-radius: 4px; height: 14px; position: relative; }
.hbar .fill { height: 100%; border-radius: 4px; background: #2a78d6; }
.hbar .val { flex: 0 0 11rem; font-size: .85rem; font-variant-numeric: tabular-nums; }
.legend { display: flex; flex-wrap: wrap; gap: .9rem; font-size: .82rem; color: #52514e; margin: .3rem 0 .8rem; }
.legend i { display: inline-block; width: 10px; height: 10px; border-radius: 2px; margin-right: .35rem; }
.caveats li { color: #52514e; font-size: .88rem; }
code { background: #f4f3f0; border-radius: 3px; padding: 0 .25rem; font-size: .85em; }
.footer { margin-top: 2.5rem; color: #7a7975; font-size: .8rem; border-top: 1px solid #e4e3df; padding-top: .8rem; }
"""


def esc(value) -> str:
    return html.escape(str(value))


def html_table(headers: list, rows: list[list], numeric_from: int | None = None) -> str:
    head = "".join(
        f"<th{' class=\"num\"' if numeric_from is not None and i >= numeric_from else ''}>{esc(h)}</th>"
        for i, h in enumerate(headers)
    )
    body = []
    for row in rows:
        cells = "".join(
            f"<td{' class=\"num\"' if numeric_from is not None and i >= numeric_from else ''}>{cell}</td>"
            for i, cell in enumerate(row)
        )
        body.append(f"<tr>{cells}</tr>")
    return f'<div class="scroll"><table><thead><tr>{head}</tr></thead><tbody>{"".join(body)}</tbody></table></div>'


def provenance_bar(estimate: dict) -> str:
    folded = folded_provenance(provenance_totals(estimate))
    total = sum(value for *_, value in folded) or 1
    segments = []
    for _, label, color, value in folded:
        if value <= 0:
            continue
        width = max(value / total * 100, 0.5)
        segments.append(
            f'<span style="width:{width:.2f}%;background:{color}" title="{esc(label)}: {value:,} SLOC"></span>'
        )
    legend = "".join(
        f'<span><i style="background:{color}"></i>{esc(label)} — {value:,} SLOC</span>'
        for _, label, color, value in folded if value > 0
    )
    return (
        f'<div class="bar-track">{"".join(segments)}</div>'
        f'<div class="legend">{legend}</div>'
    )


def hours_bars(entries: list[tuple[str, float, str]]) -> str:
    """Horizontal magnitude bars (one measure, one hue); values labeled."""
    peak = max((value for _, value, _ in entries), default=0) or 1
    rows = []
    for label, value, note in entries:
        width = max(value / peak * 100, 0.6)
        rows.append(
            f'<div class="hbar"><span class="lbl">{esc(label)}</span>'
            f'<span class="trk"><span class="fill" style="width:{width:.2f}%" title="{esc(label)}: {value:,.1f} h"></span></span>'
            f'<span class="val">{value:,.1f} h{esc(note)}</span></div>'
        )
    return "".join(rows)


def render_html(estimate: dict, measures: dict | None, model_config: dict | None) -> str:
    generated = estimate["hamr_generated_value"]
    profiles = sorted(generated["estimates"])
    scenario_names = [scenario["name"] for scenario in estimate["scenarios"]]
    h_lo, h_hi, c_lo, c_hi = replacement_range(estimate)
    comparison = estimate["comparisons"][0]
    session = estimate["session_totals"]
    mh_lo, mh_hi, mc_lo, mc_hi = modeled_human_range(estimate)
    active_h = (session["active_seconds"] or 0) / 3600.0
    parts: list[str] = []
    add = parts.append

    add(f"<h1>Effort report — {esc(estimate['project']['name'])}</h1>")
    add(
        f'<p class="meta">{esc(estimate["generated"][:10])} · workflow {esc(estimate["manifest"].get("workflow"))} '
        f'({esc(estimate["manifest"].get("profile"))} profile) · harness {esc(estimate["manifest"].get("harness"))} · '
        f'<span class="badge">{UNCALIBRATED_LABEL}</span></p>'
    )
    add(
        '<p>Every headline is <span class="tag">Observed</span> (measured), <span class="tag">Modeled</span> '
        "(estimated under a named profile, scenario, and counterfactual — always uncalibrated), or "
        '<span class="tag">Compared</span> (scope-gated).</p>'
    )

    # --- executive summary boxes
    add("<h2>1. Executive summary</h2>")
    add('<div class="box"><h3>HAMR vs Manual <span class="tag">Modeled</span></h3>')
    add(
        f"<p>HAMR generated <strong>{generated['sloc']:,} SLOC</strong> of infrastructure, configuration, and "
        f"assurance artifacts. Hand-building a manual equivalent is modeled at "
        f"<strong>{esc(fmt_hours(h_lo))}–{esc(fmt_hours(h_hi))}</strong> "
        f"(<strong>{esc(fmt_money(c_lo))}–{esc(fmt_money(c_hi))}</strong>) across profiles and equivalence "
        f"scenarios ({esc(', '.join(scenario_names))}). This is a <em>modeled manual-equivalent replacement "
        f"value</em>, never a measured net saving.</p>"
    )
    bar_entries = []
    for profile in profiles:
        for scenario in scenario_names:
            entry = generated["estimates"][profile][scenario]
            bar_entries.append((f"{profile} · {scenario}", entry["hours"], f" · {fmt_money(entry['cost_usd'])}"))
    add(hours_bars(bar_entries))
    add("</div>")

    add('<div class="box"><h3>HAMR-Agent vs HAMR <span class="tag">Compared</span></h3>')
    cost_value = session["cost_usd"]["total"]
    add(
        f"<p>Observed agent session: <strong>{esc(fmt_money(cost_value)) if cost_value is not None else '—'}</strong> "
        f"({esc(session['cost_usd']['kind'] or 'unavailable')}), <strong>{active_h:.2f} active hours</strong>. "
        f"Modeled human effort for the same authored artifacts plus tool-running overheads: "
        f"<strong>{esc(fmt_hours(mh_lo))}–{esc(fmt_hours(mh_hi))}</strong> "
        f"(<strong>{esc(fmt_money(mc_lo))}–{esc(fmt_money(mc_hi))}</strong>) by profile.</p>"
    )
    agent_bars = [("agent session (Observed)", active_h, f" · {fmt_money(cost_value)}" if cost_value is not None else "")]
    for profile, values in comparison["values"]["modeled_human"].items():
        agent_bars.append((f"modeled human · {profile}", values["hours"], f" · {fmt_money(values['cost_usd'])}"))
    add(hours_bars(agent_bars))
    if comparison["ratio"]:
        ratio_rows = []
        for profile, values in comparison["ratio"]["by_profile"].items():
            ratio_rows.append([esc(profile), f"{values['time_ratio']}×", f"{values.get('cost_ratio', '—')}×"])
        add(f"<p>Within-experiment ratio (scope <code>exact</code>, <span class='badge'>{UNCALIBRATED_LABEL}</span>):</p>")
        add(html_table(["Profile", "Time ratio (modeled human ÷ agent active)", "Cost ratio"], ratio_rows, numeric_from=1))
    else:
        add(f"<p>Scope alignment <code>{esc(comparison['scope_alignment'])}</code>: no ratio rendered.</p>")
    add("</div>")
    add(
        f'<p><strong>{UNCALIBRATED_LABEL}.</strong> Modeled numbers use uncalibrated placeholder productivity '
        "rates (§7); no calibrated prediction accuracy is claimed.</p>"
    )
    add("<h3>Measured SLOC by provenance <span class='tag'>Observed</span></h3>")
    add(provenance_bar(estimate))

    # --- per-step table
    add("<h2>2. Per-workflow-step view</h2>")
    add("<p>Per-step agent actuals do not exist in Wave 1 — the session total appears once in §6.</p>")
    step_rows = []
    for step in estimate["steps"]:
        measures_entry = step["measures"]
        prov = measures_entry["by_provenance"]
        gen = sum(prov.get(name, 0) for name in ("hamr_generated", "hamr_woven", "hamr_template_retained"))
        auth = prov.get("developer_authored", 0)
        unk = sum(prov.get(name, 0) for name in UNKNOWN_FOLD)
        hours_cell = " / ".join(esc(fmt_hours(step["human_estimates"][p]["hours"])) for p in sorted(step["human_estimates"]))
        step_rows.append([
            f"<code>{esc(step['step_key'])}</code>{esc(GRANULARITY_MARKS.get(step['granularity'], ''))}",
            esc(step["status"] or "—"),
            esc(", ".join(measures_entry["artifact_ids"]) or (measures_entry.get("activity") or "—")),
            f"{gen:,} / {auth:,} / {unk:,}" if measures_entry["sloc_total"] else "—",
            esc(summarize_step_assurance(step.get("assurance")) or "—"),
            hours_cell,
            "—",
        ])
    add(html_table(
        ["Step", "Status", "Artifacts", "SLOC gen/auth/unk", "Assurance (Observed)",
         f"Modeled hours ({' / '.join(sorted(next(iter(estimate['steps']))['human_estimates']))})", "Agent actuals"],
        step_rows, numeric_from=3,
    ))
    add('<p class="meta">† workflow-only granularity · ‡ inferred step · § unattributed bucket.</p>')

    # --- generation value
    add("<h2>3. HAMR generation value detail <span class='tag'>Modeled</span></h2>")
    gen_rows = []
    for artifact_type, sloc in sorted(generated["by_artifact_type"].items()):
        row = [esc(artifact_type), f"{sloc:,}"]
        for profile in profiles:
            for scenario in scenario_names:
                by_type = generated["estimates"][profile][scenario]["by_type"]
                row.append(esc(fmt_hours(by_type.get(artifact_type, {}).get("hours", 0.0))))
        gen_rows.append(row)
    total_row = ["<strong>Total</strong>", f"<strong>{generated['sloc']:,}</strong>"]
    for profile in profiles:
        for scenario in scenario_names:
            entry = generated["estimates"][profile][scenario]
            total_row.append(f"<strong>{esc(fmt_hours(entry['hours']))} ({esc(fmt_money(entry['cost_usd']))})</strong>")
    gen_rows.append(total_row)
    gen_headers = ["Artifact type", "SLOC"] + [f"{p} · {s}" for p in profiles for s in scenario_names]
    add(html_table(gen_headers, gen_rows, numeric_from=1))
    accounting = generated["template_accounting"]
    add(
        f"<p>Generate-once templates: HAMR supplied <strong>{accounting['supplied']:,}</strong> template SLOC "
        f"in {accounting['files']} files; <strong>{accounting['retained']:,}</strong> retained (counted above), "
        f"<strong>{accounting['replaced']:,}</strong> replaced by the developer/agent (counted as authored work)."
        f" Unknown-provenance bucket: <strong>{estimate['unknown_provenance']['sloc']:,} SLOC</strong>.</p>"
    )
    if measures and measures.get("template_accounting"):
        add(html_table(
            ["Generate-once file", "Supplied", "Retained", "Replaced"],
            [[f"<code>{esc(e['file'])}</code>", f"{e['supplied']:,}", f"{e['retained']:,}", f"{e['replaced']:,}"]
             for e in measures["template_accounting"]], numeric_from=1,
        ))

    # --- common mode
    add("<h2>4. Common-mode artifacts <span class='tag'>Modeled</span></h2>")
    add("<p>Authored in both counterfactuals; never counted toward the HAMR generation-value headline.</p>")
    common = estimate["common_mode"]
    common_rows = []
    for artifact_type, sloc in sorted(common["by_artifact_type"].items()):
        row = [esc(artifact_type), f"{sloc:,}"]
        for profile in sorted(common["estimates"]):
            row.append(esc(fmt_hours(common["estimates"][profile]["by_type"].get(artifact_type, {}).get("hours", 0.0))))
        common_rows.append(row)
    add(html_table(["Artifact type", "SLOC", *[f"Modeled ({p})" for p in sorted(common["estimates"])]], common_rows, numeric_from=1))

    # --- assurance detail (markdown-rendered content reused as simple HTML)
    add("<h2>5. Assurance detail <span class='tag'>Observed</span></h2>")
    if measures:
        assurance = measures["assurance"]
        test_rows = []
        for component, static in sorted(assurance["tests"]["defined_static"].items()):
            runtime = assurance["tests"]["runtime"].get(component, {}).get("value", {})
            reconciliation = assurance["tests"]["reconciliation"].get(component, {})
            test_rows.append([
                esc(component),
                f"{static['total']} ({static['tests_rs']} + {static['cb_apis_rs']})",
                f"{runtime.get('passed', '—')} / {runtime.get('failed', '—')}",
                {True: "yes", False: "<strong>no</strong>", None: "—"}[reconciliation.get("consistent")],
                esc(assurance["coverage"].get(component, {}).get("value", {}).get("entrypoint_lines_percent") or "—"),
                esc(f"{assurance['component_verus'][component]['value'].get('verified', '—')} / "
                    f"{assurance['component_verus'][component]['value'].get('errors', '—')}"),
            ])
        add(html_table(
            ["Component", "Tests defined (static: tests.rs + cb_apis.rs)", "Run passed/failed (notes)",
             "Static = runtime?", "Entrypoint coverage % (notes)", "Verus verified/errors (notes)"],
            test_rows, numeric_from=1,
        ))
        proof = assurance["system_proof"]
        proof_bits = []
        if proof.get("structure"):
            for crate in proof["structure"]["value"]:
                proof_bits.append(
                    f"crate <code>{esc(crate['crate'])}</code>: {crate['property_count']} properties, "
                    f"{crate['rs_files']} files, {crate['total_lines']:,} lines (structure, exact)"
                )
        results = proof["results"]["value"]
        proof_bits.append(
            f"system VCs {results['verified']} verified / {results['errors']} errors (notes, derived)"
            if results.get("verified") is not None else "system VCs: — (step deferred or absent)"
        )
        integration = assurance["integration_check"]["value"]
        vacuous = " — <strong>vacuous by design</strong>" if integration.get("vacuous") else ""
        proof_bits.append(f"integration handshakes N={integration.get('expected_handshakes', '—')}{vacuous} (notes)")
        audit = assurance["audit_findings"]
        if audit.get("present"):
            severities = ", ".join(f"{count} {name}" for name, count in sorted(audit["value"]["by_severity"].items()))
            proof_bits.append(f"assessment findings: {audit['value']['total']} ({esc(severities)}) (assessment-summary.json, exact)")
        else:
            proof_bits.append("assessment findings: not available")
        add("<ul>" + "".join(f"<li>{bit}</li>" for bit in proof_bits) + "</ul>")
    else:
        add("<p><em>project-measures.json not supplied; see the Markdown report.</em></p>")

    # --- session totals
    add("<h2>6. Agent session totals <span class='tag'>Observed</span></h2>")
    tokens = session["tokens"]
    add(html_table(["Metric", "Value"], [
        ["Harness", esc(f"{session['harness'] or '—'} {session['harness_version'] or ''}".strip())],
        ["Models", esc(", ".join(session["models"]) or "—")],
        ["Metrics schema / adapter", esc(session["schema_or_adapter"] or "—")],
        ["Tokens in / out / cache read / cache write",
         f"{tokens['input']:,} / {tokens['output']:,} / {tokens['cache_read']:,} / {tokens['cache_write']:,}"],
        ["Cost", (esc(fmt_money(session['cost_usd']['total'])) if session['cost_usd']['total'] is not None else "—")
         + f" ({esc(session['cost_usd']['kind'] or 'unavailable')})"],
        ["Active / wall-clock seconds", f"{session['active_seconds']:,} / {session['wall_clock_seconds']:,}"
         if session["active_seconds"] is not None else "—"],
        ["Friction: escalations / dynamic shell / sandbox failures",
         f"{session['friction']['escalation_requests']} / {session['friction']['dynamic_shell_commands']} / {session['friction']['sandbox_failures']}"],
    ]))
    add("<ul class='caveats'>" + "".join(f"<li>{esc(c)}</li>" for c in session["caveats"]) + "</ul>")

    # --- assumptions
    add("<h2>7. Assumptions and data quality</h2>")
    add(f"<p>Estimator <code>{esc(estimate['estimator']['model'])}</code> — calibration status "
        f"<strong>{esc(estimate['estimator']['calibration_status'])}</strong>. {esc(estimate['estimator']['scope_note'])}</p>")
    if model_config:
        add(html_table(
            ["Artifact type", "SLOC/hour (placeholder)", "Rationale"],
            [[esc(t), esc(str(e["sloc_per_hour"]) + (f" (fixed {e['fixed_hours_override']} h override)" if e.get("fixed_hours_override") is not None else "")), esc(e["rationale"])]
             for t, e in sorted(model_config["rates"].items())], numeric_from=1,
        ))
    add("<p>Manual-path effort deliberately not modeled (the honest asymmetry):</p>")
    add("<ul>" + "".join(f"<li>{esc(item)}</li>" for item in estimate["unmodeled_manual"]) + "</ul>")
    add("<p>Data-quality caveats:</p>")
    add("<ul class='caveats'>" + "".join(f"<li>{esc(c)}</li>" for c in estimate["caveats"]) + "</ul>")
    add(
        f'<div class="footer">Attribution coverage {estimate["attribution_summary"]["coverage"]:.1%} · '
        f'effort-model SHA <code>{esc(estimate["estimator"]["config_sha256"][:12])}…</code> · '
        f'counterfactual SHA <code>{esc(estimate["counterfactual"]["config_sha256"][:12])}…</code> · '
        f'generated {esc(estimate["generated"])}</div>'
    )

    body = "\n".join(parts)
    title = f"Effort report — {esc(estimate['project']['name'])}"
    return (
        "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n"
        f"<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n<title>{title}</title>\n"
        f"<style>{CSS}</style>\n</head>\n<body>\n{body}\n</body>\n</html>\n"
    )


# --------------------------------------------------------------------------
# Comparison page (§10.3)
# --------------------------------------------------------------------------

COMPARABLE_WORKFLOWS = [
    "SysPlanAndReq", "SysModeling", "CompGUMBOSpec", "SysGUMBOIntegrationCheck",
    "CodeGen", "CompDev", "SysSchedDef", "SysGUMBOSysSpecCheck",
]

_STEP_ORDER = [
    r"^SysPlanAndReq\.1$", r"^SysPlanAndReq\.2$", r"^SysPlanAndReq\.3$", r"^SysPlanAndReq\.4$",
    r"^SysModeling", r"^CompGUMBOSpec", r"^SysGUMBOIntegrationCheck",
    r"^CodeGen\.2$", r"^CodeGen\.4$", r"^CompDev\([^)]*\)\.2$", r"^CompDev\([^)]*\)\.3$",
    r"^CompDev\([^)]*\)\.4$", r"^CompDev\([^)]*\)\.6$", r"^SysSchedDef",
    r"^SysGUMBOSysSpecCheck$", r"^SysGUMBOSysSpecCheck\.4$", r"^SysGUMBOSysSpecCheck\.5$",
]


def step_sort_key(step_key: str) -> tuple[int, str]:
    import re as _re
    for rank, pattern in enumerate(_STEP_ORDER):
        if _re.match(pattern, step_key):
            return (rank, step_key)
    if step_key == "unattributed":
        return (len(_STEP_ORDER) + 1, step_key)
    return (len(_STEP_ORDER), step_key)


def workflow_of(step_key: str) -> str:
    import re as _re
    match = _re.match(r"^([A-Za-z]+)", step_key)
    return match.group(1) if match else step_key


def collapse_step_key(step_key: str) -> str:
    import re as _re
    return _re.sub(r"\(([^)*]+)\)", "(*)", step_key)


def render_compare_markdown(estimates: list[dict], date: str) -> str:
    names = [estimate["project"]["name"] for estimate in estimates]
    lines = [f"# Effort comparison — {' vs '.join(names)} ({date})", ""]
    lines.append(
        f"**{UNCALIBRATED_LABEL}** — modeled values use uncalibrated placeholder rates. Cross-experiment "
        "comparisons are side-by-side only over the commonly completed workflow subset; **no cross-experiment "
        "ratios are rendered** (scopes and caveat structures differ). The only ratio permitted anywhere is each "
        "experiment's own within-experiment modeled-vs-observed juxtaposition, shown in its per-experiment report."
    )
    lines.append("")

    # completed sets and common subset
    completed = {
        estimate["project"]["name"]: set(estimate.get("workflow_completion", {}).get("completed", []))
        for estimate in estimates
    }
    common = set.intersection(*completed.values()) if completed else set()
    common &= set(COMPARABLE_WORKFLOWS)

    lines.append("## Session totals (Observed, once per experiment)")
    lines.append("")
    rows = []
    for estimate in estimates:
        session = estimate["session_totals"]
        tokens = session["tokens"]
        rows.append([
            estimate["project"]["name"],
            f"{session['harness'] or '—'} {session['harness_version'] or ''}".strip(),
            ", ".join(session["models"]) or "—",
            session["schema_or_adapter"] or "—",
            fmt_int(tokens["output"]),
            (fmt_money(session["cost_usd"]["total"]) if session["cost_usd"]["total"] is not None else "—")
            + f" ({session['cost_usd']['kind'] or '—'})",
            fmt_int(session["active_seconds"]),
        ])
    lines.append(md_table(
        ["Experiment", "Harness", "Models", "Metrics schema", "Output tokens", "Cost", "Active s"], rows))
    lines.append("")
    lines.append(
        "Caveats are asymmetric across harnesses (Claude metrics exclude subagents; Codex costs are "
        "API-list-price equivalents with undercounted friction) — see each per-experiment report §6."
    )
    lines.append("")

    lines.append("## Commonly completed workflow subset")
    lines.append("")
    lines.append(f"Common subset: {', '.join(sorted(common, key=COMPARABLE_WORKFLOWS.index)) if common else '(empty)'}.")
    scope_rows = []
    for workflow in COMPARABLE_WORKFLOWS:
        statuses = [
            estimate.get("workflow_completion", {}).get("workflows", {}).get(workflow, "—")
            for estimate in estimates
        ]
        in_common = workflow in common
        scope_rows.append([
            workflow,
            *statuses,
            "compared" if in_common else "**not comparable (scope)** — " + ", ".join(
                f"{name}: {status}" for name, status in zip(names, statuses) if status != "done"
            ),
        ])
    lines.append("")
    lines.append(md_table(["Workflow", *names, "Comparison"], scope_rows))
    lines.append("")

    lines.append("## Per-step side-by-side (common subset; authored SLOC and modeled hours)")
    lines.append("")
    lines.append(
        "Component-parameterized steps are collapsed to `(*)` totals because component names differ "
        "across experiments. Modeled hours shown for the `experienced_sel4` profile; per-step agent "
        "actuals do not exist in Wave 1."
    )
    lines.append("")
    collapsed: dict[str, dict[str, dict]] = {}
    for estimate in estimates:
        name = estimate["project"]["name"]
        for step in estimate["steps"]:
            if workflow_of(step["step_key"]) not in common:
                continue
            key = collapse_step_key(step["step_key"])
            slot = collapsed.setdefault(key, {}).setdefault(name, {"sloc": 0, "auth": 0, "hours": 0.0, "assurance": []})
            prov = step["measures"]["by_provenance"]
            slot["sloc"] += step["measures"]["sloc_total"]
            slot["auth"] += prov.get("developer_authored", 0)
            profile = "experienced_sel4" if "experienced_sel4" in step["human_estimates"] else sorted(step["human_estimates"])[0]
            slot["hours"] += step["human_estimates"][profile]["hours"]
            summary = summarize_step_assurance(step.get("assurance"))
            if summary:
                slot["assurance"].append(summary)
    rows = []
    for key in sorted(collapsed, key=step_sort_key):
        row = [f"`{key}`"]
        for name in names:
            slot = collapsed[key].get(name)
            if slot is None:
                row.append("—")
            else:
                cell = f"auth {fmt_int(slot['auth'])} SLOC · {fmt_hours(round(slot['hours'], 2))}"
                if slot["assurance"]:
                    cell += f" · {'; '.join(slot['assurance'])}"
                row.append(cell)
        rows.append(row)
    lines.append(md_table(["Step", *names], rows))
    lines.append("")

    lines.append("## Headline values per experiment (no cross-experiment ratios)")
    lines.append("")
    rows = []
    for estimate in estimates:
        h_lo, h_hi, c_lo, c_hi = replacement_range(estimate)
        mh_lo, mh_hi, _, _ = modeled_human_range(estimate)
        comparison = estimate["comparisons"][0]
        own_ratio = "—"
        if comparison["ratio"]:
            fragments = [
                f"{values['time_ratio']}× ({profile})"
                for profile, values in comparison["ratio"]["by_profile"].items()
            ]
            own_ratio = "; ".join(fragments) + f" ({UNCALIBRATED_LABEL})"
        rows.append([
            estimate["project"]["name"],
            fmt_int(estimate["hamr_generated_value"]["sloc"]),
            f"{fmt_hours(h_lo)}–{fmt_hours(h_hi)}",
            fmt_int(estimate["authored_hamr_path"]["sloc"] + estimate["common_mode"]["sloc"]),
            f"{fmt_hours(mh_lo)}–{fmt_hours(mh_hi)}",
            fmt_int(estimate["unknown_provenance"]["sloc"]),
            own_ratio,
        ])
    lines.append(md_table(
        ["Experiment", "Generated SLOC", "Replacement value (Modeled)", "Authored SLOC",
         "Modeled human (authored)", "Unknown SLOC", "Own within-experiment time ratio"],
        rows,
    ))
    lines.append("")

    # replicate variance note for same-harness pairs
    by_harness: dict[str, list[dict]] = {}
    for estimate in estimates:
        by_harness.setdefault(estimate["manifest"].get("harness", "?"), []).append(estimate)
    for harness, group in sorted(by_harness.items()):
        if len(group) < 2:
            continue
        lines.append(f"## Same-harness replicate variance ({harness})")
        lines.append("")
        rows = []
        for estimate in group:
            vcs = None
            properties = None
            for step in estimate["steps"]:
                assurance = step.get("assurance") or {}
                if "system_proof_results" in assurance:
                    value = assurance["system_proof_results"]["value"]
                    if value.get("verified") is not None:
                        vcs = value["verified"]
                if "system_proof_structure" in assurance:
                    crates = assurance["system_proof_structure"]["value"]
                    if crates:
                        properties = crates[0]["property_count"]
            tests = sum(
                (step.get("assurance") or {}).get("tests_runtime", {}).get("value", {}).get("passed") or 0
                for step in estimate["steps"]
            )
            proof = "—" if vcs is None else f"{vcs} VCs" + (f" ({properties} properties)" if properties else "")
            rows.append([
                estimate["project"]["name"],
                fmt_int(estimate["authored_hamr_path"]["sloc"]),
                fmt_int(tests),
                proof,
            ])
        lines.append(md_table(
            ["Experiment", "HAMR-path authored SLOC (excl. common-mode)", "Tests passed (notes)", "System proof"], rows))
        lines.append("")
        lines.append(
            "Same concept, same harness: differences above are replicate variance (model/agent "
            "nondeterminism and design freedom), not tooling differences."
        )
        lines.append("")
    return "\n".join(lines)


def render_compare_html(estimates: list[dict], date: str, markdown: str) -> str:
    """Comparison HTML: session totals + headline tables with SLOC bars."""
    names = [estimate["project"]["name"] for estimate in estimates]
    parts: list[str] = []
    add = parts.append
    add(f"<h1>Effort comparison — {esc(' vs '.join(names))}</h1>")
    add(f'<p class="meta">{esc(date)} · <span class="badge">{UNCALIBRATED_LABEL}</span> · side-by-side only; '
        "no cross-experiment ratios.</p>")
    add("<h2>Session totals (Observed)</h2>")
    rows = []
    for estimate in estimates:
        session = estimate["session_totals"]
        tokens = session["tokens"]
        rows.append([
            esc(estimate["project"]["name"]),
            esc(f"{session['harness'] or '—'} {session['harness_version'] or ''}".strip()),
            esc(", ".join(session["models"]) or "—"),
            esc(session["schema_or_adapter"] or "—"),
            f"{tokens['output']:,}",
            (esc(fmt_money(session["cost_usd"]["total"])) if session["cost_usd"]["total"] is not None else "—")
            + f" ({esc(session['cost_usd']['kind'] or '—')})",
            f"{session['active_seconds']:,}" if session["active_seconds"] is not None else "—",
        ])
    add(html_table(["Experiment", "Harness", "Models", "Schema", "Output tokens", "Cost", "Active s"], rows, numeric_from=4))
    add("<h2>Measured SLOC by provenance</h2>")
    for estimate in estimates:
        add(f"<h3>{esc(estimate['project']['name'])}</h3>")
        add(provenance_bar(estimate))
    add("<h2>Full comparison</h2>")
    add("<p>The complete scope-gated per-step comparison is in the Markdown page "
        "(same basename, <code>.md</code>); it is the canonical rendering of this comparison.</p>")
    _ = markdown
    body = "\n".join(parts)
    return (
        "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n"
        f"<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n"
        f"<title>Effort comparison — {esc(', '.join(names))}</title>\n<style>{CSS}</style>\n</head>\n<body>\n{body}\n</body>\n</html>\n"
    )


# --------------------------------------------------------------------------
# CLI
# --------------------------------------------------------------------------

def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--estimate", help="effort-estimate.json (per-experiment mode)")
    parser.add_argument("--measures", help="project-measures.json (assurance/template detail)")
    parser.add_argument("--config", default=str(DEFAULT_MODEL_CONFIG), help="effort model config (rate table display)")
    parser.add_argument("--compare", nargs="+", help="two or more effort-estimate.json paths (comparison mode)")
    parser.add_argument("--out", help="output path (.md; .html written alongside). Per-experiment default: EFFORT-<project>-<date>.md next to the estimate")
    args = parser.parse_args(argv)

    if args.compare:
        estimates = [json.loads(Path(path).read_text(encoding="utf-8")) for path in args.compare]
        date = max(estimate["generated"][:10] for estimate in estimates)
        markdown = render_compare_markdown(estimates, date)
        out_md = Path(args.out) if args.out else Path(f"EFFORT-COMPARE-{date}.md")
        out_md.parent.mkdir(parents=True, exist_ok=True)
        out_md.write_text(markdown, encoding="utf-8")
        out_html = out_md.with_suffix(".html")
        out_html.write_text(render_compare_html(estimates, date, markdown), encoding="utf-8")
        print(f"wrote {out_md.resolve()}")
        print(f"wrote {out_html.resolve()}")
        return 0

    if not args.estimate:
        print("error: pass --estimate (per-experiment) or --compare (comparison page)", file=sys.stderr)
        return 2
    estimate_path = Path(args.estimate)
    estimate = json.loads(estimate_path.read_text(encoding="utf-8"))
    measures = json.loads(Path(args.measures).read_text(encoding="utf-8")) if args.measures else None
    model_config = None
    config_path = Path(args.config)
    if config_path.is_file():
        model_config = json.loads(config_path.read_text(encoding="utf-8"))
        actual_sha = hashlib.sha256(config_path.read_bytes()).hexdigest()
        if actual_sha != estimate["estimator"]["config_sha256"]:
            print("warning: model config on disk differs from the config used for this estimate", file=sys.stderr)
    markdown = render_markdown(estimate, measures, model_config)
    name = estimate["project"]["name"]
    date = estimate["generated"][:10]
    out_md = Path(args.out) if args.out else estimate_path.parent / f"EFFORT-{name}-{date}.md"
    out_md.parent.mkdir(parents=True, exist_ok=True)
    out_md.write_text(markdown, encoding="utf-8")
    out_html = out_md.with_suffix(".html")
    out_html.write_text(render_html(estimate, measures, model_config), encoding="utf-8")
    print(f"wrote {out_md.resolve()}")
    print(f"wrote {out_html.resolve()}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
