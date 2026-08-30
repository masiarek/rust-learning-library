//! Kata solution: four silent losses, each with the fix beside it.
//!
//!   rustc --edition 2024 casting_with_as_kata.rs -o /tmp/ck && /tmp/ck

fn main() {
    println!("1. Turnout, computed four ways, and only two of them right");
    let voted: u32 = 4;
    let eligible: u32 = 6;
    println!("   voted = {voted}, eligible = {eligible}");
    println!("   voted / eligible * 100                = {}", voted / eligible * 100);
    println!("   voted * 100 / eligible                = {}", voted * 100 / eligible);
    println!("   (voted as f64 / eligible as f64) * 100 = {:.1}",
             (f64::from(voted) / f64::from(eligible)) * 100.0);
    println!("   (voted / eligible) as f64 * 100        = {:.1}",
             (voted / eligible) as f64 * 100.0);
    println!("   The first and last are the same bug: integer division happened");
    println!("   BEFORE the widening, so 4/6 was 0 and the cast preserved it");
    println!("   perfectly. `as` cannot repair a value that was already lost.");
    println!("   Note the third line uses f64::from, not `as` — for a widening");
    println!("   that cannot fail there is a From impl, and using it means the");
    println!("   compiler rejects the day someone changes u32 to u64.");

    println!();
    println!("2. The index that went negative");
    let scores = [5u8, 3, 0];
    let position: i32 = -1;
    let as_index = position as usize;
    println!("   position = {position}, position as usize = {as_index}");
    println!("   scores.get(that) = {:?}   <- .get still refuses", scores.get(as_index));
    println!("   The guard that lets it through is the one written in the SIGNED");
    println!("   world and cast afterwards:");
    println!("     if position < scores.len() as i32 {{ scores[position as usize] }}");
    println!("   -1 < 3 is {}, so the guard passes, and the index is then the",
             position < scores.len() as i32);
    println!("   20-digit number above.");
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let boom = std::panic::catch_unwind(|| {
        if position < scores.len() as i32 { scores[position as usize] } else { 0 }
    });
    std::panic::set_hook(hook);
    println!("   running exactly that: {}", if boom.is_err() { "panicked" } else { "returned" });
    println!("   Two fixes. Compare in the unsigned world — usize::try_from(position)");
    println!("   = {:?} — or keep the index a usize from the start", usize::try_from(position));
    println!("   and let the type make a negative one unrepresentable.");

    println!();
    println!("3. The count that stopped counting");
    let mut total: u8 = 250;
    for _ in 0..3 {
        total = total.saturating_add(5);
    }
    println!("   250u8, three saturating_adds of 5 -> {total}   (u8::MAX = 255)");
    let wrapped = 250u8.wrapping_add(10);
    println!("   250u8.wrapping_add(10)            -> {wrapped}");
    let checked = 250u8.checked_add(10);
    println!("   250u8.checked_add(10)             -> {checked:?}");
    println!("   `250 + 10` on a u8 PANICS in a debug build and wraps in release,");
    println!("   which is the one case where debug and release disagree about the");
    println!("   answer. The three named methods each pick one behaviour and say");
    println!("   so, and one of them is the only one that lets the caller decide.");

    println!();
    println!("4. The money that did not add up");
    let cents: i64 = 1_00 + 2_00 + 3_00;
    let as_float = 0.1_f64 + 0.2_f64;
    println!("   in cents (i64):   {cents} -> {}.{:02}", cents / 100, cents % 100);
    println!("   0.1 + 0.2 in f64: {as_float}");
    println!("   0.1 + 0.2 == 0.3: {}", as_float == 0.3);
    println!("   Not a casting bug, but the reason the first line exists: money");
    println!("   is counted in the smallest unit as an integer, and converted to");
    println!("   a decimal string only for display.");

    println!();
    println!("5. The rule, as a table you can apply without thinking");
    println!("   u32 -> u64     f64::from / u64::from    cannot fail, so From");
    println!("   i64 -> u8      u8::try_from(n)?         can fail, so TryFrom");
    println!("   f64 -> i64     n.round() as i64         say which rounding first");
    println!("   u64 -> f64     n as f64                 lossy above 2^53, and");
    println!("                                           there is no From for it");
    println!("   The last row is the honest exception: some conversions are lossy");
    println!("   and unavoidable, and `as` is what you have. Write the comment.");
}
