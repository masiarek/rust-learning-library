// The claims around Rust in Action, listing 2.2 (ch2/ch2-first-steps.rs),
// checked one at a time. The listing itself lives in the book's repository.
use std::any::type_name_of_val;

fn main() {
    // 1. What the listing computes: add(add(10, 20), add(30, 30)).
    let e = add(add(10, 20), add(30, 30));
    println!("1. the listing's e = {e}"); // 1. the listing's e = 90

    // 2. "Types can be inferred": from how the value is used, not from the literal.
    let a = 10;
    let _ = add(a, 0);
    let wide = 10;
    let _ = add_wide(wide, 0);
    let alone = 10;
    println!(
        "2. a is {}, wide is {}, alone is {}",
        type_name_of_val(&a),
        type_name_of_val(&wide),
        type_name_of_val(&alone),
    );

    // 3. An annotation, a suffix, and a suffix after an underscore: one type.
    let b: i32 = 20;
    let c = 30i32;
    let d = 30_i32;
    println!(
        "3. b is {}, c is {}, d is {}, c == d: {}",
        type_name_of_val(&b),
        type_name_of_val(&c),
        type_name_of_val(&d),
        c == d,
    );

    // 4. Immutable forbids a second assignment, not a late first one.
    let size;
    if e > 50 {
        size = "big";
    } else {
        size = "small";
    }
    println!("4. size was assigned once, after its let, with no mut: {size}");

    // 5. A macro call is expanded into code, and that code has a value.
    let unit: () = println!("5. println! is an expression...");
    let text: String = format!("{e}");
    println!("   ...whose value is {unit:?}; format! gives a String: {text:?}");

    // 6. Item order does not matter, and a fn can be declared inside another.
    fn double(x: i32) -> i32 {
        x * 2
    }
    println!("6. add is below main, double is inside it: {}", double(add(1, 2)));

    // 7. The tail expression is the value; `return` is for leaving early.
    println!("7. sign(-5) = {}, sign(5) = {}", sign(-5), sign(5));

    // 8. Single quotes make a char: one Unicode scalar value, always 4 bytes.
    let letter = 'é';
    let word = "é";
    println!(
        "8. 'é' is a {} of {} bytes; \"é\" is a {} holding {} bytes",
        type_name_of_val(&letter),
        size_of_val(&letter),
        type_name_of_val(&word),
        word.len(),
    );
}

fn add(i: i32, j: i32) -> i32 {
    i + j
}

fn add_wide(i: i64, j: i64) -> i64 {
    i + j
}

fn sign(n: i32) -> &'static str {
    if n < 0 {
        return "negative";
    }
    "not negative"
}
