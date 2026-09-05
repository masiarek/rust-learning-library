fn main() {
    let v = [1, 2, 3, 4];
    for w in v.windows(2) {
        print!("{w:?} ");
    }
    println!();
    println!("{} windows of 2 from {} elements", v.windows(2).len(), v.len());

    // Neighbour differences: the job windows exists for.
    let temps = [20, 23, 21, 25];
    let deltas: Vec<i32> = temps.windows(2).map(|w| w[1] - w[0]).collect();
    println!("{deltas:?}");

    // Sorted? Every neighbouring pair in order.
    println!("{} {}", v.windows(2).all(|w| w[0] <= w[1]), temps.windows(2).all(|w| w[0] <= w[1]));

    // A window larger than the slice yields nothing, silently.
    println!("{}", v.windows(9).count());

    // windows overlap; chunks do not.
    println!("{:?}", v.chunks(2).collect::<Vec<_>>());

    // The panic is caught here so the program can report it and go on.
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let r = std::panic::catch_unwind(|| v.windows(0).count());
    std::panic::set_hook(hook);
    println!("windows(0) panicked: {}", r.is_err());
}
