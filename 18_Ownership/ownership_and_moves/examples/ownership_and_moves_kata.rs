//! Kata solution: follow the responsibility, not the bytes.
//!
//!   rustc --edition 2024 ownership_and_moves_kata.rs -o /tmp/oamk && /tmp/oamk

/// Announces its own free, so you can see exactly when — and where — it happens.
#[derive(Debug)]
struct Parcel {
    label: &'static str,
}

impl Drop for Parcel {
    fn drop(&mut self) {
        println!("      [dropped: {} — freed here]", self.label);
    }
}

/// Takes ownership. The box is freed when this function ends, not the caller's.
fn seal(b: Parcel) {
    println!("  sealing {}", b.label);
}

/// Borrows. Nothing is freed here; the caller still owns it.
fn inspect(b: &Parcel) {
    println!("  inspecting {}", b.label);
}

/// Takes it and gives it back — the shape you write before you know borrowing.
fn stamp(mut b: Parcel) -> Parcel {
    b.label = "P7 (stamped)";
    b
}

fn main() {
    println!("Borrowing: responsibility never moves.");
    let kept = Parcel { label: "P12" };
    inspect(&kept);
    println!("  still usable afterwards -> {kept:?}");

    println!("\nMoving into a function: the free happens THERE.");
    let handed_over = Parcel { label: "P3" };
    seal(handed_over);
    println!("  (the drop line above printed before this one — inside `seal`)");
    println!("  `handed_over` is now unusable: E0382, borrow of moved value.");

    println!("\nMove out and back again:");
    let b = Parcel { label: "P7" };
    let b = stamp(b);
    println!("  returned -> {b:?}");

    println!("\nMoves are tracked per field, not per variable:");
    let pair = (Parcel { label: "P1" }, String::from("tracking number"));
    let note = pair.1; // only the String moves
    println!("  moved out the note -> {note:?}");
    println!("  pair.0 is still owned here -> {:?}", pair.0);

    println!("\nAnd integers only *feel* different because they are Copy:");
    let count = 461;
    let also = count; // a copy, not a move
    println!("  count {count}, also {also} — both usable, nothing was transferred");

    println!("\nEnd of main — everything still owned here is freed now, in reverse:");
}
