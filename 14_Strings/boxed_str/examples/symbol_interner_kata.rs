//! Kata solution: an interner that hands out numbers — because one that hands
//! out `&str` stops compiling at its second call.
//!
//!   rustc --edition 2024 symbol_interner_kata.rs -o /tmp/sik && /tmp/sik

use std::collections::HashMap;
use std::mem::size_of;
use std::rc::Rc;

/// The handle: four bytes, `Copy`, and compared as an integer.
#[derive(Clone, Copy, PartialEq, Eq)]
struct Symbol(u32);

/// Each distinct text is stored once, as an `Rc<str>` that both tables share:
/// the map turns text into a symbol, the Vec turns a symbol back into text.
struct Interner {
    ids: HashMap<Rc<str>, Symbol>,
    names: Vec<Rc<str>>,
}

impl Interner {
    fn new() -> Self {
        Interner { ids: HashMap::new(), names: Vec::new() }
    }

    /// Needs `&mut self` to insert — but it returns a `Symbol`, which borrows
    /// nothing, so the mutable borrow ends when the call does.
    fn intern(&mut self, s: &str) -> Symbol {
        if let Some(&id) = self.ids.get(s) {
            return id;
        }
        let id = Symbol(u32::try_from(self.names.len()).expect("fewer than 2^32 strings"));
        let text: Rc<str> = Rc::from(s);
        self.names.push(Rc::clone(&text));
        self.ids.insert(text, id);
        id
    }

    /// Only `&self`, so any number of resolved strings can be held at once.
    fn resolve(&self, id: Symbol) -> &str {
        &self.names[id.0 as usize]
    }
}

fn main() {
    println!("1. Intern a column: eight values, three distinct");
    let column = ["Ada", "Ben", "Ada", "Cara", "Ben", "Ada", "Cara", "Ada"];
    let mut interner = Interner::new();
    let symbols: Vec<Symbol> = column.iter().map(|s| interner.intern(s)).collect();
    println!("   symbols               {:?}", symbols.iter().map(|s| s.0).collect::<Vec<_>>());
    println!("   texts stored          {}", interner.names.len());
    println!("   owners of each text   {} (the map's key and the Vec's entry)", Rc::strong_count(&interner.names[0]));

    println!();
    println!("2. Handles compare as integers, and are small");
    println!("   symbols[0] == symbols[2], both Ada   {}", symbols[0] == symbols[2]);
    println!("   a Symbol                             {} bytes", size_of::<Symbol>());
    println!("   a &str                               {} machine words", size_of::<&str>() / size_of::<usize>());

    println!();
    println!("3. Resolve when the text is needed, as many at once as you like");
    let (a, b, c) = (interner.resolve(symbols[0]), interner.resolve(symbols[1]), interner.resolve(symbols[3]));
    println!("   {a}, {b}, {c}");
    println!("   resolve takes &self, so three shared borrows can be alive together.");

    println!();
    println!("4. Why intern does not hand out &str");
    println!("   fn intern(&mut self, s: &str) -> &str ties the &str it returns to the");
    println!("   MUTABLE borrow of the interner. Keep the first result and call intern");
    println!("   again, and rustc stops with E0499: two mutable borrows at once.");
    println!("   A Symbol borrows nothing, so that question never comes up.");
}
