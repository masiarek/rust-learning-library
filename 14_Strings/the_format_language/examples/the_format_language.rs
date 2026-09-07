//! `{:>8.3}` is a small language parsed by `std::fmt`, not Rust syntax — and
//! the spec is a REQUEST: every impl decides for itself whether to honour it.
//!
//!   rustc --edition 2024 the_format_language.rs -o /tmp/tfl && /tmp/tfl

use std::fmt::Write;

fn main() {
    println!("1. fill, align, width — and the defaults that differ by type");
    println!("   [{:*^11}]  fill '*', centre, width 11", "mid");
    println!("   [{:-<11}]  fill '-', left", "left");
    println!("   [{:.>11}]  fill '.', right", "right");
    println!("   [{:8}]  a number with no align defaults RIGHT", 42);
    println!("   [{:8}]  a string with no align defaults LEFT", "ab");

    println!("\n2. sign, #, and the zero that is not a fill character");
    println!("   {{:+}}       {:+} and {:+}", 42, -42);
    println!("   {{:-}}       {:-}   <- accepted, and does nothing at all", 42);
    println!("   {{:#x}}      {:#x}     {{:#b}} {:#b}     {{:#o}} {:#o}", 255, 5, 8);
    println!("   {{:#010x}}   {:#010x}   <- the 0x counts toward the 10", 255);
    println!("   {{:010}}     {:010}   <- the sign goes BEFORE the zeros", -42);
    println!("   {{:08.3}}    {:08.3}", 3.14159);

    println!("\n3. precision means two different things");
    println!("   on a float, digits after the point:  {{:.3}} of 3.14159 = {:.3}", 3.14159);
    println!("   on a string, a MAXIMUM LENGTH:       {{:.3}} of \"hello\" = {:.3}", "hello");
    println!("   and they compose:                    {{:>8.3}}          = [{:>8.3}]", "hello");
    println!("   A spec that truncates text is easy to write by accident when you");
    println!("   meant to pad it. `{{:8}}` pads; `{{:.8}}` cuts.");

    println!("\n4. THE SURPRISE: the spec is state, and an impl has to ask for it");
    println!("   Display str    [{:>8}]   <- honours the width", "ab");
    println!("   Debug   str    [{:>8?}]   <- ignores it entirely", "ab");
    println!("   Debug   char   [{:>8?}]   <- ignores it too", 'a');
    println!("   Debug   i32    [{:>8?}]   <- honours it", 42);
    println!("   Debug   bool   [{:>8?}]   <- honours it", true);
    println!("   Debug   Vec    [{:>8?}]", vec![1, 2]);
    println!("   Debug   Option [{:>8?}]", Some(1));
    println!("   The last two are the ones to look at twice. A container does not");
    println!("   pad ITSELF — it hands your formatter down to each element, so the");
    println!("   width lands inside the brackets, once per item.");

    println!("\n5. The type characters");
    println!("   {{}}      Display     {}", 1.0);
    println!("   {{:?}}    Debug       {:?}   <- a float keeps its .0 here", 1.0);
    println!("   {{:x?}}   Debug, hex  {:x?}", vec![255u8, 1]);
    println!("   {{:02x?}} width rides down to the elements  {:02x?}", vec![255u8, 1]);
    println!("   {{:e}}    {:e}    <- no '+', no padded exponent, unlike C and Python", 1234.5);
    println!("   {{:#?}}   pretty Debug, one field per line:");
    println!("{:#?}", vec![(1, "a")]);

    println!("\n6. Rounding is half-to-even, and the exceptions are not exceptions");
    print!("   {{:.0}} of 0.5 1.5 2.5 3.5 = ");
    for v in [0.5f64, 1.5, 2.5, 3.5] {
        print!("{:.0} ", v);
    }
    println!("  <- ties go to the EVEN digit");
    println!("   {{:.2}} of 1.005 = {:.2}, which looks wrong until you look at the value:", 1.005f64);
    println!("      1.005 is stored as {:.20}", 1.005f64);
    println!("      2.675 is stored as {:.20}", 2.675f64);
    println!("   Neither is a tie, so neither is a rounding decision. `fmt` rounded");
    println!("   the number it was given correctly; the literal was never that number.");
    println!("      2.5 really is stored as {:.20}", 2.5f64);

    println!("\n7. Width and precision computed at run time");
    let width = 9;
    let precision = 2;
    println!("   {{:>width$.precision$}} -> [{:>width$.precision$}]", 3.14159);
    println!("   {{:>1$}} with a positional -> [{:>1$}]", "ab", 6);
    println!("   {{:.*}} takes the precision first -> [{:.*}]", 3, 3.14159);
    let rows = [("alpha", 3.5), ("b", 12.25), ("gamma_long", 0.125)];
    let widest = rows.iter().map(|(name, _)| name.len()).max().unwrap_or(0);
    println!("   A column sized from the data itself (widest = {widest}):");
    for (name, value) in rows {
        println!("      {name:<widest$} | {value:>8.2}");
    }

    println!("\n8. write! into one buffer, format! into a new one");
    let mut buffer = String::with_capacity(64);
    let before = buffer.capacity();
    for (i, (name, _)) in rows.iter().enumerate() {
        write!(buffer, "{i}:{name};").unwrap();
    }
    println!("   write!:  capacity {before} -> {} (unchanged: {})", buffer.capacity(), buffer.capacity() == before);
    println!("            {buffer}");
    let mut same = String::with_capacity(64);
    for (i, (name, _)) in rows.iter().enumerate() {
        same.push_str(&format!("{i}:{name};"));
    }
    println!("   format!: identical text ({}) — and one temporary String per row,", same == buffer);
    println!("            allocated and dropped, which is why write! belongs in a loop.");
    println!("   Both need `use std::fmt::Write`; writing to a String cannot fail,");
    println!("   so the fmt::Result it hands back is always {:?}.", write!(String::new(), "x"));
}
