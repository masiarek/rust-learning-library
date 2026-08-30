fn main() {
    // Vec<[T; N]> becomes Vec<T>, with no copying: the buffer already holds
    // the elements contiguously, so only the length changes.
    let nested: Vec<[u8; 3]> = vec![[1, 2, 3], [4, 5, 6]];
    let flat: Vec<u8> = nested.into_flattened();
    println!("{flat:?} len {}", flat.len());

    // The new length is old_len * N.
    let nested: Vec<[i32; 4]> = vec![[0; 4]; 5];
    println!("5 arrays of 4 -> len {}", nested.into_flattened().len());

    // Note the type: it is arrays, not vectors. A Vec<Vec<T>> cannot be
    // flattened this way, because its rows are separate allocations.
    let rows: Vec<Vec<u8>> = vec![vec![1, 2], vec![3]];
    let flat: Vec<u8> = rows.into_iter().flatten().collect();
    println!("Vec<Vec<u8>> needs an iterator: {flat:?}");

    // N == 0 collapses everything to nothing.
    let empty: Vec<[u8; 0]> = vec![[], []];
    println!("two empty arrays flatten to len {}", empty.into_flattened().len());

    // Real shape: pixels or samples arriving grouped, consumed flat.
    let pixels: Vec<[u8; 3]> = vec![[255, 0, 0], [0, 255, 0]];
    let bytes = pixels.into_flattened();
    println!("{} bytes ready to write: {bytes:?}", bytes.len());

    // The reverse trip needs chunks, and it is not free of a check:
    // the length must divide by N.
    let back: Vec<[u8; 3]> = bytes.chunks_exact(3)
        .map(|c| [c[0], c[1], c[2]])
        .collect();
    println!("regrouped: {back:?}");
}
