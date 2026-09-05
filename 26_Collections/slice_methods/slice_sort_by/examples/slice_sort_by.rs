use std::cmp::Ordering;

fn main() {
    let mut nums = vec![5, 3, 9, 1];
    nums.sort_by(|a, b| b.cmp(a));
    println!("descending: {nums:?}");

    // Two keys: by length, then alphabetically among equal lengths.
    let mut words = vec!["pear", "fig", "apple", "kiwi", "date"];
    words.sort_by(|a, b| a.len().cmp(&b.len()).then_with(|| a.cmp(b)));
    println!("by length, then name: {words:?}");

    // Longest first, still alphabetical within a length: reverse only the first key.
    words.sort_by(|a, b| match a.len().cmp(&b.len()) {
        Ordering::Equal => a.cmp(b),
        other => other.reverse(),
    });
    println!("longest first: {words:?}");

    // f64 has no Ord. partial_cmp().unwrap() works until a NaN arrives.
    let mut floats = vec![2.5, -1.0, 0.0];
    floats.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{floats:?}");
    let mut with_nan = vec![2.5, f64::NAN, -1.0];
    with_nan.sort_by(f64::total_cmp);
    println!("{with_nan:?}   <- total_cmp puts NaN last, and never panics");

    // The closure sees references, so a field comparison is a.0.cmp(&b.0).
    let mut pairs = vec![(3, "c"), (1, "a"), (2, "b")];
    pairs.sort_by(|a, b| a.0.cmp(&b.0));
    println!("{pairs:?}");
}
