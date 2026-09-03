//! A shadow does not drop: the name is not the storage.
//!
//! Shadowing is usually explained as "the new one hides the old". True, and it
//! leaves the interesting question unasked: what happened to the OLD VALUE? The
//! answer is nothing at all. It is still alive, still owned, still borrowable —
//! it has merely lost the only name that could reach it.
//!
//! Every claim below is made by a value that announces its own death, so "when
//! was this freed?" stops being a matter of opinion.
//!
//!   rustc --edition 2024 shadowing_does_not_drop.rs -o /tmp/sdnd && /tmp/sdnd

/// A value whose `Drop` prints. This is the whole instrument.
struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("  DROP {}", self.0);
    }
}

fn banner(n: u32, title: &str) {
    println!("\n──── Step {n}: {title}");
}

fn main() {
    // ───────────────────────────────────────────────────────────── 1
    banner(1, "The program that raises the question");
    {
        let s = String::from("something important");
        let keep = &s; // borrow the first `s`
        let s = 5; // shadow it with an i32
        println!("  s = {s}, but the String is still there: {keep}");
    }
    println!("      `keep` still reads it, so the String was neither moved nor");
    println!("      freed. The shadow took away a NAME, not a value.");

    // ───────────────────────────────────────────────────────────── 2
    banner(2, "Where the value actually dies");
    {
        let item = Noisy("first  — the shadowed one");
        let keep = &item;
        let item = Noisy("second — the shadow");
        println!("  both alive: keep -> {}, item -> {}", keep.0, item.0);
        println!("      Nothing has dropped yet. Watch the brace:");
    }
    println!("      Drop runs in REVERSE declaration order, so the shadow dies");
    println!("      first and the value it hid dies last. The shadowed value");
    println!("      outlived the thing that shadowed it.");

    // ───────────────────────────────────────────────────────────── 3
    banner(3, "The cost: a shadowed value cannot be released early");
    {
        let buffer = Noisy("the original — soon unreachable");
        println!("  before the shadow, the original has a name: {}", buffer.0);
        let buffer = Noisy("the shadow");
        drop(buffer); // drops the SHADOW: the name resolves to the newest binding
        println!("  drop(buffer) took the shadow, as the name says it must.");
        println!("      The original is still alive with no name left to reach");
        println!("      it — it cannot be used, moved, or dropped until the brace:");
    }
    println!("      So shadowing is not a way to release something early. It is");
    println!("      the opposite: it removes the handle you would have needed.");

    // ───────────────────────────────────────────────────────────── 4
    banner(4, "An inner-block shadow ends; the outer value never moved");
    {
        let item = Noisy("outer");
        {
            let item = Noisy("inner");
            println!("  inside the block -> {}", item.0);
        }
        println!("  after the block  -> {}   the outer value was never touched", item.0);
    }
    println!("      Two different mechanisms that look alike: the inner binding");
    println!("      DIED at its brace, while a same-scope shadow merely goes");
    println!("      nameless and lives to the end of the enclosing scope.");

    // ───────────────────────────────────────────────────────────── 5
    banner(5, "What the borrow checker is guaranteeing while this happens");
    {
        let s = String::from("something important");
        let keep = &s;
        // drop(s);                        // <- uncomment for the error below
        let s = 5;
        println!("  s = {s}, keep = {keep}");
    }
    println!("      Uncomment that `drop(s)` and rustc declines:");
    println!("        error[E0505]: cannot move out of `s` because it is borrowed");
    println!("      THAT is the guarantee. Not that shadowing is harmless, but");
    println!("      that no reachable path frees the String while `keep` lives.");

    // ───────────────────────────────────────────────────────────── 6
    banner(6, "The same shape in C, where the name IS the storage");
    println!("  C rejects `int s = 5;` after `char *s` in one block outright:");
    println!("        error: redefinition of 's' with a different type");
    println!("  ...and it is not the type change. `int x = 1; int x = 2;` in one");
    println!("  block is `error: redefinition of 'x'` too. A name belongs to its");
    println!("  block, once. Shadowing needs new braces, and -Wshadow warns.");
    println!("      C++ answers identically, and adds destructors — so a");
    println!("      std::string really does die at its closing brace, the way");
    println!("      Noisy does above. What neither language adds is Step 5:");
    println!("      nothing checks that the alias dies before the value does.");
}
