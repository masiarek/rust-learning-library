//! Two printing traits, two audiences: `{}` is `Display`, `{:?}` is `Debug`.
//!
//! The habit forms on an integer, where the two are indistinguishable, and it
//! goes wrong on everything else. This program prints both forms of the same
//! values side by side so the difference stops being a rule to remember.
//!
//!   rustc --edition 2024 debug_vs_display.rs -o /tmp/dvd && /tmp/dvd

use std::collections::BTreeMap;
use std::fmt;
use std::panic::{self, AssertUnwindSafe};
use std::path::Path;
use std::sync::{Arc, Mutex};

fn banner(title: &str) {
    println!("\n=== {title} ===");
}

// ───────────────────────────────────────────────── the types used below

/// Derived `Debug`, hand-written `Display`. That asymmetry is the lesson:
/// only one of the two can be generated, because only one of them is structural.
#[derive(Debug)]
struct Ballot {
    voter: String,
    scores: [u8; 3],
}

impl fmt::Display for Ballot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let scores: Vec<String> = self.scores.iter().map(|s| s.to_string()).collect();
        write!(f, "{} scored {}", self.voter, scores.join("/"))
    }
}

/// An error, written the way the standard library asks for one: `Debug` for you,
/// `Display` for whoever has to fix the ballot file.
#[derive(Debug)]
enum TallyError {
    ScoreTooHigh { got: u8, max: u8 },
}

impl fmt::Display for TallyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TallyError::ScoreTooHigh { got, max } => {
                write!(f, "score {got} is above the {max} cap")
            }
        }
    }
}

impl std::error::Error for TallyError {}

/// Derived `Debug` prints every field it can reach, including this one.
#[derive(Debug)]
struct Session {
    voter_id: u32,
    token: String,
}

/// The same data, with `Debug` written by hand instead of derived.
struct Guarded {
    voter_id: u32,
    token: String,
}

impl fmt::Debug for Guarded {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Guarded")
            .field("voter_id", &self.voter_id)
            .field("token", &format_args!("<redacted, {} chars>", self.token.len()))
            .finish()
    }
}

// ─────────────────────────────────────────────────────────── 1
fn where_the_habit_forms() {
    banner("where the habit forms: on an integer, the two agree exactly");

    let x: i32 = 11;
    println!("  {{}}    on 11i32     -> {x}");
    println!("  {{:?}}  on 11i32     -> {x:?}");
    println!("  same characters, so nothing tells you two different traits ran");

    let name = "Ada";
    println!("\n  {{}}    on \"Ada\"     -> {name}");
    println!("  {{:?}}  on \"Ada\"     -> {name:?}");
    println!("  the quotes are not decoration: Debug for str prints a Rust LITERAL");

    let c = 'A';
    println!("\n  {{}}    on 'A'       -> {c}");
    println!("  {{:?}}  on 'A'       -> {c:?}");

    let f = 1.0f64;
    println!("\n  {{}}    on 1.0f64    -> {f}");
    println!("  {{:?}}  on 1.0f64    -> {f:?}   <- Debug keeps the point, so you can");
    println!("                            see this is a float and not an integer");
}

// ─────────────────────────────────────────────────────────── 2
fn what_display_hides() {
    banner("the same trait pair, read the other way round: Debug shows the data");

    let names = ["Ada", "Ben ", "", "Ca\tra", "O'Neill\n", "Ada\u{200b}"];
    println!("  {:<16}{:<7}{}", "with {:?}", "bytes", "with {}, bracketed and last on the line");
    for n in names {
        println!("  {:<16}{:<7}[{}]", format!("{n:?}"), n.len(), n);
    }
    println!();
    println!("  Five of those six names are a bug, and {{}} renders every one of them as");
    println!("  something a reviewer would sign off: a trailing space, an empty string, a");
    println!("  tab, a newline, a zero-width space. The byte count is the tell, and {{:?}}");
    println!("  is how you see the cause -- it escapes anything a Rust literal would.");
    println!();
    println!("  Note what the O'Neill row did to this table: it wrapped, because the name");
    println!("  really does contain a newline. That is Display doing its job faithfully,");
    println!("  and it is why the Debug column had to come first to stay readable.");
}

// ─────────────────────────────────────────────────────────── 3
fn derived_and_written() {
    banner("one is generated, one is written -- and that is not an oversight");

    let b = Ballot { voter: "Ada".to_string(), scores: [5, 2, 0] };

    println!("  {{}}    -> {b}");
    println!("  {{:?}}  -> {b:?}");
    println!("  {{:#?}} ->");
    for line in format!("{b:#?}").lines() {
        println!("      {line}");
    }
    println!();
    println!("  #[derive(Debug)] can be generated because the answer is structural:");
    println!("  the type's name, its fields, their names. There is no #[derive(Display)]");
    println!("  in std, because nothing about the type says whether a human wants");
    println!("  \"Ada scored 5/2/0\", \"Ada: 5, 2, 0\", or a row in a table.");
}

