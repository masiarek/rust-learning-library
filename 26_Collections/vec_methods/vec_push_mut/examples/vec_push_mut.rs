fn main() {
    // push_mut appends and hands back a &mut to the element it just stored.
    let mut v: Vec<String> = Vec::new();
    let slot = v.push_mut(String::from("Ada"));
    slot.push_str(" Lovelace");
    println!("{v:?}");

    // Without it the idiom is push-then-index-the-last, which repeats the
    // bounds check and reads worse.
    let mut old = Vec::new();
    old.push(String::from("Ben"));
    let n = old.len();
    old[n - 1].push_str(" Franklin");
    println!("{old:?}");

    // It shines when the element is built in stages.
    let mut rows: Vec<Vec<u8>> = Vec::new();
    for start in [1u8, 10, 100] {
        let row = rows.push_mut(Vec::new());
        row.push(start);
        row.push(start + 1);
    }
    println!("{rows:?}");

    // The borrow is exclusive and lasts as long as you hold it: no second
    // push while `slot` is alive. Dropping it first is the whole discipline.
    let mut v = vec![1, 2];
    {
        let slot = v.push_mut(3);
        *slot *= 10;
    }
    v.push(4);
    println!("{v:?}");

    // Stable since 1.95 — before that this was push followed by last_mut().
    let mut v = vec![1];
    v.push(2);
    if let Some(last) = v.last_mut() { *last = 99; }
    println!("the pre-1.95 spelling: {v:?}");
}
