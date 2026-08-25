//! Returning a trait: why `-> Method` cannot compile, and the two things that do.
//!
//!   rustc --edition 2024 returning_a_trait.rs -o /tmp/rat && /tmp/rat

trait Method {
    fn name(&self) -> &'static str;
    fn seats(&self) -> usize {
        1
    }
}

struct Star;
struct BlocStar {
    seats: usize,
}

impl Method for Star {
    fn name(&self) -> &'static str {
        "STAR"
    }
}

impl Method for BlocStar {
    fn name(&self) -> &'static str {
        "Bloc STAR"
    }
    fn seats(&self) -> usize {
        self.seats
    }
}

// ---------------------------------------------------------------------------
// The function that cannot be written `-> Method`. Two branches, two types, two
// different sizes — and the caller has to know how much stack to set aside
// before the branch is taken. `Box` puts the value on the heap and returns a
// pointer, and a pointer's size is known no matter what it points at.
// ---------------------------------------------------------------------------
fn method_for(seats: usize) -> Box<dyn Method> {
    if seats == 1 {
        Box::new(Star)
    } else {
        Box::new(BlocStar { seats })
    }
}

// When only ONE type can come back, `impl Method` says "some single type that
// implements this, and I am not writing down which". No heap, no vtable: the
// compiler knows it is a Star, it is just not in the signature.
fn single_winner_method() -> impl Method {
    Star
}

fn main() {
    println!("1. One function, two concrete types — chosen at run time");
    for seats in [1, 3] {
        let m = method_for(seats);
        println!("   {} seat(s) -> {:<9} counts {} seat(s)", seats, m.name(), m.seats());
    }

    println!();
    println!("2. `impl Trait` when the answer is always the same type");
    let m = single_winner_method();
    println!("   single_winner_method() -> {}", m.name());

    println!();
    println!("3. Why the box was needed: sizes");
    println!("   Star              {:>2} bytes   a unit struct holds nothing", std::mem::size_of::<Star>());
    println!("   BlocStar          {:>2} bytes   one usize", std::mem::size_of::<BlocStar>());
    println!("   &Star             {:>2} bytes   one pointer; the type is known", std::mem::size_of::<&Star>());
    println!("   &dyn Method       {:>2} bytes   TWO pointers: the data, and the vtable", std::mem::size_of::<&dyn Method>());
    println!("   Box<dyn Method>   {:>2} bytes   same pair, owned", std::mem::size_of::<Box<dyn Method>>());

    println!();
    println!("4. The vtable is what makes the call land in the right impl");
    let boxed: Vec<Box<dyn Method>> = vec![Box::new(Star), Box::new(BlocStar { seats: 5 })];
    for m in &boxed {
        println!("   {:<9} -> {} seat(s)", m.name(), m.seats());
    }
}
