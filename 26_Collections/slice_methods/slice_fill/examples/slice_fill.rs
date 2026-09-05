fn main() {
    let mut v = vec![1, 2, 3, 4];
    v.fill(0);
    println!("{v:?}");

    // A sub-range.
    let mut buf = [b'.'; 8];
    buf[2..5].fill(b'#');
    println!("{}", String::from_utf8_lossy(&buf));

    // T: Clone — every slot gets a clone of the one value.
    let mut names = vec![String::new(); 3];
    names.fill(String::from("x"));
    println!("{names:?}");

    // fill_with: a closure per slot, for values that differ.
    let mut counter = 0;
    let mut ids = [0; 4];
    ids.fill_with(|| {
        counter += 1;
        counter
    });
    println!("{ids:?}");

    // The length never changes: an empty Vec stays empty.
    let mut empty: Vec<i32> = vec![];
    empty.fill(7);
    println!("{empty:?} len {}", empty.len());
}
