//! `Option` as a struct field: modelling what may legitimately be absent.
//!
//! The other lessons treat Option as something a function RETURNS. This one is
//! about Option in a type definition, which is where you meet it when modelling
//! data or deserializing someone else's JSON.
//!
//!   rustc --edition 2024 option_fields.rs -o /tmp/of && /tmp/of

fn banner(n: u32, title: &str) {
    println!("\n──── Step {n}: {title}");
}

// Every field is required unless its type says otherwise. `Option` is how a type
// says "this one may be absent" — and then the compiler holds every reader to it.
#[derive(Debug)]
struct Book {
    title: String,
    isbn: Option<String>,
    page_count: Option<u32>,
    tags: Vec<String>,
}

// ─────────────────────────────────────────────────────────── Step 1
fn step1() -> (Book, Book) {
    banner(1, "Required by default; Option marks the exceptions");

    let known = Book {
        title: "Great book".into(),
        isbn: Some("1-123-456".into()),
        page_count: Some(320),
        tags: vec!["rust".into(), "reference".into()],
    };
    let draft = Book {
        title: "Untitled draft".into(),
        isbn: None,
        page_count: None,
        tags: vec![],
    };

    println!("  known.isbn = {:?}   tags: {}", known.isbn, known.tags.join(", "));
    println!("  draft.isbn = {:?}   tags: {} (none)", draft.isbn, draft.tags.len());
    println!("      You cannot forget a field: leaving `isbn` out is a compile error, and");
    println!("      writing `None` is you SAYING it is absent rather than leaving it undefined.");
    (known, draft)
}

// ─────────────────────────────────────────────────────────── Step 2
fn describe(b: &Book) -> String {
    match &b.isbn {
        Some(i) => format!("{} — ISBN {i}", b.title),
        None => format!("{} — no ISBN on record", b.title),
    }
}

fn step2(known: &Book, draft: &Book) {
    banner(2, "Reading the field: match on a REFERENCE to it");

    println!("  {}", describe(known));
    println!("  {}", describe(draft));
    println!("      `match &b.isbn`, not `match b.isbn` — the latter tries to move a String");
    println!("      out of a borrowed struct, which the borrow checker refuses.");
}

// ─────────────────────────────────────────────────────────── Step 3
fn step3(known: &Book, draft: &Book) {
    banner(3, "The three one-liners you will actually use");

    // as_deref: Option<String> -> Option<&str>, without cloning
    let as_str: Option<&str> = known.isbn.as_deref();
    println!("  as_deref()          {as_str:?}");

    // map_or: transform, with a fallback for None
    println!("  map_or (known)      {}", known.page_count.map_or("unknown".into(), |n| format!("{n} pages")));
    println!("  map_or (draft)      {}", draft.page_count.map_or("unknown".to_string(), |n| format!("{n} pages")));

    // unwrap_or_default: for when empty and absent mean the same thing
    println!("  unwrap_or_default   {:?}", draft.isbn.clone().unwrap_or_default());

    // as_ref: borrow the inside so the original survives
    let len: Option<usize> = known.isbn.as_ref().map(|s| s.len());
    println!("  as_ref().map(len)   {len:?}   (known.isbn still owned: {:?})", known.isbn);
}

// ─────────────────────────────────────────────────────────── Step 4
// The distinction that decides whether a field should be Option at all.
#[derive(Debug)]
struct Survey {
    question: String,
    /// None = not collected yet. Some(vec![]) = collected, and nobody answered.
    responses: Option<Vec<u32>>,
}

fn step4() {
    banner(4, "Option<Vec<T>> — usually wrong, occasionally exactly right");

    let unsent = Survey { question: "Best method?".into(), responses: None };
    let ignored = Survey { question: "Best method?".into(), responses: Some(vec![]) };
    let answered = Survey { question: "Best method?".into(), responses: Some(vec![5, 3]) };

    println!("  question: {:?}", unsent.question);
    for s in [&unsent, &ignored, &answered] {
        let summary = match &s.responses {
            None => "not collected yet".to_string(),
            Some(v) if v.is_empty() => "collected — nobody answered".to_string(),
            Some(v) => format!("collected — {} responses", v.len()),
        };
        println!("  {summary}");
    }
    println!("      A plain Vec cannot tell those first two apart. If they mean the same thing");
    println!("      in your domain, use Vec and delete the Option; if they differ, keep it.");
}

// ─────────────────────────────────────────────────────────── Step 5
#[derive(Debug, Default)]
struct Config {
    name: Option<String>,
    retries: Option<u32>,
    verbose: bool,
}

fn step5() {
    banner(5, "Option<T> defaults to None for every T");

    println!("  Config::default() = {:?}", Config::default());

    let cfg = Config { retries: Some(3), ..Default::default() };
    println!("  one field set     = {cfg:?}");
    println!("      Option<T>: Default is None regardless of T — so a config struct of Options");
    println!("      derives Default even when the inner types cannot.");
    println!("  retries.unwrap_or(1)      = {}", cfg.retries.unwrap_or(1));
    println!("  name.unwrap_or(\"anon\")    = {}", cfg.name.unwrap_or_else(|| "anon".into()));
    println!("  verbose (a plain bool)   = {}   <- not optional: false IS the answer", cfg.verbose);
}

// ─────────────────────────────────────────────────────────── Step 6
#[derive(Debug)]
struct Person {
    first: String,
    last: String,
    age: Option<u8>,
}

fn step6() {
    banner(6, "What an optional field costs");

    let p = Person { first: "Ada".into(), last: "Lovelace".into(), age: None };
    println!("  {} {} — age {:?}", p.first, p.last, p.age);
    println!("  Person                 {:>3} bytes", size_of::<Person>());
    println!("  String (each)          {:>3}", size_of_val(&p.last));
    println!("  Option<u8>             {:>3}", size_of_val(&p.age));
    println!("  u8                     {:>3}", size_of::<u8>());
    println!("      u8 uses all 256 of its bit patterns, so there is no spare one for None");
    println!("      and Option<u8> must carry a separate tag byte. Compare Option<Box<T>>,");
    println!("      which is free because a null pointer was never a legal Box.");
}

fn main() {
    let (known, draft) = step1();
    step2(&known, &draft);
    step3(&known, &draft);
    step4();
    step5();
    step6();
    println!();
}
