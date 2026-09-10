//! Kata solution: one contract, two spellings.
//!
//!   rustc --edition 2024 lifetime_safety_in_clang_kata.rs -o /tmp/k && /tmp/k

// The result may point into `a` or into `b`, so both must outlive it.
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

// The result points into `a` only; `b` may die as soon as the call returns.
fn first<'a>(a: &'a str, _b: &str) -> &'a str {
    a
}

fn main() {
    println!("THE CONTRACT, TWO SPELLINGS");
    println!("  Rust: fn longer<'a>(a: &'a str, b: &'a str) -> &'a str");
    println!("  C++:  std::string_view longer(std::string_view a [[clang::lifetimebound]],");
    println!("                                std::string_view b [[clang::lifetimebound]]);");
    println!("  Both say the result may point into a or into b. Leave the lifetimes");
    println!("  out and rustc refuses the function (E0106). Leave the attributes out");
    println!("  and C++ compiles it, and -Wlifetime-safety assumes the result borrows");
    println!("  from nothing -- so it stays silent at every caller.");
    println!();

    let a = String::from("borrow");
    let b = String::from("checker");
    println!("BOTH ARGUMENTS ALIVE");
    println!("  longer(&a, &b) = {}", longer(&a, &b));
    println!();

    println!("ONE ARGUMENT DIES FIRST");
    println!("  let r;");
    println!("  {{ let b = String::from(\"checker\"); r = longer(&a, &b); }}");
    println!("  println!(\"{{r}}\");");
    println!("  -> E0597: `b` does not live long enough. Refused even on a call where");
    println!("     the body would have returned `a`: the checker judges a call by the");
    println!("     signature and never reads the body. That is the compositional");
    println!("     analysis the talk proposes for C++, and how rustc has always worked.");
    println!();

    let r;
    {
        let short_lived = String::from("checker");
        r = first(&a, &short_lived);
    }
    println!("THE RESULT TIED TO `a` ONLY");
    println!("  first(&a, &short_lived) outlives the block: r = {r}");
    println!("  In C++ that is [[clang::lifetimebound]] on `a` alone. Clang 23 checks");
    println!("  that claim against the body too -- under -Wlifetime-safety-all, as a");
    println!("  warning you opt into; rustc checks the body against 'a on every build.");

    assert_eq!(longer(&a, &b), "checker");
    assert_eq!(r, "borrow");
}
