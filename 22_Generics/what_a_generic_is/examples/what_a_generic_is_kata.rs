// Kata solution: a struct with two type parameters, and the swap that
// returns a different type from the one it was called on.

#[derive(Debug)]
struct Pair<A, B> {
    left: A,
    right: B,
}

impl<A, B> Pair<A, B> {
    fn new(left: A, right: B) -> Self {
        Self { left, right }
    }

    // Note the return type: Pair<B, A>, not Self. Swapping the fields
    // swaps the type parameters with them.
    fn swap(self) -> Pair<B, A> {
        Pair { left: self.right, right: self.left }
    }
}

fn main() {
    let ballot = Pair::new("Ada", 5u8);
    println!("ballot          {ballot:?}");
    println!("ballot.swap()   {:?}", ballot.swap());

    // Both parameters may be filled with the same type. They are still two.
    let both = Pair::new(3u8, 5u8);
    println!("same type twice {both:?}");

    // And they may be filled with anything, independently.
    let nested = Pair::new(vec!['a', 'b'], Pair::new(1i8, "one"));
    println!("nested          {nested:?}");
    println!("sizes: Pair<u8, u8> {} · Pair<u8, u64> {}",
        size_of::<Pair<u8, u8>>(), size_of::<Pair<u8, u64>>());
}
