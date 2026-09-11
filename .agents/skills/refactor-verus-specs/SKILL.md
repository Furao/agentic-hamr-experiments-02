---
name: refactor-verus-specs
description: Improve readability and maintainability of Verus specifications and ghost proofs using meaningful names, specification field getters, and equivalent predicates, informed by supplied domain references, while preserving executable code and specification meaning. Use for specification readability refactors, not implementation changes or requirement redesign.
---

# Refactor Verus specifications

Refactor the requested crate, component, or files so their Verus specifications are easier to read and maintain. Preserve specification meaning and leave executable code alone. Infer the target and reference material from the request and conversation; ask only if the target cannot be determined.

## Establish the boundary

- Read applicable repository instructions, specifications, proof dependencies, executable callers, tests, and generator ownership markers.
- Read supplied protocol guides, schemas, XML definitions, or other domain references. Use their terminology, field layouts, widths, and units to inform names. Prefer bundled references when supplied.
- Capture the pre-edit state, including uncommitted work. Protect executable function bodies and signatures, runtime types and layouts, existing executable constants, runtime imports, test implementations, build configuration, and generator executable code from edits. Avoid formatters that rewrite protected regions or recursively format unrelated modules.
- A `verus!` block can contain executable code: do not treat the entire block as specification code. Editable regions include `spec fn` definitions, contracts, invariants, and ghost-only proof functions/blocks. Existing executable signatures stay fixed even when their attached contracts are refactored. Within mixed functions, change only specification or ghost regions.
- Keep existing verification modes, trust boundaries, public specification interfaces, and unfolding visibility stable. If a proposed refactor requires changing executable code or externally used specification interfaces, choose another approach or report that limitation.
- Domain references supply naming context, not authorization to change requirements. Report discovered specification discrepancies separately rather than correcting them during a readability refactor.

## Refactor the specifications

- Reuse existing named constants without changing their definitions. For additional names, prefer ghost-only constant-valued specification functions or local specification bindings so executable declarations remain untouched.
- For whole-byte shifts, use names representing explicit bit counts: `ONE_BYTE_SHIFT` = 8, `TWO_BYTE_SHIFT` = 16, and `THREE_BYTE_SHIFT` = 24, rather than `BITS_PER_BYTE` or multiplication in shift operands. Reuse suitable constants owned by the target crate/component; avoid importing another crate's shift constants solely for this purpose.
- If suitable local constants are absent or inaccessible from an open specification, introduce ghost-only constant-valued functions or local specification bindings with explicit values and the appropriate shift type. Do not add, redefine, expose, or rename executable constants or runtime imports in this specification-only workflow. Apply the same approach to further byte widths only when used, and preserve shift types and semantics.
- Specification expressions are ghost code: do not claim runtime multiplication savings from this cleanup. Leave corresponding executable shifts unchanged unless the user separately authorizes executable edits.
- Extract small `spec fn` field getters and predicates to clarify message IDs, offsets, lengths, flags, byte order, and policy conditions. Give units and offset origins where ambiguity is possible.
- Use descriptive bindings and tuple destructuring instead of opaque variable names or positional metadata access. Group related conditions by meaning without gratuitous abstraction.
- Preserve arithmetic types and overflow semantics, casts, quantifier domains, bounds guards, implication direction, and behavior outside valid-input domains. Moving guards or factoring expressions must not silently change a total specification's meaning.
- Preserve both sides of the contract: neither strengthen preconditions nor weaken postconditions to make verification pass. Preserve exact allowed-input and promised-output sets, including invalid and boundary cases.
- Keep policy in the component that owns it. Leave generated numeric lookup tables as data; do not create hundreds of aliases solely to remove literals.
- Adjust ghost proofs only as needed to support equivalent specification refactoring. Do not add assumptions, axioms, external bodies, admitted obligations, disabled verification, or new trusted specifications. Preserve termination and invariant obligations.
- Do not edit generated specifications in place. If a durable change requires modifying generator executable code or model artifacts outside the requested scope, leave that portion unchanged and explain the limitation.

## Validate equivalence and preservation

- Compare executable regions against the captured pre-edit state, not only Git HEAD. Confirm that executable bodies, signatures, declarations, and test code remain verbatim unchanged. Account explicitly for ghost regions embedded in executable functions; whole-file hashes alone cannot establish this boundary.
- Establish semantic equivalence independently of implementation verification. An implementation passing a weakened contract is not evidence of equivalence. For straightforward substitutions, inspect definitions and types; for nontrivial predicate or contract restructuring, use verified ghost equivalence lemmas against the captured original formulas over their original domains, including boundary and invalid-input cases.
- Run affected Verus verification and checks for dependent proofs when shared specifications change. Run existing affected tests or compilation checks when needed to confirm ghost erasure and macro compatibility; do not rewrite runtime tests or add runtime-only helpers for this task.
- Review the diff for changed requirements, new trust, executable edits, stale references, and formatting churn. Resolve failures within the specification-only boundary; do not patch executable code to accommodate a changed specification.
- Summarize the readability changes, evidence of semantic equivalence and executable preservation, verification/test results, and any material validation limits. Distinguish equivalence established by inspection from equivalence mechanically proved.

Example invocation:

```text
$refactor-verus-specs hamr/microkit/crates/mavlink_core
Use action-requests/CR-01-add-mavlink-firewall/mavlink_spec/ as context.
Leave executable code unchanged.
```

The example paths are invocation arguments, not defaults for other projects.
