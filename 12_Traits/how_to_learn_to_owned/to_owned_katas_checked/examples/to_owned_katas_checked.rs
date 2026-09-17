//! Three `ToOwned` katas that circulate together, each claim run on rustc
//! 1.98.0. Kata 3's design is rebuilt here as it was handed out — a borrowed
//! view over any `str` whose `to_owned` canonicalizes — to show what it does
//! to `Borrow`, `Cow` and `HashMap`, which all assume `to_owned` is a copy.
//!
//!   rustc --edition 2024 to_owned_katas_checked.rs -o /tmp/tokc && /tmp/tokc

use std::any::type_name_of_val;
use std::borrow::{Borrow, Cow};
use std::collections::HashMap;
use std::path::Path;

/// Kata 3's owned half: meant to hold only canonical names.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Username(String);

/// Kata 3's borrowed half: a view over ANY `str`, canonical or not.
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Hash)]
struct UsernameRef(str);

impl UsernameRef {
    fn new(s: &str) -> &UsernameRef {
        // SAFETY: UsernameRef is #[repr(transparent)] over str.
        unsafe { &*(std::ptr::from_ref::<str>(s) as *const UsernameRef) }
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

impl Borrow<UsernameRef> for Username {
    fn borrow(&self) -> &UsernameRef {
        UsernameRef::new(&self.0)
    }
}

impl ToOwned for UsernameRef {
    type Owned = Username;

    // The kata's answer: the conversion is where the canonicalizing happens.
    fn to_owned(&self) -> Username {
        let words: Vec<String> = self.0.split_whitespace().map(str::to_lowercase).collect();
        Username(words.join("-"))
    }
}

fn clone_each<T: ?Sized + ToOwned>(xs: &[&T]) -> Vec<T::Owned> {
    xs.iter().map(|x| (*x).to_owned()).collect()
}

fn main() {
    println!("Kata 1. \"str: ToOwned<Owned = String>, [T]: ToOwned<Owned = Vec<T>>\"");
    let text: &str = "a bb ccc";
    let nums: &[i32] = &[1, 2, 3];
    let words: Vec<String> = text.split_whitespace().map(str::to_owned).collect();
    println!("   text.to_owned() -> {}", type_name_of_val(&text.to_owned()));
    println!("   nums.to_owned() -> {}", type_name_of_val(&nums.to_owned()));
    println!("   words           -> {words:?}   holds.");

    println!();
    println!("Kata 2. \"x.to_owned() on a &&T would silently return &T\"");
    let strs: [&str; 2] = ["a", "bb"];
    // Nothing downstream asks for String, so nothing objects.
    let copies: Vec<_> = strs.iter().map(|x| x.to_owned()).collect();
    println!("   Vec<_> from .map(|x| x.to_owned()) -> {}   silent here", type_name_of_val(&copies));
    println!("   inside clone_each, Vec<T::Owned> is asked for -> E0277, not silent");
    let paths = [Path::new("/a"), Path::new("/b/c")];
    println!("   clone_each(&[&str])  -> {}", type_name_of_val(&clone_each(&strs)));
    println!("   clone_each(&[&Path]) -> {}", type_name_of_val(&clone_each(&paths)));
    println!("   clone_each(&[&i32])  -> {}   (the blanket impl: Owned = T)", type_name_of_val(&clone_each(&[&1, &2])));

    println!();
    println!("Kata 3. \"a custom ToOwned whose Owned form is a transformed value\"");
    let tidy = UsernameRef::new("grace-hopper");
    let raw = UsernameRef::new("  Grace  Hopper ");
    println!("   the kata's own round trip starts from a canonical name:");
    let tidy_owned = tidy.to_owned();
    let tidy_back: &UsernameRef = tidy_owned.borrow();
    println!("      tidy.to_owned().borrow() == tidy ? {}", tidy_back == tidy);
    println!("   the same round trip from the input the kata is about:");
    let owned = raw.to_owned();
    let back: &UsernameRef = owned.borrow();
    println!("      raw.to_owned().borrow() == raw  ? {}", back == raw);

    println!();
    println!("   Cow: to_mut() is documented to CLONE; here it rewrites the value");
    let mut cow: Cow<'_, UsernameRef> = Cow::Borrowed(raw);
    let before = cow.as_str().to_owned();
    let _ = cow.to_mut();
    println!("      before to_mut: {before:?}   after: {:?}", cow.as_str());
    let borrowed: Cow<'_, UsernameRef> = Cow::Borrowed(raw);
    let owned_cow: Cow<'_, UsernameRef> = Cow::Owned(raw.to_owned());
    println!("      Cow::Borrowed(raw) == Cow::Owned(raw.to_owned()) ? {}", borrowed == owned_cow);

    println!();
    println!("   HashMap<Username, _>: a lookup by the borrowed form, as Borrow promises");
    let mut seats: HashMap<Username, u32> = HashMap::new();
    seats.insert(UsernameRef::new("Ada").to_owned(), 3);
    println!("      insert UsernameRef::new(\"Ada\").to_owned(), then:");
    println!("      get(UsernameRef::new(\"Ada\")) = {:?}", seats.get(UsernameRef::new("Ada")));
    println!("      get(UsernameRef::new(\"ada\")) = {:?}", seats.get(UsernameRef::new("ada")));
}
