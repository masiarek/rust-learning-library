fn main() {
    let s = "  \t value  ";
    println!("{:?}", s.trim_ascii_start());
    println!("{:?}", s.trim_ascii_end());
    println!("{:?}", s.trim_ascii());

    // const, which the Unicode version cannot be.
    const HEADER: &str = "   Content-Type";
    const NAME: &str = HEADER.trim_ascii_start();
    println!("{NAME:?}");

    // Parsing a header value: split, then trim the ASCII padding.
    let line = "Accept:   text/plain  ";
    if let Some((k, v)) = line.split_once(':') {
        println!("{:?} = {:?}", k, v.trim_ascii());
    }
}
