fn main() {
    let mut v = vec![1, 2, 3, 4];
    if let Some(x) = v.get_mut(1) {
        *x = 20;
    }
    println!("{v:?}");

    // Out of range: None, and the if-let body never runs.
    if let Some(x) = v.get_mut(9) {
        *x = 90;
    }
    println!("{v:?}");

    // A range gives a &mut [T].
    if let Some(tail) = v.get_mut(2..) {
        tail.fill(0);
    }
    println!("{v:?}");

    // Two at once is refused: a second get_mut while the first is alive is
    // error[E0499]. get_disjoint_mut checks the indices differ, at run time.
    let [a, b] = v.get_disjoint_mut([0, 3]).unwrap();
    std::mem::swap(a, b);
    println!("{v:?}");
    println!("{:?}", v.get_disjoint_mut([0, 0]).is_err());
}
