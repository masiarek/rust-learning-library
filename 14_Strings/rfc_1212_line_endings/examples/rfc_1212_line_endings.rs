#![allow(deprecated)]

use std::io::{BufRead, Cursor};

/// Pre-1.4 `str::lines` was exactly this: split on '\n', drop the empty
/// piece a trailing '\n' leaves behind, and hand back whatever else is
/// there — including the '\r'.
fn lines_before_1_4(s: &str) -> Vec<&str> {
    s.split_terminator('\n').collect()
}

fn main() {
    let unix = "name,qty\nbolt,12\n";
    let windows = "name,qty\r\nbolt,12\r\n";

    println!("WHAT RFC 1212 CHANGED");
    println!("  before  unix    {:?}", lines_before_1_4(unix));
    println!("  before  windows {:?}", lines_before_1_4(windows));
    println!("  today   unix    {:?}", unix.lines().collect::<Vec<&str>>());
    println!("  today   windows {:?}", windows.lines().collect::<Vec<&str>>());
    println!("  the two agree now: {}", unix.lines().eq(windows.lines()));

    println!();
    println!("BURNTSUSHI'S DIAGNOSIS: THE OLD BEHAVIOUR WAS NOT EVEN CONSISTENT");
    for (label, line) in [("lf  ", "bolt,12\n"), ("crlf", "bolt,12\r\n")] {
        let old = lines_before_1_4(line)[0];
        println!(
            "  {label} ended {:<6} -> {:<11} trimmed {}",
            format!("{:?}", &line[7..]),
            format!("{old:?}"),
            if old.ends_with('\r') { "the \\n only" } else { "all of it" }
        );
    }

    println!();
    println!("THE SURPRISE \"NONE I CAN THINK OF\" DID NOT SEE");
    let truncated = "name,qty\r\nbolt,12\r";
    println!("  no final \\n   {:?}", truncated.lines().collect::<Vec<&str>>());
    println!("  same, lf only {:?}", "name,qty\nbolt,12".lines().collect::<Vec<&str>>());
    let last = truncated.lines().next_back().unwrap();
    println!("  last line ends with \\r: {}", last.ends_with('\r'));
    println!("  and it fails the obvious test: last == \"bolt,12\" is {}", last == "bolt,12");
    println!("  a lone \\r splits nothing: {:?}", "old\rmac".lines().collect::<Vec<&str>>());

    println!();
    println!("BufRead::lines GOT THE SAME CHANGE; read_line DID NOT");
    let read = |s: &'static str| {
        Cursor::new(s).lines().map(|l| l.unwrap()).collect::<Vec<String>>()
    };
    println!("  lines      {:?}", read("name,qty\r\nbolt,12\r\n"));
    println!("  lines tail {:?}", read("name,qty\r\nbolt,12\r"));
    let mut buf = String::new();
    Cursor::new(windows).read_line(&mut buf).unwrap();
    println!("  read_line  {:?}   <- the terminator is still yours to strip", buf);

    println!();
    println!("BRSON'S QUESTION, ASKED IN THE THREAD AND NEVER ANSWERED BY THE RFC");
    let copied: String = windows.lines().map(|l| format!("{l}\n")).collect();
    println!("  read {} bytes, wrote {} bytes back, identical: {}",
             windows.len(), copied.len(), windows == copied);

    println!();
    println!("THE UNICODE SEPARATORS THE RFC WAS ASKED FOR AND REFUSED");
    println!("  char         lines()  is_whitespace  split_whitespace");
    for (name, c) in [
        ("VT   U+000B", '\u{0B}'), ("FF   U+000C", '\u{0C}'),
        ("NEL  U+0085", '\u{85}'), ("LS   U+2028", '\u{2028}'),
        ("PS   U+2029", '\u{2029}'),
    ] {
        let s = format!("a{c}b");
        println!("  {name}  {}        {:<5}          {:?}",
                 s.lines().count(), c.is_whitespace(),
                 s.split_whitespace().collect::<Vec<&str>>());
    }

    println!();
    println!("THE ALTERNATIVES THE RFC POINTED AT, AS THEY BEHAVE TODAY");
    println!("  split('\\n')        {:?}", windows.split('\n').collect::<Vec<&str>>());
    println!("  split_inclusive    {:?}", windows.split_inclusive('\n').collect::<Vec<&str>>());
    let raw: Vec<Vec<u8>> = Cursor::new("ab\r\ncd\r\n").split(b'\n').map(|v| v.unwrap()).collect();
    println!("  BufRead::split     {raw:?}");
    println!("                     ^ Vec<u8>, not String -- and the 13 is the \\r");
    println!("  lines_any          {:?}", windows.lines_any().collect::<Vec<&str>>());
}
