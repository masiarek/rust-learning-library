//! What makes a type an error: two traits you must write, one method you should.
//!
//!   rustc --edition 2024 the_error_trait.rs -o /tmp/tet && /tmp/tet

use std::error::Error;
use std::fmt;
use std::io;

// ---------------------------------------------------------------- the minimum

/// Everything `Error` requires is here: `Debug` (derived) and `Display`.
/// `source()` is not overridden, so this error is the end of its own chain.
#[derive(Debug)]
struct QuorumNotMet {
    cast: u32,
    needed: u32,
}

impl fmt::Display for QuorumNotMet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // lowercase, no trailing period, no "error:" prefix — it gets embedded
        write!(f, "quorum not met: {} ballots cast, {} needed", self.cast, self.needed)
    }
}

impl Error for QuorumNotMet {}

// ------------------------------------------------------- an error with a cause

#[derive(Debug)]
struct SheetUnreadable {
    path: String,
    cause: io::Error,
}

impl fmt::Display for SheetUnreadable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // names only what THIS layer knew: the path. Not the cause's message.
        write!(f, "could not read the tally sheet {}", self.path)
    }
}

impl Error for SheetUnreadable {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.cause)
    }
}

/// The same struct with `source()` left at its default, to show the difference.
#[derive(Debug)]
struct SheetUnreadableSilent {
    path: String,
    #[allow(dead_code)] // held, but never handed on — that is the point
    cause: io::Error,
}

impl fmt::Display for SheetUnreadableSilent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "could not read the tally sheet {}", self.path)
    }
}

impl Error for SheetUnreadableSilent {}

/// The whole chain, printed the way a program should print one. On stable this
/// is a hand-written loop: `Error::sources()` is still unstable (error_iter).
fn report(e: &dyn Error) -> String {
    let mut out = e.to_string();
    let mut cursor = e.source();
    while let Some(inner) = cursor {
        out.push_str(&format!("\n     caused by: {inner}"));
        cursor = inner.source();
    }
    out
}

fn missing(path: &str) -> SheetUnreadable {
    SheetUnreadable {
        path: path.to_string(),
        cause: io::Error::new(io::ErrorKind::NotFound, "no such file or directory"),
    }
}

fn main() {
    println!("1. The trait, in full");
    println!("   pub trait Error: Debug + Display {{");
    println!("       fn source(&self) -> Option<&(dyn Error + 'static)> {{ None }}");
    println!("   }}");
    println!("   That is all of the stable surface. Two supertraits you must satisfy,");
    println!("   one method with a default. `impl Error for T {{}}` is a real impl.");

    println!("\n2. The two supertraits are two audiences");
    let q = QuorumNotMet { cast: 41, needed: 60 };
    println!("   Display  {{}}    {q}");
    println!("   Debug    {{:?}}  {q:?}");
    println!("   Display is the sentence a person reads. Debug is what `unwrap`");
    println!("   prints, and what the runtime prints for an `Err` out of `main`.");
    println!("   Writing only one of them is not a shortcut — it is E0277.");

    println!("\n3. `source()` is what makes an error chain");
    let told = missing("ballots.txt");
    let silent = SheetUnreadableSilent {
        path: "ballots.txt".to_string(),
        cause: io::Error::new(io::ErrorKind::NotFound, "no such file or directory"),
    };
    println!("   with source():    {}", report(&told));
    println!("   without source(): {}", report(&silent));
    println!("   Same struct, same fields, same Display. The second one is holding");
    println!("   the cause and not handing it on, so the reader is told the WHAT");
    println!("   and never the WHY. Overriding source() is four lines.");

    println!("\n4. Which is why Display names one layer only");
    println!("   each layer's Display: {}", told);
    println!("   the cause's Display:  {}", told.source().unwrap());
    println!("   If the outer Display had written \"could not read ballots.txt: no");
    println!("   such file or directory\", the chain would print that sentence and");
    println!("   then repeat its second half. Say what only THIS layer knew.");

    println!("\n5. The house style for a Display, and why it is not taste");
    println!("   lowercase, no trailing period, no \"error:\" prefix:");
    println!("     good  \"{}\"", QuorumNotMet { cast: 41, needed: 60 });
    println!("     bad   \"Error: Quorum not met.\"");
    println!("   The bad one composes into: \"error: Error: Quorum not met.: ...\"");
    println!("   std follows the same rule: {}", io::Error::new(io::ErrorKind::NotFound, "no such file or directory"));

    println!("\n6. What implementing Error buys you");
    let boxed: Box<dyn Error> = Box::new(missing("ballots.txt"));
    println!("   Box<dyn Error> from any Error, via a blanket From impl:");
    println!("     {boxed}");
    println!("   so `?` accepts your type in any fn returning Box<dyn Error>.");
    let recovered = boxed.downcast_ref::<SheetUnreadable>();
    println!("   and downcast_ref gets the type back: {:?}", recovered.map(|e| &e.path));
    println!("   downcast to the wrong type: {:?}", boxed.downcast_ref::<QuorumNotMet>().is_some());

    println!("\n7. What it does NOT buy you");
    println!("   No automatic chain printing: `{{}}` on an error prints ONE layer.");
    println!("   The loop in `report()` above is what every program writes, because");
    println!("   Error::sources() is still unstable (error_iter, issue 58520).");
    println!("   No backtrace on stable, no automatic context, and no `From` impls");
    println!("   — those are the three gaps thiserror and anyhow exist to fill.");
}
