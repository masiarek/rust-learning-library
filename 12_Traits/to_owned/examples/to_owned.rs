//! `ToOwned`: `Clone` for types whose owned twin is a DIFFERENT type.
//!
//!   rustc --edition 2024 to_owned.rs -o /tmp/tow && /tmp/tow

use std::borrow::Cow;
use std::rc::Rc;

fn needs_string(s: String) -> usize {
    s.len()
}

// The reason `Owned: Borrow<Self>` is in the trait: generic code can take
// either half of the pair and only pay for an allocation on the branch that
// needs one.
fn tidy(raw: &str) -> Cow<'_, str> {
    if raw.contains("  ") {
        Cow::Owned(raw.split_whitespace().collect::<Vec<_>>().join(" "))
    } else {
        Cow::Borrowed(raw)
    }
}

// A user-defined type CAN implement ToOwned — but only if it is Sized and does
// NOT implement Clone. `type Owned = Self` satisfies the `Owned: Borrow<Self>`
// bound through the blanket `impl<T> Borrow<T> for T`, and with no `Clone` impl
// there is nothing for the blanket `impl<T: Clone> ToOwned for T` to conflict
// with. Add `#[derive(Clone)]` and this file stops compiling with E0119.
#[derive(Debug)]
struct Tally {
    seats: i32,
    name: String,
}

impl ToOwned for Tally {
    type Owned = Tally;
    fn to_owned(&self) -> Tally {
        Tally { seats: self.seats, name: self.name.clone() }
    }
}

fn main() {
    let name: &str = "Adam";
    let nums: &[i32] = &[3, 1, 2];

    println!("1. The owned twin is a different TYPE — that is the whole point");
    let owned: String = name.to_owned();
    let owned_nums: Vec<i32> = nums.to_owned();
    println!("   {:<11} (&str)    .to_owned() -> String {:?}", format!("{:?}", name), owned);
    println!("   {:<11} (&[i32])  .to_owned() -> Vec    {:?}", format!("{:?}", nums), owned_nums);

    println!();
    println!("2. `.clone()` cannot do that: on a &str it hands back another &str");
    // rustc warns on this line (`noop_method_call`), and its wording is the
    // lesson: "the type `str` does not implement `Clone`, so calling `clone`
    // on `&str` copies the reference". Allowed here so the call survives to be
    // looked at; the warning itself is quoted on the page.
    #[allow(noop_method_call)]
    let cloned: &str = name.clone();
    println!("   name.clone()    is still a &str: {:?}", cloned);
    println!("   needs_string(name.to_owned()) = {}", needs_string(name.to_owned()));
    println!("   needs_string(name.clone())    would be E0308 — expected String, found &str");
    println!("   size_of::<&str>()   = {:>2}   pointer + length", std::mem::size_of::<&str>());
    println!("   size_of::<String>() = {:>2}   pointer + length + capacity", std::mem::size_of::<String>());

    println!();
    println!("2b. WHY .clone() behaves differently on &str than on &String");
    let owned_string = String::from("Ada");
    let r: &String = &owned_string;
    // Method lookup tries the receiver type FIRST. `Clone::clone` takes &self,
    // so for `&String` the very first candidate is <String as Clone> — you get
    // a String. For `&str` that candidate does not exist, because `str` is
    // unsized and has no Clone impl at all, so lookup falls through to the
    // reference's OWN Clone and hands back another &str.
    let from_ref: String = r.clone();
    println!("   (&String).clone() -> String  {:?}   <- String: Clone exists", from_ref);
    println!("   (&str).clone()    -> &str    {:?}  <- str: Clone does NOT", cloned);
    println!("   str is !Sized, and Clone requires Sized. That is the whole reason");
    println!("   ToOwned exists at all.");

    println!();
    println!("3. For everything that is Clone, they are the SAME call");
    let s = String::from("Ada");
    println!("   s.clone()    = {:?}", s.clone());
    println!("   s.to_owned() = {:?}   <- blanket impl<T: Clone> ToOwned for T", s.to_owned());

    println!();
    println!("4. The trap that blanket impl sets: on an Rc it clones the POINTER");
    let shared = Rc::new(String::from("ballot"));
    println!("   strong_count before      = {}", Rc::strong_count(&shared));
    let second = shared.to_owned();
    println!("   strong_count after       = {}", Rc::strong_count(&shared));
    println!("   same allocation?           {}", Rc::ptr_eq(&shared, &second));
    let deep: String = (*shared).clone();
    println!("   (*shared).clone() is the real copy: {:?}", deep);

    println!();
    println!("5. `clone_into`: the provided method that reuses a buffer");
    let mut buf = String::with_capacity(64);
    let before = buf.capacity();
    "reuse me".clone_into(&mut buf);
    println!("   capacity {} -> {} , contents {:?}   (no new allocation)", before, buf.capacity(), buf);

    println!();
    println!("6. Why the trait is shaped that way: Cow pays only when it must");
    for raw in ["one  two", "one two"] {
        let out = tidy(raw);
        let kind = match out {
            Cow::Borrowed(_) => "borrowed — nothing allocated",
            Cow::Owned(_) => "owned — one allocation",
        };
        println!("   {:<12} -> {:<11} {}", format!("{:?}", raw), format!("{:?}", tidy(raw)), kind);
    }

    println!();
    println!("7. Implementing it yourself: legal, and pointless");
    let t = Tally { seats: 3, name: String::from("Ada") };
    let copy = (&t).to_owned();
    println!("   {:?}.to_owned() -> {:?}", t, copy);
    println!("   type Owned = Self, so this is Clone wearing a different name.");
    println!("   Give Tally a #[derive(Clone)] and it is E0119 instead: the");
    println!("   blanket impl<T: Clone> ToOwned for T already covers it.");
}
