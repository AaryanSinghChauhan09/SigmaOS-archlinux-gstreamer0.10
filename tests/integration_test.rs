//! Integration tests for SigmaOS-archlinux-gstreamer0.10.

use sigmaos_archlinux_gstreamer0.10_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
