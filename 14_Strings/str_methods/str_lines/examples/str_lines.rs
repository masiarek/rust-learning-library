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

    // Blank lines are KEPT. The terminator rule is about the last newline.
    println!("{:?}", "a\n\nb".lines().collect::<Vec<&str>>());
    println!("{:?} <- one empty line, not none", "\n".lines().collect::<Vec<&str>>());
    println!("{:?} vs {:?} <- the empty string is the only one lines() gives nothing for",
             "".lines().collect::<Vec<&str>>(), "".split('\n').collect::<Vec<&str>>());

    // With no \r and no final newline there is nothing left to disagree about.
    println!("identical on \"a\\n\\nb\": {}", "a\n\nb".lines().eq("a\n\nb".split('\n')));

    // The trap that outlives the \r: a blank line in a Windows file is "\r".
    let blank = "a\r\n\r\nb\r\n";
    println!("{:?}", blank.split('\n').collect::<Vec<&str>>());
    println!("blank line is_empty()? split {} / lines {}",
             blank.split('\n').nth(1).unwrap().is_empty(),
             blank.lines().nth(1).unwrap().is_empty());
}
