fn main() {
    let mut v = vec!["a", "b", "c", "d"];
    v.swap(0, 3);
    println!("{v:?}");

    v.swap(1, 1);
    println!("{v:?}  <- a == b is a no-op");

    // std::mem::swap(&mut v[0], &mut v[3]) is error[E0499]: two &mut into one Vec.
    // swap takes indices instead and does the exchange inside std.

    // reverse, by hand: swap from both ends inward.
    let mut w = vec![1, 2, 3, 4, 5];
    let n = w.len();
    for i in 0..n / 2 {
        w.swap(i, n - 1 - i);
    }
    println!("{w:?}");

    // The panic is caught here so the program can report it and go on.
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let r = std::panic::catch_unwind(move || w.swap(0, 9));
    std::panic::set_hook(hook);
    println!("swap(0, 9) on 5 elements panicked: {}", r.is_err());
}
