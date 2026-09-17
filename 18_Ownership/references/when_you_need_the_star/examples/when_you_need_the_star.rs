//! When you need the `*`: every form below compiles on a `&bool`, `&i32` or
//! `&mut bool`. The forms that are refused live on the page, as rustc output.
//!
//!   rustc --edition 2024 when_you_need_the_star.rs -o /tmp/when_you_need_the_star && /tmp/when_you_need_the_star

use std::any::type_name_of_val;

fn show(flag: &bool) -> bool {
    *flag
}

fn main() {
    let on = true;
    let flag: &bool = &on;
    let other = false;
    let x = -7;
    let n: &i32 = &x;

    println!("1. The dot dereferences the receiver for you");
    println!("   n.abs()                        -> {}", n.abs());
    println!("   n.pow(2)                       -> {}", n.pow(2));

    println!();
    println!("2. An operator with an impl for &T needs no `*`");
    println!("   !flag                          -> {}", !flag);
    println!("   flag & other                   -> {}", flag & other);
    println!("   flag | other                   -> {}", flag | other);
    println!("   n + 1                          -> {}", n + 1);
    println!("   -n                             -> {}", -n);

    println!();
    println!("3. Formatting reads through the reference");
    println!("   println!(\"{{n}}\")                -> {n}");

    println!();
    println!("4. A literal pattern matched against a reference");
    let word = match flag {
        true => "on",
        false => "off",
    };
    println!("   match flag {{ true => .. }}      -> {word}");

    println!();
    println!("5. Deref coercion: a `&bool` parameter accepts `&&bool` and `&Box<bool>`");
    let r: &bool = flag;
    let boxed: Box<bool> = Box::new(false);
    println!("   {:<31}-> {}", "show(&r)", show(&r)); // fn show(flag: &bool)
    println!("   {:<31}-> {}", "show(&boxed)", show(&boxed));

    println!();
    println!("6. A condition wants exactly `bool`: write the `*`");
    if *flag {
        println!("   if *flag                       -> taken");
    }
    println!("   *flag && !other                -> {}", *flag && !other);

    println!();
    println!("7. `==` and `>` want both sides at the same depth");
    println!("   *flag == true                  -> {}", *flag == true);
    println!("   flag == &true                  -> {}", flag == &true);
    println!("   *n > 1                         -> {}", *n > 1);
    println!("   n > &1                         -> {}", n > &1);

    println!();
    println!("8. The dot stops at the first receiver that fits");
    let via_ref = n.max(&3);
    let via_value = (*n).max(3);
    println!("   n.max(&3)                      -> {via_ref}, a {}", type_name_of_val(&via_ref));
    println!("   (*n).max(3)                    -> {via_value}, an {}", type_name_of_val(&via_value));

    println!();
    println!("9. A closure gets one `&` per adapter");
    let bits = vec![true, false, true];
    println!("   bits.iter().any(|b| *b)        -> {}", bits.iter().any(|b| *b));
    println!("   bits.iter().filter(|b| **b)    -> {} kept", bits.iter().filter(|b| **b).count());
    println!("   bits.iter().filter(|&&b| b)    -> {} kept", bits.iter().filter(|&&b| b).count());
    println!("   bits.contains(&true)           -> {}", bits.contains(&true));

    println!();
    println!("10. Writing through `&mut bool` names the place with `*`");
    let mut lamp = false;
    let slot: &mut bool = &mut lamp;
    *slot = true;
    println!("   *slot = true                   -> *slot = {}", *slot);
    *slot = !*slot;
    println!("   *slot = !*slot                 -> *slot = {}", *slot);
    *slot |= true;
    println!("   *slot |= true                  -> lamp = {lamp}");

    println!();
    println!("Checkpoint. `if flag` is refused. Does `assert!(flag)` compile?");
    assert!(flag);
    println!("   yes: assert!(flag) expands to `if !flag {{ panic!(..) }}`,");
    println!("   and `!` has an impl for &bool");
}