// ─────────────────────────────────────────────────────────── 4
fn the_container_gap() {
    banner("why you reach for {:?} so often: the containers have no Display");

    let v = vec![5u8, 2, 0];
    let o: Option<u8> = Some(5);
    let r: Result<u8, TallyError> = Err(TallyError::ScoreTooHigh { got: 9, max: 5 });
    let t = ("Ada", 5u8, true);
    let mut m: BTreeMap<&str, u8> = BTreeMap::new();
    m.insert("Ada", 5);
    m.insert("Ben", 2);

    println!("  Vec<u8>          {{}} does not compile   {{:?}} -> {v:?}");
    println!("  Option<u8>       {{}} does not compile   {{:?}} -> {o:?}");
    println!("  Result<u8, E>    {{}} does not compile   {{:?}} -> {r:?}");
    println!("  (&str, u8, bool) {{}} does not compile   {{:?}} -> {t:?}");
    println!("  BTreeMap         {{}} does not compile   {{:?}} -> {m:?}");
    println!();
    println!("  Every one of those has a Debug impl and no Display impl, for the same");
    println!("  reason Ballot needed a hand-written one: there is no single right way");
    println!("  to read a collection out loud. Comma-separated? One per line? With the");
    println!("  keys? std declines to guess, so `{{}}` on a Vec is error E0277.");

    let p = Path::new("/etc/ballots.yaml");
    println!("\n  Path is the case worth knowing:");
    println!("    {{:?}}        -> {p:?}");
    println!("    .display()   -> {}", p.display());
    println!("  A path is not guaranteed to be UTF-8, so Path CANNOT promise the");
    println!("  lossless string Display implies. `.display()` hands back a helper that");
    println!("  can -- by admitting it may substitute characters. The missing impl is");
    println!("  the type telling you something true.");
}

// ─────────────────────────────────────────────────────────── 5
fn what_each_trait_buys() {
    banner("Display pays a dividend Debug does not: ToString");

    let b = Ballot { voter: "Ada".to_string(), scores: [5, 2, 0] };
    let owned: String = b.to_string();
    println!("  b.to_string()      -> {owned:?}");
    println!("  format!(\"{{b:?}}\")   -> {:?}", format!("{b:?}"));
    println!("  (both shown with {{:?}} here, so you can see they really are Strings)");
    println!();
    println!("  impl<T: Display> ToString for T is a blanket impl in std, so writing");
    println!("  Display is what makes .to_string() exist. Debug has no such impl:");
    println!("  the Debug string is reachable only through format!(\"{{:?}}\").");

    println!("\n  And Debug pays a different one -- it is what the tooling is made of:");
    println!("    assert_eq!(a, b)   prints both sides on failure   => T: Debug");
    println!("    .unwrap() on Err   prints the error               => E: Debug");
    println!("    dbg!(x)            prints file:line and the value => T: Debug");
    println!("  which is why #[derive(Debug)] belongs on almost everything you write,");
    println!("  and why leaving it off is felt first by your tests.");
}

// ─────────────────────────────────────────────────────────── 6
/// Run `f` and hand back the panic message instead of dying. The hook is swapped
/// only so this program can print panics as data; it is not a pattern to copy.
/// (Same helper as the `expect` lesson.)
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

fn the_paths_that_pick_for_you() {
    banner("the delivery paths choose Debug, and they do not ask");

    let e = TallyError::ScoreTooHigh { got: 9, max: 5 };
    println!("  your Display sentence   -> {e}");
    println!("  your derived Debug      -> {e:?}");
    println!();

    let make = || -> Result<u8, TallyError> { Err(TallyError::ScoreTooHigh { got: 9, max: 5 }) };

    match caught(|| make().unwrap()) {
        Ok(_) => unreachable!(),
        Err(msg) => println!("  .unwrap()               -> PANIC: {msg}"),
    }
    match caught(|| make().expect("the ballot file was validated on load")) {
        Ok(_) => unreachable!(),
        Err(msg) => println!("  .expect(\"...\")          -> PANIC: {msg}"),
    }
    println!(
        "  fn main() -> Result     -> {}   (to stderr, exit 1)",
        format!("Error: {:?}", TallyError::ScoreTooHigh { got: 9, max: 5 })
    );
    println!();
    println!("  Three of the four use Debug. The one sentence written for a human is");
    println!("  the one the standard paths never reach -- and none of this is a");
    println!("  warning, a lint, or a compile error. It is just what gets printed.");
}

// ─────────────────────────────────────────────────────────── 7
fn what_derive_reaches() {
    banner("derived Debug prints every field it can reach");

    let s = Session { voter_id: 4711, token: "sk_live_9f3a2b8c1d".to_string() };
    println!("  #[derive(Debug)]  -> {s:?}");
    println!("                       ^ that is now in your log file, your CI output,");
    println!("                         and the panic message of any failing unwrap");

    let g = Guarded { voter_id: s.voter_id, token: s.token.clone() };
    println!("\n  hand-written      -> {g:?}   <- the same two fields");
    println!("  and it still honours {{:#?}}, because f.debug_struct() does that for you:");
    for line in format!("{g:#?}").lines() {
        println!("      {line}");
    }
    println!();
    println!("  The other half of the same rule: Debug output is NOT a stable format.");
    println!("  std reserves the right to change how any of its types debug-print, so a");
    println!("  program that parses a {{:?}} string has built on sand. If you need the");
    println!("  text to hold still, that is a Display impl -- or serde -- not Debug.");
}

fn main() {
    where_the_habit_forms();
    what_display_hides();
    derived_and_written();
    the_container_gap();
    what_each_trait_buys();
    the_paths_that_pick_for_you();
    what_derive_reaches();
}
