fn main() {
    let line = "  GET /index.html \r\n";
    println!("{:?}", line.trim_ascii_end());
    println!("{:?}", line.trim_ascii());

    // \r\n both go, since both are ASCII whitespace.
    for raw in ["ok\n", "ok\r\n", "ok"] {
        println!("{:<8} -> {:?}", format!("{raw:?}"), raw.trim_ascii_end());
    }

    // const.
    const PADDED: &str = "value   ";
    const TIDY: &str = PADDED.trim_ascii_end();
    println!("{TIDY:?} ({} bytes)", TIDY.len());
}
