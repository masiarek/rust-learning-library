//! Compiles the table with every mistake in it, and fails if the errors differ
//! from the copy recorded beside it in `main.stderr`.

#[test]
fn rejected_routes() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("../routes_mistakes/src/main.rs");
}
