//! One failing input, held to a `.stderr` file recorded before the derive's
//! message was reworded.

#[test]
fn ui_stale() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui_stale/fail_enum.rs");
}
