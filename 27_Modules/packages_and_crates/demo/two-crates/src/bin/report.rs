//! A second binary crate, from src/bin/. Same library, same spelling.

use pg_pkg_two_crates::hello;

fn main() {
    println!("{}", hello("src/bin/report.rs"));
    println!("this crate is called {}", module_path!());
}
