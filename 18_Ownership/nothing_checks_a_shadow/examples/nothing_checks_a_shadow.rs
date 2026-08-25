//! Nothing checks a shadow.
//!
//! `rustc` has no lint for a shadowed local — `rustc -W help` lists none. What
//! gets mistaken for protection is the TYPE ERROR that a mistaken shadow often
//! trips on its way past. When the shadow has the same type as the thing it
//! hides, there is no error, no warning, and a wrong answer.
//!
//!   rustc --edition 2024 nothing_checks_a_shadow.rs -o /tmp/ncas && /tmp/ncas

fn banner(n: u32, title: &str) {
    println!("\n──── Step {n}: {title}");
}

// Shadowed in step 3 by a `fn` of the same name inside a block.
fn threshold() -> u32 {
    50
}

fn main() {
    // ────────────────────────────────────────────────────────────── 1
    banner(1, "The shadow that compiles clean");
    let scores = [5u32, 3, 0, 4];

    let total = 0;
    for s in scores {
        let total = total + s; // a NEW `total`, built fresh from the OUTER 0
        println!("  running total: {total}");
    }
    println!("  final total:   {total}");
    println!("      The running totals are 5, 3, 0, 4 — the scores themselves,");
    println!("      not a sum. Each iteration read the outer `total` (still 0),");
    println!("      added one score, and threw the result away at the brace.");
    println!("      Zero warnings. `rustc` compiled this without a word.");

    // ────────────────────────────────────────────────────────────── 2
    banner(2, "Why nothing warned");
    println!("  There is no shadowing lint in rustc. `rustc -W help` mentions");
    println!("  'shadow' four times and every one is about trait items or glob");
    println!("  re-exports — none about a `let`.");
    println!("      Two things nearly catch it, and neither is a shadow check:");
    println!("      * `unused_variables`, if the shadow is never READ. Read it");
    println!("        once inside the loop — as the println above does — and");
    println!("        that lint has nothing to say.");
    println!("      * a TYPE MISMATCH downstream, if the shadow's type differs");
    println!("        from what later code expects: `E0308: mismatched types`.");
    println!("      Both are accidents. Same type, read once: total silence.");

    // ────────────────────────────────────────────────────────────── 3
    banner(3, "Items get shadowed too, just as quietly");
    println!("  outer threshold() -> {}", threshold());
    {
        fn threshold() -> u32 {
            5
        }
        println!("  inner threshold() -> {}   <- a different function", threshold());
    }
    println!("  after the block   -> {}", threshold());
    println!("      A `fn` inside a block shadows one outside it, with no");
    println!("      diagnostic at all. `fn` and `let` share the VALUE namespace,");
    println!("      so a variable can hide a function too — and that one rustc");
    println!("      does catch, because calling a u32 is a type error:");
    println!("        error[E0618]: expected function, found `u32`");
    println!("          | this function of the same name is available here,");
    println!("          | but it's shadowed by the local binding");
    println!("      Note rustc's own word for it there: shadowed.");

    // ────────────────────────────────────────────────────────────── 4
    banner(4, "The same shape, where the compiler DOES stop you");
    let name = String::from("Ada");
    let mut jobs: Vec<Box<dyn Fn()>> = Vec::new();
    for i in 1..=3 {
        let name = name.clone(); // the idiom — and here it is load-bearing
        jobs.push(Box::new(move || println!("  job {i}: {name}")));
    }
    for j in &jobs {
        j();
    }
    println!("  original still here: {name}");
    println!("      Delete that `let name = name.clone();` and it does not");
    println!("      compile: E0382, 'value moved into closure here, in previous");
    println!("      iteration of loop'. Structurally the SAME shadow-in-a-loop as");
    println!("      step 1 — and this one is a hard error.");
    println!("      The difference is not the shadow. It is that ownership is");
    println!("      checked and arithmetic is not. Step 1 lost a number, which");
    println!("      no rule forbids; step 4 would have lost a String twice over,");
    println!("      which every rule forbids.");

    // ────────────────────────────────────────────────────────────── 5
    banner(5, "Three ways to write step 1 so it cannot go quiet");
    let mut running = 0u32;
    for s in scores {
        running += s;
    }
    println!("  1. `mut`, no shadow          -> {running}");

    let summed: u32 = scores.iter().sum();
    println!("  2. no accumulator at all     -> {summed}");

    let total_score = scores.iter().fold(0u32, |acc, s| acc + s);
    println!("  3. a name that cannot collide-> {total_score}");
    println!("      This is the one place `mut` genuinely beats shadowing, and");
    println!("      it is the opposite of the usual advice: an accumulator is");
    println!("      supposed to survive the iteration, so the thing shadowing");
    println!("      gives you — a fresh binding each time — is exactly the bug.");
    println!("      Option 2 is the one to ship: with no binding to shadow,");
    println!("      the mistake has nowhere to live.");
}
