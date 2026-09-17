use debug_generics::DebugBoundingParams;

/// A lifetime, a bounded type parameter, a const parameter and a `where` clause.
#[derive(DebugBoundingParams)]
struct Wrapper<'a, T: Clone, const N: usize>
where
    T: PartialEq,
{
    label: &'a str,
    items: [T; N],
}

fn main() {
    let evens = Wrapper {
        label: "evens",
        items: [2, 4, 6],
    };
    let words = Wrapper {
        label: "words",
        items: ["foo", "bar"],
    };
    println!("{evens:?}");
    println!("{words:?}");
}
