//! Two of Rust's six comment forms are not comments.
//!
//! `//` and `/* */` are thrown away by the lexer: the compiler never sees them,
//! and you can write anything at all inside one. The other four — `///`, `//!`,
//! `/** */` and `/*! */` — are PARSED: they become `#[doc = "..."]` attributes,
//! they must be attached to something, and the code inside them is compiled and
//! run as a test.
//!
//! This very block is the third form. It documents the file it is inside, which
//! is why it has to come before any item — and why it is `//!` and not `///`.
//!
//!   rustc --edition 2024 comments_that_compile.rs -o /tmp/ctc && /tmp/ctc

/// Reveals what the compiler turned a doc comment INTO.
///
/// A `meta` fragment matches an attribute, and `stringify!` prints the tokens
/// it captured — so if `///` really does desugar to `#[doc = "..."]`, this
/// macro will say so in its own words rather than ours.
macro_rules! reveal_attrs {
    ($reveal:ident; $label:expr; $(#[$m:meta])* struct $name:ident;) => {
        #[allow(dead_code)]
        struct $name;
        fn $reveal() {
            println!("  {} carries:", $label);
            $( println!("      #[{}]", stringify!($m)); )*
        }
    };
}

reveal_attrs! {
    reveal_ballot;
    "`Ballot`";
    /// A ballot.
    /** The same thing, in the block form. */
    #[doc = "Written the long way."]
    struct Ballot;
}

fn banner(n: u32, title: &str) {
    println!("\n──── Step {n}: {title}");
}

fn main() {
    // ───────────────────────────────────────────────────────────── 1
    banner(1, "`//` is erased before the parser runs");
    // let x: = = = ;   <- not Rust, and not a problem
    /* fn ( ) ] } "unterminated                                       */
    println!("  The two lines above this one are syntactic garbage.");
    println!("  The program compiled anyway, so nothing ever parsed them.");
    println!("      That is what \"the compiler ignores comments\" means, and");
    println!("      it is true of exactly two of the six forms.");

    // ───────────────────────────────────────────────────────────── 2
    banner(2, "`///` is not erased — it becomes an attribute");
    reveal_ballot();
    println!("      The macro captured a `meta` fragment, so those are real");
    println!("      attributes, not text. `///` is sugar for `#[doc = \"...\"]`,");
    println!("      which is why the two forms sit side by side above and why");
    println!("      the leading space after the slashes is preserved verbatim.");

    // ───────────────────────────────────────────────────────────── 3
    banner(3, "Which way each one points");
    println!("  ///  documents the item BELOW it        (outer — points down)");
    println!("  //!  documents the item it is INSIDE    (inner — points out)");
    println!("      So the `//!` block at the top of this file documents the");
    println!("      file. Put it after an item and there is nothing enclosing");
    println!("      it to describe, which is a compile error, not a warning.");

    // ───────────────────────────────────────────────────────────── 4
    banner(4, "A misplaced doc comment: warning, or error, or neither");
    {
        // /// this one attaches to the println   <- uncomment for the warning
        println!("  There are three answers, not one, and the first surprises");
        println!("  people: a doc comment on a STATEMENT is only a warning —");
        println!("        warning: unused doc comment");
        println!("        ...rustdoc does not generate documentation for");
        println!("           macro invocations");
        println!("  It attached to something, so it parsed. Nothing will ever");
        println!("  read it, and the build still succeeds.");
        // /// nothing at all follows this one    <- uncomment for error[E0585]
    }
    println!("      Uncomment the LAST line of that block, where nothing");
    println!("      follows, and it stops being a warning:");
    println!("        error[E0585]: found a documentation comment that");
    println!("                      doesn't document anything");
    println!("        help: doc comments must come before what they document,");
    println!("              if a comment was intended use `//`");
    println!("      And an inner `//!` at item level, after an item has begun:");
    println!("        error[E0753]: expected outer doc comment");
    println!("      So the trap is not the error. The error tells you. The trap");
    println!("      is the WARNING — a `///` you wrote inside a function is not");
    println!("      documentation, it is a comment with extra steps, and the");
    println!("      only thing that says so is a line in the build log.");

    // ───────────────────────────────────────────────────────────── 5
    banner(5, "Block comments nest, unlike C's");
    /* outer /* inner */ still inside the outer one */
    println!("  /* outer /* inner */ still commented */ compiles here.");
    println!("      In C the first `*/` ends it and the rest is a syntax error,");
    println!("      so commenting out a region that already contains a comment");
    println!("      breaks. In Rust the lexer counts the pairs, so it works.");

    // ───────────────────────────────────────────────────────────── 6
    banner(6, "The block DOC forms exist, and one character breaks them");
    println!("  /** ... */  is `///` with a block delimiter (outer)");
    println!("  /*! ... */  is `//!` with a block delimiter (inner)");
    println!("      Step 2 above proves it: the `/** */` line desugared to the");
    println!("      same `#[doc = ...]` as the slashes did. rustdoc renders the");
    println!("      two identically, and even strips a leading `*` column, so");
    println!("      the Javadoc habit costs nothing on the rendered page.");
    println!("  What it costs is here — a doc example that mentions `*/`:");
    println!("        /** Strips a C comment.");
    println!("");
    println!("        ```");
    println!("        let s = \"/* hi */\";");
    println!("        assert!(s.ends_with(\"*/\"));");
    println!("        ```");
    println!("        */");
    println!("      The comment ENDS at the `*/` inside the string, four lines");
    println!("      early. Everything after it is parsed as code, and the errors");
    println!("      land wherever the wreckage happens to stop:");
    println!("        error: prefix `B` is unknown");
    println!("        error[E0765]: unterminated double quote string");
    println!("      — reported on a `println!` further down the file that has");
    println!("      nothing wrong with it. A `///` block cannot do this: it ends");
    println!("      at the newline, so no content can terminate it early.");
    println!("      That is the whole case for the line forms, and it is why");
    println!("      you will almost never see a `/** */` in real Rust.");

    // ───────────────────────────────────────────────────────────── 7
    banner(7, "The reason any of this matters: doc comments are tested");
    println!("  A fenced block inside `///` is compiled and run by `cargo test`:");
    println!("        /// ```");
    println!("        /// assert_eq!(doubled(3), 6);");
    println!("        /// ```");
    println!("      Write 7 there and the test suite fails. So the examples in");
    println!("      your documentation cannot rot into ones that no longer");
    println!("      compile — the one kind of comment that is checked at all.");
}
