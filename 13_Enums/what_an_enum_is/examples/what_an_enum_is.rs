//! An enum is a type whose value is exactly one of several named variants.
//! Four shapes of variant, one `impl` block, one exhaustive `match`.
#![allow(dead_code)]

/// How a delivery address identifies a house. A given address is exactly one
/// of these — never two, never none.
#[derive(Debug)]
enum HouseLocation {
    Number(u32),                             // tuple variant: one field
    Name(String),                            // tuple variant: owns its data
    GridRef { easting: u32, northing: u32 }, // struct variant: named fields
    Unknown,                                 // unit variant: carries nothing
}

impl HouseLocation {
    /// One arm per variant. Add a fifth variant and this stops compiling,
    /// which is the entire point.
    fn describe(&self) -> String {
        match self {
            HouseLocation::Number(n) => format!("house number {n}"),
            HouseLocation::Name(name) => format!("the house called {name}"),
            HouseLocation::GridRef { easting, northing } => {
                format!("grid reference {easting} {northing}")
            }
            HouseLocation::Unknown => String::from("not known"),
        }
    }

    /// A question that does not need every arm still has to say so, with `_`.
    fn postable(&self) -> bool {
        match self {
            HouseLocation::Unknown => false,
            _ => true,
        }
    }
}

fn main() {
    let addresses = [
        HouseLocation::Number(4),
        HouseLocation::Name(String::from("Dunroamin")),
        HouseLocation::GridRef { easting: 51_820, northing: 17_450 },
        HouseLocation::Unknown,
    ];

    for a in &addresses {
        println!("{:<28}  postable: {}", a.describe(), a.postable());
    }

    // `use` shortens the constructor at the price of the qualifying path.
    // Worth knowing what it costs before you reach for it — see the typo lesson.
    use HouseLocation::*;
    let shorter = Number(4);
    println!("\nNumber(4) once imported: {}", shorter.describe());

    // A variant is not a subtype: both of these have the type `HouseLocation`,
    // so they sit in the same array without any conversion.
    let mixed = [Unknown, Name(String::from("The Old Post Office"))];
    println!("both are HouseLocation: {}", mixed.len());
}
