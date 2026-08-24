//! The three ways Rust panics, and the combinator that removes each one.
//!
//! Nothing here actually panics — that is the point. Each pair shows the
//! panicking line as a comment and the value-returning version beside it, so
//! the program can print both outcomes instead of dying on the first.
//!
//!   rustc --edition 2024 unwrap_is_a_todo.rs -o /tmp/uiat && /tmp/uiat

fn main() {
    println!("The three ways to panic (Rust Book, ch. 9) — and what to write instead\n");

    // ---- 1. Unwrapping an Err ------------------------------------------
    // WOULD PANIC:  "nope".parse::<i32>().unwrap()
    for input in ["41", "nope"] {
        let parsed = input.parse::<i32>();
        let with_unwrap_or_default = parsed.clone().unwrap_or_default() + 1;
        let as_a_value = match &parsed {
            Ok(n) => format!("Ok({n})"),
            Err(e) => format!("Err({e})"),
        };
        println!("  parse({:<8} ->  {as_a_value:<36} unwrap_or_default() + 1 = {with_unwrap_or_default}", format!("{input:?})"));
    }

    // ---- 2. Indexing past the end --------------------------------------
    // WOULD PANIC:  v[99]
    let v = vec![1, 2, 3];
    for i in [1, 99] {
        match v.get(i) {
            Some(n) => println!("  v.get({i})       ->  Some({n})"),
            None => println!("  v.get({i})      ->  None                         (v[{i}] would have panicked)"),
        }
    }

    // ---- 3. An explicit panic! -----------------------------------------
    // WOULD PANIC:  panic!("crash and burn")
    // The replacement is not a combinator, it is a signature: say so in the
    // return type and let the caller decide.
    println!("\n  quorum(10, 3)   ->  {:?}", quorum(10, 3));
    println!("  quorum(10, 30)  ->  {:?}", quorum(10, 30));

    // ---- the pipeline the combinators are for --------------------------
    println!("\nThe same work as a pipeline, with no unwrap anywhere:");
    for raw in ["7", "0", "not a number"] {
        let outcome = raw
            .parse::<i32>()
            .map_err(|e| format!("unreadable: {e}"))
            .and_then(|n| if n == 0 { Err("zero has no reciprocal".to_string()) } else { Ok(n) })
            .map(|n| 100 / n);
        println!("  {raw:?}  ->  {outcome:?}");
    }
}

/// Seats needed for a quorum — a failure the caller can act on, not a panic.
fn quorum(members: u32, present: u32) -> Result<u32, String> {
    let needed = members / 2 + 1;
    if present >= needed {
        Ok(needed)
    } else {
        Err(format!("{present} present, {needed} needed"))
    }
}
