//! What an invariant is: a promise carried by the type, spent by code that
//! skips a check.
//!
//!   rustc --edition 2024 what_an_invariant_is.rs -o /tmp/inv && /tmp/inv

// The invalid byte arrays below are this page's subject, not a mistake, so the
// lint that spots them is turned off rather than worked around.
#![allow(invalid_from_utf8)]

mod ascii_text {
    /// A `String` known to hold only ASCII.
    pub struct AsciiText(String); // INVARIANT: every byte is < 0x80

    impl AsciiText {
        /// The one door in. Everything below is entitled to assume the invariant.
        pub fn new(s: String) -> Option<AsciiText> {
            if s.is_ascii() { Some(AsciiText(s)) } else { None }
        }

        /// O(1), because one byte is one `char` here -- which is true only
        /// because `new` refused everything else.
        pub fn nth(&self, i: usize) -> Option<char> {
            self.0.as_bytes().get(i).copied().map(char::from)
        }

        pub fn as_str(&self) -> &str {
            &self.0
        }
    }
}

use ascii_text::AsciiText;

fn main() {
    println!("1. An invariant is a promise, and a door that will not let it through");
    let good = [0xE2, 0x9D, 0xA4]; // UTF-8 for U+2764
    let s = str::from_utf8(&good).unwrap();
    println!("   str::from_utf8({good:02X?})  = Ok({s:?})");
    let bad = [0x68, 0x69, 0xFF];
    match str::from_utf8(&bad) {
        Ok(_) => unreachable!(),
        Err(e) => println!(
            "   str::from_utf8({bad:02X?})  = Err(valid_up_to = {})",
            e.valid_up_to()
        ),
    }
    println!("   std states it as a heading, `# Invariant`, on the `str` primitive:");
    println!("   \"Rust libraries may assume that string slices are always valid UTF-8.\"");
    println!();

    println!("2. Spending it: `Chars::next` validates nothing at all");
    let text = "héllo ❤";
    println!("   {text:?} is {} bytes, {} chars", text.len(), text.chars().count());
    let first: char = text.chars().next().unwrap();
    println!("   chars().next() is Option<char>, not Option<Result<char, _>> -> {first:?}");
    println!("   The missing error case IS the invariant, showing up in the type.");
    println!();

    println!("3. Why the SAFETY comment's second clause follows from its first");
    println!("   char::from_u32(0xD7FF) = {:?}", char::from_u32(0xD7FF));
    println!(
        "   char::from_u32(0xD800) = {:?}   <- a surrogate is not a scalar value",
        char::from_u32(0xD800)
    );
    println!("   char::from_u32(0xE000) = {:?}", char::from_u32(0xE000));
    let surrogate = [0xED, 0xA0, 0x80]; // what U+D800 would encode to
    println!(
        "   str::from_utf8({surrogate:02X?}).is_err() = {}",
        str::from_utf8(&surrogate).is_err()
    );
    println!("   The validator refuses the only bytes that could decode into");
    println!("   that gap, so `from_u32_unchecked` can never be reached with one.");
    println!();

    println!("4. What the invariant costs: std panics rather than break it");
    let h = "héllo";
    println!("   {h:?}: is_char_boundary(1) = {}", h.is_char_boundary(1));
    println!(
        "   {h:?}: is_char_boundary(2) = {}   <- &h[1..2] panics here",
        h.is_char_boundary(2)
    );
    println!("   Handing back half of 'é' would be a &str that is not UTF-8,");
    println!("   so the slice aborts instead. The promise outranks the answer.");
    println!();

    println!("5. Your own invariant needs no `unsafe` to pay off");
    let t = AsciiText::new(String::from("status: 404")).unwrap();
    println!("   AsciiText::new({:?}) accepted", t.as_str());
    println!("   t.nth(8) = {:?}   <- a byte index, O(1)", t.nth(8));
    println!(
        "   AsciiText::new(\"héllo\").is_none() = {}",
        AsciiText::new(String::from("héllo")).is_none()
    );
    println!("   `nth` re-checks nothing. `new` paid once, for every call after it.");
}
