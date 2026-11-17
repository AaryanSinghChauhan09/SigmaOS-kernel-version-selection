// Integration test for SigmaOS-kernel-version-selection
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use kernel_version_selection as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
