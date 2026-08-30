fn main() {
    let mut v = vec![3, 1, 2];
    let s: &mut [i32] = v.as_mut_slice();
    s.sort();
    println!("{v:?}");

    // Same free view as as_slice, with write access. The LENGTH is fixed:
    // a slice can reorder and overwrite, never push or remove.
    let mut v = vec![1, 2, 3];
    let s = v.as_mut_slice();
    s[0] = 99;
    s.swap(1, 2);
    s.reverse();
    println!("{v:?}");

    // DerefMut inserts it, so most slice mutation needs no as_mut_slice at all.
    let mut v = vec![5, 4, 3];
    v.sort();
    v.fill_with(|| 0);
    println!("{v:?}");

    // Where it earns its keep: handing a &mut [T] to a function.
    fn double_all(xs: &mut [i32]) { for x in xs { *x *= 2; } }
    let mut v = vec![1, 2, 3];
    double_all(v.as_mut_slice());
    double_all(&mut v);
    println!("{v:?}");

    // split_at_mut gives two disjoint &mut halves — impossible with indexes,
    // routine with slices.
    let mut v = vec![1, 2, 3, 4];
    let (left, right) = v.as_mut_slice().split_at_mut(2);
    left[0] = 10;
    right[0] = 30;
    println!("{v:?}");

    // While the slice is alive the Vec is exclusively borrowed, so a push in
    // between is a compile error — which is what stops the slice dangling.
    let mut v = vec![1, 2];
    { let s = v.as_mut_slice(); s[0] = 7; }
    v.push(3);
    println!("{v:?}");
}
