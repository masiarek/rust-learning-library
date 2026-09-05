fn main() {
    let nums = [10, 20, 30];
    let empty: [i32; 0] = [];
    println!("{:?} {:?}", nums.last(), empty.last());

    // v[v.len() - 1] on an empty slice overflows before it can even index.
    // last() is the safe spelling, and the Python xs[-1].
    let newest = nums.last().copied().unwrap_or(0);
    println!("newest {newest}");

    // Borrowed, not removed.
    println!("{:?} still has {} elements", nums.last(), nums.len());

    // Everything before the last one.
    if let Some((last, rest)) = nums.split_last() {
        println!("last {last}, rest {rest:?}");
    }
}
