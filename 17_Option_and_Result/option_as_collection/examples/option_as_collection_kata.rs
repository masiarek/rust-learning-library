//! Kata solution: count the rows that exist, without writing a match.
//!
//!   rustc --edition 2024 option_as_collection_kata.rs -o /tmp/oack && /tmp/oack

fn main() {
    // One entry per question on the form; None means it was left unanswered.
    let returned: Vec<Option<u8>> = vec![Some(5), None, Some(3), Some(0), None];

    // An Option is a container of zero or one item, so `flatten` drops the
    // empty ones exactly as it would for nested Vecs.
    let total: u32 = returned.iter().flatten().map(|s| *s as u32).sum();
    let counted = returned.iter().flatten().count();

    println!("Iterating an Option like the one-item collection it is:");
    println!("  entries       -> {}", returned.len());
    println!("  answered      -> {counted}");
    println!("  rating total  -> {total}");
    println!("  mean answered -> {:.2}", total as f64 / counted as f64);

    // filter_map is the same move with the transform folded in.
    let doubled: Vec<u32> = returned.iter().filter_map(|s| s.map(|n| n as u32 * 2)).collect();
    println!("  doubled       -> {doubled:?}");

    println!("\nThe zero-or-one shape shows up directly, too:");
    println!("  Some(5).iter().count() -> {}", Some(5).iter().count());
    println!("  None::<u8>.iter().count() -> {}", None::<u8>.iter().count());
    println!("  Some(5).into_iter().chain(None).collect::<Vec<_>>() -> {:?}",
        Some(5).into_iter().chain(None).collect::<Vec<_>>());

    // take(): move the value out of a field you only have a &mut to, leaving
    // None behind. The borrow checker would refuse a plain move.
    let mut pending: Option<String> = Some("Cara's late row".to_string());
    let collected = collect_row(&mut pending);
    println!("\ntake() — moving out of something you only borrow:");
    println!("  collected -> {collected:?}");
    println!("  pending   -> {pending:?}   (left as None, not a copy)");
    println!("  again     -> {:?}", collect_row(&mut pending));
}

fn collect_row(slot: &mut Option<String>) -> Option<String> {
    slot.take()
}
