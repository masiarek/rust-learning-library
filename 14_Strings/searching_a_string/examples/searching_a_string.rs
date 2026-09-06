//! Searching without splitting: the questions you can ask about a piece of text
//! without cutting it up, and the one trait that makes their arguments
//! interchangeable.
//!
//!   rustc --edition 2024 searching_a_string.rs -o /tmp/sas && /tmp/sas

fn main() {
    let line = "2026-09-06 WARN disk usage 91% on /dev/sda1";

    println!("1. Four questions about one line");
    println!("   {line:?}");
    println!("   contains(\"WARN\")      {}", line.contains("WARN"));
    println!("   find(\"WARN\")          {:?}", line.find("WARN"));
    println!("   starts_with(\"2026\")   {}", line.starts_with("2026"));
    println!("   matches('/').count()  {}", line.matches('/').count());
    println!("   contains is find(..).is_some() under a name that reads: {}",
        line.find("WARN").is_some());

    println!("\n2. One argument, four shapes -- this is the Pattern trait");
    println!("   char                  {:?}", line.find('W'));
    println!("   &str                  {:?}", line.find("WARN"));
    println!("   [char] (any one of)   {:?}", line.find(['%', '/']));
    println!("   FnMut(char) -> bool   {:?}", line.find(char::is_numeric));
    println!("   The same four shapes go to the other families unchanged:");
    println!("   split(char::is_whitespace)     {:?}",
        "a b\tc".split(char::is_whitespace).collect::<Vec<_>>());
    println!("   trim_matches(['[', ']'])       {:?}", "[warn]".trim_matches(['[', ']']));
    println!("   replace(char::is_numeric, \"#\") {:?}", "sda1".replace(char::is_numeric, "#"));

    println!("\n3. find returns a BYTE offset, not a character index");
    let s = "żółw";
    println!("   {s:?} is {} bytes and {} chars", s.len(), s.chars().count());
    let i = s.find('w').unwrap();
    println!("   find('w')  = {i}   <- bytes");
    println!("   as a character index that is {}", s[..i].chars().count());
    println!("   the byte offset is the one that slices: &s[..{i}] = {:?}", &s[..i]);
    println!("   and &s[{i}..] = {:?}", &s[i..]);
    println!("   Feed the character index to a slice instead and one of two");
    println!("   things happens. It lands inside a letter and panics:");
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let boom = std::panic::catch_unwind(|| &s[..3]);
    std::panic::set_hook(hook);
    println!("   &s[..3]  -> {}", if boom.is_ok() { "ok" } else { "panicked" });
    println!("   Or it lands on a boundary and is quietly wrong -- two characters");
    println!("   asked for, one delivered: &s[..2] = {:?}", &s[..2]);

    println!("\n4. Option, not -1");
    println!("   find(\"ERROR\") = {:?}", line.find("ERROR"));
    match line.find("ERROR") {
        Some(at) => println!("   found at {at}"),
        None => println!("   absent -- and there is no offset lying around to misuse"),
    }
    println!("   Where a search returns -1, `&line[i..]` is the waiting bug.");
    println!("   Here the -1 has no way to exist: usize has no negative value and");
    println!("   the answer is not a usize until you have unwrapped it.");

    println!("\n5. starts_with, and the method you probably wanted instead");
    let path = "/dev/sda1";
    println!("   starts_with(\"/dev/\")   {}", path.starts_with("/dev/"));
    println!("   strip_prefix(\"/dev/\")  {:?}", path.strip_prefix("/dev/"));
    println!("   strip_suffix(\"1\")      {:?}", path.strip_suffix("1"));
    println!("   strip_prefix answers the question AND hands back the rest, so the");
    println!("   length of the prefix is never written down a second time.");

    println!("\n6. Backwards");
    let p = "/usr/local/share/doc";
    println!("   {p:?}");
    println!("   find('/')         {:?}", p.find('/'));
    println!("   rfind('/')        {:?}", p.rfind('/'));
    println!("   rsplit_once('/')  {:?}", p.rsplit_once('/'));

    println!("\n7. Two things the search does not do");
    println!("   It does not fold case:   {}", "WARN".contains("warn"));
    println!("   lowercase both sides:    {}", "WARN".to_lowercase().contains("warn"));
    println!("   The empty pattern is everywhere: find(\"\") = {:?}, \"\".contains(\"\") = {}",
        line.find(""), "".contains(""));
}
