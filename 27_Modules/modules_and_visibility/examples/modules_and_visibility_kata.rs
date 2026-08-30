//! Kata solution: the door your own module left open.
//!
//!   rustc --edition 2024 modules_and_visibility_kata.rs -o /tmp/mvk && /tmp/mvk

/// Attempt 1: `pub` everywhere, which validates nothing.
mod leaky {
    #[derive(Debug)]
    pub struct Score {
        pub value: u8,
    }

    impl Score {
        pub fn new(value: u8) -> Option<Self> {
            if value <= 5 { Some(Score { value }) } else { None }
        }
    }
}

/// Attempt 2: the field is private, so the constructor is the only door.
mod sealed {
    #[derive(Debug)]
    pub struct Score {
        value: u8,
    }

    impl Score {
        pub fn new(value: u8) -> Option<Self> {
            if value <= 5 { Some(Score { value }) } else { None }
        }
        pub fn value(&self) -> u8 {
            self.value
        }
    }

    /// ...but anything INSIDE this module can still build one directly, and
    /// this helper is `pub`, so the invariant leaves by the back door.
    pub fn from_raw(value: u8) -> Score {
        Score { value }
    }

    /// The version that keeps the invariant in one place.
    pub fn clamped(value: u8) -> Score {
        Score { value: value.min(5) }
    }
}

fn main() {
    println!("1. The leaky version");
    let mut s = leaky::Score::new(4).unwrap();
    println!("   leaky::Score::new(9) = {:?}   <- the constructor works", leaky::Score::new(9));
    s.value = 200;
    println!("   ...and then: s.value = 200 -> {s:?}");
    println!("   A `pub` field makes the constructor advice, not a rule. Nothing");
    println!("   in the type system stops the assignment, because there is no");
    println!("   invariant recorded anywhere — only a habit.");

    println!();
    println!("2. The sealed version");
    let t = sealed::Score::new(4).unwrap();
    println!("   sealed::Score::new(4) = {t:?}, .value() = {}", t.value());
    println!("   t.value = 200      -> E0616: field `value` of struct `Score` is private");
    println!("   sealed::Score {{ value: 9 }} -> E0451: field `value` of struct");
    println!("   `Score` is private — you cannot even write the literal.");

    println!();
    println!("3. The door that is still open, and it is inside the module");
    let cheat = sealed::from_raw(200);
    println!("   sealed::from_raw(200) = {cheat:?}   <- 200 on a 0-5 scale");
    println!("   `from_raw` is inside `sealed`, so the private field is visible to");
    println!("   it, and it is `pub`, so it is visible to everyone. Privacy is per");
    println!("   MODULE, not per struct: every line in the same file as the type");
    println!("   can construct one. That is the thing to check when an invariant");
    println!("   escapes — not the callers, the module.");
    println!("   sealed::clamped(200) = {:?}   <- the same door, honestly labelled",
             sealed::clamped(200));

    println!();
    println!("4. What actually closes it");
    println!("   a. Delete from_raw, or make it private — then the only way in is");
    println!("      new() and clamped(), and both enforce the range.");
    println!("   b. pub(crate) it, if the crate's own tests need the shortcut but");
    println!("      no downstream user should have it.");
    println!("   c. Move the type into a module of its own with NOTHING else in");
    println!("      it. The smaller the module, the fewer lines can reach the");
    println!("      private field — which is the real argument for one type per");
    println!("      module in a library.");

    println!();
    println!("5. The reading that generalises");
    println!("   `pub struct` publishes the TYPE. Each field is published");
    println!("   separately, and a private field is what makes a constructor the");
    println!("   only entrance. Then check who shares the module with it, because");
    println!("   that list — not the `pub` keywords — is the real access list.");
}
