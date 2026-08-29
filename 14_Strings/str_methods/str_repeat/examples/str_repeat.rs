fn main() {
    println!("{:?}", "ab".repeat(3));
    println!("{:?}", "-".repeat(20));
    println!("{:?}", "x".repeat(0));

    // Indentation.
    for depth in 0..4 {
        println!("{}node {depth}", "  ".repeat(depth));
    }

    // No separator argument; join supplies one.
    let cells = vec!["x"; 4];
    println!("{:?}", cells.join(" | "));

    // Allocated once, at the final size.
    let line = "=".repeat(10);
    println!("{} bytes, capacity {}", line.len(), line.capacity());

    // Bound a count that came from input, or the multiplication can abort.
    let requested: usize = 1_000_000_000;
    let safe = requested.min(32);
    println!("{:?}", "!".repeat(safe));
}
