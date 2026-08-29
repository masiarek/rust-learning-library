fn main() {
    let mut s = String::from("hello world");

    // Deref coercion supplies the &mut str here.
    s.make_ascii_uppercase();
    println!("{s:?}");

    // The explicit form, and a sub-slice edit.
    let mut t = String::from("hello world");
    t.as_mut_str()[..5].make_ascii_uppercase();
    println!("{t:?}");

    // Two disjoint mutable halves.
    let mut u = String::from("abcdef");
    let (a, b) = u.as_mut_str().split_at_mut(3);
    a.make_ascii_uppercase();
    println!("{a:?} {b:?}");

    // Length-preserving: growth needs a String method.
    let mut v = String::from("abc");
    let before = v.len();
    v.as_mut_str().make_ascii_uppercase();
    v.push('!');
    println!("{before} -> {} (the push, not the edit)", v.len());
}
