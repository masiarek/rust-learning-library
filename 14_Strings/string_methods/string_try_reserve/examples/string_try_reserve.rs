fn main() {
    let mut s = String::from("hi");

    // The ordinary case succeeds and behaves like reserve.
    println!("{:?}", s.try_reserve(100).is_ok());
    println!("capacity {}", s.capacity());

    // An impossible request is an error, not an abort.
    let mut t = String::new();
    match t.try_reserve(usize::MAX) {
        Ok(()) => println!("reserved"),
        Err(e) => println!("refused: {e}"),
    }

    // The pattern: trust the length only as far as the allocator agrees.
    fn read_claimed(claimed: usize) -> Result<String, String> {
        let mut buf = String::new();
        buf.try_reserve(claimed).map_err(|e| format!("cannot hold {claimed}: {e}"))?;
        buf.push_str("payload");
        Ok(buf)
    }
    println!("{:?}", read_claimed(16));
    println!("{:?}", read_claimed(usize::MAX).is_err());
}
