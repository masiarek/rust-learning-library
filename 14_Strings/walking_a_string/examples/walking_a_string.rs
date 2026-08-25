//! Ways to iterate over text: bytes, chars, and the split/match families.
//!
//!   rustc --edition 2024 walking_a_string.rs -o /tmp/ws && /tmp/ws

fn main() {
    let slice = "bête noir  d'Arrrgh ";
    println!("The slice: {slice:?}");
    println!("   {} bytes, {} chars", slice.len(), slice.chars().count());

    println!("\n1. Item type u8 — .bytes()");
    let bytes: Vec<u8> = slice.bytes().take(6).collect();
    println!("   first 6: {bytes:?}   ({} in total)", slice.bytes().count());

    println!("\n2. Item type char — .chars()");
    let chars: Vec<char> = slice.chars().take(6).collect();
    println!("   first 6: {chars:?}   ({} in total)", slice.chars().count());
    println!("   'ê' is one char here and two bytes above. Same text, two rulers.");

    println!("\n3. .char_indices() is not .chars().enumerate()");
    print!("   char_indices():      ");
    for (i, c) in slice.char_indices().take(6) {
        print!("({i},{c:?}) ");
    }
    println!();
    print!("   chars().enumerate(): ");
    for (i, c) in slice.chars().enumerate().take(6) {
        print!("({i},{c:?}) ");
    }
    println!();
    println!("   They diverge at 't'. char_indices gives BYTE OFFSETS you can slice with;");
    println!("   enumerate gives ordinals you cannot.");

    println!("\n4. Item type &str — the split family");
    let show = |label: &str, v: Vec<&str>| println!("   {label:<24} {} -> {:?}", v.len(), v);
    show(".split(' ')", slice.split(' ').collect());
    show(".split_terminator(' ')", slice.split_terminator(' ').collect());
    show(".rsplit(' ')", slice.rsplit(' ').collect());
    show(".split_whitespace()", slice.split_whitespace().collect());
    show(".splitn(3, ' ')", slice.splitn(3, ' ').collect());
    show(".rsplitn(3, ' ')", slice.rsplitn(3, ' ').collect());

    println!("\n5. Splits are the gaps between matches");
    show(".matches(\"rr\")", slice.matches("rr").collect());
    show(".split(\"rr\")", slice.split("rr").collect());
    println!("   One match, two pieces. n matches always give n+1 pieces — which is");
    println!("   where the empty strings above come from: two adjacent spaces have");
    println!("   nothing between them, and so does a separator at the very end.");

    println!("\n6. The pattern can be a char, a &str, or a closure");
    show(".split('r')", slice.split('r').collect());
    show(".split(\"rr\")", slice.split("rr").collect());
    show(".split(char::is_whitespace)", slice.split(char::is_whitespace).collect());
    show(".split(|c: char| c == '\\'')", slice.split(|c: char| c == '\'').collect());

    println!("\n7. The two you will actually reach for");
    let config = "name = Ada\nscore = 5\nnote = has = signs\n";
    println!("   config = {config:?}");
    for line in config.lines() {
        if let Some((key, value)) = line.split_once(" = ") {
            println!("      split_once  {key:<6} -> {value:?}");
        }
    }
    println!("   split_once takes the FIRST separator and returns the rest whole, so");
    println!("   the value containing ' = ' survives. splitn(2, …) does the same job.");
    println!("   trim():  {:?} -> {:?}", "  padded  ", "  padded  ".trim());
}
