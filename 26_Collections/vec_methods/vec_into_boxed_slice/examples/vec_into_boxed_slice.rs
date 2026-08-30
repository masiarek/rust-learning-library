fn main() {
    // Drops the capacity field: the result is exactly as long as it is.
    let v = vec![1, 2, 3];
    let boxed: Box<[i32]> = v.into_boxed_slice();
    println!("{boxed:?} len {}", boxed.len());

    // Two words instead of three. A Box<[T]> is pointer + length.
    println!("Vec<i32> {} bytes, Box<[i32]> {} bytes",
             size_of::<Vec<i32>>(), size_of::<Box<[i32]>>());

    // It shrinks first, so spare capacity is handed back on the way.
    let mut v: Vec<u8> = Vec::with_capacity(100);
    v.extend_from_slice(&[1, 2, 3]);
    println!("before: len {} cap {}", v.len(), v.capacity());
    let boxed = v.into_boxed_slice();
    println!("after: len {}, and there is no capacity to ask about", boxed.len());

    // It is still a slice, so slice methods work and it derefs the same way.
    let boxed: Box<[i32]> = vec![3, 1, 2].into_boxed_slice();
    println!("first {:?} contains 2: {}", boxed.first(), boxed.contains(&2));

    // Mutable in place, still fixed length.
    let mut boxed: Box<[i32]> = vec![3, 1, 2].into_boxed_slice();
    boxed.sort();
    println!("{boxed:?}");

    // The round trip is free in both directions.
    let back: Vec<i32> = boxed.into_vec();
    println!("into_vec: {back:?} cap {}", back.capacity());
    let again: Box<[i32]> = back.into();
    println!("and back: {again:?}");

    // Why bother: a field that is built once and then only read costs one
    // word less per value, and the type says "this will not grow".
    struct Frozen { data: Box<[u8]> }
    let f = Frozen { data: vec![1, 2, 3].into_boxed_slice() };
    println!("{:?}", f.data);
}
