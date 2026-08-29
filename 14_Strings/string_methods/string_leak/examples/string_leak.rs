fn main() {
    let built = format!("config-{}", 42);
    let forever: &'static str = String::leak(built);
    println!("{forever:?}");

    // 'static, so it satisfies a bound that a borrowed String could not.
    fn needs_static(s: &'static str) -> usize { s.len() }
    println!("{}", needs_static(forever));

    // The reference is mutable before you narrow it.
    let mut_ref: &'static mut str = String::from("hello").leak();
    mut_ref.make_ascii_uppercase();
    println!("{mut_ref:?}");

    // The older spelling.
    let boxed: &'static str = Box::leak(String::from("also static").into_boxed_str());
    println!("{boxed:?}");

    // The bug shape: leaking per iteration is an unbounded leak.
    let leaked_total: usize = (0..3).map(|i| String::leak(format!("item{i}")).len()).sum();
    println!("{leaked_total} bytes leaked deliberately");
}
