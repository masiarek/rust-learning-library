//! What `dbg!` does that `println!("{:?}")` does not.
//!
//!   rustc --edition 2024 what_dbg_does.rs -o /tmp/wdd && /tmp/wdd
//!
//! NOTE: every `dbg!` line below goes to STDERR. This page's recorded output
//! captures stdout only, which is exactly the lesson in section 3 — so run the
//! program yourself to see them, or the page will look like they never fired.

#[derive(Debug, Clone)]
struct Ballot {
    voter: String,
    score: u8,
}

fn main() {
    println!("1. `dbg!` returns its argument, and that is the whole point");
    let doubled = dbg!(2 + 3) * 10;
    println!("   let doubled = dbg!(2 + 3) * 10;   ->  {doubled}");
    println!("   It evaluated to 5 and handed it straight back, so you can wrap any");
    println!("   sub-expression without restructuring the code around it.");
    println!("   println! returns (), so the same move would not compile.");

    println!("\n2. It prints three things, not one");
    println!("   file:line:col, the EXPRESSION SOURCE TEXT, and the value:");
    println!("       [what_dbg_does.rs:19:19] 2 + 3 = 5");
    println!("   `2 + 3` is not a string you passed — the macro captured the source.");
    println!("   That is why `dbg!(x)` beats `println!(\"x = {{:?}}\", x)`: the label");
    println!("   cannot go stale when you rename x.");

    println!("\n3. It writes to STDERR");
    let b = Ballot { voter: "Ada".into(), score: 5 };
    dbg!(&b);
    println!("   A dbg! line just fired above and you may not see it here — `2>/dev/null`");
    println!("   hides it, and a pipe reorders it, because stdout is block-buffered when");
    println!("   piped while stderr never is.");
    println!("   Practical consequence: `cargo run > out.txt` keeps your program's real");
    println!("   output clean and leaves the debugging on the terminal. In THIS repo it");
    println!("   means run_examples.py cannot record dbg! output at all — it captures");
    println!("   stdout only, so a lesson must print with println! to have an answer key.");

    println!("\n4. It formats with `{{:#?}}`, always");
    println!("   dbg! is hard-wired to the alternate (pretty) form, one field per line.");
    println!("   For a derived Debug that is a gift. For a HAND-WRITTEN one it is a trap:");
    println!("   if your impl never asks f.alternate(), `{{:?}}` and `{{:#?}}` print the");
    println!("   same thing, and dbg! silently gets the flat version.");

    println!("\n5. It MOVES a non-Copy argument");
    let owned = Ballot { voter: "Ben".into(), score: 3 };
    let owned = dbg!(owned); // moved in, handed back — rebinding keeps it alive
    println!("   dbg!(owned) took ownership. It gives the value back, so `let x = dbg!(x)`");
    println!("   is fine — but a bare `dbg!(owned);` on its own line drops it, and the");
    println!("   next use is E0382. `dbg!(&owned)` is the habit: borrow, print, move on.");
    println!("   still here: {} scored {}", owned.voter, owned.score);

    println!("\n6. It is NOT removed in release builds");
    println!("   Unlike debug_assert!, dbg! has no cfg gate. Compile with -O and it still");
    println!("   prints. It is a thing you delete, not a logging macro you leave in.");
    println!("   (`cargo build --release` will not save you; a code review has to.)");

    println!("\n7. The confusion worth naming: field vs whole value");
    println!("   dbg!(b.score) works with no derive, because u8 implements Debug.");
    println!("   dbg!(b) needs Ballot to implement it. These four are the SAME error:");
    println!("       dbg!(named_struct)        println!(\"{{:?}}\", named_struct)");
    println!("       dbg!(unit_struct)         println!(\"{{:?}}\", unit_struct)");
    println!("   A unit struct only LOOKS stricter — it has no field to name, so the");
    println!("   lenient move is not available. There is no unit-struct rule, and no");
    println!("   dbg!-specific rule: whatever you NAME must implement Debug.");
    dbg!(b.score);
    println!("   (that dbg! fired on stderr too — a u8, no derive needed)");
}
