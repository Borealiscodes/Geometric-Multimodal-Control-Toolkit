use gmc::operators::RoutingOperator;

#[test]
fn routing_operator_runs() {
    let op = RoutingOperator::new();
    let _ = op.route(&[0.0]);
}
