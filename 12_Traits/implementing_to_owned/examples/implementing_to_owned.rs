//! `ToOwned` for a type of your own goes on the REFERENT, never on a
//! reference-like struct. The borrowed half is an unsized `#[repr(transparent)]`
//! wrapper around `str`, the owned half wraps a `String`, a pointer cast lends
//! one out of the other, and a check guards the only public way in.
//!
//!   rustc --edition 2024 implementing_to_owned.rs -o /tmp/ito && /tmp/ito

use std::borrow::{Borrow, Cow};
use std::collections::HashMap;

use ascii::{AsciiStr, AsciiString};

mod ascii {
    use std::borrow::Borrow;
    use std::fmt;

    /// The borrowed half, in `str`'s role. Its only field is unsized, so the
    /// struct is too: nobody holds an `AsciiStr`, only a `&AsciiStr`.
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    pub struct AsciiStr {
        text: str,
    }

    /// The owned half, in `String`'s role.
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct AsciiString {
        text: String,
    }

    #[derive(Debug)]
    pub struct NotAscii {
        pub at: usize,
    }

    impl AsciiStr {
        /// Private on purpose. Every `&AsciiStr` in the program comes out of
        /// this cast, and only two callers reach it: `try_from`, after the
        /// check, and `borrow`, on bytes that were checked on the way in.
        fn from_str_unchecked(s: &str) -> &AsciiStr {
            // SAFETY: `AsciiStr` is `#[repr(transparent)]` over `str`, so the
            // two pointers have the same layout and carry the same length.
            // Soundness rests on that attribute alone, not on the ASCII check:
            // a non-ASCII `AsciiStr` would be a wrong value, not undefined
            // behaviour, because the bytes are still a valid `str`.
            unsafe { &*(s as *const str as *const AsciiStr) }
        }

        pub fn as_str(&self) -> &str {
            &self.text
        }

        pub fn has_lowercase(&self) -> bool {
            self.text.bytes().any(|b| b.is_ascii_lowercase())
        }

        /// Uppercasing ASCII can only produce ASCII, so this builds the owned
        /// half directly instead of checking a second time.
        pub fn to_uppercase(&self) -> AsciiString {
            AsciiString { text: self.text.to_ascii_uppercase() }
        }
    }

    impl AsciiString {
        pub fn as_str(&self) -> &str {
            &self.text
        }
    }

    impl<'a> TryFrom<&'a str> for &'a AsciiStr {
        type Error = NotAscii;

        fn try_from(s: &'a str) -> Result<Self, NotAscii> {
            match s.bytes().position(|b| !b.is_ascii()) {
                Some(at) => Err(NotAscii { at }),
                None => Ok(AsciiStr::from_str_unchecked(s)),
            }
        }
    }

    /// Owned -> borrowed. It returns a *reference*, so it needs something
    /// inside `self` to point at: here, the `String`'s own bytes.
    impl Borrow<AsciiStr> for AsciiString {
        fn borrow(&self) -> &AsciiStr {
            AsciiStr::from_str_unchecked(&self.text)
        }
    }

    /// Borrowed -> owned. The trait's `Owned: Borrow<Self>` bound is met by
    /// the impl directly above.
    impl ToOwned for AsciiStr {
        type Owned = AsciiString;

        fn to_owned(&self) -> AsciiString {
            AsciiString { text: self.text.to_owned() }
        }
    }

    impl fmt::Display for AsciiStr {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(&self.text)
        }
    }

    impl fmt::Display for AsciiString {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(&self.text)
        }
    }
}

/// A `Cow` over your own type, which is what `ToOwned` is for: borrowed when
/// the input is already uppercase, one new `AsciiString` when it is not.
fn shout(s: &AsciiStr) -> Cow<'_, AsciiStr> {
    if s.has_lowercase() {
        Cow::Owned(s.to_uppercase())
    } else {
        Cow::Borrowed(s)
    }
}

// ---- The question's own types ----------------------------------------------

#[derive(Clone)]
struct DataRef<'a> {
    text: &'a str,
}

struct DataOwned {
    text: String,
}

