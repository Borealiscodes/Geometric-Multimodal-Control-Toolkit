use gmc::worlds::SoftWorld;

#[test]
fn soft_world_holonomy_defined() {
    let w = SoftWorld::new();
    let h = w.holonomy_loop();
    assert!(h.is_finite());
}
