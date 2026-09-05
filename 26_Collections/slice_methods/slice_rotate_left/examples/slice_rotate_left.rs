fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    v.rotate_left(2);
    println!("{v:?}");
    v.rotate_right(2);
    println!("{v:?}  <- rotate_right undoes it");

    // mid == len is a no-op. (Computed first: v.rotate_left(v.len()) is E0502.)
    let n = v.len();
    v.rotate_left(n);
    println!("{v:?}");

    // A turn order: front to back, in place.
    let mut turn = vec!["Ann", "Bob", "Cal"];
    for _ in 0..4 {
        print!("{} ", turn[0]);
        turn.rotate_left(1);
    }
    println!();

    // It does not wrap: rotate by k % len, not by k. The k is computed on
    // its own line: `r.rotate_left(7 % r.len())` is error[E0502], because the
    // &mut borrow for the call starts before the len() read inside the argument.
    let mut r = vec![1, 2, 3];
    let k = 7 % r.len();
    r.rotate_left(k);
    println!("{r:?}");

    // The panic is caught here so the program can report it and go on.
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let r2 = std::panic::catch_unwind(move || r.rotate_left(7));
    std::panic::set_hook(hook);
    println!("rotate_left(7) on 3 elements panicked: {}", r2.is_err());
}
