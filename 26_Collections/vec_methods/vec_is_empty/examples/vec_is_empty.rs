fn main() {
    let empty: Vec<i32> = Vec::new();
    let full = vec![1];
    println!("{} {}", empty.is_empty(), full.is_empty());

    // Exactly `len() == 0`, spelled so it reads.
    println!("same answer: {}", empty.is_empty() == (empty.len() == 0));

    // Empty is about LENGTH, never about capacity or allocation.
    let mut v: Vec<u8> = Vec::with_capacity(64);
    println!("cap 64 and still empty: {}", v.is_empty());
    v.push(1);
    v.clear();
    println!("cleared: empty {} but cap {}", v.is_empty(), v.capacity());

    // The guard that makes indexing safe.
    let v: Vec<i32> = vec![];
    if !v.is_empty() { println!("{}", v[0]); } else { println!("nothing to index"); }

    // Usually you want the Option-returning form instead of a guard.
    println!("first {:?} last {:?}", v.first(), v.last());

    // A Vec of empty vectors is not itself empty.
    let rows: Vec<Vec<u8>> = vec![vec![], vec![]];
    println!("outer empty {} inner all empty {}",
             rows.is_empty(), rows.iter().all(|r| r.is_empty()));

    // clippy::len_zero is the lint that pushes you here.
    let names = vec!["Ada"];
    if !names.is_empty() { println!("{} name(s)", names.len()); }
}
