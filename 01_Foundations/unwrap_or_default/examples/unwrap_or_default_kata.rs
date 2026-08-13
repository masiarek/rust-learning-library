//! Kata solution: the type's zero is not always your domain's zero.
//!
//!   rustc --edition 2024 unwrap_or_default_kata.rs -o /tmp/uodk && /tmp/uodk

/// Derived `Default` fills every field with its type's zero. Read the result as
/// a ballot and it says something false: a voter who scored everyone 0.
#[derive(Debug, Default, Clone, PartialEq)]
struct Ballot {
    ada: u8,
    ben: u8,
    cara: u8,
}

/// The same data with the domain's answer written down instead.
impl Ballot {
    /// A ballot that was handed in blank still exists — it is not scores of 0.
    fn blank() -> Option<Ballot> {
        None
    }
}

fn main() {
    let handed_in: Option<Ballot> = Some(Ballot { ada: 5, ben: 2, cara: 0 });
    let not_handed_in: Option<Ballot> = Ballot::blank();

    println!("unwrap_or_default fills in the TYPE's zero:");
    println!("  handed in     -> {:?}", handed_in.clone().unwrap_or_default());
    println!("  not handed in -> {:?}", not_handed_in.clone().unwrap_or_default());
    println!("      The second line is a ballot nobody cast, and it is now");
    println!("      indistinguishable from a voter who scored everyone 0:");
    println!("      equal? {}", not_handed_in.clone().unwrap_or_default() == Ballot { ada: 0, ben: 0, cara: 0 });

    println!("\nSo the count has to ask before it defaults:");
    let cast = [handed_in.clone(), not_handed_in.clone(), Some(Ballot { ada: 0, ben: 0, cara: 0 })];
    let turnout = cast.iter().filter(|b| b.is_some()).count();
    let zeroed = cast.iter().flatten().filter(|b| **b == Ballot::default()).count();
    println!("  ballots returned: {turnout} of {}", cast.len());
    println!("  of those, all-zero ballots: {zeroed}");

    println!("\nEmpty is not absent — the same trap one level up:");
    let no_list: Option<Vec<u8>> = None;
    let empty_list: Option<Vec<u8>> = Some(vec![]);
    println!("  None.unwrap_or_default()      -> {:?}", no_list.unwrap_or_default());
    println!("  Some(vec![]).unwrap_or_default() -> {:?}", empty_list.unwrap_or_default());
    println!("      Same value out, two different facts in: 'no list was given'");
    println!("      and 'a list was given and it was empty'. Default erases which.");

    println!("\nWhere it is exactly right — a counter with no entry yet:");
    let seen: Option<u32> = None;
    println!("  seen.unwrap_or_default() + 1 -> {}", seen.unwrap_or_default() + 1);
}
