//! The library crate. Its name is the package's, with `-` turned into `_`,
//! because `pg-pkg-two-crates` is not a Rust identifier and `pg_pkg_two_crates` is.

mod greeting {
    pub fn hello(from: &str) -> String {
        format!("hello from {from}, through the library crate {}", module_path!().split("::").next().unwrap_or("?"))
    }
}

pub use greeting::hello;
