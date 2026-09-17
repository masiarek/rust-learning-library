use still_rules::{count, hex, method};

const NAMES: usize = count!(requests, errors, retries);

fn main() {
    let slots = [0u64; count!(requests, errors)];
    println!("{NAMES} names, {} slots", slots.len()); // 3 names, 2 slots
    println!("{}", hex!("c0ffee")); // c0ffee
    println!("{}", method!(POST)); // POST
}
