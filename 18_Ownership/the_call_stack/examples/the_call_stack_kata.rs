//! Kata solution: three predictions about what a call does to memory.
//!
//! Every answer is a comparison. A printed address is different next run and
//! could not be an answer key -- and predicting one would teach nothing.
//!
//!   rustc --edition 2024 the_call_stack_kata.rs -o /tmp/tcsk && /tmp/tcsk

/// Part 1. Takes the String BY VALUE, and reports where it found it.
#[inline(never)]
fn receive(s: String) -> (usize, usize) {
    let header_at = &s as *const String as usize;
    let bytes_at = s.as_ptr() as usize;
    (header_at, bytes_at)
}

/// Part 2. A value that says when it is freed.
struct Marker(&'static str);

impl Drop for Marker {
    fn drop(&mut self) {
        println!("      [drop] {} freed", self.0);
    }
}

#[inline(never)]
fn consume(r: Marker) {
    println!("      consume() has it: {}", r.0);
} //                                        <- freed HERE

#[inline(never)]
fn pass_through(r: Marker) -> Marker {
    println!("      pass_through() has it: {}", r.0);
    r
} //                                        <- freed by whoever takes it

/// Part 3.
#[inline(never)]
fn here(anchor: &u8) -> usize {
    anchor as *const u8 as usize
}
#[inline(never)]
fn d4(f: &mut Vec<usize>) {
    let a = 0u8;
    f.push(here(&a));
}
#[inline(never)]
fn d3(f: &mut Vec<usize>) {
    let a = 0u8;
    f.push(here(&a));
    d4(f);
}
#[inline(never)]
fn d2(f: &mut Vec<usize>) {
    let a = 0u8;
    f.push(here(&a));
    d3(f);
}
#[inline(never)]
fn d1(f: &mut Vec<usize>) {
    let a = 0u8;
    f.push(here(&a));
    d2(f);
}

fn main() {
    println!("Part 1 — a String passed by value.\n");
    let text = String::from("the quick brown fox jumps over it");
    let header_before = &text as *const String as usize;
    let bytes_before = text.as_ptr() as usize;

    let (header_inside, bytes_inside) = receive(text);

    println!("    header at a different address inside the callee?  {}",
             header_before != header_inside);
    println!("    heap buffer relocated?                            {}",
             bytes_before != bytes_inside);
    println!("\n  Predicted right if you said yes and no. The 24-byte header was");
    println!("  copied into the callee's own slot; the text never moved, however");
    println!("  long it is. That asymmetry is why Rust can afford to move by");
    println!("  default -- a move is a fixed, small cost that does not grow with");
    println!("  the data. And `text` is unusable here now: it went IN.");

    println!("\nPart 2 — where the free happens.\n");
    println!("    (a) passed in, not returned:");
    consume(Marker("consumed"));
    println!("        ...and the free already happened, inside consume()");

    println!("\n    (b) passed in and returned:");
    let returned = pass_through(Marker("returned"));
    println!("        ...and nothing was freed: the value moved out through");
    println!("        the return slot, into `returned`, which now owns it");

    println!("\n  The frame is a place; the value is a thing. Ending a frame frees");
    println!("  whatever is still standing in it, and a returned value is not.");

    println!("\nPart 3 — four nested frames.\n");
    let anchor = 0u8;
    let main_frame = here(&anchor);
    let mut frames = Vec::new();
    d1(&mut frames);
    let mut distinct = frames.clone();
    distinct.sort_unstable();
    distinct.dedup();

    println!("    four distinct frames?                    {}", distinct.len() == 4);
    println!("    each below the frame that called it?     {}",
             frames.windows(2).all(|w| w[1] < w[0]));
    println!("    the whole chain under 4 KiB from main?   {}",
             main_frame - frames[3] < 4096);

    println!("\n  Which of those three is a claim about RUST?");
    println!("    None of them. All three are claims about this build.");
    println!("    Compile the same functions without #[inline(never)] and at -O");
    println!("    and the descending answer flips to false -- the four calls were");
    println!("    folded into one frame, so there is no `d3` frame to be below");
    println!("    `d2`. What survives optimization is the part the borrow checker");
    println!("    actually enforces: `d3`'s local cannot outlive `d3`, whether or");
    println!("    not `d3` still exists as a frame.");

    println!("\n  `returned` is still alive until here:");
    drop(returned);
}
