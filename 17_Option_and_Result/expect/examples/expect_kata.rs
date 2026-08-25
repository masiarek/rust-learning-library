//! Kata solution: four sentences, one of them a hope.
//!
//! Part 1 audits four `expect` messages by asking one question of each — who
//! guaranteed this? Three can name a guarantor; one cannot, and that one is
//! not a wording problem, it is the wrong return type.
//! Part 2 runs all four on input a user could plausibly type, which is what
//! stops the audit being a matter of opinion.
//! Part 3 fixes the unprovable one by changing the signature, not the sentence.
//! Part 4 fixes the fourth call, whose proof is sound but whose message is
//! built on every successful call — and counts the difference.
//!
//!   rustc --edition 2024 expect_kata.rs -o /tmp/exk && /tmp/exk

use std::panic::{self, AssertUnwindSafe};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

fn banner(n: u32, title: &str) {
    println!("\n──── Part {n}: {title}");
}

/// Run `f` and hand back the panic message instead of dying. The hook is
/// replaced only so this program can print panics as data; nothing in here is
/// a pattern to copy into real code.
fn caught<T>(f: impl FnOnce() -> T) -> Result<T, String> {
    let slot: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
    let sink = Arc::clone(&slot);
    let prior = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        let payload = info.payload();
        let message = payload
            .downcast_ref::<&str>()
            .map(|s| (*s).to_string())
            .or_else(|| payload.downcast_ref::<String>().cloned())
            .unwrap_or_else(|| "<payload was not a string>".to_string());
        *sink.lock().unwrap() = Some(message);
    }));
    let outcome = panic::catch_unwind(AssertUnwindSafe(f));
    panic::set_hook(prior);
    outcome.map_err(|_| slot.lock().unwrap().take().unwrap_or_default())
}

fn show(label: &str, outcome: Result<String, String>) {
    match outcome {
        Ok(v) => println!("      {label:<26} -> {v}"),
        Err(msg) => println!("      {label:<26} -> PANIC: {msg}"),
    }
}

// ── The four call sites under audit ───────────────────────────────────────

/// #1 — the guarantor is a literal three characters to the left.
fn max_score() -> u8 {
    "5".parse().expect("the literal \"5\" parses as a u8")
}

/// #2 — the guarantor is the early return four lines up.
fn middle_score(scores: &[u8]) -> Option<u8> {
    if scores.is_empty() {
        return None; //   <- this line is the proof
    }
    let mut sorted = scores.to_vec();
    sorted.sort_unstable();
    Some(
        sorted
            .get(sorted.len() / 2)
            .copied()
            .expect("a non-empty slice has a middle element, and we returned early above if it was empty"),
    )
}

/// #3 — the guarantor is nobody. A *user* typed this file.
fn quorum_by_expect(config: &[(&str, &str)]) -> u32 {
    let (_, raw) = config
        .iter()
        .find(|(k, _)| *k == "quorum")
        .expect("the config should have a quorum");
    raw.parse().expect("the quorum should be a number")
}

/// #3, fixed — the same information, to the same reader, plus the bad value,
/// and the caller decides whether it is fatal.
fn quorum_by_result(config: &[(&str, &str)]) -> Result<u32, String> {
    let (_, raw) = config
        .iter()
        .find(|(k, _)| *k == "quorum")
        .ok_or_else(|| "no `quorum` key in [election]".to_string())?;
    raw.parse()
        .map_err(|e| format!("quorum = {raw:?} is not a number ({e})"))
}

/// #4 — the proof is sound; the *message* is the problem. Every call to this
/// function allocates a String, whether or not anything is about to fail.
static PROOFS_BUILT: AtomicUsize = AtomicUsize::new(0);

fn proof(ballot: usize, name: &str) -> String {
    PROOFS_BUILT.fetch_add(1, Ordering::Relaxed);
    format!("ballot {ballot} should score {name}; the loader pads short rows with 0")
}

fn total_eager(rows: &[Vec<u8>], col: usize, name: &str) -> u32 {
    rows.iter()
        .enumerate()
        .map(|(i, row)| row.get(col).copied().expect(&proof(i, name)) as u32)
        .sum()
}

fn total_lazy(rows: &[Vec<u8>], col: usize, name: &str) -> u32 {
    rows.iter()
        .enumerate()
        .map(|(i, row)| {
            row.get(col)
                .copied()
                .unwrap_or_else(|| panic!("{}", proof(i, name))) as u32
        })
        .sum()
}

