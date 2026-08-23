//! Kata solution: three bugs about *when*, none of them about *what*.
//!
//! Part 1 is a report that writes its totals after closing the file it was
//! writing them to, because struct fields drop in declaration order.
//! Part 2 is a guard released one identifier early, and the counter that
//! proves the critical section ran unprotected.
//! Part 3 is a borrow that outlives its last use, fixed three ways, none of
//! which is `.clone()`.
//!
//!   rustc --edition 2024 scope_is_about_names_kata.rs -o /tmp/scopek && /tmp/scopek

use std::cell::Cell;
use std::rc::Rc;

fn banner(title: &str) {
    println!("\n──── {title}");
}

// ---------------------------------------------------------------- Part 1 ---

struct FileHandle(&'static str);

impl Drop for FileHandle {
    fn drop(&mut self) {
        println!("      [{}] file closed", self.0);
    }
}

struct Totals(&'static str);

impl Drop for Totals {
    fn drop(&mut self) {
        println!("      [{}] totals written", self.0);
    }
}

/// The bug: `file` is declared first, so it is dropped first.
struct BrokenReport {
    _file: FileHandle,
    _totals: Totals,
}

/// The fix, and it is only a reordering: what must happen first is declared first.
struct FixedReport {
    _totals: Totals,
    _file: FileHandle,
}

fn the_report_that_closed_too_early() {
    banner("1. Fields drop in DECLARATION order, not in reverse");

    println!("    broken — `_file` first, so the file closes before the write:");
    {
        let _report = BrokenReport {
            _file: FileHandle("broken"),
            _totals: Totals("broken"),
        };
    }

    println!("    fixed — `_totals` first, and nothing else changed:");
    {
        let _report = FixedReport {
            _totals: Totals("fixed"),
            _file: FileHandle("fixed"),
        };
    }

    println!("      The reverse-order rule you learned for locals is not the rule");
    println!("      for fields. Two locals in the same order would have worked;");
    println!("      moving them into one struct silently flipped the sequence.");
}

// ---------------------------------------------------------------- Part 2 ---

/// A hand-rolled guard: it counts itself in on creation and out on drop.
struct Guard {
    held: Rc<Cell<u32>>,
}

impl Drop for Guard {
    fn drop(&mut self) {
        self.held.set(self.held.get() - 1);
    }
}

struct Registry {
    held: Rc<Cell<u32>>,
}

impl Registry {
    fn new() -> Self {
        Registry {
            held: Rc::new(Cell::new(0)),
        }
    }

    fn lock(&self) -> Guard {
        self.held.set(self.held.get() + 1);
        Guard {
            held: Rc::clone(&self.held),
        }
    }
}

fn the_guard_released_one_character_early() {
    banner("2. `let _` is not a name, so it holds nothing");

    let registry = Registry::new();

    {
        let _ = registry.lock();
        println!(
            "      let _      = registry.lock();   guards held here: {}",
            registry.held.get()
        );
    }

    {
        let _guard = registry.lock();
        println!(
            "      let _guard = registry.lock();   guards held here: {}",
            registry.held.get()
        );
    }

    println!("      Zero, then one. The first critical section was unprotected,");
    println!("      and it is the spelling an `unused variable` warning suggests.");
    println!("      With std's Mutex, rustc refuses the first line outright:");
    println!("          error: non-binding let on a synchronization lock");
    println!("          note: `#[deny(let_underscore_lock)]` on by default");
    println!("      That deny is a special case for std's locks, not for the");
    println!("      pattern — your own guard, above, drew no diagnostic at all.");
}

// ---------------------------------------------------------------- Part 3 ---

fn the_borrow_that_ended_too_late() {
    banner("3. Three ways to end a borrow sooner, and none of them clone");

    println!("      The starting point does not compile:");
    println!("          let leader = &standings[0];");
    println!("          standings.push(9);            <- E0502, borrow still live");
    println!("          println!(\"{{leader}}\");         <- because of THIS line");

    // Fix A — move the last use up, so the borrow ends before the mutation.
    let mut standings = vec![41_u32, 12];
    let leader = &standings[0];
    println!("      A) read first:        leader = {leader}");
    standings.push(9);
    println!("         then mutate:       {standings:?}");

    // Fix B — give the borrow a block, so it cannot reach the mutation.
    let mut standings = vec![41_u32, 12];
    {
        let leader = &standings[0];
        println!("      B) borrow in a block: leader = {leader}");
    }
    standings.push(9);
    println!("         mutate after it:   {standings:?}");

    // Fix C — take a copy, so there is no borrow to end.
    let mut standings = vec![41_u32, 12];
    let leader = standings[0]; //           u32 is Copy: this is a read, not a borrow
    standings.push(9);
    println!("      C) copy the value:    leader = {leader}, {standings:?}");

    println!("      A is the one to ship: it is the same program with one line");
    println!("      moved, and it says the read happens before the write.");
    println!("      B is for when the borrow's users are a group, not a line.");
    println!("      C changes the meaning — `leader` is now a snapshot, and on a");
    println!("      non-Copy type its spelling is `.clone()`, with the cost that");
    println!("      implies. Reach for it when you wanted a snapshot anyway.");
}

fn main() {
    the_report_that_closed_too_early();
    the_guard_released_one_character_early();
    the_borrow_that_ended_too_late();
}
