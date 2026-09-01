//! `r#name` — the escape that lets a keyword be an ordinary name.
//!
//! Rust's keyword list is not fixed: it is a property of the EDITION. `async`
//! was an ordinary name until 2018, `gen` until 2024. Raw identifiers are what
//! keeps a 2015 crate that exported `fn async()` callable from code written
//! under a later edition, and they are why adding a keyword to the language
//! is not a breaking change.
//!
//!   rustc --edition 2024 raw_identifiers.rs -o /tmp/ri && /tmp/ri

/// `match` is a keyword. `r#match` is a function.
fn r#match(r#type: u8) -> u8 {
    r#type + 1
}

/// `r#` on a name that was never a keyword. Legal, and a no-op.
fn r#ordinary() -> &'static str {
    "same function, either way"
}

/// A field named after a keyword — the usual reason to reach for this.
struct Header {
    r#type: &'static str,
    len: usize,
}

/// A lifetime may be raw too, for exactly the same reason.
fn longest<'r#fn>(a: &'r#fn str, b: &'r#fn str) -> &'r#fn str {
    if a.len() >= b.len() { a } else { b }
}

fn banner(n: u32, title: &str) {
    println!("\n──── Step {n}: {title}");
}

fn main() {
    // ───────────────────────────────────────────────────────────── 1
    banner(1, "A keyword is not a name, and rustc says how to make it one");
    println!("  let type = 1u8;");
    println!("        error: expected identifier, found keyword `type`");
    println!("        help: escape `type` to use it as an identifier");
    println!("           |     let r#type = 1u8;");
    println!("           |         ++");
    println!("      The fix is in the error. `r#` is not obscure trivia — it is");
    println!("      the first thing the compiler suggests.");

    // ───────────────────────────────────────────────────────────── 2
    banner(2, "`r#` works on items, bindings, fields and lifetimes");
    let r#type = 4u8;
    let h = Header { r#type: "text/csv", len: 3 };
    println!("  fn r#match(r#type: u8)      r#match({}) = {}", r#type, r#match(r#type));
    println!("  struct Header {{ r#type }}    {} ({} fields set)", h.r#type, h.len);
    println!("  fn longest<'r#fn>(..)       {:?}", longest("Ada", "Cara"));
    println!("      A raw lifetime is `'r#name`, not `r#'name` — the tick comes");
    println!("      first, because what is being escaped is the identifier.");

    // ───────────────────────────────────────────────────────────── 3
    banner(3, "`r#` is an escape, not part of the name");
    println!("  fn r#ordinary() declared once, called two ways:");
    println!("      ordinary()   -> {}", ordinary());
    println!("      r#ordinary() -> {}", r#ordinary());
    println!("      One function, two spellings, no ambiguity — so `r#` on a");
    println!("      non-keyword is legal and does nothing. What it escapes is");
    println!("      the PARSER, not the name.");
    println!("  One place the distinction leaks: a macro re-printing the token");
    println!("      stringify!(r#type)     = {}", stringify!(r#type));
    println!("      stringify!(r#ordinary) = {}", stringify!(r#ordinary));
    println!("      keeps the prefix, because it is printing what you wrote.");

    // ───────────────────────────────────────────────────────────── 4
    banner(4, "Four names refuse it");
    println!("  r#crate  r#self  r#Self  r#super");
    println!("        error: `crate` cannot be a raw identifier");
    println!("      These four are not keywords in the ordinary sense — they are");
    println!("      PATHS, resolved by the module system rather than parsed as");
    println!("      names, so there is nothing an escape could turn them into.");
    println!("      Every other keyword takes `r#`, including `r#async`,");
    println!("      `r#dyn`, `r#gen`, `r#union` and `r#macro_rules`.");

    // ───────────────────────────────────────────────────────────── 5
    banner(5, "Do not confuse it with the raw STRING");
    let raw_string = r#"a "quoted" path: C:\tmp"#;
    println!("  r#foo      a raw IDENTIFIER — a name that was a keyword");
    println!("  r#\"...\"#   a raw STRING — a literal with no escape sequences");
    println!("      {raw_string}");
    println!("      Same letter, unrelated features. The tell is what follows");
    println!("      the `#`: a quote means string, a letter means identifier.");
}
