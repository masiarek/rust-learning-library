//! Kata solution: predict which half of the base survives a `..base`.
//!
//!   rustc --edition 2024 struct_update_kata.rs -o /tmp/suk && /tmp/suk

#[derive(Debug, Default)]
struct Config {
    retries: u32,        // Copy
    verbose: bool,       // Copy
    host: String,        // NOT Copy
    notes: String,       // NOT Copy
}

fn main() {
    let original = Config {
        retries: 12,
        verbose: false,
        host: "alpha".to_string(),
        notes: "staging box".to_string(),
    };

    // Only `notes` is named, so `..original` supplies the other THREE.
    let amended = Config { notes: "promoted".to_string(), ..original };

    println!("Predict, field by field, what is still readable on `original`:\n");
    println!("  retries   Copy      -> copied   -> {} still readable", original.retries);
    println!("  verbose   Copy      -> copied   -> {} still readable", original.verbose);
    println!("  host      NOT Copy  -> MOVED    -> reading it is E0382");
    println!("  notes     NOT Copy  -> not taken (we named it) -> {:?} still readable", original.notes);
    println!("\n  amended = {amended:?}");

    println!("\nThe rule in one line:");
    println!("  `..base` moves exactly the fields you did NOT name,");
    println!("  and only the non-Copy ones among those actually go dead.");

    println!("\nThree ways to keep the base whole:");
    let base = Config { retries: 3, verbose: true,
                        host: "beta".to_string(), notes: "n/a".to_string() };

    // 1. Name every non-Copy field yourself.
    let a = Config { host: "gamma".to_string(), notes: "n/a".to_string(), ..base };
    println!("  1. name every non-Copy field   -> base alive: {:?}", base.host);

    // 2. Clone the base into the update position.
    let b = Config { retries: 99, ..clone_config(&base) };
    println!("  2. clone into the base slot    -> base alive: {:?}", base.host);

    // 3. Use a temporary nobody holds.
    let c = Config { host: "delta".to_string(), ..Default::default() };
    println!("  3. ..Default::default()        -> nothing to strand");

    println!("\n  {a:?}\n  {b:?}\n  {c:?}");

    println!("\nAnd the syntax trap, which is its own error:");
    println!("  Config {{ retries: 1, ..base, }}");
    println!("    error: cannot use a comma after the base struct");
    println!("    note: the base struct must always be the last field");
}

// Config does not derive Clone here on purpose — this spells out that the
// "clone" in option 2 is ordinary code, not something `..` does for you.
fn clone_config(c: &Config) -> Config {
    Config {
        retries: c.retries,
        verbose: c.verbose,
        host: c.host.clone(),
        notes: c.notes.clone(),
    }
}
