//! A trait's methods do not exist until the trait itself is in scope.
//!
//!   rustc --edition 2024 trait_in_scope.rs -o /tmp/tis && /tmp/tis

mod loud {
    pub trait Shout {
        fn shout(&self) -> String;

        // No `self` => an associated function. There is nothing to put a dot
        // after, so it is always called on the type.
        fn motto() -> &'static str {
            "LOUD BY DEFAULT"
        }
    }

    pub struct Dog;
    pub struct Fox;

    impl Shout for Dog {
        fn shout(&self) -> String {
            "WOOF".to_string()
        }
    }

    impl Shout for Fox {
        fn shout(&self) -> String {
            "RING-DING-DING".to_string()
        }
        fn motto() -> &'static str {
            "LOUD ONLY WHEN ASKED"
        }
    }

    // An INHERENT method on Fox with the same name as the trait's. Perfectly
    // legal: they are two different methods that happen to share a spelling.
    impl Fox {
        pub fn shout(&self) -> String {
            "(the fox's own shout)".to_string()
        }
    }
}

// Importing the TYPES is not enough. Drop `Shout` from this line and
// `dog.shout()` stops compiling with E0599 — "items from traits can only be
// used if the trait is in scope" — even though the impl is right there above.
use loud::{Dog, Fox, Shout};

fn main() {
    let dog = Dog;

    println!("1. The method call, now that the trait is in scope");
    println!("   dog.shout() = {}", dog.shout());

    println!();
    println!("2. The same call, spelled out three ways");
    println!("   Shout::shout(&dog)          = {}", Shout::shout(&dog));
    println!("   <Dog as Shout>::shout(&dog) = {}", <Dog as Shout>::shout(&dog));
    println!("   Dog::shout(&dog)            = {}", Dog::shout(&dog));

    println!();
    println!("3. An associated function has no receiver, so the type carries it");
    println!("   <Dog as Shout>::motto() = {}", <Dog as Shout>::motto());
    println!("   <Fox as Shout>::motto() = {}", <Fox as Shout>::motto());

    println!();
    println!("4. When an inherent method shares the name, the dot picks INHERENT");
    let fox = Fox;
    println!("   fox.shout()                 = {}", fox.shout());
    println!("   Fox::shout(&fox)            = {}", Fox::shout(&fox));
    println!("   <Fox as Shout>::shout(&fox) = {}", <Fox as Shout>::shout(&fox));
    println!("   ^ only the last spelling can reach the trait's method at all.");
}
