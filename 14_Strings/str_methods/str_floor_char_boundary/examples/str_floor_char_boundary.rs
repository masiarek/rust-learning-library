fn main() {
    let s = "héllo wörld";

    for n in [0, 1, 2, 3, 7, 8, 99] {
        let cut = s.floor_char_boundary(n);
        println!("budget {n:>2} -> cut {cut:>2}  {:?}", &s[..cut]);
    }

    // The naive version panics on exactly the offsets floor repairs.
    println!("boundary at 2? {}", s.is_char_boundary(2));

    // Always within budget, which ceil cannot promise.
    for n in [2, 8] {
        println!("n={n}: floor {} <= n, ceil {} >= n",
                 s.floor_char_boundary(n), s.ceil_char_boundary(n));
    }

    // A byte budget is not a character budget.
    let by_chars: String = s.chars().take(5).collect();
    println!("5 chars = {by_chars:?} ({} bytes)", by_chars.len());
}
