use gmc::integration::integration_check;

#[test]
fn full_runtime_pipeline_is_stable() {
    let z1 = vec![0.0, 0.1, 0.2, 0.3];
    let z2 = vec![0.3, 0.2, 0.1, 0.0];

    let ok = integration_check(&z1, &z2);
    assert!(ok);
}
