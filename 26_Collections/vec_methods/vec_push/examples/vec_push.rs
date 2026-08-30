fn main() {
    let mut v = Vec::new();
    v.push("Ada");
    v.push("Ben");
    v.push("Cara");
    println!("{v:?} len {}", v.len());

    // push takes the value by value: it moves, it does not borrow.
    let name = String::from("Dana");
    v.push(&name);              // a &String coerces to &str here
    println!("{v:?}");

    // Amortised O(1): most pushes are a write, some are a reallocation.
    let mut caps = vec![];
    let mut v: Vec<u8> = Vec::new();
    for n in 0..17 {
        let before = v.capacity();
        v.push(n);
        if v.capacity() != before { caps.push((n, before, v.capacity())); }
    }
    println!("reallocations during 17 pushes:");
    for (at, from, to) in &caps { println!("  on push #{at}: cap {from} -> {to}"); }
    println!("  {} reallocations for 17 pushes", caps.len());

    // The first capacity depends on the element SIZE, not on the count:
    // 8 for one-byte elements, 4 for anything up to 1 KiB, 1 above that.
    let mut bytes: Vec<u8> = Vec::new();      bytes.push(0);
    let mut words: Vec<u64> = Vec::new();     words.push(0);
    let mut big: Vec<[u8; 2048]> = Vec::new(); big.push([0; 2048]);
    println!("first capacity: u8 {} u64 {} [u8; 2048] {}",
             bytes.capacity(), words.capacity(), big.capacity());

    // The value is moved, so this is the E0382 everyone meets once.
    let owned = String::from("moved");
    let mut names: Vec<String> = Vec::new();
    names.push(owned);
    // println!("{owned}");   // error[E0382]: borrow of moved value: `owned`
    println!("{names:?}");

    // Pushing while holding a reference into the Vec is refused at compile
    // time, because a reallocation would leave that reference dangling.
    let mut v = vec![1, 2, 3];
    let first = v[0];           // a copy, not a borrow — this is fine
    v.push(4);
    println!("{v:?} first was {first}");
}
