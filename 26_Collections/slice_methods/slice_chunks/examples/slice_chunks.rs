fn main() {
    let v = [1, 2, 3, 4, 5, 6, 7];
    for c in v.chunks(3) {
        print!("{c:?} ");
    }
    println!();
    println!("{} chunks; the last has {} element(s)",
             v.chunks(3).len(), v.chunks(3).last().unwrap().len());

    // chunks_exact drops the short tail and hands it back separately.
    let mut exact = v.chunks_exact(3);
    let full: Vec<_> = exact.by_ref().collect();
    println!("{full:?} remainder {:?}", exact.remainder());

    // Rows out of a flat grid: two columns.
    let grid = [1, 2, 3, 4, 5, 6];
    for row in grid.chunks(2) {
        println!("{row:?}");
    }

    // The panic is caught here so the program can report it and go on.
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let r = std::panic::catch_unwind(|| v.chunks(0).count());
    std::panic::set_hook(hook);
    println!("chunks(0) panicked: {}", r.is_err());
}
