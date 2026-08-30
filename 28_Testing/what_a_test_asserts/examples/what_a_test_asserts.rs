//! What a failing assertion actually tells you, and the one that tells you nothing.
//!
//!   rustc --edition 2024 what_a_test_asserts.rs -o /tmp/wta && /tmp/wta

/// Runs `f`, and returns the panic message instead of dying.
fn message_from(f: impl FnOnce() + std::panic::UnwindSafe) -> String {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let result = std::panic::catch_unwind(f);
    std::panic::set_hook(hook);
    match result {
        Ok(()) => "(did not panic)".to_string(),
        Err(e) => e
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| e.downcast_ref::<&str>().map(|s| (*s).to_string()))
            .unwrap_or_else(|| "(non-string panic)".to_string()),
    }
}

fn tally(scores: &[u32]) -> u32 {
    scores.iter().sum()
}

fn main() {
    println!("1. A test is a function that panics when it is unhappy");
    println!("   assert!(cond)          panics if cond is false");
    println!("   assert_eq!(a, b)       panics if a != b, and PRINTS BOTH");
    println!("   assert_ne!(a, b)       panics if a == b");
    println!("   That is the whole mechanism: #[test] marks a function, the");
    println!("   harness runs it, and a panic is a failure. There is no assertion");
    println!("   library to learn and no `expect(x).to.equal(y)` grammar.");

    println!();
    println!("2. The difference between the two, on the same wrong answer");
    let bad = message_from(|| assert!(tally(&[5, 3, 0]) == 9));
    println!("   assert!(tally(..) == 9):");
    for line in bad.lines() {
        println!("     {line}");
    }
    let good = message_from(|| assert_eq!(tally(&[5, 3, 0]), 9));
    println!("   assert_eq!(tally(..), 9):");
    for line in good.lines() {
        println!("     {line}");
    }
    println!("   The first tells you a condition was false, which you knew from");
    println!("   the line number. The second tells you the answer was 8. Reach for");
    println!("   assert_eq! whenever both sides are values.");

    println!();
    println!("3. The message, for when the values are not enough");
    let named = message_from(|| {
        let seats = 3;
        assert!(seats <= 2, "seats = {seats}, but this election has 2 to fill");
    });
    println!("   {named}");
    println!("   Everything after the condition is a format! call, run only on");
    println!("   failure. Use it when the values alone do not say what went wrong.");

    println!();
    println!("4. The assertion that passes for the wrong reason");
    println!("   assert!(tally(&[]) == 0)      passes: {}", tally(&[]) == 0);
    println!("   assert_eq!(tally(&[]), 0)     passes too");
    println!("   Both are green, and neither has tested anything: an empty input");
    println!("   summing to zero is what `0` means, not what `tally` does. A test");
    println!("   whose expected value is a type's default is worth re-reading —");
    println!("   it is the shape a test takes when it was written to pass.");

    println!();
    println!("5. Floats, where == is the wrong assertion entirely");
    let share = 1.0_f64 / 3.0;
    let recombined = share * 3.0;
    println!("   (1.0 / 3.0) * 3.0 == 1.0 is {}", recombined == 1.0);
    let third = 0.1_f64 + 0.2;
    println!("   0.1 + 0.2 == 0.3 is {}", third == 0.3);
    println!("   assert_eq! on floats tests bit equality, which is almost never");
    println!("   what you meant. Compare against a tolerance:");
    println!("   (0.1 + 0.2 - 0.3).abs() < 1e-10 is {}", (third - 0.3).abs() < 1e-10);
    println!("   Note the first line: this one happens to be true, and the second");
    println!("   is false. Which of the two you meet first is luck, and that is");
    println!("   the argument for never writing == on a float in a test at all.");
}
