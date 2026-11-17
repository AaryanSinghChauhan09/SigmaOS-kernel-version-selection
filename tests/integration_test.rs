//! Integration tests for SigmaOS-kernel-version-selection.

use sigmaos_kernel_version_selection_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
