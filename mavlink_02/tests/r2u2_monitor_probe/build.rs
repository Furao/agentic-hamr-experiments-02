use std::{env, fs, path::PathBuf};
fn main() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("../..");
    let generated = root.join("hamr/microkit/crates/seL4_MAVLinkFirewall_MAVLinkFirewall/src/component");
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    for name in ["r2u2_monitor.rs", "spec.bin"] {
        let source = generated.join(name);
        println!("cargo:rerun-if-changed={}", source.display());
        fs::copy(source, out.join(name)).unwrap();
    }
}
