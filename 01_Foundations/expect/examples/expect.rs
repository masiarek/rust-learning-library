//! `expect`: the place you write down the proof.
//!
//! Mechanically it is `unwrap` with a sentence attached. What the sentence is
//! FOR is the whole lesson: it is not an error message, it is a claim about why
//! this could not fail, addressed to whoever reads the panic six months from now
//! — and if you cannot write it, you have discovered that you do not have the
//! proof and should be returning a `Result`.
//!
//! (What the panic itself does — where it points, what unwinding restores, exit
//! code 101 — is a separate page; this one is only about the message.)
//!
//!   rustc --edition 2024 expect.rs -o /tmp/ex && /tmp/ex

use std::panic::{self, AssertUnwindSafe};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

fn banner(n: u32, title: &str) {
    println!("\n──── Step {n}: {title}");
}

/// Run `f`, and instead of dying, hand back the panic's message and the file:line
/// it was raised at. The hook is replaced only so this program can print panics
/// as data; nothing here is a pattern to copy.
fn caught<T>(f: impl FnOnce() -> T) -> Result<T, (String, String)> {
    let slot: Arc<Mutex<Option<(String, String)>>> = Arc::new(Mutex::new(None));
    let sink = Arc::clone(&slot);
    let prior = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        let payload = info.payload();
        let message = payload
            .downcast_ref::<&str>()
            .map(|s| (*s).to_string())
            .or_else(|| payload.downcast_ref::<String>().cloned())
            .unwrap_or_else(|| "<payload was not a string>".to_string());
        // Only the file NAME, never the path: the path depends on whose machine
        // this ran on, and the recorded output has to be the same everywhere.
        let where_ = info
            .location()
            .map(|l| format!("{}:{}", l.file().rsplit('/').next().unwrap_or(l.file()), l.line()))
            .unwrap_or_default();
        *sink.lock().unwrap() = Some((message, where_));
    }));
    let outcome = panic::catch_unwind(AssertUnwindSafe(f));
    panic::set_hook(prior);
    outcome.map_err(|_| slot.lock().unwrap().take().unwrap_or_default())
}

fn show(label: &str, outcome: Result<String, (String, String)>) {
    match outcome {
        Ok(v) => println!("  {label:<34} -> {v}"),
        Err((msg, _)) => println!("  {label:<34} -> PANIC: {msg}"),
    }
}

// ─────────────────────────────────────────────────────────── Step 1
fn step1() {
    banner(1, "unwrap's message is the standard library's; expect's is yours");

    let missing: Option<u8> = None;
    let bad: Result<u8, String> = Err("invalid digit found in string".to_string());

    show("None.unwrap()", caught(|| missing.unwrap().to_string()));
    show(
        "None.expect(\"…\")",
        caught(|| missing.expect("the config should list a quorum").to_string()),
    );
    show("Err(e).unwrap()", caught(|| bad.clone().unwrap().to_string()));
    show(
        "Err(e).expect(\"…\")",
        caught(|| bad.clone().expect("the quorum line should be a number").to_string()),
    );
    show(
        "Ok(5).expect_err(\"…\")",
        caught(|| Ok::<u8, String>(5).expect_err("no quorum should parse from an empty file").to_string()),
    );

    println!("      Four panics, and only two of them tell you anything. Note what the");
    println!("      Result form does: `{{your sentence}}: {{the error, Debug-printed}}`, so");
    println!("      the claim AND the cause survive — expect never costs you the error.");
    println!("      expect_err is the mirror, for when the surprise is that it worked.");
}

// ─────────────────────────────────────────────────────────── Step 2
fn step2() {
    banner(2, "The panic names your line, not the standard library's");

    let missing: Option<u8> = None;
    let outcome = caught(|| missing.expect("the ballot file should have been validated by now"));
    match outcome {
        Ok(_) => println!("  (unreachable)"),
        Err((msg, where_)) => {
            println!("  panicked at {where_}");
            println!("  {msg}");
        }
    }
    println!("      That location is the `expect` call in THIS file — not a line inside");
    println!("      core/src/option.rs, which is where the panic is physically raised.");
    println!("      So the two halves of a good panic report come from two different");
    println!("      places: the address from the attribute on the method, the meaning");
    println!("      from the sentence you wrote.");
}

// ─────────────────────────────────────────────────────────── Step 3
fn step3() {
    banner(3, "Say what SHOULD be true, not what went wrong");

    let missing: Option<u8> = None;

    show(
        "expect(\"failed to get quorum\")",
        caught(|| missing.expect("failed to get quorum").to_string()),
    );
    show(
        "expect(\"unwrap failed\")",
        caught(|| missing.expect("unwrap failed").to_string()),
    );
    show(
        "expect(\"[election] should set…\")",
        caught(|| {
            missing
                .expect("the [election] section should set a quorum; the loader fills it in from defaults")
                .to_string()
        }),
    );
    println!("      Same bug, three panics, one of them useful. The standard library's");
    println!("      own guidance is to describe the reason you expected a value — so the");
    println!("      line reads as a CLAIM, and a reader who sees it knows both what was");
    println!("      supposed to hold and who was supposed to make it hold. 'Failed to");
    println!("      get X' only restates that the program stopped, which the word PANIC");
    println!("      already said.");
}

