//! What a trait is: a set of behaviour a type promises to have, written down.
//!
//!   rustc --edition 2024 what_a_trait_is.rs -o /tmp/wti && /tmp/wti

// ---------------------------------------------------------------------------
// A trait can hold three kinds of thing: methods, associated constants, and
// associated types. Two of the three are here; the third gets its own page.
// ---------------------------------------------------------------------------
trait Method {
    // No body => ABSTRACT. Every implementor is forced to write this one.
    fn name(&self) -> String;

    // An associated CONSTANT: the value belongs to the implementing type.
    const BALLOT: &'static str;

    // A body => a DEFAULT. An implementor may take it or replace it.
    // `Self::BALLOT` reaches whichever type is implementing.
    fn describe(&self) -> String {
        format!("{} reads a {} ballot", self.name(), Self::BALLOT)
    }
}

// `Self` is the implicit type parameter every trait has: "the type implementing
// me". Here it is used as a RETURN type, which makes one signature build two
// different types.
trait Fresh {
    fn fresh() -> Self;
}

// ---------------------------------------------------------------------------
// Two types. Note they share no fields, no parent, and no code — only the
// promise. There is no inheritance anywhere in this file.
// ---------------------------------------------------------------------------
struct Star;
struct Approval {
    threshold: u8,
}

impl Method for Star {
    const BALLOT: &'static str = "0-5 score";
    fn name(&self) -> String {
        "STAR".to_string()
    }
    // `describe` not written: Star takes the default body.
}

impl Method for Approval {
    const BALLOT: &'static str = "yes/no";
    fn name(&self) -> String {
        format!("Approval (approve at {}+)", self.threshold)
    }
    // ... and Approval replaces it.
    fn describe(&self) -> String {
        format!("{} — one bubble per candidate", self.name())
    }
}

impl Fresh for Star {
    fn fresh() -> Self {
        Star
    }
}

impl Fresh for Approval {
    fn fresh() -> Self {
        Approval { threshold: 3 }
    }
}

fn main() {
    let star = Star;
    let approval = Approval { threshold: 3 };

    println!("1. The abstract method — the compiler forced both to write it");
    println!("   star.name()     = {}", star.name());
    println!("   approval.name() = {}", approval.name());

    println!();
    println!("2. The default body — taken by one, replaced by the other");
    println!("   star.describe()     = {}", star.describe());
    println!("   approval.describe() = {}", approval.describe());

    println!();
    println!("3. The associated constant belongs to the TYPE, not the value");
    println!("   <Star as Method>::BALLOT     = {}", <Star as Method>::BALLOT);
    println!("   <Approval as Method>::BALLOT = {}", <Approval as Method>::BALLOT);

    println!();
    println!("4. `Self` = the implementing type, so one signature builds two");
    println!("   Star::fresh().name()     = {}", Star::fresh().name());
    println!("   Approval::fresh().name() = {}", Approval::fresh().name());

    println!();
    println!("5. A trait is a promise, not a payload: implementing it adds no bytes");
    println!("   size_of::<Star>()     = {}", std::mem::size_of::<Star>());
    println!("   size_of::<Approval>() = {}", std::mem::size_of::<Approval>());
}
