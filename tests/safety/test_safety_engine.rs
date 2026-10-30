use gmc::safety::engine::SafetyEngine;

#[test]
fn safety_engine_pipeline_is_stable() {
    let engine = SafetyEngine::new();

    let z_prev = vec![0.0, 0.1, 0.2, 0.3];
    let z_next = vec![0.3, 0.2, 0.1, 0.0];

    let result = engine.step(&z_prev, &z_next);

    assert!(result.is_some());
}

#[test]
fn safety_engine_rejects_world_rule_violation() {
    let engine = SafetyEngine::new();

    // Force misalignment
    let z_prev = vec![1.0, 1.0, 1.0];
    let z_next = vec![-1.0, -1.0, -1.0];

    let result = engine.step(&z_prev, &z_next);

    assert!(result.is_none());
}
