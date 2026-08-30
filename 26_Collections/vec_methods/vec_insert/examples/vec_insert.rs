fn main() {
    let mut v = vec!["Ada", "Cara"];
    v.insert(1, "Ben");
    println!("{v:?}");

    // index == len is the one past-the-end index that is legal: it appends.
    v.insert(v.len(), "Dana");
    println!("append via insert: {v:?}");

    // insert(0, ..) is the expensive one: every element shifts right.
    let mut v = vec![3, 4, 5];
    v.insert(0, 2);
    v.insert(0, 1);
    println!("front inserts: {v:?}");

    // O(n - index), not O(1) — the elements after `index` are all moved.
    // Building a list front-first with insert(0, ..) is quadratic; pushing
    // and reversing once is linear.
    let mut by_insert = Vec::new();
    for n in 1..=5 { by_insert.insert(0, n); }
    let mut by_push = Vec::new();
    for n in 1..=5 { by_push.push(n); }
    by_push.reverse();
    println!("{by_insert:?} == {by_push:?}: {}", by_insert == by_push);

    // index > len panics. Catching it here so the page can show the message.
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let caught = std::panic::catch_unwind(|| {
        let mut v = vec![1, 2, 3];
        v.insert(9, 0);
    });
    std::panic::set_hook(hook);
    println!("insert(9, ..) into a len-3 vec panicked: {}", caught.is_err());
}
