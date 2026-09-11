---
name: refactor-verified-code
description: Improve readability and maintainability of executable Verus/Rust code using meaningful constants and field getters, informed by supplied domain references, while preserving existing specifications and proofs. Use for implementation readability refactors, not specification changes or policy redesign.
---

# Refactor verified executable code

Refactor the requested crate, component, or files for readability without changing behavior or existing Verus specification code. Infer the target and reference material from the request and conversation; ask only if the target cannot be determined.

## Establish the boundary

- Read applicable repository instructions and the target's executable code, specifications, callers, tests, and generator ownership markers.
- Read the supplied protocol guides, schemas, XML definitions, or other domain references. Use their field names, layouts, widths, and terminology to inform names and comments. Prefer bundled references when supplied.
- Capture the pre-edit state of protected code, including uncommitted work. Preserve existing `spec fn` definitions, proof functions and blocks, contracts (`requires`, `ensures`, etc.), loop invariants, decreases clauses, and verification attributes verbatim. Avoid formatters that rewrite these regions or recursively format modules containing them.
- Preserve validation order, accepted/rejected inputs, diagnostic reasons, bounds behavior, and public APIs where practical. Keep application policy in its owning component.
- If the reference reveals a behavioral discrepancy, report it separately. A readability refactor does not authorize correcting protocol behavior or weakening the proof.

## Refactor the implementation

- Replace meaningful wire-format literals with descriptive constants: magic bytes, flags, header lengths, field offsets, trailer sizes, and CRC parameters. Derive related sizes where this makes their relationship clearer.
- Keep ordinary arithmetic literals when naming them would obscure the operation. Leave generated numeric lookup data as data; do not introduce hundreds of aliases solely to eliminate literals.
- Reuse existing constants and preserve existing exports. Constants consumed by verified executable code must be visible to Verus, typically through a `verus!` block.
- For whole-byte shifts, use explicitly valued constants such as `ONE_BYTE_SHIFT: u32 = 8`, `TWO_BYTE_SHIFT: u32 = 16`, and `THREE_BYTE_SHIFT: u32 = 24`. Replace shift operands such as `BITS_PER_BYTE`, `2 * BITS_PER_BYTE`, and `3 * BITS_PER_BYTE` with the matching constant; add further widths only when used. Do not express these shift constants as multiplications.
- Keep byte-shift constants local to the target crate/component instead of importing them from another crate solely for shifting. Reuse local definitions and have affected tests use them too; use explicit imports when glob imports would make ownership ambiguous. This local-ownership rule is specific to byte shifts, not a requirement to duplicate all protocol constants. Preserve old constants if protected specifications or existing public exports still depend on them.
- Apply byte-shift changes only to executable regions authorized by this skill. Do not edit protected specifications to align their spelling. Constant shift arithmetic is normally folded by the compiler; describe the change as explicit naming, not a demonstrated runtime speedup without measurement.
- Extract small field getters where they make reads and checks clearer, such as payload length, message ID, incompatibility flags, or checksum. Make offset origins, byte order, and caller bounds requirements clear.
- New executable helpers may have the minimal new contracts needed to verify them against existing specifications. Do not change existing contracts, add trust boundaries, or introduce assumptions. If the user also prohibits adding contracts, use refactorings that work within that stricter limit.
- Use descriptive local variables and tuple destructuring instead of opaque names or positional metadata access. Separate framing, metadata lookup, length validation, and checksum validation into readable sections without adding unnecessary abstractions.
- Update affected test fixture builders to use named fields and constants. Retain meaningful boundary and rejection cases; avoid tests that only mirror helper implementations.
- Respect generator ownership. When generator cleanup is relevant, update its executable code and confirm generated output remains unchanged. Do not rewrite generated specifications.

## Validate and report

- Compare protected regions with the captured pre-edit state, not only with Git HEAD. Confirm existing specification and proof code is unchanged.
- Run the affected crate tests and Verus verification, plus checks for direct consumers when the refactor crosses crate boundaries. Use the repository's established commands and toolchain. Do not run unrelated workflows solely because they are available.
- Review the diff for behavior changes, removed coverage, stale references, unintended generated edits, and formatting churn. Resolve failures within the executable-code boundary; report a blocker if fixing it requires prohibited specification changes.
- Summarize the readability changes, specification preservation, tests and verification results, and any material validation limits.

Example invocation:

```text
$refactor-verified-code hamr/microkit/crates/mavlink_core
Use action-requests/CR-01-add-mavlink-firewall/mavlink_spec/ as context.
Do not modify existing Verus spec code.
```

The example paths are invocation arguments, not defaults for other projects.
