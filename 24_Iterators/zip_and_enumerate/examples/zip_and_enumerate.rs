//! Walking two sequences together: `zip` pairs them, `enumerate` numbers them.
//!
//!   rustc --edition 2024 zip_and_enumerate.rs -o /tmp/zae && /tmp/zae

fn main() {
    println!("1. zip pairs, enumerate numbers");
    let ids = vec![10, 20, 30];
    let tags = vec!["a", "b", "c"];
    let mut lines: Vec<String> = Vec::new();
    for (i, (id, tag)) in ids.iter().zip(tags.iter()).enumerate() {
        lines.push(format!("{i}: {id} => {tag}"));
    }
    println!("   {lines:?}");

    println!();
    println!("2. The other order gives the same data in a different shape");
    let zip_then_enum: Vec<(usize, (&i32, &&str))> =
        ids.iter().zip(tags.iter()).enumerate().collect();
    let enum_then_zip: Vec<((usize, &i32), &&str)> =
        ids.iter().enumerate().zip(tags.iter()).collect();
    println!("   .zip().enumerate()  {zip_then_enum:?}");
    println!("   .enumerate().zip()  {enum_then_zip:?}");
    println!("   Same six values. The pattern that destructures them is not the");
    println!("   same: (i, (id, tag)) above, ((i, id), tag) below.");

    println!();
    println!("3. zip stops at the shorter side, and says nothing");
    let four = [10, 20, 30, 40];
    let three = ["a", "b", "c"];
    let pairs: Vec<(&i32, &&str)> = four.iter().zip(three.iter()).collect();
    println!("   {} ids, {} tags -> {} pairs", four.len(), three.len(), pairs.len());
    println!("   {pairs:?}");
    println!("   40 is gone. No panic, no warning, no `Result` — the loop just");
    println!("   runs one row short, and every count downstream is wrong by one.");

    println!();
    println!("4. enumerate numbers what reaches IT, not the source");
    let xs = [10, 25, 30, 45, 50];
    let after: Vec<(usize, &i32)> = xs.iter().filter(|x| **x % 10 == 0).enumerate().collect();
    let before: Vec<(usize, &i32)> = xs.iter().enumerate().filter(|(_, x)| **x % 10 == 0).collect();
    println!("   .filter().enumerate()  {after:?}");
    println!("   .enumerate().filter()  {before:?}");
    println!("   Put enumerate first for a position in the ORIGINAL sequence;");
    println!("   last for a row number in the output.");

    println!();
    println!("5. zip takes an IntoIterator, not an iterator");
    let owned = vec![100, 200, 300];
    let borrowed: Vec<(&i32, &i32)> = ids.iter().zip(&owned).collect();
    println!("   .zip(&owned)       {borrowed:?}   owned still usable: {owned:?}");
    let moved: Vec<(&i32, i32)> = ids.iter().zip(owned).collect();
    println!("   .zip(owned)        {moved:?}   owned is gone: E0382 on any further use");
    println!("   The pairs print the same; the item types do not. .zip(&owned)");
    println!("   yields (&i32, &i32), .zip(owned) yields (&i32, i32).");

    println!();
    println!("6. The index is a usize");
    let scaled: Vec<i32> = ids.iter().enumerate().map(|(i, n)| i as i32 * n).collect();
    println!("   {scaled:?}");
    println!("   Drop the `as i32` and this stops compiling — but WHERE rustc");
    println!("   points depends on the annotation. `let ids: Vec<i32>` gets you");
    println!("   `cannot multiply usize by &i32`; without it, `ids` is inferred");
    println!("   as Vec<usize> and the error lands on collect, blaming");
    println!("   FromIterator and never mentioning the multiply.");
}
