//! Kata solution: retry with a budget, as a `loop` whose `break` carries a
//! `Result` — and the budget of 0 that decides where the check belongs.
//!
//!   rustc --edition 2024 the_loop_keyword_kata.rs -o /tmp/tlkk && /tmp/tlkk

/// A server that answers from a fixed table, then keeps failing.
fn fetch(responses: &[u16], attempt: usize) -> u16 {
    responses.get(attempt).copied().unwrap_or(503)
}

/// Check at the bottom: the body runs before the budget is looked at.
fn retry_loop(responses: &[u16], budget: usize) -> Result<usize, usize> {
    let mut attempts = 0;
    loop {
        let code = fetch(responses, attempts);
        attempts += 1;
        if code == 200 {
            break Ok(attempts);
        }
        if attempts >= budget {
            break Err(attempts);
        }
    }
}

/// Check at the top: a budget of 0 sends nothing.
fn retry_checked_first(responses: &[u16], budget: usize) -> Result<usize, usize> {
    let mut attempts = 0;
    loop {
        if attempts >= budget {
            break Err(attempts);
        }
        let code = fetch(responses, attempts);
        attempts += 1;
        if code == 200 {
            break Ok(attempts);
        }
    }
}

/// The same bottom check written with `==`, run with a safety cap so the
/// program can report what it would have done instead of doing it.
fn retry_equals_capped(responses: &[u16], budget: usize, cap: usize) -> Result<usize, usize> {
    let mut attempts = 0;
    loop {
        let code = fetch(responses, attempts);
        attempts += 1;
        if code == 200 {
            break Ok(attempts);
        }
        if attempts == budget || attempts == cap {
            break Err(attempts);
        }
    }
}

fn retry_iter(responses: &[u16], budget: usize) -> Result<usize, usize> {
    (0..budget)
        .position(|attempt| fetch(responses, attempt) == 200)
        .map(|i| i + 1)
        .ok_or(budget)
}

fn main() {
    let responses = [503, 503, 200];
    println!("responses = {responses:?}, then 503 forever");
    println!();
    println!("budget   check at bottom   check at top   iterator");
    for budget in [3, 5, 2, 1, 0] {
        println!(
            "{budget:>6}   {:<15}   {:<12}   {:?}",
            format!("{:?}", retry_loop(&responses, budget)),
            format!("{:?}", retry_checked_first(&responses, budget)),
            retry_iter(&responses, budget),
        );
    }
    println!();
    println!("Budget 0 is the only row where the columns disagree. With the check");
    println!("at the bottom the loop is a do...while: one request goes out before");
    println!("anyone asks whether it was allowed.");
    println!();
    let failing = [503];
    println!("Now `attempts == budget` instead of `>=`, against a server that never succeeds:");
    println!("   budget 3: {:?}", retry_equals_capped(&failing, 3, 10_000));
    println!("   budget 0: {:?}   <- stopped by the 10_000 cap, not by the budget", retry_equals_capped(&failing, 0, 10_000));
    println!("attempts is already 1 when it is first compared, so 1 == 0 is the first");
    println!("of an endless run of false answers.");
}
