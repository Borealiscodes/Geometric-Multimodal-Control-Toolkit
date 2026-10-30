use gmc::worlds::alignment_score;

#[test]
fn alignment_score_defined() {
    let score = alignment_score(&[0.0], &[1.0]);
    assert!(score.is_finite());
}
