//! Coercion: the fifth conversion, and the only one you never write down.
//!
//!   rustc --edition 2024 coercion.rs -o /tmp/coe && /tmp/coe

use std::fmt::Debug;

trait Shape {
    fn area(&self) -> u32;
}

struct Square(u32);

impl Shape for Square {
    fn area(&self) -> u32 {
        self.0 * self.0
    }
}

fn width(s: &str) -> usize {
    s.len()
}

fn count(v: &[i32]) -> usize {
    v.len()
}

fn peek(v: &Vec<i32>) -> i32 {
    v[0]
}

fn apply(f: fn(u32) -> u32, x: u32) -> u32 {
    f(x)
}

fn double(x: u32) -> u32 {
    x * 2
}

/// A generic parameter is NOT a coercion site: both arguments must arrive
/// as the same type, and nothing is adjusted to make that happen.
fn longer<T: AsRef<str>>(a: T, b: T) -> bool {
    a.as_ref().len() > b.as_ref().len()
}

fn main() {
    println!("1. Seven coercions, none of them written down");
    let owned = String::from("ballot");
    println!("     &String  -> &str      width(&owned)      = {}", width(&owned));
    let v = vec![10, 20, 30];
    println!("     &Vec<i32>-> &[i32]    count(&v)          = {}", count(&v));
    let arr = [1, 2, 3, 4];
    println!("     &[i32; 4]-> &[i32]    count(&arr)        = {}", count(&arr));
    let mut m = vec![7, 8];
    println!("     &mut Vec -> &Vec      peek(&mut m)       = {}", peek(&mut m));
    println!("     fn item  -> fn ptr    apply(double, 21)  = {}", apply(double, 21));
    let boxed: Box<dyn Shape> = Box::new(Square(3));
    println!("     Box<Square> -> Box<dyn Shape>            = {}", boxed.area());
    let seen: &dyn Debug = &42u8;
    println!("     &u8      -> &dyn Debug                   = {seen:?}");
    println!("   Every one of those arguments was written as `&x` and arrived as");
    println!("   a different type. Nothing on the call site says so.\n");

    println!("2. It happens at named places, not everywhere");
    let ascribed: &str = &owned;
    println!("     let with a type          let s: &str = &owned;   -> {ascribed}");
    fn give_back(s: &String) -> &str {
        s
    }
    println!("     a return value           fn f(s: &String) -> &str -> {}", give_back(&owned));
    struct Holder<'a> {
        text: &'a str,
    }
    let h = Holder { text: &owned };
    println!("     a struct field           Holder {{ text: &owned }} -> {}", h.text);
    println!("     ...plus function arguments, as above. Those are `coercion");
    println!("     sites`: the compiler only looks for one where it already");
    println!("     knows the type it wants.\n");

    println!("3. Where it does not happen");
    println!("     Option<&String> -> Option<&str>   : no. Coercion does not");
    println!("       reach inside another type. Fix: `.as_deref()`, or");
    println!("       `.map(|s| s.as_str())`.");
    let held: Option<String> = Some(String::from("x"));
    let fixed: Option<&str> = held.as_deref();
    println!("       held.as_deref() = {fixed:?}");
    println!("     u8 + u16                         : no. Rust has no numeric");
    println!("       promotion at all. Fix: `u16::from(small) + big`.");
    let small: u8 = 1;
    let big: u16 = 2;
    println!("       u16::from(small) + big = {}", u16::from(small) + big);
    println!("     longer(&owned, \"vote\")           : no. A generic parameter");
    println!("       is not a coercion site; `T` is inferred from the first");
    println!("       argument and the second must already be that type.");
    println!("       longer(owned.as_str(), \"vote\") = {}",
             longer(owned.as_str(), "vote"));
    println!("     width(owned)                     : no. Deref coercion works");
    println!("       on references; `String` by value is not `&String`.");
    println!("       width(&owned) = {}\n", width(&owned));

    println!("4. Method calls look like coercion and are a separate rule");
    println!("     owned.len()      = {}   <- auto-deref: String, then str", owned.len());
    println!("     v.first()        = {:?}   <- auto-deref: Vec, then [i32]", v.first());
    println!("     (&owned).len()   = {}   <- auto-deref again, through &", (&owned).len());
    println!("   The receiver of a method call gets `&`, `&mut` and `*` inserted");
    println!("   until something fits. That is why `v.first()` finds a slice");
    println!("   method — and why removing a `&` often changes nothing.\n");

    println!("5. What to write when nothing fires for you");
    println!("     &String -> &str      &owned  ·  owned.as_str()");
    println!("     Vec<String> -> Vec<&str>          v.iter().map(String::as_str)");
    println!("     Option<String> -> Option<&str>    held.as_deref()");
    println!("     Vec<T>  -> &[T]      &v      ·  v.as_slice()");
    println!("     u8      -> u16       u16::from(small)");
    println!("   The named method is never wrong, and it is what to reach for");
    println!("   the moment the type sits inside anything else.");
}
