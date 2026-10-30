use gmc::geometry::curvature;

#[test]
fn curvature_is_defined() {
    let c = curvature(&[0.0]);
    assert!(c.is_finite());
}