// ── The exercise ──────────────────────────────────────────────────────────

fn part1_name_the_guarantor() {
    banner(1, "Name the guarantor, or admit there isn't one");

    println!("      #1  \"the literal \\\"5\\\" parses as a u8\"");
    println!("            guarantor: the literal, three characters to the left.   KEEP");
    println!("      #2  \"a non-empty slice has a middle element, and we");
    println!("           returned early above if it was empty\"");
    println!("            guarantor: the `return None` four lines up.             KEEP");
    println!("      #3  \"the config should have a quorum\"");
    println!("            guarantor: nobody. A user typed the file.               FIX THE TYPE");
    println!("      #4  \"ballot N should score X; the loader pads short rows\"");
    println!("            guarantor: the loader, named in the sentence.           KEEP THE PROOF,");
    println!("                                                                    MOVE THE MESSAGE");
    println!("      Three can name a guarantor. #3 can only say \"should\" — a hope");
    println!("      wearing the grammar of a proof. The tell is precise: you can write");
    println!("      the words, but you cannot name anyone who makes them true.");
}

fn part2_run_them(good: &[(&str, &str)], typo: &[(&str, &str)]) {
    banner(2, "Which one dies on input a user could plausibly type?");

    let scores = [3u8, 5, 0];
    show("#1 max_score()", caught(|| max_score().to_string()));
    show(
        "#2 middle_score(&[3,5,0])",
        caught(move || format!("{:?}", middle_score(&scores))),
    );
    show(
        "#2 middle_score(&[])",
        caught(|| format!("{:?}", middle_score(&[]))),
    );
    show(
        "#3 well-formed config",
        caught(|| quorum_by_expect(good).to_string()),
    );
    show(
        "#3 typo'd key",
        caught(|| quorum_by_expect(typo).to_string()),
    );

    println!("      #1 and #2 cannot fail — #2 answers `None` rather than panicking,");
    println!("      because emptiness is a caller's question, not a bug. #3 dies on a");
    println!("      config that is merely misspelled, with a message blaming nothing.");
}

fn part3_change_the_type(good: &[(&str, &str)], typo: &[(&str, &str)], junk: &[(&str, &str)]) {
    banner(3, "Fix #3 by changing the signature, not the wording");

    println!("      well-formed  -> {:?}", quorum_by_result(good));
    println!("      typo'd key   -> {:?}", quorum_by_result(typo));
    println!("      not a number -> {:?}", quorum_by_result(junk));
    println!("      No panic, and the third message carries the offending value —");
    println!("      which the `expect` version could not, because a &str message is");
    println!("      chosen before anyone knows what went wrong.");
}

fn part4_count_the_messages() {
    banner(4, "The proof is fine; the message is built 4 times for nothing");

    let rows = vec![vec![3u8, 5, 0], vec![5, 5, 1], vec![0, 4, 4], vec![2, 2, 2]];

    PROOFS_BUILT.store(0, Ordering::Relaxed);
    let eager = total_eager(&rows, 1, "Ada");
    let built_eager = PROOFS_BUILT.load(Ordering::Relaxed);

    PROOFS_BUILT.store(0, Ordering::Relaxed);
    let lazy = total_lazy(&rows, 1, "Ada");
    let built_lazy = PROOFS_BUILT.load(Ordering::Relaxed);

    let eager_call = "expect(&proof(i, name))";
    let lazy_call = "unwrap_or_else(|| panic!(proof(i, name)))";
    println!("      {eager_call:<42} total {eager}, proof() ran {built_eager} times");
    println!("      {lazy_call:<42} total {lazy}, proof() ran {built_lazy} times");
    println!("      Same total, same proof, same sentence if it ever fires. `expect`");
    println!("      takes a &str — an ordinary eager argument — so the String is built,");
    println!("      allocated and dropped on every SUCCESSFUL row, forever, to be read");
    println!("      never. A plain literal costs nothing; a formatted one belongs in");
    println!("      the sad path.");
}

fn main() {
    let good = [("quorum", "50"), ("seats", "1")];
    let typo = [("quourm", "50"), ("seats", "1")];
    let junk = [("quorum", "fifty"), ("seats", "1")];

    part1_name_the_guarantor();
    part2_run_them(&good, &typo);
    part3_change_the_type(&good, &typo, &junk);
    part4_count_the_messages();
}
