use std::marker::PhantomData;

use debug_generics::DebugBoundingFields;

/// A unit of measure: a marker type, never constructed, with no `Debug`.
struct Meters;

#[derive(DebugBoundingFields)]
struct Length<T> {
    value: u32,
    unit: PhantomData<T>,
}

fn main() {
    let length: Length<Meters> = Length {
        value: 5,
        unit: PhantomData,
    };
    println!("{length:?}");
}
