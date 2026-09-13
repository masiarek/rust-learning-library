//! When the `impl` does not match the trait: the trait owns the signature, the
//! impl owns the body. What an impl may change without an error, and where the
//! signature the trait refused can legally go.
//!
//!   rustc --edition 2024 matching_the_trait.rs -o /tmp/mtt && /tmp/mtt

use std::fmt::Display;

trait Animal {
    fn name(&self) -> String; // required: no body
    fn speak(&self, times: u32) {
        // provided: a default body
        for _ in 0..times {
            println!("   {} makes a sound", self.name());
        }
    }
    fn tag<T: Display>(&self, label: T) -> String {
        format!("{label}: {}", self.name())
    }
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn name(&self) -> String {
        "Rex".to_string()
    }
    // Every TYPE is the trait's. What changed is what the trait cannot see:
    // the parameter's name, and `mut` on its binding.
    fn speak(&self, mut n: u32) {
        while n > 0 {
            println!("   {}: woof", self.name());
            n -= 1;
        }
    }
    // A LOOSER bound than the trait's is allowed: this body never formats
    // `label`, so it has no use for `T: Display`. A stricter one is E0276.
    fn tag<T>(&self, _label: T) -> String {
        format!("dog {}", self.name())
    }
}

impl Animal for Cat {
    fn name(&self) -> String {
        "Tom".to_string()
    }
    // `speak` and `tag` are not written, so the trait's default bodies run.
}

// The signature the trait refused (`-> u32`), written where it is legal: an
// inherent method. It compiles with no warning, and it wins the dot call.
impl Cat {
    fn speak(&self, times: u32) -> u32 {
        println!("   {}: meow x{times}", self.name());
        times
    }
}

// Generic code can only see the trait, so this always reaches the trait's `speak`.
fn chorus(animal: &impl Animal) {
    animal.speak(1);
}

fn main() {
    println!("1. The required method, written by each type");
    println!("   Dog.name() = {}", Dog.name());
    println!("   Cat.name() = {}", Cat.name());

    println!();
    println!("2. Dog replaced the provided method: the trait's types, its own parameter name");
    Dog.speak(2);

    println!();
    println!("3. Dog's tag drops the trait's Display bound; Cat's is the default");
    println!("   Dog.tag(7) = {}", Dog.tag(7));
    println!("   Cat.tag(7) = {}", Cat.tag(7));

    println!();
    println!("4. Cat's inherent speak returns a value, and the dot call picks it");
    let n = Cat.speak(3);
    println!("   Cat.speak(3) returned {n}");

    println!();
    println!("5. Anything that reaches Cat through the trait gets the default body");
    println!("   chorus(&Cat):");
    chorus(&Cat);
    println!("   Animal::speak(&Cat, 1):");
    Animal::speak(&Cat, 1);
}
