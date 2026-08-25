//! Kata solution: an approval ballot packed into one byte, and the ninth candidate.

use std::hint::black_box;
use std::mem::size_of;

const NAMES: [&str; 8] = ["Ada", "Ben", "Cara", "Dev", "Elif", "Fay", "Gus", "Hal"];

/// Eight approvals in eight bits. Seat `n` is bit `n`, counting from the right.
#[derive(Clone, Copy, Default)]
struct Ballot(u8);

impl Ballot {
    fn approve(self, seat: u32) -> Self {
        Ballot(self.0 | (1 << seat))
    }
    fn is_approved(self, seat: u32) -> bool {
        self.0 & (1 << seat) != 0
    }
    fn count(self) -> u32 {
        self.0.count_ones()
    }
    fn names(self) -> Vec<&'static str> {
        (0..8).filter(|&s| self.is_approved(s)).map(|s| NAMES[s as usize]).collect()
    }
}

fn main() {
    println!("=== three ballots, one byte each ===");
    let ballots = [
        Ballot::default().approve(0).approve(2).approve(5),
        Ballot::default().approve(2).approve(3),
        Ballot::default().approve(0).approve(2).approve(7),
    ];
    for (i, b) in ballots.iter().enumerate() {
        println!(
            "  voter {}: {:08b}  {} approval(s)  {:?}",
            i + 1,
            b.0,
            b.count(),
            b.names()
        );
    }
    println!("  size_of::<Ballot>() = {}   for all eight candidates", size_of::<Ballot>());

    println!("\n=== the tally, read one bit at a time ===");
    for seat in 0..8u32 {
        let votes = ballots.iter().filter(|b| b.is_approved(seat)).count();
        println!("  {:<5} {}{}", NAMES[seat as usize], "#".repeat(votes), if votes == 0 { " -" } else { "" });
    }

    println!("\n=== then a ninth candidate signs up ===");
    let ninth: u32 = black_box(8);
    println!("  1u8.checked_shl({ninth})  = {:?}          <- the honest answer", 1u8.checked_shl(ninth));
    println!("  1u8.wrapping_shl({ninth}) = {}              <- what a RELEASE build silently does", 1u8.wrapping_shl(ninth));
    println!("       ...which is bit 0: Ivy's approval lands on {}", NAMES[0]);

    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let plain = std::panic::catch_unwind(|| 1u8 << black_box(8u32));
    std::panic::set_hook(hook);
    match plain {
        Ok(v) => println!("  plain 1u8 << {ninth}       = {v}              (overflow checks OFF)"),
        Err(_) => println!("  plain 1u8 << {ninth}       = panic 'attempt to shift left with overflow'"),
    }

    println!("\n=== the fix is a wider byte-count, and it has its own ceiling ===");
    #[derive(Clone, Copy, Default)]
    struct Wide(u16);
    let ivy = Wide(1 << 8);
    println!("  size_of::<Wide>()     = {}   (two bytes, sixteen seats)", size_of::<Wide>());
    println!("  Ivy at seat 8         = {:016b}", ivy.0);
    println!("  u16 ceiling           = seat {} is the last one that fits", u16::BITS - 1);
    println!("  u128 would buy you    = {} seats, and no more", u128::BITS);
}
