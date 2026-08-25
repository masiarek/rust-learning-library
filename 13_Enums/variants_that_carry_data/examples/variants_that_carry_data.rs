//! A struct multiplies its possibilities; an enum adds them. Counted, then measured.
#![allow(dead_code)]

use std::mem::{discriminant, size_of};

#[derive(Debug, Clone, Copy)]
enum Coin { Penny, Nickel, Dime }        // 3 values

#[derive(Debug, Clone, Copy)]
enum Note { Five, Ten }                  // 2 values

/// A product type: a coin AND a note. 3 x 2.
#[derive(Debug, Clone, Copy)]
struct Wallet { coin: Coin, note: Note }

/// A sum type: a coin OR a note. 3 + 2.
#[derive(Debug, Clone, Copy)]
enum Payment { InCoin(Coin), InNote(Note) }

const COINS: [Coin; 3] = [Coin::Penny, Coin::Nickel, Coin::Dime];
const NOTES: [Note; 2] = [Note::Five, Note::Ten];

/// The enum from the previous lesson, reused so the sizes mean something.
enum HouseLocation {
    Number(u32),
    Name(String),
    GridRef { easting: u32, northing: u32 },
    Unknown,
}

enum TagCosts { A(u64), B(u64) }   // every bit pattern of a u64 is a valid u64
enum TagFree  { A(Box<u64>), B }   // a Box is never null, so null can mean `B`
enum Never {}                      // no variants: no value can ever exist

fn main() {
    // -- counted ------------------------------------------------------------
    let wallets: Vec<Wallet> = COINS
        .iter()
        .flat_map(|&coin| NOTES.iter().map(move |&note| Wallet { coin, note }))
        .collect();

    let payments: Vec<Payment> = COINS
        .iter()
        .map(|&c| Payment::InCoin(c))
        .chain(NOTES.iter().map(|&n| Payment::InNote(n)))
        .collect();

    println!("Coin has {} values, Note has {}", COINS.len(), NOTES.len());
    println!("Wallet  (struct, AND) has {} values  = 3 x 2", wallets.len());
    println!("Payment (enum,   OR ) has {} values  = 3 + 2", payments.len());
    println!();
    for p in &payments {
        println!("  {p:?}");
    }

    // -- measured -----------------------------------------------------------
    println!("\nsizes on this target ({}-bit pointers):", size_of::<usize>() * 8);
    println!("  String                 {:>2}", size_of::<String>());
    println!("  HouseLocation          {:>2}   <- same as its largest payload", size_of::<HouseLocation>());
    println!("  u64                    {:>2}", size_of::<u64>());
    println!("  TagCosts               {:>2}   <- payload + a tag it must store", size_of::<TagCosts>());
    println!("  Box<u64>               {:>2}", size_of::<Box<u64>>());
    println!("  TagFree                {:>2}   <- tag hidden in the null pointer", size_of::<TagFree>());
    println!("  Option<Box<u64>>       {:>2}   <- the same trick, in the library", size_of::<Option<Box<u64>>>());
    println!("  Never                  {:>2}   <- no variants, so no bytes", size_of::<Never>());

    // -- which variant is this? ---------------------------------------------
    let a = Payment::InCoin(Coin::Penny);
    let b = Payment::InCoin(Coin::Dime);
    let c = Payment::InNote(Note::Ten);
    println!("\nsame variant, different payload: {}", discriminant(&a) == discriminant(&b));
    println!("different variant:               {}", discriminant(&a) == discriminant(&c));
}
