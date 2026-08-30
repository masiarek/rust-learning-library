fn main() {
    let v = vec![10, 20, 30];
    println!("len {}", v.len());

    // len counts ELEMENTS. For a Vec<u8> that happens to equal the byte
    // count; for anything else it does not.
    let bytes: Vec<u8> = vec![1, 2, 3, 4];
    let words: Vec<u64> = vec![1, 2, 3, 4];
    println!("both len 4: {} {} — but {} bytes vs {} bytes",
             bytes.len(), words.len(),
             bytes.len() * size_of::<u8>(), words.len() * size_of::<u64>());

    // len is not capacity. One is what is there, the other what fits.
    let mut v: Vec<i32> = Vec::with_capacity(10);
    v.push(1);
    println!("len {} capacity {}", v.len(), v.capacity());

    // It is O(1) — a field read, not a walk — so calling it in a loop
    // condition costs nothing.
    let v = vec![0u8; 5];
    let mut total = 0;
    for i in 0..v.len() { total += i; }
    println!("index sum {total}");

    // The last valid index is len - 1, which is why the empty case needs
    // care: 0 - 1 underflows on usize.
    let v: Vec<i32> = vec![];
    println!("last index of an empty vec: {:?}", v.len().checked_sub(1));
    println!("prefer .last(): {:?}", v.last());

    // is_empty() is len() == 0, and reads better.
    let v = vec![1];
    println!("{} {}", v.len() == 0, v.is_empty());

    // Nested: len is the outer count only.
    let grid = vec![vec![1, 2, 3], vec![4, 5]];
    println!("rows {} total cells {}", grid.len(),
             grid.iter().map(|r| r.len()).sum::<usize>());
}
