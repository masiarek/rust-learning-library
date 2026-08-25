//! Kata solution: predict len and capacity through five operations,
//! then make one allocation serve all of them.
//!
//!   rustc --edition 2024 anatomy_of_a_string_kata.rs -o /tmp/anatk && /tmp/anatk

fn row(step: &str, s: &String, note: &str) {
    println!("   {step:<26} len {:>2}  capacity {:>2}   {note}", s.len(), s.capacity());
}

fn main() {
    println!("Round 1 — the naive build, watching the buffer move");
    let mut s = String::new();
    row("String::new()", &s, "no heap buffer at all");
    s.push_str("STAR");
    row("push_str(\"STAR\")", &s, "first allocation");
    s.push_str(" voting");
    row("push_str(\" voting\")", &s, "outgrew it — reallocated, bytes moved");
    s.push_str(" is score");
    row("push_str(\" is score\")", &s, "outgrew it again");
    s.push_str(" + runoff");
    row("push_str(\" + runoff\")", &s, "fits — no reallocation this time");

    println!("\nRound 2 — the same text, one allocation, because we knew the size");
    let mut planned = String::with_capacity(29);
    row("with_capacity(29)", &planned, "bought once, up front");
    for piece in ["STAR", " voting", " is score", " + runoff"] {
        planned.push_str(piece);
    }
    row("four push_str calls", &planned, "capacity never changed");

    println!("\nRound 3 — the two builds are equal; capacity is not content");
    println!("   s == planned?  {}   (capacities {} vs {})",
        s == planned, s.capacity(), planned.capacity());

    println!("\nThe rule to carry away:");
    println!("   len is the text, capacity is the room. Growth doubles so that");
    println!("   repeated pushes stay cheap; with_capacity skips the moves when");
    println!("   you can name the size; shrink_to_fit hands the spare room back.");
}
