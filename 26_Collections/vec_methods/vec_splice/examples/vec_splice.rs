fn main() {
    // Replace a range with an iterator, and receive what was removed.
    let mut v = vec![1, 2, 3, 4, 5];
    let removed: Vec<i32> = v.splice(1..4, [20, 30]).collect();
    println!("removed {removed:?}  now {v:?}");

    // The replacement need not be the same length — that is the point.
    let mut v = vec!["a", "b", "c"];
    v.splice(1..2, ["x", "y", "z"]);
    println!("one out, three in: {v:?}");

    // An empty range inserts without removing.
    let mut v = vec![1, 4];
    v.splice(1..1, [2, 3]);
    println!("insert at 1: {v:?}");

    // An empty replacement removes without inserting — that is drain's job,
    // and splice does it too.
    let mut v = vec![1, 2, 3, 4];
    v.splice(1..3, []);
    println!("remove 1..3: {v:?}");

    // The removed elements are dropped if you ignore them, so `let _ =` is
    // enough when you only want the replacement to happen.
    let mut v = vec![0, 0, 0];
    let _ = v.splice(..2, [9]);
    println!("{v:?}");

    // Any IntoIterator works as the replacement, including another Vec
    // and an adapter chain.
    let mut v = vec![1, 2, 3];
    v.splice(..1, vec![7, 8]);
    println!("from a Vec: {v:?}");
    let mut v = vec![1, 2, 3];
    v.splice(1.., (10..13).map(|n| n * 2));
    println!("from an adapter: {v:?}");

    // Replacing the whole thing.
    let mut v = vec![1, 2, 3];
    let old: Vec<i32> = v.splice(.., [0]).collect();
    println!("old {old:?}  new {v:?}");
}
