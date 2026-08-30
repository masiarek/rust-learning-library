// What an unbounded T can do, where the bound belongs, and what derive writes for you.

use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug)]
struct Handle(u32); // deliberately NOT Clone

struct Container<T> {
    // No bound on the struct, on purpose.
    value: T,
}

impl<T> Container<T> {
    fn new(value: T) -> Self {
        Self { value }
    }

    fn into_inner(self) -> T {
        self.value
    }
}

// The bound sits on the impl that needs it. Everything above stays open.
impl<T: Clone> Container<T> {
    fn duplicate(&self) -> (T, T) {
        (self.value.clone(), self.value.clone())
    }
}

// One bound, three spellings — the first two identical to the compiler.
fn shout<T: Display>(v: T) -> String {
    format!("{v}!")
}
fn shout_where<T>(v: T) -> String
where
    T: Display,
{
    format!("{v}!")
}
fn shout_impl(v: impl Display) -> String {
    format!("{v}!")
}

// ...but the third does not tie its parameters together. Two `impl Display`
// arguments are two independent types; `<T: Display>` twice is one type twice.
fn pair_loose(a: impl Display, b: impl Display) -> String {
    format!("{a}/{b}")
}

// derive writes `impl<T: Clone> Clone for Pair<T>` — the bound lands on the impl,
// never on the struct, so a Pair of something unclonable is still a legal Pair.
#[derive(Clone, Debug)]
struct Pair<T> {
    left: T,
    right: T,
}

// A hand-written impl can be more precise than the derive: an Rc is Clone
// whatever it points at, so this one needs no bound on T at all.
#[derive(Debug)]
struct Shared<T> {
    inner: Rc<T>,
}

impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Shared { inner: Rc::clone(&self.inner) }
    }
}

fn main() {
    // A T with no traits at all: storable, movable, droppable. That is the list.
    let opaque = Container::new(Handle(7));
    // opaque.duplicate();                    // error[E0599]: `Handle: Clone` is not satisfied
    // println!("{}", opaque.value);          // error[E0277]: `Handle` doesn't implement `Display`
    println!("a Container<Handle> is fine: {:?}", opaque.into_inner());

    // Add the trait and the method appears — same struct, same impl block.
    let name = Container::new(String::from("Ada"));
    let (a, b) = name.duplicate();
    println!("duplicate() on Container<String>: {a:?} {b:?}");
    println!();

    println!("{} {} {}", shout(1), shout_where("two"), shout_impl(3.5));
    println!("two independent types: {}", pair_loose(1u8, "two"));
    println!();

    // Pair<Handle> constructs and prints; it just cannot be cloned.
    let handles = Pair { left: Handle(1), right: Handle(2) };
    println!("Pair<Handle>  built and printable: {handles:?}");
    println!("              and readable:        {} {}", handles.left.0, handles.right.0);
    // handles.clone();                       // error[E0599]: `Handle: Clone` is not satisfied

    let words = Pair { left: String::from("yes"), right: String::from("no") };
    println!("Pair<String>  cloneable:          {:?}", words.clone());
    println!();

    // The hand-written impl clones a Shared<Handle> that derive would have refused.
    let shared = Shared { inner: Rc::new(Handle(9)) };
    let second = shared.clone();
    println!("Shared<Handle> cloned by hand: value {}, {} owners",
        second.inner.0, Rc::strong_count(&shared.inner));
}
