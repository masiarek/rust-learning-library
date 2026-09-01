// Every textual literal Rust has, one row each: what you type, the type the
// compiler gives it, what it holds.
//
// Every binding below is annotated, so the middle column is checked by the
// compiler and not by the author — a wrong type would not build.

use std::ffi::CStr;

fn row(typed: &str, ty: &str, holds: String) {
    println!("  {typed:<22} {ty:<18} {holds}");
}

fn main() {
    println!("THE TWELVE LITERALS");
    println!("  {:<22} {:<18} {}", "you type", "you get", "it holds");

    let plain: &'static str = "hello";
    row(r#""hello""#, "&'static str", format!("{plain:?}"));

    let escaped: &'static str = "a\tb\nc\0d\\e";
    row(r#""a\tb\nc\0d\\e""#, "&'static str", format!("{escaped:?}"));

    let ascii: &'static str = "\x36";
    row(r#""\x36""#, "&'static str", format!("{ascii:?}"));

    let uni: &'static str = "\u{7fff}";
    let cp = uni.chars().next().unwrap() as u32;
    row(r#""\u{7fff}""#, "&'static str", format!("{uni:?} = U+{cp:04X}"));

    let raw: &'static str = r"C:\temp\new";
    row(r#"r"C:\temp\new""#, "&'static str", format!("{raw:?}"));

    let raw_hash: &'static str = r#"he said "hi""#;
    row(r##"r#"he said "hi""#"##, "&'static str", format!("{raw_hash:?}"));

    let cstr: &'static CStr = c"linker";
    row(r#"c"linker""#, "&'static CStr", format!("{:?} {:?}", cstr, cstr.to_bytes_with_nul()));

    let craw: &'static CStr = cr#"C:\a"b"#;
    row(r##"cr#"C:\a"b"#"##, "&'static CStr", format!("{craw:?}"));

    let bytes: &'static [u8; 2] = b"Hi";
    row(r#"b"Hi""#, "&'static [u8; 2]", format!("{bytes:?}"));

    let braw: &'static [u8; 2] = br"\n";
    row(r#"br"\n""#, "&'static [u8; 2]", format!("{braw:?}  <- backslash, n"));

    let byte: u8 = b'A';
    row("b'A'", "u8", format!("{byte}"));

    let crab: char = '\u{1F980}';
    row("'🦀'", "char", format!("{crab:?} = U+{:X}, {} bytes of UTF-8, {} bytes as a char",
                                crab as u32, crab.len_utf8(), size_of::<char>()));

    println!();
    println!("SAME ELEVEN KEYSTROKES, TWO DIFFERENT STRINGS");
    let cooked = "C:\temp\new";
    let literal = r"C:\temp\new";
    println!("   \"C:\\temp\\new\"    {} bytes   {cooked:?}", cooked.len());
    println!("  r\"C:\\temp\\new\"   {} bytes   {literal:?}", literal.len());

    println!();
    println!("COUNTING THE # UNTIL THE TEXT FITS");
    let one = r#"a " quote"#;
    let two = r##"a "# sequence"##;
    println!("  r#\"a \" quote\"#          {one:?}");
    println!("  r##\"a \"# sequence\"##    {two:?}");

    println!();
    println!("\\x STOPS AT 7F IN TEXT, REACHES FF IN BYTES");
    println!("  {:<10} {:<10} the highest a str will take", "\"\\x7F\"", format!("{:?}", "\x7F"));
    // "\xF5" is a compile error: out of range for a str, which must be UTF-8.
    let odd: &'static [u8; 1] = b"\xF5";
    println!("  {:<10} {:<10} a byte string owes UTF-8 nothing", "b\"\\xF5\"", format!("{odd:?}"));

    println!();
    println!("A LITERAL SPANS LINES UNLESS YOU END ONE WITH \\");
    let across = "one
    two";
    let joined = "one \
    two";
    println!("  no backslash   {across:?}");
    println!("  backslash      {joined:?}   <- newline AND the indent are eaten");

    println!();
    println!("DISPLAY RENDERS THE ESCAPE, DEBUG PRINTS IT BACK");
    println!("  {{}}    {}", "a\tb");
    println!("  {{:?}}  {:?}", "a\tb");
}
