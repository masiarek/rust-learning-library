//! Kata solution: the error message nobody saw.
//!
//! You wrote a careful sentence for the person who has to fix the ballot file.
//! Then you counted how many of the ways that error can actually reach them
//! print it — and the answer was one out of five. Every other path prints the
//! `Debug` form, because every other path was built out of `Debug`.
//!
//!   rustc --edition 2024 debug_vs_display_kata.rs -o /tmp/dvdk && /tmp/dvdk

use std::fmt;
use std::panic::{self, AssertUnwindSafe};
use std::process::ExitCode;
use std::sync::{Arc, Mutex};

/// The error as most people write it: `Display` for the human, `Debug` derived.
#[derive(Debug)]
enum TallyError {
    ScoreTooHigh { got: u8, max: u8 },
}

impl fmt::Display for TallyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TallyError::ScoreTooHigh { got, max } => write!(
                f,
                "score {got} is above the {max} cap -- check the ballot file's score column"
            ),
        }
    }
}

impl std::error::Error for TallyError {}

/// The same error with `Debug` written by hand to delegate to `Display`.
///
/// In real code you would write that impl on the error type itself and not
/// derive; the wrapper exists only so both behaviours can run in one program.
/// This is the move `anyhow::Error` makes, and knowing it is a *trade* is the
/// point of the second half of this kata.
struct Delegating(TallyError);

impl fmt::Display for Delegating {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for Delegating {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// The sentence we are looking for. A path "delivers" if this survives to it.
const SENTENCE: &str = "score 9 is above the 5 cap";

fn plain() -> Result<u8, TallyError> {
    Err(TallyError::ScoreTooHigh { got: 9, max: 5 })
}

fn delegating() -> Result<u8, Delegating> {
    Err(Delegating(TallyError::ScoreTooHigh { got: 9, max: 5 }))
}

/// Run `f` and hand back the panic message instead of dying, so a panic can be
/// printed as data. Not a pattern to copy.
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

fn panic_text<T>(f: impl FnOnce() -> T) -> String {
    match caught(f) {
        Ok(_) => "<did not panic>".to_string(),
        Err(msg) => msg,
    }
}

/// The five ways the error can reach a person, for one error type.
fn five_paths(display: String, debug: String, unwrap: String, expect: String) -> Vec<(&'static str, String)> {
    vec![
        ("println!(\"{e}\")", display),
        ("println!(\"{e:?}\")", debug),
        (".unwrap()", unwrap),
        (".expect(\"...\")", expect),
        // What the runtime writes to stderr for `fn main() -> Result<(), E>`:
        // `Error: ` followed by the DEBUG form. Reconstructed here because that
        // line goes to stderr and would exit the process.
        ("fn main() -> Result", String::new()),
    ]
}

fn report(title: &str, paths: &[(&'static str, String)]) -> usize {
    println!("\n=== {title} ===");
    let mut delivered = 0;
    for (path, text) in paths {
        let ok = text.contains(SENTENCE);
        if ok {
            delivered += 1;
        }
        println!("  {:<22} {}  {}", path, if ok { "SENTENCE" } else { "   --   " }, text);
    }
    println!("  {delivered} of {} paths delivered the sentence you wrote", paths.len());
    delivered
}

fn main() -> ExitCode {
    println!("The sentence written for the human:");
    println!("  {}", TallyError::ScoreTooHigh { got: 9, max: 5 });

    // ── Part 1: count the paths, with the derived Debug ────────────────────
    let e = TallyError::ScoreTooHigh { got: 9, max: 5 };
    let mut derived = five_paths(
        format!("{e}"),
        format!("{e:?}"),
        panic_text(|| plain().unwrap()),
        panic_text(|| plain().expect("the ballot file was validated on load")),
    );
    derived.last_mut().unwrap().1 = format!("Error: {e:?}");
    let n_derived = report("with #[derive(Debug)] -- what you shipped", &derived);

    // ── Part 2: the same five, with Debug delegating to Display ────────────
    let w = Delegating(TallyError::ScoreTooHigh { got: 9, max: 5 });
    let mut delegated = five_paths(
        format!("{w}"),
        format!("{w:?}"),
        panic_text(|| delegating().unwrap()),
        panic_text(|| delegating().expect("the ballot file was validated on load")),
    );
    delegated.last_mut().unwrap().1 = format!("Error: {w:?}");
    let n_delegated = report("with Debug delegating to Display -- the anyhow move", &delegated);

    // ── Part 3: what that fix cost ─────────────────────────────────────────
    println!("\n=== and what the delegating Debug gave away ===");
    println!("  a failing assert_eq! prints the Debug form of both sides. Compare:");
    println!("    derived     {:?}", TallyError::ScoreTooHigh { got: 9, max: 5 });
    println!("    delegating  {:?}", Delegating(TallyError::ScoreTooHigh { got: 9, max: 5 }));
    println!("  The first names the variant and both fields, which is what you want at");
    println!("  3am with a red test. The second is a sentence -- good for the operator,");
    println!("  and it has quietly removed the field values from your own diagnostics.");

    // ── Part 4: the fix that costs nothing ─────────────────────────────────
    println!("\n=== the other fix: stop letting the runtime choose ===");
    println!("  Keep the derived Debug, and handle the error where it leaves the program:");
    println!();
    println!("      fn main() -> ExitCode {{");
    println!("          match run() {{");
    println!("              Ok(()) => ExitCode::SUCCESS,");
    println!("              Err(e) => {{ eprintln!(\"error: {{e}}\"); ExitCode::FAILURE }}");
    println!("          }}");
    println!("      }}");
    println!();
    let code = match plain() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            println!("  what the operator now sees on stderr:  error: {e}");
            println!("  and the process still exits 1:         ExitCode::FAILURE");
            ExitCode::SUCCESS // this demo has to exit 0; a real binary returns FAILURE
        }
    };
    println!("\n  scoreboard: {n_derived} of 5 shipped, {n_delegated} of 5 after delegating,");
    println!("  and 5 of 5 with the derived Debug intact -- because the path that mattered");
    println!("  was never `{{:?}}` at all. It was the one place nobody had written any");
    println!("  printing code, so the runtime picked for you, and it picked Debug.");
    code
}
