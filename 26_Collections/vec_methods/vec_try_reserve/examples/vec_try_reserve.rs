use std::collections::TryReserveError;

fn main() {
    // The fallible reserve: a Result instead of an abort.
    let mut v: Vec<u32> = Vec::new();
    match v.try_reserve(10) {
        Ok(()) => println!("reserved, capacity now at least 10: {}", v.capacity() >= 10),
        Err(e) => println!("could not reserve: {e:?}"),
    }

    // A request that cannot be satisfied comes back as an Err rather than
    // killing the process. reserve() would abort here.
    let mut v: Vec<u64> = Vec::new();
    let huge = usize::MAX / 4;
    match v.try_reserve(huge) {
        Ok(()) => println!("somehow reserved usize::MAX/4 u64s"),
        Err(_) => println!("a usize::MAX/4 request returned Err — the process is still alive"),
    }
    println!("vector is untouched: len {} cap {}", v.len(), v.capacity());

    // Which is the whole point: it composes with ? in a fallible function.
    fn collect_bytes(src: &[u8]) -> Result<Vec<u8>, TryReserveError> {
        let mut out = Vec::new();
        out.try_reserve(src.len())?;
        out.extend_from_slice(src);
        Ok(out)
    }
    println!("{:?}", collect_bytes(&[1, 2, 3]));

    // The error type is opaque on stable — it prints, and that is all it
    // promises. (`TryReserveError::kind()` is still unstable.)
    let mut v: Vec<u8> = Vec::new();
    if let Err(e) = v.try_reserve(usize::MAX) {
        println!("error: {e}");
    }

    // Same "additional on top of len" arithmetic as reserve.
    let mut v = vec![1u8, 2, 3];
    v.try_reserve(5).unwrap();
    println!("len {} capacity at least 8: {}", v.len(), v.capacity() >= 8);
}
