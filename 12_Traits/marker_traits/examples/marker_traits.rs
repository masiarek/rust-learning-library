//! A marker trait has no methods. It states a property, and the compiler holds
//! you to it.
//!
//!   rustc --edition 2024 marker_traits.rs -o /tmp/mt && /tmp/mt

use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Arc;

// A marker of our own: no methods, no default bodies, nothing to call.
trait Audited {}

struct Ballot;
struct Draft;

impl Audited for Ballot {}
// Draft deliberately does NOT implement it.

fn publish<T: Audited>(_thing: &T) -> &'static str {
    "published"
}

// `fn f<T>` silently means `fn f<T: Sized>`. `?Sized` is how you opt out — and
// it is the only "negative" bound in the language.
fn width<T: ?Sized>(value: &T) -> usize {
    std::mem::size_of_val(value)
}

// A bound that compiles or does not: this is how you ask the compiler a
// question about a type.
fn assert_send<T: Send>() {}

// PhantomData: a marker on a STRUCT rather than on an impl. It makes the type
// parameter real to the type system while costing nothing at run time.
struct Tagged<Unit> {
    value: f64,
    _unit: PhantomData<Unit>,
}

struct Metres;
struct Feet;

impl<Unit> Tagged<Unit> {
    fn new(value: f64) -> Self {
        Tagged { value, _unit: PhantomData }
    }
}

fn main() {
    println!("1. A marker trait gates a function without adding a method");
    println!("   publish(&Ballot) = {}", publish(&Ballot));
    println!("   publish(&Draft)  would be E0277: `Draft: Audited` is not satisfied");
    let _ = Draft; // Draft exists; it just cannot be published

    println!();
    println!("2. `Sized` is the marker you never wrote — it is implicit on every T");
    println!("   width(\"hello\")     = {}   a str: 5 bytes, and str is NOT Sized", width("hello"));
    println!("   width(&[1i32, 2, 3][..]) = {}   a slice of three i32", width(&[1i32, 2, 3][..]));
    println!("   Without `T: ?Sized` on that function, neither call would compile.");

    println!();
    println!("3. `Send` and `Sync` are AUTO traits: nobody wrote the impls");
    assert_send::<Arc<i32>>();
    assert_send::<i32>();
    println!("   assert_send::<Arc<i32>>()  compiles — Arc's count is atomic");
    println!("   assert_send::<Rc<i32>>()   does NOT — Rc's count is not");
    println!("   size_of::<Rc<i32>>() = {} and size_of::<Arc<i32>>() = {}: the same",
        std::mem::size_of::<Rc<i32>>(), std::mem::size_of::<Arc<i32>>());
    println!("   size. The difference is a promise, not a byte.");

    println!();
    println!("4. PhantomData marks the TYPE and costs nothing");
    let track = Tagged::<Metres>::new(400.0);
    let field = Tagged::<Feet>::new(300.0);
    println!("   Tagged<Metres> {} · Tagged<Feet> {}", track.value, field.value);
    println!("   size_of::<f64>()            = {}", std::mem::size_of::<f64>());
    println!("   size_of::<Tagged<Metres>>() = {}   the unit is free", std::mem::size_of::<Tagged<Metres>>());
    println!("   ...and adding them is a compile error, which is the entire point.");
}
