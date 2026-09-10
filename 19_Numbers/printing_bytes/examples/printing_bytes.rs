//! Printing bytes: `{:?}` shows numbers, `escape_ascii()` shows what Python's `b'…'` shows.
//!
//!   rustc --edition 2024 printing_bytes.rs -o /tmp/pb && /tmp/pb

fn main() {
    let data: &[u8] = "Real 🐍".as_bytes();

    println!("=== one byte string, five questions ===");
    let rows: [(&str, &str, String); 5] = [
        ("which numbers?", "{:?}", format!("{data:?}")),
        ("...in hex?", "{:02x?}", format!("{data:02x?}")),
        ("what does it say?", "escape_ascii()", data.escape_ascii().to_string()),
        ("one hex string?", "{:02x} per byte", data.iter().map(|b| format!("{b:02x}")).collect()),
        ("is it text?", "str::from_utf8", format!("{:?}", std::str::from_utf8(data))),
    ];
    for (question, call, shown) in rows {
        println!("  {question:<18} {call:<16} {shown}");
    }

    println!("\n=== escape_ascii: an ASCII byte stays a letter, the rest are escaped ===");
    let samples: [(&str, &[u8]); 7] = [
        (r#"b"hello""#, b"hello"),
        ("[0u8; 5]", &[0; 5]),
        (r#"b"tab\there""#, b"tab\there"),
        (r#"b"CR LF\r\n""#, b"CR LF\r\n"),
        (r#"b"back\\slash""#, b"back\\slash"),
        (r#"b"it's \"q\"""#, b"it's \"q\""),
        ("[0x7f, 0x80, 0xff]", &[0x7f, 0x80, 0xff]),
    ];
    for (written, bytes) in samples {
        println!("  {written:<20} -> {}", bytes.escape_ascii());
    }

    println!("\n=== the output is ASCII, and longer than the input ===");
    let shown = data.escape_ascii().to_string();
    println!("  {} bytes in, {} characters out, all of them ASCII: {}",
             data.len(), shown.len(), shown.is_ascii());
    let classes: [(&str, u8); 4] = [
        ("printable ASCII", b'A'),
        ("named escape", b'\n'),
        ("a quote", b'"'),
        ("anything else", 0xf0),
    ];
    for (what, b) in classes {
        let one = b.escape_ascii().to_string();
        println!("  {what:<16} 0x{b:02x}  ->  {one:<5} {} character(s)", one.len());
    }

    println!("\n=== the output is source: paste it into b\"...\" and the bytes come back ===");
    let pasted: &[u8] = b"Real \xf0\x9f\x90\x8d";
    println!("  b\"{}\"", data.escape_ascii());
    println!("  that literal, compiled into this program, equals the data: {}", pasted == data);

    println!("\n=== the hand-rolled version escapes the letters too ===");
    let every: String = data.iter().map(|b| format!("\\x{b:02x}")).collect();
    println!("  every byte as \\xNN      b\"{every}\"");
    println!("  escape_ascii           b\"{}\"", data.escape_ascii());
    println!("  \"Real \" + the loop     b\"Real {every}\"");
    println!("  ...which says Real twice: once as letters, once as \\x52\\x65\\x61\\x6c\\x20");

    println!("\n=== when the bytes are not UTF-8, only one view keeps all of them ===");
    // "café" the way Latin-1 writes it: each code point below 256 as one byte.
    let latin1: Vec<u8> = "café".chars().map(|c| c as u8).collect();
    println!("  str::from_utf8     {:?}", std::str::from_utf8(&latin1).map_err(|e| e.to_string()));
    println!("  from_utf8_lossy    {:?}", String::from_utf8_lossy(&latin1));
    println!("  escape_ascii       {}", latin1.escape_ascii());
    println!("  the lossy view replaced 0xe9 with U+FFFD; escape_ascii still says e9");
}
