//! Kata solution: the type's zero is not always your domain's zero.
//!
//!   rustc --edition 2024 unwrap_or_default_kata.rs -o /tmp/uodk && /tmp/uodk

/// Derived `Default` fills every field with its type's zero. Read the result as
/// a survey response and it says something false: someone who rated everything 0.
#[derive(Debug, Default, Clone, PartialEq)]
struct Response {
    speed: u8,
    price: u8,
    support: u8,
}

/// The same data with the domain's answer written down instead.
impl Response {
    /// A form that was never returned still exists as a fact — it is not ratings of 0.
    fn not_returned() -> Option<Response> {
        None
    }
}

fn main() {
    let returned: Option<Response> = Some(Response { speed: 5, price: 2, support: 0 });
    let never_returned: Option<Response> = Response::not_returned();

    println!("unwrap_or_default fills in the TYPE's zero:");
    println!("  returned       -> {:?}", returned.clone().unwrap_or_default());
    println!("  never returned -> {:?}", never_returned.clone().unwrap_or_default());
    println!("      The second line is a form nobody filled in, and it is now");
    println!("      indistinguishable from someone who rated everything 0:");
    println!("      equal? {}", never_returned.clone().unwrap_or_default() == Response { speed: 0, price: 0, support: 0 });

    println!("\nSo the summary has to ask before it defaults:");
    let sent = [returned.clone(), never_returned.clone(), Some(Response { speed: 0, price: 0, support: 0 })];
    let response_rate = sent.iter().filter(|r| r.is_some()).count();
    let zeroed = sent.iter().flatten().filter(|r| **r == Response::default()).count();
    println!("  forms returned: {response_rate} of {}", sent.len());
    println!("  of those, all-zero responses: {zeroed}");

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
