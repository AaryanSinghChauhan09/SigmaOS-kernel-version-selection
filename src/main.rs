//! SigmaOS-kernel-version-selection — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_kernel_version_selection_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-kernel-version-selection: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-kernel-version-selection: ok");
}
