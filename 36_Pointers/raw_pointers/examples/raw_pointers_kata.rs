//! Kata solution: walk a slice with a raw pointer, and say why each step is sound.
//!
//!   rustc --edition 2024 raw_pointers_kata.rs -o /tmp/rpk && /tmp/rpk

/// Sums a slice by moving a pointer from its first element to one past its last.
fn sum_by_pointer(values: &[u32]) -> u32 {
    let mut cursor: *const u32 = values.as_ptr();
    // Making the one-past-the-end pointer is allowed, and `wrapping_add` is safe.
    let end: *const u32 = values.as_ptr().wrapping_add(values.len());
    let mut total = 0;
    while cursor != end {
        // SAFETY: cursor starts at element 0 and stops before `end`, so it
        // always points at an element of `values`, which is borrowed for the
        // whole call: aligned, initialized, alive, and not written elsewhere.
        total += unsafe { *cursor };
        // SAFETY: cursor < end, so cursor + 1 is at most one past the end,
        // which `add` allows.
        cursor = unsafe { cursor.add(1) };
    }
    total
}

/// The same walk, but the reads go through `get`, which checks the index.
fn sum_by_index(values: &[u32]) -> u32 {
    (0..values.len()).map(|i| values.get(i).copied().unwrap_or(0)).sum()
}

fn main() {
    let cases: [&[u32]; 3] = [&[10, 20, 30], &[], &[7]];
    println!("Walking a slice with a raw pointer:");
    for values in cases {
        println!(
            "  {:<12} by pointer {:>2}   by index {:>2}   iter().sum() {:>2}",
            format!("{values:?}"),
            sum_by_pointer(values),
            sum_by_index(values),
            values.iter().sum::<u32>()
        );
    }

    println!();
    println!("The empty slice is the case to think about:");
    let empty: &[u32] = &[];
    println!("  start == one-past-the-end  {}", empty.as_ptr() == empty.as_ptr().wrapping_add(0));
    println!("  as_ptr() on an empty slice is never null (rustc's useless_ptr_null_checks");
    println!("  lint says so if you test it), but it points at no element. The loop");
    println!("  body never runs, so that start is never read; reading it would be");
    println!("  undefined behaviour.");

    println!();
    println!("What the unsafe version bought: nothing here. std's slice::Iter holds");
    println!("the same pair, a pointer to the next element and one past the end,");
    println!("and std wrote the SAFETY argument once.");
}
