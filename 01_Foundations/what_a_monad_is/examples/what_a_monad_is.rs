//! What the word "monad" names, shown as a shape three of Rust's types share.
//!
//! Nothing here imports anything. The point is that `Option`, `Result` and
//! `Vec` already do this, and the only thing missing is a name Rust declines
//! to use.
//!
//!   rustc --edition 2024 what_a_monad_is.rs -o /tmp/wami && /tmp/wami

fn half(n: i32) -> Option<i32> {
    if n % 2 == 0 { Some(n / 2) } else { None }
}

fn parse(s: &str) -> Result<i32, String> {
    s.parse::<i32>().map_err(|e| format!("{s:?}: {e}"))
}

fn recip(n: i32) -> Result<f64, String> {
    if n == 0 { Err("no reciprocal of zero".into()) } else { Ok(1.0 / f64::from(n)) }
}

fn main() {
    println!("1. `map` NESTS when the function itself can fail");
    // half returns Option<i32>, so mapping it over an Option gives Option<Option<i32>>.
    let nested: Option<Option<i32>> = Some(8).map(half);
    println!("   Some(8).map(half)       = {nested:?}      <- two layers, unusable");

    println!("\n2. `and_then` is the same thing, flattened");
    let flat: Option<i32> = Some(8).and_then(half);
    println!("   Some(8).and_then(half)  = {flat:?}            <- one layer");
    println!("   Some(8).and_then(half).and_then(half) = {:?}", Some(8).and_then(half).and_then(half));
    println!("   Some(7).and_then(half)  = {:?}              <- short-circuits", Some(7).and_then(half));

    println!("\n3. The SAME shape on a different type");
    let chained: Result<f64, String> = parse("4").and_then(recip);
    println!("   parse(\"4\").and_then(recip)   = {chained:?}");
    println!("   parse(\"0\").and_then(recip)   = {:?}", parse("0").and_then(recip));
    println!("   parse(\"x\").and_then(recip)   = {:?}", parse("x").and_then(recip));

    println!("\n4. `?` is the same chain, written as statements");
    println!("   recip_of(\"4\") = {:?}", recip_of("4"));
    println!("   recip_of(\"0\") = {:?}", recip_of("0"));

    println!("\n5. And Vec does it too — `flat_map` is the same operation");
    let v: Vec<i32> = vec![1, 2, 3].into_iter().flat_map(|n| vec![n, n * 10]).collect();
    println!("   [1,2,3].flat_map(|n| [n, n*10]) = {v:?}");

    println!("\n6. The three laws, checked");
    // Left identity:  wrap(x).and_then(f)  ==  f(x)
    let left = Some(8).and_then(half) == half(8);
    // Right identity: m.and_then(wrap)     ==  m
    let right = Some(8).and_then(|n| Some(n)) == Some(8);
    // Associativity:  m.and_then(f).and_then(g) == m.and_then(|x| f(x).and_then(g))
    let m = Some(8);
    let assoc = m.and_then(half).and_then(half) == m.and_then(|x| half(x).and_then(half));
    println!("   left identity   Some(8).and_then(half) == half(8)          {left}");
    println!("   right identity  Some(8).and_then(Some) == Some(8)          {right}");
    println!("   associativity   (m>>=f)>>=g == m>>=(\\x -> f x >>= g)       {assoc}");
    assert!(left && right && assoc);
    println!("\n   all three hold — which is what makes it a monad rather than just a method");
}

/// Statement form. Every `?` is an `and_then`, and the compiler writes it.
fn recip_of(s: &str) -> Result<f64, String> {
    let n = parse(s)?;
    let r = recip(n)?;
    Ok(r)
}
