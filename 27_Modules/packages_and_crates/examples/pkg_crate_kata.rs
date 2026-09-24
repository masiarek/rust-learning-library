//! Kata solution: where `crate::` points from five places in one package, and
//! what this crate is called. The measured half comes from `module_path!()`.
//!
//!   rustc --edition 2024 pkg_crate_kata.rs -o /tmp/pck && /tmp/pck

mod product {
    pub mod category {
        pub fn path_here() -> &'static str {
            module_path!()
        }
    }

    pub fn path_here() -> &'static str {
        module_path!()
    }
}

fn main() {
    println!("1. What this crate is called, measured");
    println!("   module_path!() at the root        = {}", module_path!());
    println!("   module_path!() in product         = {}", product::path_here());
    println!("   module_path!() in product::category = {}", product::category::path_here());
    println!("   Under bare rustc the crate is named after the file: pkg_crate_kata.rs");
    println!("   gives pkg_crate_kata. Under Cargo a bin target is named after its file");
    println!("   too (src/bin/report.rs -> report), except src/main.rs, which takes the");
    println!("   package's name, and src/lib.rs, which takes it with `-` turned into `_`.");

    println!();
    println!("2. Where `crate::` points, from five places in one package");
    println!("   src/lib.rs               crate:: is lib.rs; `use my_package::X` is not");
    println!("                            available (a crate has no name for itself)");
    println!("   src/main.rs              crate:: is main.rs itself, a second crate;");
    println!("                            the library is `use my_package::X`");
    println!("   src/bin/tool.rs          crate:: is tool.rs, a third crate; it sees the");
    println!("                            library by name and main.rs not at all");
    println!("   #[cfg(test)] mod tests   inside lib.rs: crate:: is lib.rs, super:: is the");
    println!("                            enclosing module, private items included");
    println!("   tests/it.rs              its own crate, like a binary: `use my_package::X`,");
    println!("                            pub items only");

    println!();
    println!("3. The trap, from the page's demo");
    println!("   shared code in src/main.rs and `crate::shared::greeting()` in src/bin/other.rs");
    println!("   -> E0433: could not find `shared` in the crate root");
    println!("   the fix is a src/lib.rs: every binary of the package can `use` it by name");
}
