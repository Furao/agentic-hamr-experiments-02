# MAVLink core consolidation

Date: 2026-09-11

Requested directly by the developer, with follow-up direction to keep firmware-flash policy in the firewall component.

## Final design

- `mavlink_core::parse(&[u8], offset, length)` is the single verified parser, returning `Result<Message, InvalidReason>`. Its postconditions establish frame validity, message ID, payload offset and length, and payload bounds. The core contains no firmware-flash predicate or routing classification.
- `seL4_MAVLinkFirewall_MAVLinkFirewall` owns `classify_mavlink` and `firmware_flash_spec`. Routing and the runtime flash predicate share this verified policy implementation. The component proof connects the parsed fields to the existing flash-command specification.
- Runtime frame checks and rejection logging use the same core parser. Existing diagnostic strings and rejection ordering are retained.
- Removed the duplicate ordinary parser, payload readers, duplicate CRC implementation, and `src/dialect.rs`. The policy-bearing core APIs `classify` and `classify_detailed` are removed. All repository callers are migrated.
- The generator emits only `dialect_verified.rs`; its executable metadata lookup and specification remain unchanged.
- Core regressions cover framing, diagnostics, and parsed field locations. Firmware-flash regressions reside in the component and cover v1/v2, both command layouts, signed frames, truncated fields, checksum rejection, and carrier offsets.

## Validation

- `cargo test --manifest-path hamr/microkit/crates/mavlink_core/Cargo.toml`: 6 passed.
- `RUSTC_BOOTSTRAP=1 cargo test --manifest-path hamr/microkit/crates/seL4_MAVLinkFirewall_MAVLinkFirewall/Cargo.toml`: 8 passed. Existing generated crates require bootstrap for feature attributes on the installed stable compiler.
- `cargo-verus verify --manifest-path hamr/microkit/crates/mavlink_core/Cargo.toml`: 9 verified, 0 errors.
- `make verus` in the MAVLinkFirewall crate, targeting aarch64-unknown-none: core 9 verified, component 17 verified, 0 errors.
- During consolidation, ran `tools/generate_dialect.py`: retained metadata file unchanged, deleted table not recreated.
- No remaining core policy API references in crate sources; `git diff --check` passed.

Validation covers host tests and target verification. Hardware execution, full system build, and coverage measurement were not rerun. Diagnostic categories are regression-tested; parser postconditions establish validity and parsed fields, not each individual diagnostic category.
