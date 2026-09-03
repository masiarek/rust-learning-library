//! Kata solution — the average that came out as a three-way tie.
//!
//! Three products, three different average scores, and an `i128` wide enough
//! to hold every number in the dataset with room to spare. The averages still
//! come out equal, because the one operation `i128` is not exact about is the
//! one the word "average" is made of.
//!
//! The fix is not a wider type. There is no wider primitive, and it would not
//! help if there were: the problem is division, not range. The fix is to stop
//! dividing — and then to notice what not-dividing costs.

/// `(name, total score awarded, raters that scored them)`.
const FIELD: [(&str, i128, i128); 3] = [
    ("Alma", 1_000_000, 3_000),
    ("Bruno", 1_000_500, 3_001),
    ("Cara", 999_000, 3_000),
];

fn gcd(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a.abs()
}

/// Compare `a/b` against `c/d` without ever dividing, and refuse to wrap.
///
/// `a/b > c/d` exactly when `a*d > c*b`, for positive denominators. The
/// products are what overflow, so divide out the common factors of each
/// *pair* first — the comparison is unchanged and the operands get smaller.
fn compare_exact(a: i128, b: i128, c: i128, d: i128) -> Option<std::cmp::Ordering> {
    let g1 = gcd(a, c);
    let g2 = gcd(b, d);
    let left = (a / g1).checked_mul(d / g2)?;
    let right = (c / g1).checked_mul(b / g2)?;
    Some(left.cmp(&right))
}

fn main() {
    // ------------------------------------------------------------ 1
    println!("1. The average, by division — the mistake worth making first");
    for (name, total, raters) in FIELD {
        println!("     {name:<6} {total:>9} / {raters:<5} = {}", total / raters);
    }
    println!("     Three identical averages, and a three-way tie for the top spot.");

    // ------------------------------------------------------------ 2
    println!("\n2. What the truncation actually did");
    println!("     The real averages, to six places, computed only to show them:");
    for (name, total, raters) in FIELD {
        println!(
            "       {name:<6} {:.6}",
            total as f64 / raters as f64
        );
    }
    println!("     They were never equal. Integer division floors, and flooring is");
    println!("     monotone — so it can never REVERSE two products, only collapse");
    println!("     them onto the same number. That sounds like the harmless failure");
    println!("     until you remember what a tie does: it hands the top spot to the");
    println!("     tiebreak ladder, and at the bottom of that ladder is a lot.");
    println!("     A manufactured tie is not a rounding error. It is a coin flip");
    println!("     between products who were not actually tied.");

    // ------------------------------------------------------------ 3
    println!("\n3. The fix: rank them without dividing at all");
    let mut order: Vec<(&str, i128, i128)> = FIELD.to_vec();
    order.sort_by(|&(_, at, ab), &(_, bt, bb)| {
        compare_exact(bt, bb, at, ab).expect("these products fit easily")
    });
    for (i, (name, total, raters)) in order.iter().enumerate() {
        println!("     {}. {name:<6} ({total}/{raters})", i + 1);
    }
    let (a, at, ab) = FIELD[0];
    let (b, bt, bb) = FIELD[1];
    println!("     The comparison {a} vs {b} is one multiplication each side:");
    println!("       {at} × {bb} = {}", at * bb);
    println!("       {bt} × {ab} = {}", bt * ab);
    println!("     No division, so nothing truncates, so no tie is invented.");

    // ------------------------------------------------------------ 4
    println!("\n4. The bill: cross-multiplying needs headroom dividing did not");
    // A national count whose totals are already scaled by a large denominator —
    // which is exactly what you are holding after the scaled-integer trick.
    let (big_a, big_b): (i128, i128) = (90_000_000_000_000_000_000_000_000_000_000_000_000, 6_000_000_000_000);
    let (big_c, big_d): (i128, i128) = (80_000_000_000_000_000_000_000_000_000_000_000_000, 5_000_000_000_000);
    println!("     Alma  {big_a} / {big_b}");
    println!("     Bruno {big_c} / {big_d}");
    match big_a.checked_mul(big_d) {
        Some(v) => println!("     naive cross-multiply -> {v}"),
        None => {
            println!(
                "     naive cross-multiply -> None: that product needs about {} bits,",
                big_a.ilog2() + big_d.ilog2()
            );
            println!("                                   and an i128 has 127 to spend");
        }
    }
    println!("     Both operands fit i128 comfortably. Their product does not, and");
    println!("     in a release build a bare `*` would have wrapped to a negative");
    println!("     number and ranked the field by it.");

    println!("\n     Reducing each pair by its gcd first, the same comparison is tiny:");
    let g1 = gcd(big_a, big_c);
    let g2 = gcd(big_b, big_d);
    println!("       gcd of the totals   {g1}");
    println!("       gcd of the counts   {g2}");
    println!(
        "       {} × {} = {}   vs   {} × {} = {}",
        big_a / g1,
        big_d / g2,
        (big_a / g1) * (big_d / g2),
        big_c / g1,
        big_b / g2,
        (big_c / g1) * (big_b / g2),
    );
    println!(
        "     -> {}",
        match compare_exact(big_a, big_b, big_c, big_d) {
            Some(std::cmp::Ordering::Greater) => "Alma leads",
            Some(std::cmp::Ordering::Less) => "Bruno leads",
            Some(std::cmp::Ordering::Equal) => "a genuine tie",
            None => "still overflows",
        }
    );
    println!("     Two 38-digit numbers compared by multiplying 9 by 5 and 8 by 6.");
    println!("     That is the same move the lesson called reduce-before-multiply,");
    println!("     and it is the reason a rational library calls gcd as often as it");
    println!("     does: the reduction is not tidiness, it is the headroom.");
}
