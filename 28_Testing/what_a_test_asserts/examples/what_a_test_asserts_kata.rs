//! Kata solution: five assertions, and two of them cannot fail.
//!
//!   rustc --edition 2024 what_a_test_asserts_kata.rs -o /tmp/wtak && /tmp/wtak

fn message_from(f: impl FnOnce() + std::panic::UnwindSafe) -> String {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let result = std::panic::catch_unwind(f);
    std::panic::set_hook(hook);
    match result {
        Ok(()) => "PASSED".to_string(),
        Err(e) => e
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| e.downcast_ref::<&str>().map(|s| (*s).to_string()))
            .unwrap_or_else(|| "(non-string panic)".to_string())
            .lines()
            .next()
            .unwrap_or("")
            .to_string(),
    }
}

/// The function under test: an average that refuses an empty ballot.
fn average(scores: &[u32]) -> Option<f64> {
    if scores.is_empty() {
        return None;
    }
    Some(f64::from(scores.iter().sum::<u32>()) / scores.len() as f64)
}

fn main() {
    println!("1. The five assertions, and what each one proves");
    println!("   a. average(&[5, 3, 4]) is Some(4.0)   -> {}",
             message_from(|| assert_eq!(average(&[5, 3, 4]), Some(4.0))));
    println!("   b. average(&[]) is None               -> {}",
             message_from(|| assert_eq!(average(&[]), None)));
    println!("   c. average(&[1, 2]) == Some(1.5)      -> {}",
             message_from(|| assert_eq!(average(&[1, 2]), Some(1.5))));
    println!("   d. average(&[1, 1, 1]) == Some(1.0)   -> {}",
             message_from(|| assert_eq!(average(&[1, 1, 1]), Some(1.0))));
    println!("   e. average(&[1, 2, 2]) == Some(5.0/3.0) -> {}",
             message_from(|| assert_eq!(average(&[1, 2, 2]), Some(5.0 / 3.0))));

    println!();
    println!("2. Which ones cannot fail, and why");
    println!("   (d) is the tautology: 1, 1, 1 averages to 1 for any function that");
    println!("   divides a sum by a count, and also for one that just returns its");
    println!("   first element, and also for one that returns the maximum. An");
    println!("   input whose every plausible answer is the same number tests the");
    println!("   arithmetic, not the definition.");
    println!("   (e) passes for a different reason: both sides compute 5.0/3.0 the");
    println!("   same way, in the same rounding mode, so the bits match exactly.");
    println!("   Write the literal 1.6666666666666667 instead and it still passes;");
    println!("   write 1.666666666666667 and it does not. That is a test of your");
    println!("   typing, not of the code.");

    println!();
    println!("3. The float assertion that means something");
    let a = average(&[1, 2, 2]).unwrap();
    println!("   (a - 5.0/3.0).abs() < 1e-12 -> {}",
             message_from(move || assert!((a - 5.0 / 3.0).abs() < 1e-12,
                          "average was {a}, expected about 1.6667")));
    println!("   A tolerance says what \"close enough\" means for THIS domain. On a");
    println!("   0-5 ballot, 1e-12 is absurd precision and 0.005 would be honest.");
    println!("   Picking the number is the work; == avoids the question.");

    println!();
    println!("4. What a failure actually prints");
    println!("   {}", message_from(|| assert_eq!(average(&[5, 3, 4]), Some(3.0))));
    println!("   ...then two lines naming left and right. assert! would have said");
    println!("   only \"assertion failed: average(&[5, 3, 4]) == Some(3.0)\" — true,");
    println!("   unhelpful, and one debugging session longer.");

    println!();
    println!("5. The message, and when to spend one");
    println!("   {}", message_from(|| {
        let seats = 3;
        let candidates = 2;
        assert!(seats <= candidates,
                "cannot fill {seats} seats from {candidates} candidates");
    }));
    println!("   Values alone would have printed `3` and `2` with no clue which is");
    println!("   which. The message is a format! run only on failure, so it costs");
    println!("   nothing on the happy path — spend one whenever the two numbers do");
    println!("   not name themselves.");
}
