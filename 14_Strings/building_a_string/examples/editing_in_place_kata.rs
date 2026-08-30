//! Kata solution: five edits that reuse the buffer — retain, insert, drain,
//! the `+` that eats its left operand, and push/pop.
//!
//!   rustc --edition 2024 editing_in_place_kata.rs -o /tmp/eipk && /tmp/eipk

/// Drop every vowel, in place. `retain` keeps what the closure says `true` to
/// and shifts the rest down — one pass, no second allocation.
fn strip_vowels(s: &mut String) {
    s.retain(|c| !"aeiouAEIOU".contains(c));
}

/// The byte offset of character `n` — what every String edit actually wants.
/// `insert(4, …)` means byte 4, and byte 4 of "café" is inside the é.
fn byte_of_char(s: &str, n: usize) -> usize {
    s.char_indices().nth(n).map(|(b, _)| b).unwrap_or(s.len())
}

fn main() {
    println!("1. retain — keep the consonants");
    let mut motto = String::from("Score Then Automatic Runoff");
    let before = (motto.len(), motto.capacity());
    strip_vowels(&mut motto);
    println!("   before  {:?}", "Score Then Automatic Runoff");
    println!("   after   {motto:?}");
    println!("   len {} -> {}, capacity {} -> {}  <- same buffer, nothing allocated",
        before.0, motto.len(), before.1, motto.capacity());

    println!("\n2. insert — at the middle character, not the middle byte");
    let mut name = String::from("vote🦀here");
    let half_byte = name.len() / 2;
    println!("   {name:?}: {} chars, {} bytes", name.chars().count(), name.len());
    println!("   the middle BYTE is {half_byte}, and is_char_boundary({half_byte}) = {}",
        name.is_char_boundary(half_byte));
    println!("   so insert({half_byte}, '|') would panic — it is inside the crab.");
    let middle_char = name.chars().count() / 2;
    let at = byte_of_char(&name, middle_char);
    println!("   the middle CHARACTER is #{middle_char}, which starts at byte {at}");
    name.insert(at, '|');
    println!("   after insert({at}, '|')  {name:?}");
    println!("   Every String edit is byte-indexed: insert, remove, replace_range,");
    println!("   truncate, split_off. len()/2 is a byte, and text is not bytes.");

    println!("\n3. drain — remove a range and keep what came out");
    let mut ballot = String::from("Ada,Ben,Cara,Dev");
    let removed: String = ballot.drain(4..8).collect();
    println!("   removed {removed:?}");
    println!("   left    {ballot:?}");
    println!("   drain returns an iterator over the removed chars; the String is");
    println!("   edited whether you collect them or not.");

    println!("\n4. `+` moves its left operand");
    let a = String::from("Score");
    let b = " then ";
    let c = String::from("Runoff");
    let joined = a + b + &c;
    // println!("{a}");   // error[E0382]: borrow of moved value: `a`
    println!("   let joined = a + b + &c;   -> {joined:?}");
    println!("   a: String  MOVED   — `+` takes it by value and reuses its buffer");
    println!("   b: &str    borrowed — the right side is always a &str");
    println!("   c: String  borrowed — because it was passed as &c");
    println!("   `c` is still here: {c:?}, `a` is gone. One allocation total,");
    println!("   which is why `+` exists at all.");

    println!("\n5. push and pop");
    let mut alphabet = String::new();
    for c in 'A'..='Z' {
        alphabet.push(c);
    }
    println!("   after 26 pushes  {alphabet:?}  (len {}, capacity {})",
        alphabet.len(), alphabet.capacity());
    let mut popped = String::new();
    for _ in 0..5 {
        if let Some(c) = alphabet.pop() {
            popped.push(c);
        }
    }
    println!("   popped 5         {popped:?}   <- reversed: pop takes from the end");
    println!("   left             {alphabet:?}  (len {}, capacity {})",
        alphabet.len(), alphabet.capacity());
    println!("   pop returns Option<char> — None on an empty String, never a panic —");
    println!("   and it pops a whole character, however many bytes that is.");
    let mut crab = String::from("go🦀");
    println!("   {:?}.pop() = {:?}, leaving {:?}", "go🦀", crab.pop(), crab);
}
