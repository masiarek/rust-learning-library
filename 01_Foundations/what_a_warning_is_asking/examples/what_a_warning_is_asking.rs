//! A warning is the compiler asking whether you meant it.
//!
//! `unused_variables` is the first one almost everybody meets, and the
//! suggested fix — "prefix it with an underscore" — is usually read as a way
//! to make the compiler shut up. It is not. `_name` and `_` are two different
//! answers to the question, and one of them changes when your value is
//! destroyed. Step 2 makes that visible.
//!
//!   rustc --edition 2024 what_a_warning_is_asking.rs -o /tmp/wawia && /tmp/wawia

fn banner(n: u8, title: &str) {
    println!("\n──── Step {n}: {title}");
}

// ── Step 2's instrument ──────────────────────────────────────────────────────
// A value that announces its own destruction, so "when does this drop?" stops
// being a claim on a page and becomes a line of output.

struct Guard(&'static str);

impl Drop for Guard {
    fn drop(&mut self) {
        println!("      DROP {}", self.0);
    }
}

// ── Step 3's instrument ──────────────────────────────────────────────────────
// `dead_code` is a different lint from `unused_variables`, and it fires on
// items rather than bindings. The `#[allow]` is the point of the step, not an
// apology for it: the attribute is how you answer a lint for one item.

#[allow(dead_code)]
fn never_called_but_allowed() -> u8 {
    42
}

fn main() {
    println!("What a warning is asking");

    // ── Step 1 ───────────────────────────────────────────────────────────────
    banner(1, "Three ways to not use a binding");

    let used = 10u8;
    println!("      let used     -> {used}, and no warning: it was read");

    let _computed = used * 2;
    println!("      let _computed -> computed on purpose, unused on purpose");

    let _ = used * 3;
    println!("      let _         -> discarded outright; there is no name to read");

    println!("      All three compile with zero warnings. They are not the same.");

    // ── Step 2 ───────────────────────────────────────────────────────────────
    banner(2, "The two underscores disagree about WHEN");

    println!("    (a) let _named = Guard(..)  — binds, lives to the end of scope");
    {
        let _named = Guard("a: _named");
        println!("      still inside the block");
    }

    println!("    (b) let _ = Guard(..)       — binds nothing, drops immediately");
    {
        let _ = Guard("b: bare _");
        println!("      still inside the block");
    }

    println!("      Read the order above: (a) dropped after its line, (b) before.");
    println!("      A mutex guard bound to `_` is unlocked before you use it.");

    // ── Step 3 ───────────────────────────────────────────────────────────────
    banner(3, "The four lint levels, and where you set them");

    for (level, effect) in [
        ("allow", "say nothing at all"),
        ("warn", "print it; compile anyway; exit 0"),
        ("deny", "print it as an error; compile fails"),
        ("forbid", "deny, and refuse any later allow of this lint"),
    ] {
        println!("      #[{level:<6}] {effect}");
    }
    println!("      Set on an item, a block, or the crate (`#![deny(...)]`),");
    println!("      or from outside with `-D warnings` / RUSTFLAGS in CI.");
    println!("      never_called_but_allowed() = {}", never_called_but_allowed());

    // ── Step 4 ───────────────────────────────────────────────────────────────
    banner(4, "A warning is not an error, and that is the trap");

    println!("      Every warning in this file's history still exited 0.");
    println!("      Nothing fails until someone chooses to make it fail —");
    println!("      `-D warnings` in CI is that choice, made once.");
    println!("      `cargo fix` applies the compiler's own suggestions, so");
    println!("      read them first: it will happily add an underscore where");
    println!("      the honest fix was to USE the variable.");
}
