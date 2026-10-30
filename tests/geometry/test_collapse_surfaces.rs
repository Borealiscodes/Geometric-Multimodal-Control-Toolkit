use gmc::geometry::{compute_hessian, collapse_detected};

#[test]
fn collapse_surface_check_runs() {
    let h = compute_hessian(&[0.0]);
    let c = collapse_detected(&h);
    assert!(!c);
}
