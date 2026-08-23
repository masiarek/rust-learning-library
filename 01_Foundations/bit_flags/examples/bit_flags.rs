//! Bit flags: a flag is a one-bit field, and a header field is an n-bit flag.

use std::fmt;
use std::ops::BitOr;

fn rule(t: &str) {
    println!("\n=== {t} ===");
}

// ------------------------------------------------------------------ open(2)
// Linux values, written here as data rather than read from the system, so this
// prints the same everywhere. They are NOT the same numbers on macOS or BSD --
// which is the entire argument for writing the name instead of the number.
const O_ACCMODE: u32 = 0o3;
const O_RDONLY: u32 = 0o0; // zero. remember this.
const O_WRONLY: u32 = 0o1;
const O_RDWR: u32 = 0o2;
const O_CREAT: u32 = 0o100;
const O_TRUNC: u32 = 0o1000;
const O_APPEND: u32 = 0o2000;

const ONE_BIT_FLAGS: [(u32, &str); 3] =
    [(O_CREAT, "O_CREAT"), (O_TRUNC, "O_TRUNC"), (O_APPEND, "O_APPEND")];

fn access_mode(flags: u32) -> &'static str {
    match flags & O_ACCMODE {
        O_RDONLY => "O_RDONLY",
        O_WRONLY => "O_WRONLY",
        O_RDWR => "O_RDWR",
        _ => "invalid",
    }
}

// --------------------------------------------------------------- TCP word 12
// offset: 4 bits | reserved: 3 bits | flags: 9 bits, all inside one u16.
const OFF_SHIFT: u32 = 12;
const OFF_MASK: u16 = 0xF;
const RSV_SHIFT: u32 = 9;
const RSV_MASK: u16 = 0x7;
const FLAGS_MASK: u16 = 0x01FF;

const FIN: u16 = 1 << 0;
const SYN: u16 = 1 << 1;
const ACK: u16 = 1 << 4;

fn pack(offset: u16, reserved: u16, flags: u16) -> u16 {
    ((offset & OFF_MASK) << OFF_SHIFT) | ((reserved & RSV_MASK) << RSV_SHIFT) | (flags & FLAGS_MASK)
}

// ----------------------------------------------------------------- a newtype
#[derive(Clone, Copy, PartialEq)]
struct Mode(u32);

impl BitOr for Mode {
    type Output = Mode;
    fn bitor(self, rhs: Mode) -> Mode {
        Mode(self.0 | rhs.0)
    }
}

impl Mode {
    fn contains(self, other: Mode) -> bool {
        self.0 & other.0 == other.0
    }
}

impl fmt::Debug for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = vec![access_mode(self.0)];
        for (bit, name) in ONE_BIT_FLAGS {
            if self.0 & bit != 0 {
                parts.push(name);
            }
        }
        write!(f, "Mode({})", parts.join(" | "))
    }
}

fn main() {
    rule("a flag is a one-bit field");
    let flags = O_RDWR | O_CREAT | O_TRUNC;
    println!("  O_RDWR | O_CREAT | O_TRUNC = {flags:>5}  = {flags:018b}");
    for (bit, name) in ONE_BIT_FLAGS {
        println!("    {name:<9} {bit:>5} = {bit:018b}  set? {}", flags & bit != 0);
    }
    println!("  flags.count_ones() = {}   <- BITS set, not flags set: one of them is the O_RDWR field", flags.count_ones());

    rule("TRAP 1: a zero-valued flag cannot be tested with &");
    println!("  O_RDONLY = {O_RDONLY}, so `flags & O_RDONLY` is 0 for EVERY flags:");
    for (label, f) in [("read-only", O_RDONLY), ("write-only", O_WRONLY), ("read-write", O_RDWR)] {
        println!(
            "    {label:<11} flags & O_RDONLY != 0  ->  {:<5}   (wrong for all three)",
            f & O_RDONLY != 0
        );
    }
    println!("  the low two bits are a FIELD, not three flags. mask it and compare:");
    for (label, f) in [("read-only", O_RDONLY), ("write-only", O_WRONLY), ("read-write", O_RDWR)] {
        println!("    {label:<11} flags & O_ACCMODE == ...  ->  {}", access_mode(f));
    }

    rule("an n-bit field is the same mechanism, wider");
    let word = pack(8, 0, ACK);
    println!("  pack(offset=8, reserved=0, flags=ACK) = 0x{word:04X} = {word:016b}");
    println!("                                           offset|rsv|flags");
    println!("  offset   = (w >> {OFF_SHIFT}) & 0x{OFF_MASK:X}    = {}", (word >> OFF_SHIFT) & OFF_MASK);
    println!("  reserved = (w >> {RSV_SHIFT})  & 0x{RSV_MASK:X}    = {}", (word >> RSV_SHIFT) & RSV_MASK);
    println!("  flags    =  w        & 0x{FLAGS_MASK:03X} = {:09b}", word & FLAGS_MASK);
    println!("  ACK set? {}   SYN set? {}   FIN set? {}",
        word & ACK != 0, word & SYN != 0, word & FIN != 0);

    rule("TRAP 2: the top field forgives a missing mask; a middle field does not");
    println!("  offset   w >> {OFF_SHIFT}          = {:>5}  correct -- nothing lives above it", word >> OFF_SHIFT);
    println!("  reserved w >> {RSV_SHIFT}           = {:>5}  WRONG -- the offset bled in", word >> RSV_SHIFT);
    println!("  reserved (w >> {RSV_SHIFT}) & 0x{RSV_MASK:X}    = {:>5}  right", (word >> RSV_SHIFT) & RSV_MASK);
    println!("  ...which is how the habit forms: you learn >> on the top field, where it works.");

    rule("the newtype: names in the output, and no cross-type mixing");
    let m = Mode(O_RDWR) | Mode(O_CREAT) | Mode(O_APPEND);
    println!("  {m:?}");
    println!("  contains(O_CREAT)  = {}", m.contains(Mode(O_CREAT)));
    println!("  contains(O_TRUNC)  = {}", m.contains(Mode(O_TRUNC)));
    println!("  bare u16 vs Mode   : `Mode(O_RDWR) | ACK` does not compile (E0308)");
    println!("  contains() uses `& x == x`, not `!= 0` -- so a multi-bit mask means ALL of it:");
    let both = Mode(O_CREAT | O_TRUNC);
    println!("    Mode(O_CREAT).contains(both)      = {}", Mode(O_CREAT).contains(both));
    println!("    Mode(O_CREAT|O_TRUNC).contains(both) = {}", Mode(O_CREAT | O_TRUNC).contains(both));
}
