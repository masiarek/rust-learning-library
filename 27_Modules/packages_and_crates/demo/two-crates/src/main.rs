//! The binary crate that shares the package's name. It sees the library by
//! that name, not by `crate::`, which here means main.rs itself.

use pg_pkg_two_crates::hello;

fn main() {
    println!("{}", hello("src/main.rs"));
    println!("this crate is called {}", module_path!());
}
