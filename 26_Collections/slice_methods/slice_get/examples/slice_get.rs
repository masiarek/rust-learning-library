fn main() {
    let v = vec![10, 20, 30, 40];
    println!("{:?} {:?}", v.get(1), v.get(9));

    // A range index returns a sub-slice, or None if ANY of it is out of range.
    println!("{:?}", v.get(1..3));
    println!("{:?}  <- not clamped, not a panic", v.get(1..99));
    println!("{:?}  <- reversed range", v.get(3..1));
    println!("{:?}  <- empty range at the end is fine", v.get(4..));

    // v[idx] would panic on 9; get is the question form.
    let idx = 9;
    println!("{}", v.get(idx).copied().unwrap_or(0));

    // The Some holds a reference; copied() turns Option<&i32> into Option<i32>.
    let third = v.get(2).copied();
    println!("{third:?}");
}
