//! A `bool` parameter is two states with no names. Naming them with an enum
//! costs nothing in memory and lets the compiler reject the backwards call.
#![allow(dead_code)]

use std::mem::size_of;

// -- the signature everybody writes first -----------------------------------

fn print_page_bools(both_sides: bool, color: bool) -> String {
    format!(
        "{} / {}",
        if both_sides { "both sides" } else { "single side" },
        if color { "colour" } else { "black and white" },
    )
}

// -- the same API with the states named -------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
enum Sides {
    Both,
    Single,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Output {
    BlackAndWhite,
    Color,
}

fn print_page(sides: Sides, output: Output) -> String {
    format!(
        "{} / {}",
        match sides {
            Sides::Both => "both sides",
            Sides::Single => "single side",
        },
        match output {
            Output::BlackAndWhite => "black and white",
            Output::Color => "colour",
        },
    )
}

// -- the bool INSIDE a struct, which is the same mistake --------------------

/// A three-value stand-in for an RGB colour, small enough to enumerate.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Shade {
    Black,
    Red,
    White,
}

const SHADES: [Shade; 3] = [Shade::Black, Shade::Red, Shade::White];

/// The invariant lives in a comment, so the type permits states that break it.
#[derive(Debug, Clone, Copy)]
struct PropsBool {
    monochrome: bool,
    // `fg` must be Black if `monochrome` is true.
    fg: Shade,
}

impl PropsBool {
    /// What the comment says. Nothing calls this unless somebody remembers to.
    fn obeys_the_comment(&self) -> bool {
        !self.monochrome || self.fg == Shade::Black
    }
}

/// The invariant is the type: a monochrome display has no foreground colour
/// to disagree with, because there is nowhere to put one.
#[derive(Debug, Clone, Copy)]
enum Color {
    Monochrome,
    Foreground(Shade),
}

#[derive(Debug, Clone, Copy)]
struct PropsEnum {
    color: Color,
}

fn main() {
    // -- the call the compiler cannot check ---------------------------------
    println!("two bools, called as intended:");
    println!("  print_page(true, false)            -> {}", print_page_bools(true, false));
    println!("two bools, arguments swapped -- compiles, prints the wrong job:");
    println!("  print_page(false, true)            -> {}", print_page_bools(false, true));

    println!("\ntwo enums, called as intended:");
    println!(
        "  print_page(Sides::Both, Output::BlackAndWhite)\n                                     -> {}",
        print_page(Sides::Both, Output::BlackAndWhite)
    );
    println!("two enums, arguments swapped -- does not compile:");
    println!("  print_page(Output::BlackAndWhite, Sides::Single)");
    println!("                                     -> error[E0308], with a fix-it");
    // print_page(Output::BlackAndWhite, Sides::Single);   // <- E0308: swap these arguments

    // -- the states each shape of the struct permits ------------------------
    let by_bool: Vec<PropsBool> = [true, false]
        .into_iter()
        .flat_map(|monochrome| SHADES.iter().map(move |&fg| PropsBool { monochrome, fg }))
        .collect();
    let nonsense = by_bool.iter().filter(|p| !p.obeys_the_comment()).count();

    let by_enum: Vec<PropsEnum> = std::iter::once(Color::Monochrome)
        .chain(SHADES.iter().map(|&s| Color::Foreground(s)))
        .map(|color| PropsEnum { color })
        .collect();

    println!("\nstates the type permits, with 3 shades:");
    println!("  bool + shade   (AND)  {} states, {} of them break the comment", by_bool.len(), nonsense);
    println!("  Color          (OR )  {} states, every one meaningful", by_enum.len());
    for p in &by_bool {
        let verdict = if p.obeys_the_comment() { "ok" } else { "contradicts the comment" };
        println!("    monochrome: {:<5} fg: {:<5}  {}", p.monochrome, format!("{:?}", p.fg), verdict);
    }

    // The real type has 3 x u8 = 2^24 colours; the arithmetic is the same.
    let colours: u64 = 1 << 24;
    println!("\nand with a real RgbColor ({colours} colours):");
    println!("  bool + RgbColor  {} states, {} of them break the comment", 2 * colours, colours - 1);
    println!("  Color            {} states", 1 + colours);

    // -- what the naming costs ----------------------------------------------
    println!("\nsizes:");
    println!("  bool        {}", size_of::<bool>());
    println!("  Sides       {}", size_of::<Sides>());
    println!("  Output      {}", size_of::<Output>());
    println!("  PropsBool   {}", size_of::<PropsBool>());
    println!("  PropsEnum   {}", size_of::<PropsEnum>());
}
