fn main() {
    let s = "the rain in spain";

    for (i, m) in s.match_indices("in") {
        println!("byte {i:>2}: {m:?}  ...{:?}", &s[i..(i + m.len() + 2).min(s.len())]);
    }

    // Offsets plus lengths reconstruct both the matches and the gaps.
    let hits: Vec<(usize, &str)> = s.match_indices("in").collect();
    println!("{hits:?}");
    println!("{:?}", s.split("in").collect::<Vec<&str>>());

    // Marking up every hit.
    let mut out = String::new();
    let mut last = 0;
    for (i, m) in s.match_indices("in") {
        out.push_str(&s[last..i]);
        out.push('[');
        out.push_str(m);
        out.push(']');
        last = i + m.len();
    }
    out.push_str(&s[last..]);
    println!("{out}");
}
