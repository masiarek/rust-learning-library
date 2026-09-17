use std::marker::PhantomData;

use debug_generics::DebugBoundingParams;

/// A unit of measure: a marker type, never constructed, with no `Debug`.
struct Meters;

/// `T` appears only inside `PhantomData`, which is `Debug` for every `T`.
#[derive(DebugBoundingParams)]
struct Length<T> {
    value: u32,
    unit: PhantomData<T>,
}

/// The same struct under the compiler's own derive.
#[derive(Debug)]
struct StdLength<T> {
    value: u32,
    unit: PhantomData<T>,
}

fn main() {
    let ours: Length<Meters> = Length {
        value: 5,
        unit: PhantomData,
    };
    let std: StdLength<Meters> = StdLength {
        value: 5,
        unit: PhantomData,
    };
    println!("{ours:?}");
    println!("{std:?}");
}
