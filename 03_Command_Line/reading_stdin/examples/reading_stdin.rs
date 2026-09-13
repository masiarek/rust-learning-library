//! Reading a line from standard input, and what `read_line` hands back.
//!
//!   rustc --edition 2024 reading_stdin.rs -o /tmp/reading_stdin && /tmp/reading_stdin
//!
//! The harness that records this page's answer key cannot type at a terminal,
//! so every reader here is `impl BufRead` and `main` feeds it a scripted
//! transcript through `io::Cursor`. `io::stdin().lock()` is a `BufRead` too —
//! swap it in and nothing else changes. Section 8 says why that is the point.

use std::io::{self, BufRead, Cursor};

/// Ask until the reader gives a whole number, skipping lines that are not one.
/// `Ok(None)` means the input ended before it did.
fn ask_number(input: &mut impl BufRead) -> io::Result<Option<u32>> {
    let mut line = String::new();
    loop {
        line.clear(); //                    read_line appends, so start empty
        let n = input.read_line(&mut line)?;
        if n == 0 {
            return Ok(None); //             nothing came in: end of input
        }
        match line.trim().parse::<u32>() {
            Ok(v) => return Ok(Some(v)),
            Err(e) => println!("      {:?} is not a number ({e}), asking again", line.trim()),
        }
    }
}

fn main() {
    // What the person at the keyboard "types": three lines, Enter after the
    // first two (the second from a Windows terminal, so `\r\n`), then the
    // terminal closes on the third before any Enter.
    let mut input = Cursor::new("42\n  seven \r\nlast line, no newline");

    println!("1. ONE CALL: read_line APPENDS, KEEPS THE NEWLINE, RETURNS THE COUNT");
    let mut line = String::new();
    let n = input.read_line(&mut line).unwrap();
    println!("   returned Ok({n})   buffer {line:?}");
    println!("   Three bytes: '4', '2' and the newline. read_line does not strip it.");

    println!();
    println!("2. TRIM, THEN PARSE — AND WHAT parse SAYS IF YOU FORGET TO TRIM");
    match line.parse::<u32>() {
        Ok(v) => println!("   {line:?}.parse()        -> Ok({v})"),
        Err(e) => println!("   {line:?}.parse()        -> Err: {e}"),
    }
    match line.trim().parse::<u32>() {
        Ok(v) => println!("   {:?}.parse()   -> Ok({v})", line.trim()),
        Err(e) => println!("   {:?}.parse()   -> Err: {e}", line.trim()),
    }
    println!("   The newline is an invalid digit. trim() takes it, and any spaces, off first.");

    println!();
    println!("3. A SECOND CALL WITHOUT clear(): THE BUFFER GROWS");
    let n = input.read_line(&mut line).unwrap();
    println!("   returned Ok({n})  buffer {line:?}");
    println!("   Both lines are in there. read_line appends to what you hand it; it never");
    println!("   overwrites. A loop that forgets clear() reads every line into one String.");

    println!();
    println!("4. A LAST LINE WITH NO NEWLINE STILL COMES BACK; THEN Ok(0) IS THE END");
    line.clear();
    let n = input.read_line(&mut line).unwrap();
    println!("   returned Ok({n})  buffer {line:?}");
    line.clear();
    let n = input.read_line(&mut line).unwrap();
    println!("   returned Ok({n})   buffer {line:?}");
    let n = input.read_line(&mut line).unwrap();
    println!("   returned Ok({n})   buffer {line:?}   (and again: still nothing)");
    println!("   Zero bytes is not an error. It is the only signal that the input is over —");
    println!("   Ctrl-D on Unix, Ctrl-Z Enter on Windows, or the end of whatever was piped in.");

    println!();
    println!("5. THE \\r\\n LINE: trim_end() KEEPS THE INDENT, trim() DOES NOT");
    let windows = "  seven \r\n"; //         the second line above, as typed on Windows
    println!("   line            {windows:?}");
    println!("   line.trim_end() {:?}   the line ending is gone, the two leading spaces stay", windows.trim_end());
    println!("   line.trim()     {:?}   both ends", windows.trim());
    println!("   lines() would have handed you {:?} — it strips \\n and \\r\\n for you.", windows.trim_end_matches(['\r', '\n']));

    println!();
    println!("6. THE LOOP: ASK UNTIL IT IS A NUMBER, OR UNTIL THERE IS NOTHING LEFT");
    let scripts: [(&str, &str); 3] = [
        ("abc, a blank line, then 17", "abc\n\n 17 \n"),
        ("only a wrong answer",        "forty-two\n"),
        ("nothing at all",             ""),
    ];
    for (label, typed) in scripts {
        println!("   typed: {label}");
        match ask_number(&mut Cursor::new(typed)).unwrap() {
            Some(v) => println!("      -> Some({v})"),
            None => println!("      -> None: the input ended first"),
        }
    }

    println!();
    println!("7. THE ONE FAILURE YOU WILL ACTUALLY MEET: BYTES THAT ARE NOT UTF-8");
    let latin2: &[u8] = b"caf\xe9\nnext\n"; //  'é' as Latin-1/Latin-2 byte e9, not UTF-8
    let mut input = Cursor::new(latin2);
    let mut line = String::new();
    match input.read_line(&mut line) {
        Ok(n) => println!("   Ok({n}) {line:?}"),
        Err(e) => println!("   Err: kind {:?}, {e}", e.kind()),
    }
    println!("   buffer after the error: {line:?}   (nothing was appended)");
    line.clear();
    let n = input.read_line(&mut line).unwrap();
    println!("   the next call reads on: Ok({n}) {line:?}");
    println!("   read_line promises a String, and a String promises UTF-8, so one byte that");
    println!("   is not UTF-8 is refused whole. A file saved by Windows in code page 1250 does");
    println!("   this on its first accented letter. To read bytes you did not promise anything");
    println!("   about, ask for bytes: read_until(b'\\n', &mut Vec<u8>).");

    println!();
    println!("8. WHY EVERY READER ON THIS PAGE IS `impl BufRead`");
    println!("   ask_number never named stdin. It takes any BufRead, so this program could feed");
    println!("   it a Cursor over a string and record what it printed — and your program feeds");
    println!("   it io::stdin().lock(). Same function, same body, one line different at the");
    println!("   call site. That is the whole of testing an input routine without a keyboard.");
}
