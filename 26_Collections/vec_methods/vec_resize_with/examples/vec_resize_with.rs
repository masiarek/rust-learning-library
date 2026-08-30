fn main() {
    // The closure is called once per new slot, so each value is fresh.
    let mut v = vec![1, 2];
    v.resize_with(5, || 0);
    println!("{v:?}");

    // That is the difference from resize: no Clone bound, and a new value
    // rather than n clones of one.
    let mut rows: Vec<Vec<u8>> = vec![vec![9]];
    rows.resize_with(3, Vec::new);
    rows[1].push(1);
    println!("{rows:?}");

    // The closure is FnMut, so it can count.
    let mut n = 0;
    let mut v: Vec<u32> = Vec::new();
    v.resize_with(5, || { n += 1; n * n });
    println!("squares: {v:?}");

    // Shrinking never calls it.
    let mut calls = 0;
    let mut v = vec![1, 2, 3];
    v.resize_with(1, || { calls += 1; 0 });
    println!("shrunk to {v:?}, closure called {calls} times");

    // Default::default is the common argument.
    let mut v: Vec<String> = Vec::new();
    v.resize_with(2, Default::default);
    println!("{v:?}");

    // A grid built with independent rows — the bug resize() would introduce
    // here is not aliasing (Rust has no aliasing clone) but wasted work:
    // resize would clone one prototype row n times.
    let mut grid: Vec<Vec<u8>> = Vec::new();
    grid.resize_with(3, || Vec::with_capacity(4));
    for (i, row) in grid.iter_mut().enumerate() { row.push(i as u8); }
    println!("{grid:?}");
}
