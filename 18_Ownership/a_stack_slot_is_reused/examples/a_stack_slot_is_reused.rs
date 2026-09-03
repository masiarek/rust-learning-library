//! The region a returned call occupied is not cleared. It is handed to the
//! next call, so one address holds a Point now and something else in a moment.
//!
//! Every claim below is a comparison between two addresses taken in the same
//! run. The addresses themselves differ every run and are never printed.
//!
//! What this program deliberately does NOT do is READ the old bytes. That is
//! undefined behaviour, not a demonstration -- and the fact that it cannot be
//! written in safe Rust is the lesson, not a limitation of the example.
//!
//!   rustc --edition 2024 a_stack_slot_is_reused.rs -o /tmp/assir && /tmp/assir

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

/// Returns where its own local sat, so two calls can be compared.
#[inline(never)]
fn plot(x: u32, y: u32) -> (usize, u32) {
    let p = Point { x, y };
    let at = &p as *const Point as usize;
    (at, p.x * 1000 + p.y)
}

/// A different type, a different size, called from the same place.
#[inline(never)]
fn sum_of(values: [u64; 3]) -> (usize, u64) {
    let total: u64 = values.iter().sum();
    let at = &total as *const u64 as usize;
    (at, total)
}

/// Deeper than its neighbours, so the region it uses is released again.
#[inline(never)]
fn deep(n: u32) -> usize {
    let local = n;
    if n > 0 {
        return deep(n - 1);
    }
    &local as *const u32 as usize
}

/// A value that says when it stops being valid.
struct Loud(&'static str);

impl Drop for Loud {
    fn drop(&mut self) {
        println!("       [drop] {} is no longer a valid value", self.0);
    }
}

fn main() {
    println!("1. The same call, twice: the same address");
    let (first_at, first) = plot(7, 42);
    let (second_at, second) = plot(9, 13);
    println!("   call 1 produced {first}, call 2 produced {second}");
    println!("   both locals lived at the same address:  {}", first_at == second_at);
    println!("   the first Point was not moved aside. Its region was reissued.");

    println!("\n2. A different type, from the same place: the same region");
    let (sum_at, total) = sum_of([10, 20, 30]);
    println!("   sum_of returned {total}, from a u64 rather than a Point");
    println!("   within 256 bytes of where the Points were:  {}",
             sum_at.abs_diff(first_at) < 256);
    println!("   the bytes that spelled a Point now spell something else entirely");

    println!("\n3. Depth is reused too, not just the top");
    let bottom = deep(20);
    let (again_at, _) = plot(1, 1);
    println!("   a 21-frame call bottomed out well below the shallow ones: {}",
             bottom < again_at);
    println!("   and the next shallow call still lands where the others did:  {}",
             again_at == first_at);
    println!("   so the depth a program reached is invisible afterwards --");
    println!("   the stack pointer went back, and nothing was erased on the way");

    println!("\n4. Four ways a location stops holding a valid value");
    println!("   (a) the frame was released:");
    println!("       plot() returned, and its Point's slot became free real estate");
    println!("   (b) the value was moved out:");
    let owned = Loud("moved-away");
    let moved = owned; //           `owned` names nothing now; the slot is stale
    println!("       `owned` was moved into `moved`; the old slot holds bytes,");
    println!("       and the compiler will not let you read them");
    println!("   (c) the value was dropped at the end of its scope:");
    {
        let _short = Loud("block-scoped");
        println!("       alive inside the block...");
    }
    println!("   (d) the slot was reused by a later call -- parts 1 to 3 above.");

    println!("\n5. Why the compiler cares");
    println!("   All four leave an ADDRESS that is still perfectly readable and no");
    println!("   longer means what it meant. C will hand you that pointer. Rust");
    println!("   refuses to build one: a reference may not outlive the thing it");
    println!("   borrows, which is what a lifetime is for.");
    println!("   {} was the value that survived all of this:", moved.0);
}
