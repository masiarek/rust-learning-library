//! One task three ways: list a struct's field names, and give each field a
//! getter.

/// A function cannot see `Point`'s fields. Every name is typed a second time,
/// and nothing checks the copies against the struct.
mod by_function {
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }

    pub fn field_names() -> &'static [&'static str] {
        &["x", "y"]
    }

    impl Point {
        pub fn get_x(&self) -> &i32 {
            &self.x
        }
        pub fn get_y(&self) -> &i32 {
            &self.y
        }
    }
}

/// `macro_rules!` can walk the fields, but only of a struct written inside the
/// call, and only in the shapes its pattern spells out.
macro_rules! with_field_names {
    ($vis:vis struct $name:ident { $($field_vis:vis $field:ident : $ty:ty),* $(,)? }) => {
        $vis struct $name { $($field_vis $field: $ty),* }

        impl $name {
            pub const FIELD_NAMES: &[&str] = &[$(stringify!($field)),*];
        }
    };
}

mod by_macro_rules {
    with_field_names! {
        pub struct Point {
            pub x: i32,
            pub y: i32,
        }
    }
}

/// A derive reads the struct where it is declared, and makes `get_x` from `x`.
mod by_derive {
    use field_names::FieldNames;

    #[derive(FieldNames)]
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }
}

fn main() {
    let p = by_function::Point { x: 1, y: 2 };
    let names = by_function::field_names();
    println!("a function     {names:?}  get_x() + get_y() = {}", p.get_x() + p.get_y());

    let p = by_macro_rules::Point { x: 1, y: 2 };
    let names = by_macro_rules::Point::FIELD_NAMES;
    println!("macro_rules!   {names:?}  no getters: x + y = {}", p.x + p.y);

    let p = by_derive::Point { x: 1, y: 2 };
    let names = by_derive::Point::FIELD_NAMES;
    println!("a derive       {names:?}  get_x() + get_y() = {}", p.get_x() + p.get_y());
}
