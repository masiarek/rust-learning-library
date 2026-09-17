//! Kata solution: a receipt with no placeholder variables.
//!
//! Every decision on the receipt is an `if` that IS the value, so no line reads
//! `let mut label;` followed by an assignment in each branch.
//!
//!   rustc --edition 2024 if_expressions_kata.rs -o /tmp/ifk && /tmp/ifk

/// One `if`, two formats. Both branches are a `String`, so the `if` is one.
fn price(cents: u32) -> String {
    if cents >= 100 {
        format!("${}.{:02}", cents / 100, cents % 100)
    } else {
        format!("{cents}c")
    }
}

fn main() {
    let cart = [
        ("coffee", 1_250u32, 2u32),
        ("napkins", 99, 0),
        ("mug", 899, 1),
        ("stickers", 45, 3),
        ("grinder", 2_499, 1),
    ];

    println!("1. One line per row; a quantity of 0 leaves from inside the if");
    let mut subtotal = 0;
    for (item, cents, qty) in cart {
        let line = if qty == 0 {
            println!("   {item:<9} skipped: quantity 0");
            continue; // type `!`: fits where the else branch has a u32
        } else {
            cents * qty
        };
        let marker = if line >= 2_000 { "   <- $20 or more" } else { "" };
        subtotal += line;
        println!("   {item:<9} {qty} x {:>6} = {:>7}{marker}", price(cents), price(line));
    }

    println!();
    println!("2. Shipping is free from $50, and each total is one more if away");
    for (label, goods) in [("this cart", subtotal), ("mug + stickers", 899 + 135)] {
        let shipping = if goods >= 5_000 { 0 } else { 599 };
        println!(
            "   {label:<15} goods {:>7}   shipping {:>5}   total {:>7}",
            price(goods),
            if shipping == 0 { "free".to_string() } else { price(shipping) },
            price(goods + shipping)
        );
    }

    println!();
    println!("3. The three predictions");
    println!("   (a) `;` after the first format! in price. rustc says `mismatched");
    println!("       types`, expected `String`, found `()`, and the ^ is under the");
    println!("       branch you changed: the return type set the expectation.");
    println!("   (b) The same `;` after the marker string. Now it says `if` and");
    println!("       `else` have incompatible types, expected `()`, found `&str`,");
    println!("       and the ^ is under \"\", the branch you did NOT touch: nothing");
    println!("       outside the if expected a type, so the first branch set it.");
    println!("   (c) `if qty {{`: expected `bool`, found `u32`. No truthiness; and");
    println!("       `u32` rather than `integer`, because the literal said 2u32.");
}
