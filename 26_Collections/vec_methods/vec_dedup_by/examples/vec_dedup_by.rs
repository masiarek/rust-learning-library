fn main() {
    // The closure decides what "the same" means. It is still consecutive-only.
    let mut v = vec!["foo", "FOO", "bar", "Bar", "baz"];
    v.dedup_by(|a, b| a.eq_ignore_ascii_case(b));
    println!("{v:?}");

    // Argument order matters and is easy to get backwards: the closure is
    // called as (current, previous) — `a` is the LATER element, `b` the one
    // already kept. `a` is what gets removed when you return true.
    let mut order = vec![];
    let mut v = vec![1, 2, 3];
    v.dedup_by(|a, b| { order.push((*a, *b)); false });
    println!("closure saw (a, b) pairs: {order:?}");

    // Both are &mut, so the survivor can absorb what the duplicate carried.
    // Here consecutive equal keys are merged by summing their counts.
    let mut counts = vec![("a", 1), ("a", 2), ("b", 5), ("b", 1), ("b", 1)];
    counts.dedup_by(|a, b| {
        if a.0 == b.0 { b.1 += a.1; true } else { false }
    });
    println!("merged runs: {counts:?}");

    // "Nearly equal" is a job only dedup_by can do.
    let mut samples: Vec<f64> = vec![1.0, 1.001, 1.5, 1.502, 3.0];
    samples.dedup_by(|a, b| (*a - *b).abs() < 0.01);
    println!("within 0.01 of the last kept: {samples:?}");

    // dedup() is exactly dedup_by(|a, b| a == b).
    let mut x = vec![1, 1, 2, 2, 3];
    let mut y = x.clone();
    x.dedup();
    y.dedup_by(|a, b| a == b);
    println!("{x:?} == {y:?}: {}", x == y);
}
