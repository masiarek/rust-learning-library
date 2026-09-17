use debug_generics::DebugIgnoringGenerics;

#[derive(DebugIgnoringGenerics)]
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
    println!("{evens:?}");
}
