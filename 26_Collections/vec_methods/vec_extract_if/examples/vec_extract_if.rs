fn main() {
    // retain's mirror image: it yields the elements it removes.
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let evens: Vec<i32> = v.extract_if(.., |n| *n % 2 == 0).collect();
    println!("extracted {evens:?}  left {v:?}");

    // A range restricts where it looks; everything outside is untouched.
    let mut v = vec![0, 1, 2, 3, 4, 5];
    let taken: Vec<i32> = v.extract_if(1..4, |n| *n % 2 == 1).collect();
    println!("odds within 1..4: {taken:?}  left {v:?}");

    // Partitioning in place, with no second allocation for the keepers.
    let mut jobs = vec!["ok:1", "err:2", "ok:3", "err:4"];
    let failed: Vec<&str> = jobs.extract_if(.., |j| j.starts_with("err")).collect();
    println!("failed {failed:?}  remaining {jobs:?}");

    // The predicate takes &mut T, like retain_mut.
    let mut v = vec![1, 2, 3];
    let big: Vec<i32> = v.extract_if(.., |n| { *n *= 10; *n > 15 }).collect();
    println!("mutated then split: took {big:?}, left {v:?}");

    // It is LAZY. Drop it without consuming and only the elements it reached
    // are removed — which is the difference from retain, and a real trap.
    let mut v = vec![1, 2, 3, 4];
    let mut it = v.extract_if(.., |n| *n % 2 == 1);
    it.next();
    drop(it);
    println!("stopped after the first match: {v:?}");

    // Consuming it fully is the usual intent; `for` does that.
    let mut v = vec![1, 2, 3, 4];
    for _ in v.extract_if(.., |n| *n % 2 == 1) {}
    println!("fully consumed: {v:?}");

    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let caught = std::panic::catch_unwind(|| {
        let mut v = vec![1, 2];
        let _: Vec<i32> = v.extract_if(0..9, |_| true).collect();
    });
    std::panic::set_hook(hook);
    println!("out-of-range range panicked: {}", caught.is_err());
}
