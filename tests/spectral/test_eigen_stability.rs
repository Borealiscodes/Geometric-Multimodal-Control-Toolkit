use gmc::spectral::eigenvalues;

#[test]
fn eigenvalues_are_positive() {
    let eigs = eigenvalues(&vec![vec![1.0]]);
    assert!(eigs.iter().all(|e| *e > 0.0));
}
