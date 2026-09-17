//! `macro_rules!` making `get_x` out of `x`. `${concat(…)}` does it, and is
//! not stable in 1.98.0.

macro_rules! getters {
    (struct $name:ident { $($field:ident : $ty:ty),* $(,)? }) => {
        struct $name { $($field: $ty),* }

        impl $name {
            $(fn ${concat(get_, $field)}(&self) -> &$ty { &self.$field })*
        }
    };
}

getters! {
    struct Point { x: i32, y: i32 }
}

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("{}", p.get_x() + p.get_y());
}
