fn main() {
    let s = "héllo";

    // Sound: 0 and 3 are boundaries, both within the string.
    let head = unsafe { s.get_unchecked(0..3) };
    println!("{head:?}");

    // The offsets that make it sound come from char_indices.
    let bounds: Vec<usize> = s.char_indices().map(|(i, _)| i).collect();
    println!("{bounds:?}");
    for w in bounds.windows(2) {
        println!("{:?}", unsafe { s.get_unchecked(w[0]..w[1]) });
    }

    // The safe version rejects exactly what the unsafe one would corrupt.
    println!("{:?}", s.get(0..2));
    println!("boundary at 2? {}", s.is_char_boundary(2));
}
