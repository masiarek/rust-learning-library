//! `crate::` here is other.rs, a crate with no `shared` module in it.

fn main() {
    println!("{}", crate::shared::greeting());
}
