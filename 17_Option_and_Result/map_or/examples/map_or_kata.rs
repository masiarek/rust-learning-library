//! Kata solution: transform when present, fall back when not — in one call.
//!
//!   rustc --edition 2024 map_or_kata.rs -o /tmp/mork && /tmp/mork

#[derive(Debug)]
struct Winner {
    name: &'static str,
    margin: u32,
}

fn label(w: &Option<Winner>) -> String {
    // The default is written FIRST and used LAST. Read it as:
    // "…or this, when there is nothing to transform".
    w.as_ref().map_or("no result yet".to_string(), |w| {
        format!("{} by {} votes", w.name, w.margin)
    })
}

fn main() {
    let counted = Some(Winner { name: "Ada", margin: 17 });
    let pending: Option<Winner> = None;

    println!("The type changes, which is the reason to reach for map_or:");
    println!("  {}", label(&counted));
    println!("  {}", label(&pending));
    println!("      Option<Winner> in, String out. unwrap_or cannot do that —");
    println!("      its fallback must already be the same type as the value.");

    println!("\nEager and lazy, the same rule as the unwrap family:");
    let seats: Option<u32> = Some(3);
    println!("  map_or       -> {}", seats.map_or(expensive_default(), |s| s * 2));
    println!("  map_or_else  -> {}", seats.map_or_else(expensive_default_lazy, |s| s * 2));

    println!("\nWhere a match still wins — when both arms have something to say:");
    match &counted {
        Some(w) if w.margin < 10 => println!("  {} won, but call it a recount", w.name),
        Some(w) => println!("  {} won comfortably ({} votes)", w.name, w.margin),
        None => println!("  still counting"),
    }
    println!("      Three outcomes and a guard. Squeezing that into map_or would");
    println!("      cost more than the line it saves.");
}

fn expensive_default() -> u32 {
    println!("      …building the default (ran even though seats was Some)");
    0
}

fn expensive_default_lazy() -> u32 {
    println!("      …building the default (you will not see this line)");
    0
}