// ─────────────────────────────────────────────────────────── Step 4
/// The dishonest version: this is a config file, and a user wrote it.
fn quorum_by_expect(config: &[(&str, &str)]) -> u32 {
    config
        .iter()
        .find(|(k, _)| *k == "quorum")
        .expect("the config should have a quorum")
        .1
        .parse()
        .expect("the quorum should be a number")
}

/// The honest version: absence and malformedness are outcomes, not impossibilities.
fn quorum_by_result(config: &[(&str, &str)]) -> Result<u32, String> {
    let (_, raw) = config
        .iter()
        .find(|(k, _)| *k == "quorum")
        .ok_or_else(|| "no `quorum` key in [election]".to_string())?;
    raw.parse()
        .map_err(|e| format!("quorum = {raw:?} is not a number ({e})"))
}

fn step4() {
    banner(4, "If you cannot write the sentence, you do not have the proof");

    let good = [("quorum", "50"), ("seats", "1")];
    let typo = [("qorum", "50")];

    println!("  well-formed config:");
    println!("    by expect -> {}", quorum_by_expect(&good));
    println!("    by Result -> {:?}", quorum_by_result(&good));

    println!("  config with a typo'd key (a thing users do):");
    match caught(|| quorum_by_expect(&typo).to_string()) {
        Ok(v) => println!("    by expect -> {v}"),
        Err((msg, _)) => println!("    by expect -> PANIC: {msg}"),
    }
    println!("    by Result -> {:?}", quorum_by_result(&typo));

    println!("      Read the first message again: 'the config SHOULD have a quorum' —");
    println!("      should according to whom? Nobody proved that; a user typed the file.");
    println!("      The sentence is a hope, and the tell is that you cannot name who");
    println!("      guaranteed it. That is the signal to return Result: the second form");
    println!("      says the same thing to the same reader, names the offending value,");
    println!("      and lets the caller decide whether it is fatal.");
}

// ─────────────────────────────────────────────────────────── Step 5
fn median(sorted: &[u8]) -> Option<u8> {
    if sorted.is_empty() {
        return None;
    }
    Some(sorted[sorted.len() / 2])
}

fn step5() {
    banner(5, "Where expect is exactly right: a proof the compiler cannot check");

    // 1. The value is a literal, sitting right there in the source.
    let max: u8 = "5".parse().expect("the literal \"5\" parses as a u8");
    println!("  literal in the source          -> {max}");

    // 2. An invariant this function established two lines ago.
    let scores = [0u8, 3, 5];
    let mid = median(&scores).expect("median returns Some for a non-empty slice, and this one has 3");
    println!("  invariant established locally  -> {mid}");

    // 3. A lock: the only failure is another thread having panicked while holding it.
    let shared = Mutex::new(vec![5u8, 3, 0]);
    let len = shared.lock().expect("no thread panics while holding this lock").len();
    println!("  a lock that is never poisoned  -> {len} scores");

    println!("      All three sentences are checkable by a reader: one points at a");
    println!("      literal, one at an early return four lines up, one at a claim about");
    println!("      the whole program that a reviewer can go and test. That is the bar —");
    println!("      not 'this feels safe' but 'here is the argument, go and check it'.");
    println!("      A test is the fourth case, where the panic IS the failure report.");
}

// ─────────────────────────────────────────────────────────── Step 6
static BUILT: AtomicUsize = AtomicUsize::new(0);

fn proof(name: &str) -> String {
    BUILT.fetch_add(1, Ordering::Relaxed);
    format!("every ballot should score {name}; the loader pads missing columns with 0")
}

fn step6() {
    banner(6, "The message is an argument, so it is built even when nothing fails");

    let ballot: [(&str, Option<u8>); 3] = [("Ada", Some(5)), ("Ben", Some(3)), ("Cara", Some(0))];

    BUILT.store(0, Ordering::Relaxed);
    let mut total = 0u32;
    for (name, score) in ballot {
        total += u32::from(score.expect(&proof(name)));
    }
    println!("  expect(&proof(name))                    total {total}, proof() ran {} times", BUILT.load(Ordering::Relaxed));

    BUILT.store(0, Ordering::Relaxed);
    let mut total = 0u32;
    for (name, score) in ballot {
        total += u32::from(score.unwrap_or_else(|| panic!("{}", proof(name))));
    }
    println!("  unwrap_or_else(|| panic!(proof(name)))  total {total}, proof() ran {} times", BUILT.load(Ordering::Relaxed));

    println!("      `expect` takes a &str, which is an ordinary eagerly-evaluated");
    println!("      argument — exactly like unwrap_or's default. A formatted message is");
    println!("      therefore built, allocated, and dropped on every SUCCESSFUL call, in");
    println!("      the hot loop, forever. A plain literal costs nothing, so write one;");
    println!("      when the message genuinely needs the value in it, the standard");
    println!("      library's own answer is unwrap_or_else with a panic! inside.");
}

fn main() {
    step1();
    step2();
    step3();
    step4();
    step5();
    step6();
    println!();
}
