use gmc::operators::CollapseGuard;

#[test]
fn collapse_guard_runs() {
    let guard = CollapseGuard::new();
    assert!(guard.check(&[0.0]));
}
