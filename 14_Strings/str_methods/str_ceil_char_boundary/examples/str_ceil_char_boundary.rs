fn main() {
    let s = "héllo wörld";

    for n in [0, 1, 2, 3, 7, 8, 11] {
        let cut = s.ceil_char_boundary(n);
        println!("offset {n:>2} -> {cut:>2}  rest {:?}", &s[cut..]);
    }

    // Never below the request; floor is never above it.
    for n in [2, 8] {
        println!("n={n}: floor {} ceil {}", s.floor_char_boundary(n), s.ceil_char_boundary(n));
    }

    // At most three bytes of overshoot, since that is the widest character here.
    let wide = "a\u{1F600}b";                 // 'a', a 4-byte emoji, 'b'
    for n in 1..=5 {
        println!("wide n={n} -> {}", wide.ceil_char_boundary(n));
    }

    // len() is a boundary; beyond it there is none, which is why ceil panics
    // there while floor clamps.
    println!("{} {}", s.ceil_char_boundary(s.len()), s.floor_char_boundary(999));
}
