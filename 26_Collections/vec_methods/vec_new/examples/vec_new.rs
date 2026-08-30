fn main() {
    // `new` allocates nothing. An empty Vec is three numbers and no heap.
    let v: Vec<i32> = Vec::new();
    println!("len {} cap {} empty {}", v.len(), v.capacity(), v.is_empty());

    // The type has to come from somewhere: an annotation, or later use.
    let mut inferred = Vec::new();
    inferred.push("Ada");          // now it is a Vec<&str>
    println!("{inferred:?}");

    // Three spellings of the same empty vector.
    let a: Vec<u8> = Vec::new();
    let b: Vec<u8> = vec![];
    let c: Vec<u8> = Default::default();
    println!("{} {}", a == b, b == c);

    // The first push is what allocates, and it jumps straight past 1.
    let mut v: Vec<u32> = Vec::new();
    println!("before push: cap {}", v.capacity());
    v.push(1);
    println!("after push:  cap {}", v.capacity());

    // A Vec of a zero-sized type never allocates at all.
    let mut zst: Vec<()> = Vec::new();
    for _ in 0..1000 { zst.push(()); }
    println!("1000 units: len {} cap is usize::MAX {}", zst.len(), zst.capacity() == usize::MAX);
}
