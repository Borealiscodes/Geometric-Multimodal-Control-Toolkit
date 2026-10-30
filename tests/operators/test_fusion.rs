use gmc::operators::FusionOperator;

#[test]
fn fusion_operator_is_stable() {
    let op = FusionOperator::new();
    let _ = op.apply(&[0.0], &[1.0]);
}