impl DataRef<'_> {
    /// An inherent method, which is what the question wanted in the first
    /// place. Method lookup tries inherent methods before trait methods at
    /// each receiver type, so `r.to_owned()` lands here, not in the blanket impl.
    fn to_owned(&self) -> DataOwned {
        DataOwned { text: self.text.to_owned() }
    }
}

// ---- What a generic associated type lets a trait say -----------------------

/// `Borrow::borrow` must return `&Borrowed`, a pointer into `self`. With a
/// generic associated type the owned side hands back the borrowed form *by
/// value*, so a struct holding several references is fine. Same shape as the
/// `borrowme` crate's `Borrow`.
trait Lend {
    type Target<'a>
    where
        Self: 'a;

    fn lend(&self) -> Self::Target<'_>;
}

struct Name {
    first: String,
    last: String,
}

struct NameRef<'a> {
    first: &'a str,
    last: &'a str,
}

impl Lend for Name {
    type Target<'a> = NameRef<'a>;

    fn lend(&self) -> NameRef<'_> {
        NameRef { first: &self.first, last: &self.last }
    }
}

fn main() {
    println!("1. The only public way to an &AsciiStr is through the check");
    let ada: &AsciiStr = "Ada".try_into().unwrap();
    println!("   \"Ada\"  -> Ok  {ada:?}");
    match <&AsciiStr>::try_from("caf\u{e9}") {
        Ok(s) => println!("   \"caf\u{e9}\" -> Ok  {s:?}"),
        Err(e) => println!("   \"caf\u{e9}\" -> Err, first non-ASCII byte at index {}", e.at),
    }
    println!(
        "   size_of::<&AsciiStr>() == size_of::<&str>(): {}   (address + length)",
        size_of::<&AsciiStr>() == size_of::<&str>()
    );

    println!();
    println!("2. to_owned buys the owned twin; borrow lends it back without copying");
    let owned: AsciiString = ada.to_owned();
    println!("   ada.to_owned()   -> {owned:?}");
    let lent: &AsciiStr = owned.borrow();
    println!(
        "   owned.borrow()   -> {lent:?}, pointing into owned's own buffer: {}",
        lent.as_str().as_ptr() == owned.as_str().as_ptr()
    );

    println!();
    println!("3. Borrow buys lookups by the borrowed form");
    let mut seats: HashMap<AsciiString, u32> = HashMap::new();
    seats.insert(owned.clone(), 3);
    let key: &AsciiStr = "Ada".try_into().unwrap();
    println!("   seats: HashMap<AsciiString, u32>, key: &AsciiStr");
    println!("   seats.get(key) = {:?}", seats.get(key));

    println!();
    println!("4. ToOwned buys Cow");
    for word in ["HELLO", "hello"] {
        let word: &AsciiStr = word.try_into().unwrap();
        match shout(word) {
            Cow::Borrowed(s) => println!("   shout(\"{word}\") -> Cow::Borrowed(\"{s}\")   nothing allocated"),
            Cow::Owned(s) => println!("   shout(\"{word}\") -> Cow::Owned(\"{s}\")      one new AsciiString"),
        }
    }
    let loud = shout(ada);
    println!("   a Cow<AsciiStr> derefs to &AsciiStr either way: {:?}", loud.as_str());

    println!();
    println!("5. The question's DataRef: an inherent method instead of the trait");
    let r = DataRef { text: "x" };
    let mine: DataOwned = r.to_owned();
    let blanket: DataRef<'_> = ToOwned::to_owned(&r);
    println!("   r.to_owned()          -> DataOwned {{ text: {:?} }}   inherent, found first", mine.text);
    println!("   ToOwned::to_owned(&r) -> DataRef   {{ text: {:?} }}   the blanket impl, still there", blanket.text);

    println!();
    println!("6. A GAT lends a struct of references by value: two fields, no cast");
    let name = Name { first: String::from("Ada"), last: String::from("Lovelace") };
    let view: NameRef<'_> = name.lend();
    println!("   name.lend() -> NameRef {{ first: {:?}, last: {:?} }}", view.first, view.last);
    println!(
        "   both fields point into name's own buffers: {}",
        view.first.as_ptr() == name.first.as_ptr() && view.last.as_ptr() == name.last.as_ptr()
    );
}
