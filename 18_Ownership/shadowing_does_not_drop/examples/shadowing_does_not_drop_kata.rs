//! Kata solution: the value you can no longer free.
//!
//! A shadow removes a NAME. If the value behind that name is a 40 MB ballot
//! buffer and the shadow is a one-line summary of it, the buffer is still
//! there — held to the end of the scope, with nothing left that can reach it
//! to release it early. The instrument is the same as the lesson's: a value
//! that prints when it dies.
//!
//!   rustc --edition 2024 shadowing_does_not_drop_kata.rs -o /tmp/sdndk && /tmp/sdndk

struct Buffer {
    label: &'static str,
    bytes: Vec<u8>,
}

impl Buffer {
    fn load(label: &'static str) -> Self {
        Buffer { label, bytes: vec![5, 3, 0, 5, 4, 0, 2, 5] }
    }
    /// Borrows: the caller keeps the buffer.
    fn count_marked(&self) -> usize {
        self.bytes.iter().filter(|b| **b > 0).count()
    }
    /// Consumes: the buffer is moved in and dropped here.
    fn into_count_marked(self) -> usize {
        self.count_marked()
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        println!("    DROP {} ({} bytes freed)", self.label, self.bytes.len());
    }
}

fn long_unrelated_work(tag: &str) {
    println!("    ...doing {tag} work, during which nothing above is needed...");
}

fn banner(n: &str, title: &str) {
    println!("\n──── {n}: {title}");
}

fn main() {
    // ─────────────────────────────────────────────────────────── problem
    banner("The problem", "shadowed, so it cannot be released");
    {
        let ballots = Buffer::load("A: shadowed");
        let ballots = ballots.count_marked(); // usize now — buffer is nameless
        println!("    marked = {ballots}");
        long_unrelated_work("report");
        println!("    (the buffer is STILL alive right here)");
    }
    println!("      It was freed at the brace, after all the work — not after");
    println!("      the last line that needed it. Nothing was leaked; the");
    println!("      buffer was simply held far longer than the code implies.");

    // ─────────────────────────────────────────────────────────── 1
    banner("Fix 1", "drop it explicitly, BEFORE the shadow");
    {
        let ballots = Buffer::load("B: dropped early");
        let marked = ballots.count_marked();
        drop(ballots); // must happen while the name still means the buffer
        let ballots = marked;
        println!("    marked = {ballots}");
        long_unrelated_work("report");
    }
    println!("      Correct, and it relies on a human remembering. Move that");
    println!("      `drop` one line later and it silently drops the usize.");

    // ─────────────────────────────────────────────────────────── 2
    banner("Fix 2", "give it a scope that ends: the block expression");
    {
        let ballots = {
            let raw = Buffer::load("C: scoped");
            raw.count_marked()
        }; // raw dies HERE, structurally
        println!("    marked = {ballots}");
        long_unrelated_work("report");
    }
    println!("      The one to ship. The lifetime is stated by the shape of the");
    println!("      code rather than by a call you have to remember, and the");
    println!("      buffer is unreachable afterwards because it is out of scope,");
    println!("      not merely nameless.");

    // ─────────────────────────────────────────────────────────── 3
    banner("Fix 3", "hand it to something that consumes it");
    {
        let ballots = Buffer::load("D: moved in").into_count_marked();
        println!("    marked = {ballots}");
        long_unrelated_work("report");
    }
    println!("      Best when a real function already wants ownership: the move");
    println!("      makes the buffer the callee's problem, and it drops there.");

    // ─────────────────────────────────────────────────────────── 4
    banner("The trap", "drop() AFTER the shadow drops the wrong thing");
    {
        let ballots = Buffer::load("E: still held");
        // A String summary rather than a usize one — and that detail is the trap.
        let ballots = format!("{} marked", ballots.count_marked());
        drop(ballots); // a REAL drop, of the summary. Not a warning in sight.
        println!("    dropped `ballots`... and the buffer is still alive:");
        long_unrelated_work("report");
    }
    println!("      `drop` takes whatever the name means NOW, and after a shadow");
    println!("      that is the new binding. Here it freed a short String and");
    println!("      left the buffer untouched — it reads like a fix and is not");
    println!("      one.");
    println!("      Whether you get a warning depends on the shadow's type, which");
    println!("      is a thin thing to rely on. Shadow with a `usize` and rustc");
    println!("      catches it: `calls to std::mem::drop with a value that");
    println!("      implements Copy does nothing` (lint: dropping_copy_types).");
    println!("      Shadow with anything owned — a String, a Vec, a struct — and");
    println!("      the drop is genuine, so there is nothing for that lint to");
    println!("      say. The silent case is the one you will actually write.");

    println!("\n      The through-line: shadowing is a naming tool, not a memory");
    println!("      tool. When you want a value GONE, end its scope (fix 2) or");
    println!("      move it (fix 3); reach for `drop` only where neither shape");
    println!("      fits, and put it before the name changes meaning.");
}
