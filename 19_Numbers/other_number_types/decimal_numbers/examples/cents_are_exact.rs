// Three items at $0.10: as a float, and as whole cents.
fn main() {
    let total = 0.10_f64 * 3.0;
    println!("{total}"); // 0.30000000000000004
    println!("{}", total == 0.30); // false

    let total_cents: i64 = 10 * 3;
    println!("{}.{:02}", total_cents / 100, total_cents % 100); // 0.30
    println!("{}", total_cents == 30); // true

    // What whole cents cannot do: split a dollar three ways.
    println!("{} cents each, {} left over", 100 / 3, 100 % 3); // 33 cents each, 1 left over
}
