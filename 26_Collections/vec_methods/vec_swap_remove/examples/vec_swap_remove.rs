fn main() {
    // O(1): the last element is moved into the hole instead of shifting.
    let mut v = vec!["a", "b", "c", "d"];
    println!("removed {:?}, left {v:?}", v.swap_remove(1));

    // So the order changes. That is the whole trade against `remove`.
    let mut ordered = vec![0, 1, 2, 3, 4];
    let mut fast = ordered.clone();
    ordered.remove(1);
    fast.swap_remove(1);
    println!("remove      {ordered:?}");
    println!("swap_remove {fast:?}");

    // Removing the last element is the same either way.
    let mut v = vec![1, 2, 3];
    println!("{:?} {v:?}", v.swap_remove(2));

    // Where it belongs: a bag where position carries no meaning.
    let mut pool = vec!["job1", "job2", "job3", "job4"];
    let mut done = vec![];
    while !pool.is_empty() {
        done.push(pool.swap_remove(0));       // always O(1)
    }
    println!("drained a pool of jobs in {} steps: {done:?}", done.len());

    // Cost, measured in element moves rather than in time: removing the front
    // of a 5-element vector shifts 4 with `remove` and moves exactly 1 here.
    println!("remove(0) on len 5 shifts 4; swap_remove(0) moves 1");

    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let caught = std::panic::catch_unwind(|| { let mut v = vec![1]; v.swap_remove(9); });
    std::panic::set_hook(hook);
    println!("out-of-bounds swap_remove panicked: {}", caught.is_err());
}
