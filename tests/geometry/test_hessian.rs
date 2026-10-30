use gmc::geometry::compute_hessian;

#[test]
fn hessian_is_defined() {
    let h = compute_hessian(&[0.0]);
    assert!(!h.is_empty());
}
