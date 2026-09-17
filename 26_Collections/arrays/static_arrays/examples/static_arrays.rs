//! An array in a `const` or a `static`, a table of records with borrows in it,
//! and what the `'static` in its type does and does not say.
//!
//!   rustc --edition 2024 static_arrays.rs -o /tmp/sa && /tmp/sa

use std::any::type_name_of_val;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dimension {
    Length,
    Mass,
    Time,
}

#[derive(Debug)]
pub struct Unit<'a> {
    pub names: [&'a str; 2],
    pub dimension: Dimension,
}

/// A table of records, in the binary, for the whole run.
pub static UNITS: [Unit<'static>; 3] = [
    Unit { names: ["m", "meter"], dimension: Dimension::Length },
    Unit { names: ["g", "gram"], dimension: Dimension::Mass },
    Unit { names: ["s", "second"], dimension: Dimension::Time },
];

/// The same table with the lifetime left out: in a `static` or `const` type it
/// means `'static`.
pub static UNITS_ELIDED: [Unit; 1] = [Unit { names: ["m", "meter"], dimension: Dimension::Length }];

const MY_DATA: [i8; 3] = [1, 2, 3];

fn find(name: &str) -> Option<&'static Unit<'static>> {
    UNITS.iter().find(|u| u.names.contains(&name))
}

fn wants_static<T: 'static>(_: &T) -> &'static str {
    "accepted"
}

fn main() {
    println!("1. A table of records in a static");
    for unit in &UNITS {
        println!("   {:?} -> {:?}", unit.names, unit.dimension);
    }
    println!("   find(\"gram\") = {:?}", find("gram").map(|u| u.dimension));
    println!("   find(\"inch\") = {:?}", find("inch").map(|u| u.dimension));

    println!();
    println!("2. What is in `names`");
    println!("   UNITS[0].names    is {}", type_name_of_val(&UNITS[0].names));
    println!("   UNITS[0].names[1] is {} = {:?}: a borrowed literal, not a String", type_name_of_val(&UNITS[0].names[1]), UNITS[0].names[1]);
    println!("   UNITS_ELIDED, written [Unit; 1], holds {:?}", UNITS_ELIDED[0].names);

    println!();
    println!("3. A Unit<'static> that does not live forever");
    {
        let local: Unit<'static> = Unit { names: ["h", "hour"], dimension: Dimension::Time };
        println!("   inside a block: {:?}", local.names);
        println!("   wants_static(&local) -> {}", wants_static(&local));
    } // `local` is dropped here, like any other local
    println!("   the block ended and `local` is gone; UNITS is still here: {:?}", UNITS[2].names);
    let text = String::from("meter");
    println!("   a String local is 'static too: wants_static(&text) -> {}", wants_static(&text));
    println!("   T: 'static means T holds no short borrows, not that the value is immortal");

    println!();
    println!("4. A const array, and a local that changes its contents");
    let mut my_data = [2; 3];
    my_data[0] = 1;
    my_data[2] = 3;
    println!("   MY_DATA = {MY_DATA:?}, my_data = {my_data:?}, equal: {}", MY_DATA == my_data);
    println!("   `mut` let us overwrite two elements; no method on [i8; 3] adds a fourth");
    const MY_DATA_4: [i8; 4] = [1, 2, 3, 4];
    println!("   MY_DATA[..] == MY_DATA_4[..] -> {}  (as arrays: E0277, can't compare)", MY_DATA[..] == MY_DATA_4[..]);
    println!("   MY_DATA == MY_DATA_4[..3]    -> {}", MY_DATA == MY_DATA_4[..3]);

    println!();
    println!("5. const or static");
    println!("   a const is a value pasted in at each use; a static is one place");
    println!("   find(\"m\") points into UNITS itself, no copy: {}", std::ptr::eq(find("m").unwrap(), &UNITS[0]));
    println!("   size_of_val(&UNITS) = {} bytes: 3 x (two &str of 16 + a 1-byte enum, padded)", std::mem::size_of_val(&UNITS));
}
