fn main() {
    // Capacity is a promise about the buffer, not about the contents.
    let v: Vec<i32> = Vec::with_capacity(10);
    println!("len {} cap {} empty {}", v.len(), v.capacity(), v.is_empty());

    // Ten pushes, one allocation. Watch the pointer stay put.
    let mut sized: Vec<u32> = Vec::with_capacity(9);
    let first = sized.as_ptr() as usize;
    for n in 1..=9 { sized.push(n); }
    println!("moved during 9 pushes: {}", sized.as_ptr() as usize != first);
    println!("cap after 9 pushes: {}", sized.capacity());

    // Without it, the same nine pushes reallocate three times.
    let mut grown: Vec<u32> = Vec::new();
    let mut caps = vec![];
    for n in 1..=9 {
        grown.push(n);
        if caps.last() != Some(&grown.capacity()) { caps.push(grown.capacity()); }
    }
    println!("capacity sequence without with_capacity: {caps:?}");

    // "At least": std may give you more than you asked for, never less.
    let at_least: Vec<u8> = Vec::with_capacity(5);
    println!("asked 5, got at least 5: {}", at_least.capacity() >= 5);

    // The 11th push reallocates anyway — capacity is not a limit.
    let mut ten: Vec<u8> = Vec::with_capacity(10);
    for n in 0..11 { ten.push(n); }
    println!("pushed 11 into cap 10: len {} cap {}", ten.len(), ten.capacity());
}
