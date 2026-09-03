//! `Cow`: borrow until somebody writes.
//!
//!   rustc --edition 2024 clone_on_write.rs -o /tmp/cow && /tmp/cow

use std::borrow::Cow;

/// Address lines arrive from a CSV. Most are already clean; the ones a Windows
/// machine wrote carry a stray `\r`. Only those need a new buffer — and the
/// return type has to cover both cases without deciding in advance.
fn clean(raw: &str) -> Cow<'_, str> {
    if raw.contains('\r') {
        Cow::Owned(raw.replace('\r', ""))
    } else {
        Cow::Borrowed(raw)
    }
}

/// The same job, written the obvious way. Correct, and it allocates every time.
fn clean_always_owned(raw: &str) -> String {
    raw.replace('\r', "")
}

fn kind(c: &Cow<'_, str>) -> &'static str {
    match c {
        Cow::Borrowed(_) => "Borrowed - nothing allocated",
        Cow::Owned(_) => "Owned    - one allocation",
    }
}

fn main() {
    let lines = ["Ada Lovelace", "Grace Hopper\r", "Alan Turing"];

    println!("1. One return type, two outcomes");
    for raw in lines {
        let out = clean(raw);
        println!(
            "   {:<18} -> {:<16} {}",
            format!("{raw:?}"),
            format!("{out:?}"),
            kind(&out)
        );
    }

    println!();
    println!("2. The borrowed arm really is the caller's bytes");
    let raw = "Ada Lovelace";
    let out = clean(raw);
    // Compared as a derived answer, not printed as an address: an address
    // differs every run and would poison the recorded output.
    println!("   clean(raw) points at raw's own buffer? {}", std::ptr::eq(raw.as_ptr(), out.as_ptr()));
    let dirty = clean("Grace Hopper\r");
    println!("   the owned arm allocated a new one, so it cannot: {:?}", dirty);

    println!();
    println!("3. You do not have to match on it — Cow derefs to &str");
    let c = clean("Grace Hopper\r");
    println!("   len {}   upper {:?}   starts_with(\"Grace\") {}", c.len(), c.to_uppercase(), c.starts_with("Grace"));

    println!();
    println!("4. to_mut() is the write, and the write is where the clone happens");
    let mut c: Cow<'_, str> = Cow::Borrowed("Ada");
    println!("   start          {}", kind(&c));
    c.to_mut().push_str(" Lovelace");
    println!("   after to_mut   {}   {:?}", kind(&c), c);
    c.to_mut().push('!');
    println!("   second write   {}   {:?}   <- already owned, nothing cloned", kind(&c), c);

    println!();
    println!("5. into_owned() always hands back the owned type");
    let borrowed: Cow<'_, str> = Cow::Borrowed("Alan");
    let owned: Cow<'_, str> = Cow::Owned(String::from("Turing"));
    println!("   from Borrowed -> String {:?}   (allocates here)", borrowed.into_owned());
    println!("   from Owned    -> String {:?}   (just hands the buffer over)", owned.into_owned());

    println!();
    println!("6. The tag is free: Cow<str> is no bigger than String");
    println!("   &str                 {:>3} bytes   ptr + len", size_of::<&str>());
    println!("   String               {:>3} bytes   ptr + len + capacity", size_of::<String>());
    println!("   Cow<str>             {:>3} bytes   <- same as String", size_of::<Cow<'_, str>>());
    println!("   Option<Cow<str>>     {:>3} bytes   <- and a spare niche is still left over", size_of::<Option<Cow<'_, str>>>());

    println!();
    println!("7. Not a string thing — any ToOwned pair works");
    let nums: &[i32] = &[3, 1, 2];
    let sorted: Cow<'_, [i32]> = if nums.windows(2).all(|w| w[0] <= w[1]) {
        Cow::Borrowed(nums)
    } else {
        let mut v = nums.to_vec();
        v.sort();
        Cow::Owned(v)
    };
    println!("   {:?} -> {:?}   {}", nums, sorted, match sorted { Cow::Borrowed(_) => "already sorted", Cow::Owned(_) => "had to sort, so had to allocate" });
    println!("   Cow<[i32]>           {:>3} bytes", size_of::<Cow<'_, [i32]>>());

    println!();
    println!("8. What it buys, counted");
    let allocations = lines.iter().filter(|l| matches!(clean(l), Cow::Owned(_))).count();
    println!("   clean()              {} of {} lines allocated", allocations, lines.len());
    let _: Vec<String> = lines.iter().map(|l| clean_always_owned(l)).collect();
    println!("   clean_always_owned() {} of {} lines allocated", lines.len(), lines.len());
}
