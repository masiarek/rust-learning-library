// `split` returns a plan. This program prints the plan, then reads it.
//
// The field names below belong to std's private internals, not to a stable
// API: they are what rustc 1.98.0 prints, and a later compiler may print
// something else. Nothing here should be parsed by a real program.

/// Pull one scalar field out of a `{:?}` dump — `allow_trailing_empty: true,`
/// gives `"true"`, `matcher: StrSearcher {` gives `"StrSearcher"`.
fn field<'a>(dump: &'a str, name: &str) -> &'a str {
    let key = format!("{name}: ");
    let after = dump.split_once(key.as_str()).expect("field is present").1;
    after.split([',', ' ', '(']).next().expect("a value follows")
}

fn main() {
    let s = "a:b:c";

    // What you wanted: consume the iterator.
    let pieces: Vec<&str> = s.split(":").collect();
    println!("collected      {pieces:?}");

    // What `{:?}` on the iterator itself prints.
    println!("uncollected    {:?}", s.split(":"));

    // ---- the plan, one field per line ----
    println!("\n{:#?}", s.split(":"));

    // ---- the pattern you pass picks the searcher ----
    println!("\nthe four pattern shapes build four different machines:");
    for (call, dump) in [
        ("split(\":\")", format!("{:?}", s.split(":"))),
        ("split(':')", format!("{:?}", s.split(':'))),
        ("split(char::is_numeric)", format!("{:?}", s.split(char::is_numeric))),
        ("split(&['-', '_'][..])", format!("{:?}", s.split(&['-', '_'][..]))),
    ] {
        println!("  {call:<24} -> {}", field(&dump, "matcher"));
    }

    // ---- split vs split_terminator: one bool ----
    let split = format!("{:?}", s.split(":"));
    let term = format!("{:?}", s.split_terminator(":"));
    println!("\nsplit            allow_trailing_empty: {}", field(&split, "allow_trailing_empty"));
    println!("split_terminator allow_trailing_empty: {}", field(&term, "allow_trailing_empty"));

    // ---- byteset is a 64-bit fingerprint of the needle's bytes ----
    let fingerprint = |needle: &str| needle.bytes().fold(0u64, |set, b| set | (1u64 << (b & 63)));
    println!("\nbyteset for \":\"   {}", fingerprint(":"));
    println!("1 << (b':' & 63)  {}", 1u64 << (b':' & 63));
    println!("byteset for \"ain\" {}", fingerprint("ain"));

    // ---- memory: that huge number is a sentinel ----
    let long = format!("{:?}", "the rain in spain".split("ain"));
    println!("\nmemory for \"ain\"  {}", field(&long, "memory"));
    println!("usize::MAX        {}", usize::MAX);

    // ---- the plan is a cursor; consuming moves it ----
    let mut it = s.split(":");
    println!("\nthe same struct, after each next():");
    println!("  {:<12} {:>6} {:>9} {:>9}", "next()", "start", "position", "finished");
    for _ in 0..4 {
        let got = format!("{:?}", it.next());
        let d = format!("{it:?}");
        println!(
            "  {got:<12} {:>6} {:>9} {:>9}",
            field(&d, "start"),
            field(&d, "position"),
            field(&d, "finished"),
        );
    }

    // ---- not every iterator hides its contents ----
    println!("\n{:?}", s.chars());
}
