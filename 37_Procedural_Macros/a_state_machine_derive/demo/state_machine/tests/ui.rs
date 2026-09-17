//! Every refusal the derive makes, compiled and compared with the recorded
//! message beside it: `tests/ui/<name>.stderr`.

#[test]
fn refusals() {
    trybuild::TestCases::new().compile_fail("tests/ui/*.rs");
}
