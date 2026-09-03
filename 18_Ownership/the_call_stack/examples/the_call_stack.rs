//! What a call does to memory: it reserves a region, moves the arguments into
//! it, and releases the whole region on return.
//!
//! Note what this program does NOT print: an address. Every line is a
//! COMPARISON, because a raw address differs on every run and could never be
//! an answer key. What is stable is the *relationship* between two of them.
//!
//! `#[inline(never)]` on the four level functions is not decoration. Without
//! it an optimizing build folds all four into one frame and the addresses stop
//! descending -- which is the footnote at the bottom of the lesson, run in
//! `optimized/inlined_away.rs` rather than pretended about here.
//!
//!   rustc --edition 2024 the_call_stack.rs -o /tmp/tcs && /tmp/tcs

use std::fmt;

/// A value that announces its own death, so the moment a frame lets go of it
/// is visible rather than asserted.
struct Counter(u32);

impl fmt::Display for Counter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Counter({})", self.0)
    }
}

impl Drop for Counter {
    fn drop(&mut self) {
        println!("       [drop] Counter({}) freed", self.0);
    }
}

/// Where is this frame? One address inside the region this call was given.
fn here(anchor: &u8) -> usize {
    anchor as *const u8 as usize
}

/// Takes its argument BY VALUE, so `x` is a local variable of this call.
#[inline(never)]
fn report(x: Counter, caller_frame: usize) -> u32 {
    let anchor = 0u8;
    let mine = here(&anchor);
    println!("   inside report: x = {x}");
    println!("   my frame is below the caller's?  {}", mine < caller_frame);
    println!("   x is a local of THIS call, and drops here unless it leaves");
    x.0
}

/// The same shape, but the value leaves by the return path instead of dropping.
#[inline(never)]
fn bump(mut x: Counter) -> Counter {
    x.0 += 100;
    x // moved into a slot the CALLER provided; not freed here
}

#[inline(never)]
fn level_4(frames: &mut Vec<usize>) {
    let anchor = 0u8;
    frames.push(here(&anchor));
}
#[inline(never)]
fn level_3(frames: &mut Vec<usize>) {
    let anchor = 0u8;
    frames.push(here(&anchor));
    level_4(frames);
}
#[inline(never)]
fn level_2(frames: &mut Vec<usize>) {
    let anchor = 0u8;
    frames.push(here(&anchor));
    level_3(frames);
}
#[inline(never)]
fn level_1(frames: &mut Vec<usize>) {
    let anchor = 0u8;
    frames.push(here(&anchor));
    level_2(frames);
}

fn main() {
    let anchor = 0u8;
    let main_frame = here(&anchor);

    println!("1. A call reserves a region, and it is not the caller's");
    let a = Counter(1);
    let n = report(a, main_frame); //         `a` is MOVED into the parameter slot
    println!("   report returned {n}; `a` is gone from main, because it went IN");
    println!("   and the drop line above ran inside report, before it returned");

    println!("\n2. Nesting: every further call is another region, below the last");
    let mut frames = Vec::new();
    level_1(&mut frames);
    let mut distinct = frames.clone();
    distinct.sort_unstable();
    distinct.dedup();
    println!("   four nested calls, four distinct frames:  {}", distinct.len());
    println!("   each one below the frame that called it:  {}",
             frames.windows(2).all(|w| w[1] < w[0]));
    println!("   the whole chain fits in under 4 KiB:      {}",
             main_frame - frames[3] < 4096);
    println!("   ...and all four are released by the time level_1 returns.");

    println!("\n3. A parameter is a local, so it can leave by the return path");
    let b = Counter(7);
    let bumped = bump(b); //               in by move, out by move: no drop yet
    println!("   bump returned {bumped} and freed nothing on the way out");
    println!("   the VALUE outlived the frame it was standing in; the frame did not");

    println!("\n4. A block is not a frame, but it ends a name just the same");
    {
        let c = Counter(9);
        println!("   c = {c}, alive inside this block");
    }
    println!("   past the brace the name is gone, and the value went with it");

    println!("\n5. Why `&x` is what forces a slot to exist at all");
    let small: u8 = 3;
    let small_at = here(&small);
    println!("   small = {small}, and size_of::<u8>() = {}", size_of::<u8>());
    println!("   a byte can live entirely in a register and never touch the stack;");
    println!("   asking for its address is what obliges the compiler to give it one.");
    println!("   `&small` landed inside main's own frame:  {}",
             small_at.abs_diff(main_frame) < 4096);
    println!("   so `&u8` costs {} bytes to hold a value that is {}.",
             size_of::<&u8>(), size_of::<u8>());

    println!("\n   main ends here, and `bumped` drops last:");
}
