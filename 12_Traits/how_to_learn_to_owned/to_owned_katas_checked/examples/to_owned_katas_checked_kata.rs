//! Kata solution: repair the username pair so that `to_owned` is a copy.
//! The canonicalizing moves to the only doors in — `Username::new` for raw
//! text, `UsernameRef::parse` for text that must already be canonical — and
//! then `Borrow`, `Cow` and `HashMap` agree with each other again.
//!
//!   rustc --edition 2024 to_owned_katas_checked_kata.rs -o /tmp/tokck && /tmp/tokck

use std::borrow::{Borrow, Cow};
use std::collections::HashMap;

mod username {
    use std::borrow::Borrow;
    use std::fmt;

    /// Always canonical: lowercase words joined by '-'.
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Username(String);

    /// Always canonical too — that is what makes it `Username`'s borrowed half.
    #[repr(transparent)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UsernameRef(str);

    #[derive(Debug)]
    pub struct NotCanonical {
        pub suggestion: Username,
    }

    fn canonicalize(raw: &str) -> String {
        let words: Vec<String> = raw.split_whitespace().map(str::to_lowercase).collect();
        words.join("-")
    }

    impl Username {
        /// Door 1: any text in, canonical out. The transform lives here.
        pub fn new(raw: &str) -> Username {
            Username(canonicalize(raw))
        }

        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    impl UsernameRef {
        /// Door 2: borrow without allocating, but only text that is already canonical.
        pub fn parse(s: &str) -> Result<&UsernameRef, NotCanonical> {
            let canonical = canonicalize(s);
            if canonical == s {
                Ok(UsernameRef::from_str_unchecked(s))
            } else {
                Err(NotCanonical { suggestion: Username(canonical) })
            }
        }

        /// Private: callers reach it only through a check, or from a `Username`.
        fn from_str_unchecked(s: &str) -> &UsernameRef {
            // SAFETY: UsernameRef is #[repr(transparent)] over str.
            unsafe { &*(std::ptr::from_ref::<str>(s) as *const UsernameRef) }
        }

        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    impl Borrow<UsernameRef> for Username {
        fn borrow(&self) -> &UsernameRef {
            UsernameRef::from_str_unchecked(&self.0)
        }
    }

    impl ToOwned for UsernameRef {
        type Owned = Username;

        /// A copy, like `str`'s and `Path`'s: the value was checked on the way in.
        fn to_owned(&self) -> Username {
            Username(self.0.to_owned())
        }
    }

    impl fmt::Display for UsernameRef {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(&self.0)
        }
    }
}

use username::{Username, UsernameRef};

fn main() {
    println!("1. The kata's four tests, with Username::new as the canonical() helper");
    let canonical = Username::new;
    assert_eq!(canonical("  Alice  ").as_str(), "alice");
    assert_eq!(canonical("Foo   Bar\tBaz").as_str(), "foo-bar-baz");
    assert_eq!(canonical("bob").as_str(), "bob");
    let u = canonical("  Grace  Hopper ");
    let r: &UsernameRef = u.borrow();
    assert_eq!(r.as_str(), "grace-hopper");
    assert_eq!(r.to_owned(), u);
    println!("   \"  Alice  \"       -> {:?}", canonical("  Alice  ").as_str());
    println!("   \"Foo   Bar\\tBaz\"  -> {:?}", canonical("Foo   Bar\tBaz").as_str());
    println!("   \"bob\"             -> {:?}", canonical("bob").as_str());
    println!("   borrow round trip -> {r}, r.to_owned() == u: {}", r.to_owned() == u);
    println!("   all four assertions hold, unchanged");

    println!();
    println!("2. Raw text can no longer become a UsernameRef");
    match UsernameRef::parse("  Grace  Hopper ") {
        Ok(r) => println!("   parsed {r}"),
        Err(e) => println!("   parse(\"  Grace  Hopper \") -> Err, did you mean {:?}?", e.suggestion),
    }
    let tidy = UsernameRef::parse("grace-hopper").expect("already canonical");
    println!("   parse(\"grace-hopper\")     -> Ok({tidy})");

    println!();
    println!("3. The checks the transforming to_owned failed, run again");
    let owned = tidy.to_owned();
    let back: &UsernameRef = owned.borrow();
    println!("   tidy.to_owned().borrow() == tidy       {}", back == tidy);

    let mut cow: Cow<'_, UsernameRef> = Cow::Borrowed(tidy);
    let before = cow.as_str().to_owned();
    let _ = cow.to_mut();
    println!("   to_mut() kept the value                {}", cow.as_str() == before);
    println!("   Cow::Borrowed == Cow::Owned            {}", Cow::Borrowed(tidy) == Cow::<UsernameRef>::Owned(tidy.to_owned()));

    let mut seats: HashMap<Username, u32> = HashMap::new();
    seats.insert(Username::new("Ada"), 3);
    let key = UsernameRef::parse("ada").expect("canonical");
    println!("   seats.get(parse(\"ada\"))                {:?}", seats.get(key));
}
