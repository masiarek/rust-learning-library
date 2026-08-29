fn main() {
    let mut s = String::from("ab");
    println!("{:?}", s.try_reserve_exact(5).is_ok());
    println!("capacity {}", s.capacity());

    // The four corners, side by side.
    let mut a = String::from("ab"); a.reserve(5);
    let mut b = String::from("ab"); b.reserve_exact(5);
    let mut c = String::from("ab"); c.try_reserve(5).unwrap();
    let mut d = String::from("ab"); d.try_reserve_exact(5).unwrap();
    println!("reserve {} / reserve_exact {} / try_reserve {} / try_reserve_exact {}",
             a.capacity(), b.capacity(), c.capacity(), d.capacity());

    // Failure is a value, not a process kill.
    let mut huge = String::new();
    println!("{:?}", huge.try_reserve_exact(usize::MAX).is_err());
    println!("still usable: {:?}", { huge.push_str("ok"); &huge });
}
