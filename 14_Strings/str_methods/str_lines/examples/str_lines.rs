fn main() {
    let unix = "alpha\nbeta\ngamma\n";
    let windows = "alpha\r\nbeta\r\ngamma\r\n";

    println!("{:?}", unix.lines().collect::<Vec<&str>>());
    println!("{:?}", windows.lines().collect::<Vec<&str>>());
    println!("same: {}", unix.lines().eq(windows.lines()));

    // The trailing newline is a terminator, not a separator.
    println!("lines  {}", unix.lines().count());
    println!("split  {}", unix.split('\n').count());
    println!("{:?}", unix.split('\n').collect::<Vec<&str>>());

    // A bare \r is not a line ending.
    println!("{:?}", "old\rmac".lines().collect::<Vec<&str>>());

    // No final newline: still three lines.
    println!("{}", "alpha\nbeta\ngamma".lines().count());
}
