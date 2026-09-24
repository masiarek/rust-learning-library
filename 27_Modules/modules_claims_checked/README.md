# What *Rust: The Practical Guide* says about packages, crates and modules, run

**Level:** 101 → 201 · a companion to [Packages and crates](../packages_and_crates/README.md), [Modules and visibility](../modules_and_visibility/README.md), [Bringing names in with `use`](../the_use_declaration/README.md) and [One module per file](../one_module_per_file/README.md)

**One line:** Chapter 6 of *Rust: The Practical Guide* builds an online-store library one refusal at a time, and the refusals are real: every error the book names comes out of rustc 1.98.0, most by the words the book quotes. Six statements around them do not hold — the `bin` folder beside `src`, "you must specify their absolute paths", the box that blames Rust 1.80, the "unresolved module" wording, and two sentences about what a parent module and `pub(self)` can reach. The chapter's final library, rebuilt below in its file layout, prints a bill.

## The finished library

The layout of Listing 6.29, one file per module and `product.rs` beside a `product/` folder for its submodule. The crate root keeps the three modules private and publishes four names:

<!-- file:demo/shop/src/lib.rs -->
```rust title="demo/shop/src/lib.rs"
//! The chapter's online store in the layout of Listing 6.29: one file per
//! module, and `product.rs` beside a `product/` folder for its submodule.
//! Nothing here is `pub mod`; the three `pub use` lines are the whole public
//! surface, so a user writes `pg_store_shop::Product` and never sees `product`.

mod customer;
mod order;
mod product;

pub use customer::Customer;
pub use order::Order;
pub use product::{Category, Product};
```
<!-- /file -->

<!-- file:demo/shop/src/product.rs -->
```rust title="demo/shop/src/product.rs"
mod category;

pub use category::Category;

pub struct Product {
    id: u64,
    name: String,
    price: f64,
    category: Category,
}

impl Product {
    pub fn new(id: u64, name: String, price: f64, category: Category) -> Self {
        Self { id, name, price, category }
    }

    fn calculate_tax(&self) -> f64 {
        self.price * 0.1
    }

    pub fn product_price(&self) -> f64 {
        self.price + self.calculate_tax()
    }

    /// The fields stay private; this is the door that hands their values out.
    pub fn label(&self) -> String {
        format!("{} ({:?}), product #{}", self.name, self.category, self.id)
    }
}
```
<!-- /file -->

<!-- file:demo/shop/src/product/category.rs -->
```rust title="demo/shop/src/product/category.rs"
#[derive(Debug)]
pub enum Category {
    Electronics,
    Clothing,
    Books,
}
```
<!-- /file -->

<!-- file:demo/shop/src/customer.rs -->
```rust title="demo/shop/src/customer.rs"
pub struct Customer {
    id: u64,
    name: String,
    email: String,
}

impl Customer {
    pub fn new(id: u64, name: String, email: String) -> Self {
        Self { id, name, email }
    }

    pub fn label(&self) -> String {
        format!("{} <{}>, customer #{}", self.name, self.email, self.id)
    }
}
```
<!-- /file -->

<!-- file:demo/shop/src/order.rs -->
```rust title="demo/shop/src/order.rs"
use crate::customer::Customer;
use crate::product::Product;

pub struct Order {
    id: u64,
    product: Product,
    customer: Customer,
    quantity: u32,
}

impl Order {
    /// The book never builds an `Order`; this constructor is what lets
    /// `main.rs` print a bill.
    pub fn new(id: u64, product: Product, customer: Customer, quantity: u32) -> Self {
        Self { id, product, customer, quantity }
    }

    fn calculate_discount(&self) -> f64 {
        if self.quantity > 5 { 0.1 } else { 0.0 }
    }

    pub fn total_bill(&self) -> f64 {
        let discount = self.calculate_discount();
        let total_before_discount = self.product.product_price() * self.quantity as f64;
        total_before_discount - (total_before_discount * discount)
    }

    pub fn bill(&self) -> String {
        format!(
            "order #{}: {} x {:.2} for {}, {:.0}% off -> {:.2}",
            self.id,
            self.quantity,
            self.product.product_price(),
            self.customer.label(),
            self.calculate_discount() * 100.0,
            self.total_bill(),
        )
    }
}
```
<!-- /file -->

The binary crate of the same package names the library by the package's name, not by `crate::`:

<!-- file:demo/shop/src/main.rs -->
```rust title="demo/shop/src/main.rs"
//! The binary crate of the same package. It sees the library only through the
//! library's public surface, by the package's name, never through `crate::`.

use pg_store_shop::{Category, Customer, Order, Product};

fn main() {
    let product = Product::new(1, String::from("Laptop"), 799.99, Category::Electronics);
    let customer = Customer::new(1, String::from("Alice"), String::from("alice@example.com"));
    println!("{}", product.label());
    println!("price with tax: {:.2}", product.product_price());
    let order = Order::new(1, product, customer, 6);
    println!("{}", order.bill());
}
```
<!-- /file -->

<!-- cargo:pg_store_bill -->
*Verified output of `cargo run -q -p pg_store_shop` — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
Laptop (Electronics), product #1
price with tax: 879.99
order #1: 6 x 879.99 for Alice <alice@example.com>, customer #1, 10% off -> 4751.94
```
<!-- /cargo -->

The package also has Listing 6.3's second binary, in the folder Cargo looks in, `src/bin/`. With two binaries, a plain `cargo run` has to be told which one; the manifest says so with `default-run`, the key the book does not mention:

<!-- file:demo/shop/Cargo.toml -->
```toml title="demo/shop/Cargo.toml"
[package]
name = "pg_store_shop"
version = "0.1.0"
edition = "2024"
publish = false

# The package has two binaries, src/main.rs and src/bin/my_binary.rs, so a plain
# `cargo run` needs to be told which one. This is the third way, which the book
# does not mention; `--bin` is the one it shows.
default-run = "pg_store_shop"
```
<!-- /file -->

<!-- cargo:pg_store_my_binary -->
*Verified output of `cargo run -q -p pg_store_shop --bin my_binary` — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
Hello from my_binary
```
<!-- /cargo -->

## Where it comes from

*Rust: The Practical Guide* by Nouman Azam (Rheinwerk Computing, 2025), chapter 6: §6.1 Code Organization, §6.2 Module Basics, §6.3 Visualizing and Organizing Modules and §6.4 Re-Exporting and Privacy. The book's home in this library is [*Rust: The Practical Guide*, run](../../10_Resources/rust_the_practical_guide/README.md), and the chapter's five exercises are on [*Rust: The Practical Guide*, chapter 6 exercises, run](../../10_Resources/rust_the_practical_guide/ch6_katas/README.md).

The store follows the book's shape — a `Product` with a `Category`, a `Customer`, an `Order` — with two additions the book never needs. `Order` is public with a constructor, so that `main.rs` can print a bill, and each type has a `label` method reading the fields the book leaves unread, so the finished library builds without a warning. Package names carry a `pg_store_` prefix because every Cargo demo in this library shares one target directory: where the book writes `my_package`, read `pg_store_shop`. Every listing the chapter stops at is rebuilt as a package of its own in [`demo/`](demo/cargo_runs.toml) and declared as a run that must fail, so each transcript below is an answer key rather than a paste. The book's `Cargo.toml` says `edition = "2021"`; the packages here say 2024, and every error and warning on this page is the same under either.

## Six statements that do not hold

**1. "By convention, executables are created inside the bin folder" — with `bin/` beside `src/` (§6.1, Listing 6.2).** Cargo discovers binaries in `src/bin/`, and a `bin/` folder at the package root is not a target at all. With the book's tree, `cargo metadata` lists the one binary `main.rs` makes, nothing in `bin/` is compiled, no error appears, and `cargo run` prints `Hello, world!` — [measured on Packages and crates](../packages_and_crates/README.md#the-books-bin-folder-and-what-cargo-new-writes-today). The two things the book then describes, the missing `main` and the "could not determine which binary" refusal, happen once the file is under `src/bin/`; both are keyed [below](#61-a-binary-without-main-and-two-binaries).

**2. "Since the Customer and Product modules are not in the same module as the Order, you must specify their absolute paths" (§6.2.3).** A relative path works. The `order` module of the program under [The claims, run](#the-claims-run) writes `use super::customer::Customer;` and `use super::product::Product;` and builds the same bill as the file layout above, which uses Listing 6.24's `crate::` paths. Which prefix to reach for is a question of taste and of what survives a rename; [One module per file](../one_module_per_file/README.md#the-paths-do-not-change-when-you-split) has the table.

**3. "This syntax might throw an error in the newer versions of Rust (1.80 or later)" (§6.4.1, the box after Listing 6.34).** Listing 6.34 fails on every compiler tried, and the fix the box gives is not what fixes it. The listing re-exports `Category` inside `product` and then, at the crate root, spells the path through the private `category` module anyway: `pub use product::{category::Category, Product};`. That is `E0603`, *"module `category` is private"*, on rustc 1.98.0 ([the key](#64-re-exports-and-private-fields)) and on rustc 1.79.0, the last version before the one the box names, with the book's edition 2021:

```text title="cargo build of the Listing 6.34 crate on rustc 1.79.0, in Docker (rust:1.79-slim), edition 2021 — hand-verified"
rustc 1.79.0 (129f3b996 2024-06-10)
error[E0603]: module `category` is private
 --> src/lib.rs:8:19
  |
8 | pub use product::{category::Category, Product};
  |                   ^^^^^^^^ private module
  |
help: consider importing this enum through its public re-export instead:
      crate::product::Category
 --> src/lib.rs:8:19
  |
8 | pub use product::{category::Category, Product};
  |                   ^^^^^^^^^^^^^^^^^^
note: the module `category` is defined here
 --> src/product.rs:1:1
  |
1 | mod category;
  | ^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
error: could not compile `my_package` (lib) due to 1 previous error
exit status 101
```

Listing 6.35's `pub use crate::product::Category;` builds on 1.79.0 and on 1.98.0. So does `pub use product::Category;` with no `crate::` at all, which is the line the finished library above uses. What changed between the two listings is the path — through `product`'s public re-export instead of through its private module — and rustc's own help line says so.

**4. "The compiler throws an error for the product.rs file: 'unresolved module'" (§6.3.2, Listing 6.28).** Those are rust-analyzer's words, shown in the editor. rustc's are `E0583`, *"file not found for module `category`"*, and its help names two files, not one: `product/category.rs` or `product/category/mod.rs` ([the key](#63-files-and-the-tool-that-draws-the-tree)). rust-analyzer 1.98.0's own `diagnostics` command on the same crate reports the case as `RustcHardError("E0583")` with the text *"unresolved module, can't find module file: product/category.rs, or product/category/mod.rs"* — so the book's quotation is right about the editor and wrong about the compiler, and both tools name both files.

**5. "A child module can access the contents of its parents, but parent modules cannot access the items within their child modules" (§6.2.4).** The first half holds, including private items: block 4 of the run reads a parent's private constant from its child through `super::`. The second half is true only of the child's private items. A parent reaches a child's `pub` items by an ordinary path — `category::Category` from `product`, `Category::Books` from the crate root — which is what the book's own Listing 6.15 relies on. [Privacy runs one way](../modules_and_visibility/README.md#privacy-runs-one-way) is the rule; the direction it runs in is *up*.

**6. "`pub(self)`: items are available within the current module in which they are defined" (§6.3.1).** And in every module inside it. `pub(self)` is the spelling of no keyword at all, and "private" in Rust means *this module and its descendants* — block 5 of the run calls a `pub(self)` function of `product` from `product`'s own child. The book's descriptions of `pub(crate)` and `pub` hold, with one reading to add: `pub` publishes an item to whoever can already see its module, which is why a `pub struct` inside a private `mod` still needs the `pub use` of §6.4.1. Two forms the chapter does not list, `pub(super)` and `pub(in path)`, are in block 5 too, and all four are on [Modules and visibility](../modules_and_visibility/README.md#pub-is-relative-and-there-are-four-of-them). cargo-modules prints `pub(crate)` for a private module at the crate root and `pub(self)` for a private struct: it draws each item's *reach*, not the keyword in the source.

Smaller slips, each visible in a key below. Listing 6.8 as printed produces three errors, not the one the book names: the `E0624` for `product_price` that the book meets at Listing 6.11 is already there. Listing 6.4's prose calls the methods `sales_tax` and `price`; the code says `calculate_tax` and `product_price`. Listing 6.36's `// Error` on each of four lines is one error on rustc 1.98.0, naming all four fields. Listing 6.41 imports from `my_package_book`, a crate that exists nowhere else in the chapter. And Listing 6.20's tree shows `mod order` where every listing before it says `mod Order` — the rename happened off the page, and the key for Listing 6.17 shows the warning it silences.

## The claims, run

The store's module tree as one file, in the shape of Listing 6.17, with `main` standing where `src/main.rs` would. The eight blocks are the chapter's remaining statements about modules:

<!-- output:pg_mod_claims -->
*Verified output of [`pg_mod_claims.rs`](examples/pg_mod_claims.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. `crate` is the root module: an absolute path and a relative one name one item
   crate::product::Product::new(..).label() = Laptop (Electronics), product #1
   product::Product::new(..).label()        = Laptop (Electronics), product #1

2. Listing 6.9, run
   Multiplication result: 12
   Result using `self`: 30

3. "you must specify their absolute paths": `order` above uses super:: and builds
   order #1: 6 x 879.99 for Alice <alice@example.com>, customer #1, 10% off -> 4751.94
   the same numbers as `cargo run` on the file layout in demo/

4. Private means this module and its descendants
   product::category reads product's private TAX_RATE through super:: -> 0.1
   the root reaching a child's pub item, Category::Books -> Books
   "parent modules cannot access the items within their child modules" is
   true of the child's private items, and not of its pub ones

5. Four narrower forms of pub, each measured from where it is written
   pub(self)           the same reach as no keyword: this module and its descendants
   pub(super)          reachable from product; E0603 from the crate root
   pub(in crate::product) the same reach, by the ancestor's name
   pub(crate)          every module of this crate, nothing outside it
   product::private_by_two_spellings() from here -> E0603: function is private

6. `use` binds a name; the item was already reachable
   Product::new(..).product_price()          = 22.00   <- through `use product::Product`
   product::Product::new(..).product_price() = 22.00   <- the full path, no `use` needed

7. `pub enum` publishes every variant; `pub` on one variant is E0449
   Category::Clothing named from the root -> Clothing

8. `pub struct` publishes the type, not the fields
   Product::new(..).label() = Novel (Books), product #2   <- the constructor and a method are the doors
   Product { id: 1, .. } written here -> E0451: field `id` of struct `Product` is private
```
<!-- /output -->

1. **"By default, a module called `crate` serves as the root module of our module tree."** Holds. An absolute path from `crate::` and a relative path from `main`'s own module name the same constructor and print the same label.
2. **Listing 6.9** prints what the book says it prints: a relative path with no prefix and one with `self::` both reach `math::multiply`.
3. **Statement 2 above.** `order` names `Product` and `Customer` through `super::`, and the bill is the one the file layout's `cargo run` printed at the top of the page.
4. **Private means this module and its descendants.** `category` reads `product`'s private `TAX_RATE` through `super::`; the root reads `category`'s public `Category::Books` through the re-export. Statement 5 above.
5. **The four narrower forms.** Each line was printed by a call made from where the keyword allows it: `pub(self)` from inside `product`, `pub(super)` and `pub(in crate::product)` from `product` reaching into `category`, `pub(crate)` from `main`. The last line is the refusal for the wrong direction, verified by hand: `product::private_by_two_spellings()` from `main` is `E0603`.
6. **"The `use` declaration brings items into scope … Some people might refer to this process as importing. However, this term suggests bringing something in from an external source, which isn't entirely accurate."** Holds, and the book's caution is the right one. The same `Product::new` runs through the `use` and through the full path; deleting the `use` would change the spelling and nothing else. [Bringing names in with `use`](../the_use_declaration/README.md) opens with the same test.
7. **"When an enum is made public, all of its variants automatically become public as well. Unlike structs, you cannot set the individual variants of an enum to be public independently."** Holds. `Category::Clothing` is reachable from the root with `pub` only on the enum, and `pub` on a variant is refused before the name is even resolved:

    ```text title="Abridged — real rustc output for pub_variant.rs"
    error[E0449]: visibility qualifiers are not permitted here
     --> pub_variant.rs:2:5
      |
    2 |     pub Electronics,
      |     ^^^ help: remove the qualifier
      |
      = note: enum variants and their fields always share the visibility of the enum they are in
    ```

8. **"In Rust, making a struct public does not make its fields public."** Holds. The constructor of Listing 6.39 and a method are the two doors; the struct literal from outside is the `E0451` keyed under [§6.4](#64-re-exports-and-private-fields). [A score is not a number](../../16_Structs/newtype_score/README.md) is what that door buys.

## Where each listing stops

### §6.1: a binary without `main`, and two binaries

Listing 6.2 adds an empty binary file. Under `src/bin/`, where Cargo finds it, the error the book quotes appears, with the crate named after the file:

<!-- cargo:pg_store_no_main -->
*Verified output of `cargo build -q -p pg_store_no_main`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
error[E0601]: `main` function not found in crate `my_binary`
 --> no_main/src/bin/my_binary.rs:1:67
  |
1 | // The file Listing 6.2 adds, before Listing 6.3 gives it a main.
  |                                                                  ^ consider adding a `main` function to `no_main/src/bin/my_binary.rs`

For more information about this error, try `rustc --explain E0601`.
error: could not compile `pg_store_no_main` (bin "my_binary") due to 1 previous error
```
<!-- /cargo -->

Listing 6.3 gives it a `main`, and the package has two binaries. The book's next transcript, its exit status, and the `--bin` way round it:

<!-- cargo:pg_store_two_binaries -->
*Verified output of `sh two_binaries.sh` — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
1. cargo run -p pg_store_two_binaries
error: `cargo run` could not determine which binary to run. Use the `--bin` option to specify a binary, or the `default-run` manifest key.
available binaries: my_binary, pg_store_two_binaries
exit status 101

2. cargo run -p pg_store_two_binaries --bin pg_store_two_binaries
Hello from main.rs
exit status 0
```
<!-- /cargo -->

`default-run` in the manifest, shown in the finished library's `Cargo.toml` above, is the third way, for the binary you run most; the book names only `--bin`.

### §6.2: the store, listing by listing

Listing 6.5 wraps Listing 6.4's four items in three modules, makes nothing `pub`, and names `Product` and `Customer` inside `Order` with no path:

<!-- file:demo/listing_6_5/src/lib.rs -->
```rust title="demo/listing_6_5/src/lib.rs"
//! Listing 6.5: the store wrapped in three modules, nothing `pub`, and
//! `Order` naming `Product` and `Customer` with no path.

mod product {
    struct Product {
        id: u64,
        name: String,
        price: f64,
        category: Category,
    }

    enum Category {
        Electronics,
        Clothing,
        Books,
    }

    impl Product {
        fn calculate_tax(&self) -> f64 {
            self.price * 0.1
        }

        fn product_price(&self) -> f64 {
            self.price + self.calculate_tax()
        }
    }
}

mod customer {
    struct Customer {
        id: u64,
        name: String,
        email: String,
    }
}

mod Order {
    struct Order {
        id: u64,
        product: Product,
        customer: Customer,
        quantity: u32,
    }

    impl Order {
        fn calculate_discount(&self) -> f64 {
            if self.quantity > 5 { 0.1 } else { 0.0 }
        }

        fn total_bill(&self) -> f64 {
            let discount = self.calculate_discount();
            let total_before_discount = self.product.product_price() * self.quantity as f64;
            total_before_discount - (total_before_discount * discount)
        }
    }
}
```
<!-- /file -->

<!-- cargo:pg_store_listing_6_5 -->
*Verified output of `cargo build -q -p pg_store_listing_6_5`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
error[E0425]: cannot find type `Product` in this scope
  --> listing_6_5/src/lib.rs:40:18
   |
40 |         product: Product,
   |                  ^^^^^^^ not found in this scope
   |
note: struct `crate::product::Product` exists but is inaccessible
  --> listing_6_5/src/lib.rs:5:5
   |
 5 |     struct Product {
   |     ^^^^^^^^^^^^^^ not accessible
help: consider importing this trait
   |
38 +     use std::iter::Product;
   |

error[E0425]: cannot find type `Customer` in this scope
  --> listing_6_5/src/lib.rs:41:19
   |
41 |         customer: Customer,
   |                   ^^^^^^^^ not found in this scope
   |
note: struct `crate::customer::Customer` exists but is inaccessible
  --> listing_6_5/src/lib.rs:30:5
   |
30 |     struct Customer {
   |     ^^^^^^^^^^^^^^^ not accessible

For more information about this error, try `rustc --explain E0425`.
error: could not compile `pg_store_listing_6_5` (lib) due to 2 previous errors
```
<!-- /cargo -->

The book's words, "cannot find type Product in this scope", are rustc's. The code is `E0425` on 1.98.0, where older compilers gave a missing type `E0412`; the note that the struct *exists but is inaccessible* is the real diagnosis, and the `use std::iter::Product` help is rustc reaching for the only public `Product` it knows.

Listing 6.8 writes the two paths in full, `crate::product::Product` and `crate::customer::Customer` ([the file](demo/listing_6_8/src/lib.rs)). The book reports "struct Product is private". rustc reports that twice, and a third error two listings early:

<!-- cargo:pg_store_listing_6_8 -->
*Verified output of `cargo build -q -p pg_store_listing_6_8`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
error[E0603]: struct `Product` is private
  --> listing_6_8/src/lib.rs:39:34
   |
39 |         product: crate::product::Product,
   |                                  ^^^^^^^ private struct
   |
note: the struct `Product` is defined here
  --> listing_6_8/src/lib.rs:4:5
   |
 4 |     struct Product {
   |     ^^^^^^^^^^^^^^

error[E0603]: struct `Customer` is private
  --> listing_6_8/src/lib.rs:40:36
   |
40 |         customer: crate::customer::Customer,
   |                                    ^^^^^^^^ private struct
   |
note: the struct `Customer` is defined here
  --> listing_6_8/src/lib.rs:29:5
   |
29 |     struct Customer {
   |     ^^^^^^^^^^^^^^^

error[E0624]: method `product_price` is private
  --> listing_6_8/src/lib.rs:51:54
   |
22 |         fn product_price(&self) -> f64 {
   |         ------------------------------ private method defined here
...
51 |             let total_before_discount = self.product.product_price() * self.quantity as f64;
   |                                                      ^^^^^^^^^^^^^ private method

Some errors have detailed explanations: E0603, E0624.
For more information about an error, try `rustc --explain E0603`.
error: could not compile `pg_store_listing_6_8` (lib) due to 3 previous errors
```
<!-- /cargo -->

Listing 6.10 adds `pub` to both structs ([the file](demo/listing_6_10/src/lib.rs)), and the third error is the only one left — the book's "product_price is private":

<!-- cargo:pg_store_listing_6_10 -->
*Verified output of `cargo build -q -p pg_store_listing_6_10`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
error[E0624]: method `product_price` is private
  --> listing_6_10/src/lib.rs:51:54
   |
22 |         fn product_price(&self) -> f64 {
   |         ------------------------------ private method defined here
...
51 |             let total_before_discount = self.product.product_price() * self.quantity as f64;
   |                                                      ^^^^^^^^^^^^^ private method

For more information about this error, try `rustc --explain E0624`.
error: could not compile `pg_store_listing_6_10` (lib) due to 1 previous error
```
<!-- /cargo -->

Listing 6.12 makes `product_price` public, and Listing 6.13 moves `Category` into a `category` submodule of `product` while the field still says `Category` ([the file](demo/listing_6_13/src/lib.rs)). "Cannot find type Category in this scope", as the book says, with the note that it exists one module down:

<!-- cargo:pg_store_listing_6_13 -->
*Verified output of `cargo build -q -p pg_store_listing_6_13`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
error[E0425]: cannot find type `Category` in this scope
  --> listing_6_13/src/lib.rs:9:19
   |
 9 |         category: Category,
   |                   ^^^^^^^^ not found in this scope
   |
note: enum `crate::product::category::Category` exists but is inaccessible
  --> listing_6_13/src/lib.rs:13:9
   |
13 |         enum Category {
   |         ^^^^^^^^^^^^^ not accessible

For more information about this error, try `rustc --explain E0425`.
error: could not compile `pg_store_listing_6_13` (lib) due to 1 previous error
```
<!-- /cargo -->

Listing 6.14 writes `category::Category` ([the file](demo/listing_6_14/src/lib.rs)). The book: "enum Category is private".

<!-- cargo:pg_store_listing_6_14 -->
*Verified output of `cargo build -q -p pg_store_listing_6_14`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
error[E0603]: enum `Category` is private
  --> listing_6_14/src/lib.rs:8:29
   |
 8 |         category: category::Category,
   |                             ^^^^^^^^ private enum
   |
note: the enum `Category` is defined here
  --> listing_6_14/src/lib.rs:12:9
   |
12 |         enum Category {
   |         ^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
error: could not compile `pg_store_listing_6_14` (lib) due to 1 previous error
```
<!-- /cargo -->

Listing 6.15 makes the enum public, and Listings 6.16 and 6.17 add the `use` lines ([the file](demo/listing_6_17/src/lib.rs)). This is the first version of the store that compiles, and the first that can warn. Nothing at the crate root is `pub`, so every item is dead code from the outside, and `mod Order` draws the lint that Listing 6.20 has silently fixed:

<!-- cargo:pg_store_listing_6_17 -->
*Verified output of `cargo build -q -p pg_store_listing_6_17 --message-format short` — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
listing_6_17/src/lib.rs:7:16: warning: struct `Product` is never constructed
listing_6_17/src/lib.rs:23:12: warning: methods `calculate_tax` and `product_price` are never used
listing_6_17/src/lib.rs:15:18: warning: enum `Category` is never used
listing_6_17/src/lib.rs:34:16: warning: struct `Customer` is never constructed
listing_6_17/src/lib.rs:45:12: warning: struct `Order` is never constructed
listing_6_17/src/lib.rs:53:12: warning: methods `calculate_discount` and `total_bill` are never used
listing_6_17/src/lib.rs:41:5: warning: module `Order` should have a snake case name: help: convert the identifier to snake case (notice the capitalization): `order`
```
<!-- /cargo -->

Listing 6.18 puts `use crate::product::category;` at the crate root ([the file](demo/listing_6_18/src/lib.rs)). "Public items of a private submodule can only be accessed by the parent module", the book says, and the key agrees:

<!-- cargo:pg_store_listing_6_18 -->
*Verified output of `cargo build -q -p pg_store_listing_6_18`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
error[E0603]: module `category` is private
  --> listing_6_18/src/lib.rs:4:21
   |
 4 | use crate::product::category;
   |                     ^^^^^^^^ private module
   |
note: the module `category` is defined here
  --> listing_6_18/src/lib.rs:16:5
   |
16 |     mod category {
   |     ^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
error: could not compile `pg_store_listing_6_18` (lib) due to 1 previous error
```
<!-- /cargo -->

### §6.3: files, and the tool that draws the tree

Listing 6.26 puts `product.rs` and `category.rs` both directly under `src/`, and Listing 6.28's `product.rs` declares `mod category;`:

<!-- file:demo/listing_6_28/src/product.rs -->
```rust title="demo/listing_6_28/src/product.rs"
pub struct Product {
    id: u64,
    name: String,
    price: f64,
    category: category::Category,
}

mod category;

impl Product {
    fn calculate_tax(&self) -> f64 {
        self.price * 0.1
    }

    pub fn product_price(&self) -> f64 {
        self.price + self.calculate_tax()
    }
}
```
<!-- /file -->

<!-- cargo:pg_store_listing_6_28 -->
*Verified output of `cargo build -q -p pg_store_listing_6_28`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
error[E0583]: file not found for module `category`
 --> listing_6_28/src/product.rs:8:1
  |
8 | mod category;
  | ^^^^^^^^^^^^^
  |
  = help: to create the module `category`, create file "listing_6_28/src/product/category.rs" or "listing_6_28/src/product/category/mod.rs"
  = note: if there is a `mod category` elsewhere in the crate already, import it with `use crate::...` instead

For more information about this error, try `rustc --explain E0583`.
error: could not compile `pg_store_listing_6_28` (lib) due to 1 previous error
```
<!-- /cargo -->

Statement 4 above: the wording is rust-analyzer's, and rustc's help names both files it would accept. The rule underneath is the one [One module per file](../one_module_per_file/README.md) states — a submodule's file lives in a folder named after its parent, `product/category.rs` beside `product.rs`, or `product/category/mod.rs` in the older layout. Listing 6.29 does exactly that and is the layout of the finished library above.

Section 6.3.2's second method uses `product/mod.rs` instead of `product.rs`. Both work, and both at once do not:

<!-- cargo:pg_store_both_layouts -->
*Verified output of `cargo build -q -p pg_store_both_layouts`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
error[E0761]: file for module `product` found at both "both_layouts/src/product.rs" and "both_layouts/src/product/mod.rs"
 --> both_layouts/src/lib.rs:4:1
  |
4 | mod product;
  | ^^^^^^^^^^^^
  |
  = help: delete or rename one of them to remove the ambiguity

For more information about this error, try `rustc --explain E0761`.
error: could not compile `pg_store_both_layouts` (lib) due to 1 previous error
```
<!-- /cargo -->

**cargo-modules, Listings 6.19 and 6.20.** `cargo install cargo-modules` in a `rust:slim` container failed on the first try, because the newest releases of two of its dependencies disagree about their Unicode version, and succeeded with `--locked`, which takes the crate's own lockfile. Its output cannot be an answer key here, since CI has no cargo-modules, so the three transcripts below are hand-verified and dated 2026-09-23. Without `--lib` or `--bin`, on a package with three targets, the tool refuses and lists them — Listing 6.19, as an error:

```text title="cargo modules structure -p pg_store_shop, cargo-modules 0.27.0 on rustc 1.98.0 in Docker — hand-verified"
Error: Multiple targets present in package,
please explicitly select one via --lib or --bin flag.

Targets present in package:
- pg_store_shop (--lib)
- my_binary (--bin my_binary)
- pg_store_shop (--bin pg_store_shop)
```

On the Listing 6.17 crate, the tree of Listing 6.20, with the module name the source actually has:

```text title="cargo modules structure --lib -p pg_store_listing_6_17, cargo-modules 0.27.0 — hand-verified"
crate pg_store_listing_6_17
├── mod Order: pub(crate)
│   └── struct Order: pub(self)
│       ├── fn calculate_discount: pub(self)
│       └── fn total_bill: pub(self)
├── mod customer: pub(crate)
│   └── struct Customer: pub
└── mod product: pub(crate)
    ├── struct Product: pub
    │   ├── fn calculate_tax: pub(self)
    │   └── fn product_price: pub
    └── mod category: pub(self)
        └── enum Category: pub
```

And on the finished library:

```text title="cargo modules structure --lib -p pg_store_shop, cargo-modules 0.27.0 — hand-verified"
crate pg_store_shop
├── mod customer: pub(crate)
│   └── struct Customer: pub
│       ├── fn label: pub
│       └── fn new: pub
├── mod order: pub(crate)
│   └── struct Order: pub
│       ├── fn bill: pub
│       ├── fn calculate_discount: pub(self)
│       ├── fn new: pub
│       └── fn total_bill: pub
└── mod product: pub(crate)
    ├── struct Product: pub
    │   ├── fn calculate_tax: pub(self)
    │   ├── fn label: pub
    │   ├── fn new: pub
    │   └── fn product_price: pub
    └── mod category: pub(self)
        └── enum Category: pub
```

Read the labels as reach, not as source. None of the three modules is written `pub(crate)`; a private module at the crate root can be reached from every module of the crate, and that is what the tool prints. `mod category: pub(self)` is a private module one level down, reachable from `product` and its descendants only. The `pub use` lines that make `Category` and `Product` public names are not drawn at all, which is the one thing a reader of the tree has to know: the public surface of this crate is four re-exports, and the tree shows none of them.

### §6.4: re-exports and private fields

Listing 6.33 keeps the modules private and re-exports two names, one of them through `product`'s private submodule:

<!-- file:demo/listing_6_33/src/lib.rs -->
```rust title="demo/listing_6_33/src/lib.rs"
//! Listing 6.33: the re-export path runs through `category`, which is private
//! to `product`. The `order` module is left out; it plays no part here.

mod customer;
mod product;

pub use customer::Customer;
pub use product::{category::Category, Product};
```
<!-- /file -->

<!-- cargo:pg_store_listing_6_33 -->
*Verified output of `cargo build -q -p pg_store_listing_6_33`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
error[E0603]: module `category` is private
 --> listing_6_33/src/lib.rs:8:19
  |
8 | pub use product::{category::Category, Product};
  |                   ^^^^^^^^  -------- enum `Category` is not publicly re-exported
  |                   |
  |                   private module
  |
note: the module `category` is defined here
 --> listing_6_33/src/product.rs:1:1
  |
1 | mod category;
  | ^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
error: could not compile `pg_store_listing_6_33` (lib) due to 1 previous error
```
<!-- /cargo -->

Listing 6.34 adds `pub use category::Category;` inside `product` and keeps the root line as it was ([the file](demo/listing_6_34/src/product.rs)). The path at the root still runs through `category`, so the error is the same, and rustc now points at the re-export that would work:

<!-- cargo:pg_store_listing_6_34 -->
*Verified output of `cargo build -q -p pg_store_listing_6_34`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
error[E0603]: module `category` is private
 --> listing_6_34/src/lib.rs:8:19
  |
8 | pub use product::{category::Category, Product};
  |                   ^^^^^^^^ private module
  |
help: consider importing this enum through its public re-export instead:
      product::Category
 --> listing_6_34/src/lib.rs:8:19
  |
8 | pub use product::{category::Category, Product};
  |                   ^^^^^^^^^^^^^^^^^^
note: the module `category` is defined here
 --> listing_6_34/src/product.rs:1:1
  |
1 | mod category;
  | ^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
error: could not compile `pg_store_listing_6_34` (lib) due to 1 previous error
```
<!-- /cargo -->

Statement 3 above: this is not a 1.80 change. Listing 6.35's `pub use crate::product::Category;` builds, and so does the finished library's `pub use product::{Category, Product};`, because both paths go through the re-export.

Listing 6.36 builds a `Product` in `main.rs` with a struct literal. The library is Listing 6.35 as printed, and the `use` line is the book's, `Customer` included:

<!-- file:demo/listing_6_36/src/main.rs -->
```rust title="demo/listing_6_36/src/main.rs"
//! Listing 6.36: a struct literal for a public struct whose fields are not.
//! The `use` line is the book's, `Customer` included and unused.

use pg_store_listing_6_36::{Category, Customer, Product};

fn main() {
    let product = Product {
        id: 1,
        name: String::from("Laptop"),
        price: 799.99,
        category: Category::Electronics,
    };
}
```
<!-- /file -->

<!-- cargo:pg_store_listing_6_36 -->
*Verified output of `cargo build -q -p pg_store_listing_6_36`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
warning: unused import: `Customer`
 --> listing_6_36/src/main.rs:4:39
  |
4 | use pg_store_listing_6_36::{Category, Customer, Product};
  |                                       ^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `product`
 --> listing_6_36/src/main.rs:7:9
  |
7 |     let product = Product {
  |         ^^^^^^^ help: if this is intentional, prefix it with an underscore: `_product`
  |
  = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

error[E0451]: fields `id`, `name`, `price` and `category` of struct `pg_store_listing_6_36::Product` are private
  --> listing_6_36/src/main.rs:8:9
   |
 7 |     let product = Product {
   |                   ------- in this type
 8 |         id: 1,
   |         ^^ private field
 9 |         name: String::from("Laptop"),
   |         ^^^^ private field
10 |         price: 799.99,
   |         ^^^^^ private field
11 |         category: Category::Electronics,
   |         ^^^^^^^^ private field

For more information about this error, try `rustc --explain E0451`.
error: could not compile `pg_store_listing_6_36` (bin "pg_store_listing_6_36") due to 1 previous error; 2 warnings emitted
```
<!-- /cargo -->

One `E0451` naming four fields, where the book marks four errors, and two warnings the listing earns on its own. Listing 6.39's constructor is the answer, with field init shorthand. Listing 6.40 gives `Customer` the same constructor without the `pub`, and Listing 6.41 calls it:

<!-- file:demo/listing_6_41/src/customer.rs -->
```rust title="demo/listing_6_41/src/customer.rs"
pub struct Customer {
    id: u64,
    name: String,
    email: String,
}

impl Customer {
    fn new(id: u64, name: String, email: String) -> Self {
        Self { id, name, email }
    }
}
```
<!-- /file -->

<!-- file:demo/listing_6_41/src/main.rs -->
```rust title="demo/listing_6_41/src/main.rs"
//! Listing 6.41, with the package's real name where the book wrote
//! `my_package_book`. Listing 6.40's `Customer::new` has no `pub`.

use pg_store_listing_6_41::{Category, Customer, Product};

fn main() {
    let product = Product::new(1, String::from("Laptop"), 799.99, Category::Electronics);
    let customer = Customer::new(1, String::from("Alice"), String::from("alice@example.com"));
}
```
<!-- /file -->

<!-- cargo:pg_store_listing_6_41 -->
*Verified output of `cargo build -q -p pg_store_listing_6_41`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
error[E0624]: associated function `new` is private
 --> listing_6_41/src/main.rs:8:30
  |
8 |     let customer = Customer::new(1, String::from("Alice"), String::from("alice@example.com"));
  |                              ^^^ private associated function
  |
 ::: listing_6_41/src/customer.rs:8:5
  |
8 |     fn new(id: u64, name: String, email: String) -> Self {
  |     ---------------------------------------------------- private associated function defined here

For more information about this error, try `rustc --explain E0624`.
error: could not compile `pg_store_listing_6_41` (bin "pg_store_listing_6_41") due to 1 previous error
```
<!-- /cargo -->

`E0624`, *"associated function `new` is private"* — an associated function has no `self`, so rustc's noun for it differs from the "method" it used for `product_price`. The finished library's `customer.rs` has the `pub` the listing lacks.

## Practice

**Five `pub` decisions.** Take the store as one file — `shop` holding `product` (with a `category` submodule), `customer` and `order`, and a `main` outside `shop` that builds a `Product` through `new`, names `Category::Books`, builds a `Customer` and an `Order`, and prints the bill. Start with no `pub` anywhere. Add the fewest keywords that make `main` compile while keeping all three modules private, every field private, and `calculate_tax` and `calculate_discount` private.

Before you compile, write down for each keyword you add the error rustc would give without it, with its code — there are five distinct decisions and four distinct codes among them. Then check whether `pub mod category` or `pub use category::Category` inside `product` is the smaller change, and why the tree ends up with no `pub` on any `mod` at all.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:pg_mod_kata -->
*[`pg_mod_kata.rs`](examples/pg_mod_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: five `pub` decisions for the store, and what each one
//! refuses when it is missing.
//!
//!   rustc --edition 2024 pg_mod_kata.rs -o /tmp/pgk && /tmp/pgk

mod shop {
    mod product {
        mod category {
            #[derive(Debug)]
            pub enum Category {
                Electronics,
                Clothing,
                Books,
            }
        }

        pub use category::Category; // decision 5: through the private module

        pub struct Product { // decision 2: the type, not the fields
            id: u64,
            name: String,
            price: f64,
            category: Category,
        }

        impl Product {
            pub fn new(id: u64, name: String, price: f64, category: Category) -> Self { // decision 3
                Self { id, name, price, category }
            }

            fn calculate_tax(&self) -> f64 {
                self.price * 0.1
            }

            pub fn product_price(&self) -> f64 { // decision 4
                self.price + self.calculate_tax()
            }

            pub fn label(&self) -> String {
                format!("{} ({:?}), product #{}", self.name, self.category, self.id)
            }
        }
    }

    mod customer {
        pub struct Customer {
            id: u64,
            name: String,
        }

        impl Customer {
            pub fn new(id: u64, name: String) -> Self {
                Self { id, name }
            }

            pub fn label(&self) -> String {
                format!("{}, customer #{}", self.name, self.id)
            }
        }
    }

    mod order {
        use super::customer::Customer;
        use super::product::Product;

        pub struct Order {
            product: Product,
            customer: Customer,
            quantity: u32,
        }

        impl Order {
            pub fn new(product: Product, customer: Customer, quantity: u32) -> Self {
                Self { product, customer, quantity }
            }

            fn calculate_discount(&self) -> f64 {
                if self.quantity > 5 { 0.1 } else { 0.0 }
            }

            pub fn total_bill(&self) -> f64 {
                let before = self.product.product_price() * self.quantity as f64;
                before - before * self.calculate_discount()
            }

            pub fn bill(&self) -> String {
                format!("{} x {} for {} -> {:.2}", self.quantity, self.product.label(), self.customer.label(), self.total_bill())
            }
        }
    }

    // decision 1: re-export, and keep the three modules private
    pub use customer::Customer;
    pub use order::Order;
    pub use product::{Category, Product};
}

use shop::{Category, Customer, Order, Product};

fn main() {
    println!("1. The store, used only through what `shop` re-exports");
    let product = Product::new(7, String::from("Novel"), 20.0, Category::Books);
    let customer = Customer::new(3, String::from("Alice"));
    let order = Order::new(product, customer, 6);
    println!("   {}", order.bill());

    println!();
    println!("2. The five decisions, and what rustc says when each is missing");
    println!("   1. `pub use` at shop for Product, Category, Customer, Order; the modules stay private");
    println!("      missing: `use shop::Product` -> E0432 unresolved import, or through");
    println!("      `shop::product::Product` -> E0603: module `product` is private");
    println!("   2. `pub struct Product` (and Customer, Order); the fields stay private");
    println!("      missing: E0603: struct `Product` is private");
    println!("      fields written from main as a literal: E0451: field `id` of struct `Product` is private");
    println!("   3. `pub fn new` on each type: the only way in, since the fields are private");
    println!("      missing: E0624: associated function `new` is private");
    println!("   4. `pub fn product_price`, `pub fn total_bill`, `pub fn label`; calculate_tax and");
    println!("      calculate_discount stay private, since only their own module calls them");
    println!("      missing: E0624: method `product_price` is private");
    println!("   5. `pub use category::Category` inside product, so `Category` is reachable without");
    println!("      `pub mod category`; `pub enum` already published all three variants");
    println!("      missing, with `pub use product::{{category::Category, Product}}` at shop:");
    println!("      E0603: module `category` is private");

    println!();
    println!("3. What stayed private");
    println!("   the three modules, every field, calculate_tax, calculate_discount, and the");
    println!("   category module: {} pub keywords in the tree, none on a mod", 15);
}
```
<!-- /source -->

<!-- output:pg_mod_kata -->
*Verified output of [`pg_mod_kata.rs`](examples/pg_mod_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The store, used only through what `shop` re-exports
   6 x Novel (Books), product #7 for Alice, customer #3 -> 118.80

2. The five decisions, and what rustc says when each is missing
   1. `pub use` at shop for Product, Category, Customer, Order; the modules stay private
      missing: `use shop::Product` -> E0432 unresolved import, or through
      `shop::product::Product` -> E0603: module `product` is private
   2. `pub struct Product` (and Customer, Order); the fields stay private
      missing: E0603: struct `Product` is private
      fields written from main as a literal: E0451: field `id` of struct `Product` is private
   3. `pub fn new` on each type: the only way in, since the fields are private
      missing: E0624: associated function `new` is private
   4. `pub fn product_price`, `pub fn total_bill`, `pub fn label`; calculate_tax and
      calculate_discount stay private, since only their own module calls them
      missing: E0624: method `product_price` is private
   5. `pub use category::Category` inside product, so `Category` is reachable without
      `pub mod category`; `pub enum` already published all three variants
      missing, with `pub use product::{category::Category, Product}` at shop:
      E0603: module `category` is private

3. What stayed private
   the three modules, every field, calculate_tax, calculate_discount, and the
   category module: 15 pub keywords in the tree, none on a mod
```
<!-- /output -->

</details>

## If you are coming from another language

- **Python.** A package is a directory and a module is a file, and the chapter's sentence that Rust's modules "are not mapped to the file system, unlike Python or JavaScript" is the right thing to carry over: `mod product;` is a declaration the parent makes, and a file nobody declares is not part of the program, where Python finds `product.py` by searching `sys.path` at run time. `__init__.py` is the nearest thing to `lib.rs` and to `product.rs`: the file that stands for the package and lists what it exposes, with `from .category import Category` doing the job of `pub use category::Category` — except that Python's re-export is a convenience and Rust's is the only way out of a private module. Python has no `E0603`: a leading underscore is advice, and `from product.category import Category` works whatever the author intended. The level above, package on this page, is a *distribution* in Python, the thing `pyproject.toml` names — [`pyproject.toml` ↗](https://masiarek.github.io/python-learning-library/02_Projects_and_Environments/pyproject_toml/) in the Python library is the `Cargo.toml` of that world.
- **Java.** A package is a directory, one public class per file, and the compiler enforces the mapping Rust leaves to `mod`. Java's `public` is unconditional; `pub` is relative to how visible the enclosing module already is, which is why Listing 6.31's `pub mod` and Listing 6.32's `pub use` are two different answers to one error. Java has no re-export: a class lives where it is declared, and an `import` in one file helps no other file. And package-private, Java's default, is flat, where a Rust child module sees its parent's private items and a Java subpackage sees nothing.
- **C.** `#include "category.h"` splices text where it stands, so a missing file is the preprocessor's `fatal error: 'category.h' file not found`, and a header included twice is compiled twice unless it guards itself. `mod category;` names a module exactly once, from its parent, and rustc tells you the two files it would accept. What C has instead of a module tree is the build: the list of `.c` files in a Makefile ([Makefiles ↗](https://masiarek.github.io/c-learning-library/01_Building/makefiles/) in the C library) is the closest thing to Cargo's target list, and `static` on a function is the closest thing to no `pub`.

## See also

- [Packages and crates](../packages_and_crates/README.md) — the two levels above the module tree, and the book's `bin/` folder measured
- [Modules and visibility](../modules_and_visibility/README.md) — private by default, the four `pub` forms, and privacy running one way
- [Bringing names in with `use`](../the_use_declaration/README.md) — the shortcut the chapter is careful not to call an import
- [One module per file](../one_module_per_file/README.md) — `mod name;` and the file it looks for, in both layouts
- [A crate prelude](../a_crate_prelude/README.md) — a module made of `pub use` lines, the pattern §6.4.1 arrives at
- [What an attribute is](../what_an_attribute_is/README.md) — the `#![allow(dead_code)]` in two demo packages, and the lint names in the keys
- [Running a scratch program](../../15_First_Programs/rustc_without_cargo/README.md) — `rustc` alone, `cargo new`, and `src/bin/`
- [From one `.rs` file to a Cargo project](../../05_Tooling/from_rustc_to_cargo/README.md) — what `cargo init` and `cargo run -v` do with the files this page names
- [Workspaces](../../05_Tooling/workspaces/README.md) — the level above packages, which is what `demo/` is
- [A score is not a number](../../16_Structs/newtype_score/README.md) — the private fields of block 8 doing their job
- [What an enum is](../../13_Enums/what_an_enum_is/README.md) — the variants of block 7
- [*Rust: The Practical Guide*, chapter 6 exercises, run](../../10_Resources/rust_the_practical_guide/ch6_katas/README.md) — the chapter's five exercises, each compiled as given and as solved
- [*Rust: The Practical Guide*, run](../../10_Resources/rust_the_practical_guide/README.md) — the book's home in this library
- [`and`, `or` and a first program's explanation, run](../../15_First_Programs/and_or_claims_checked/README.md) — the same kind of check on another book's chapter
- [The Cargo Book: Cargo targets ↗](https://doc.rust-lang.org/cargo/reference/cargo-targets.html) — `src/bin/`, `default-run` and target auto-discovery
- [The Rust Reference: visibility and privacy ↗](https://doc.rust-lang.org/reference/visibility-and-privacy.html) — the rules behind every `E0603` above
- [cargo-modules ↗](https://github.com/regexident/cargo-modules) — the tool behind Listings 6.19 and 6.20

## Po polsku

Rozdział 6 książki *Rust: The Practical Guide* (Nouman Azam, Rheinwerk 2025) buduje bibliotekę sklepu internetowego krok po kroku, od jednego pliku bez modułów po układ z osobnym plikiem na każdy moduł (*one module per file*) i re-eksportami (*re-exports*) `pub use` na szczycie drzewa. Błędy, na których książka zatrzymuje kolejne listingi, są prawdziwe: rustc 1.98.0 wypisuje każdy z nich, najczęściej dokładnie tymi słowami, które książka cytuje — „cannot find type Product in this scope” (`E0425`), „struct Product is private” i „module category is private” (`E0603`), „product_price is private” (`E0624`), prywatne pola przy literale struktury (`E0451`). Każdy z tych listingów jest tu osobnym pakietem w katalogu `demo/`, zadeklarowanym jako uruchomienie, które ma się nie powieść, więc każdy zapis błędu jest kluczem odpowiedzi, a nie wklejką.

Sześć zdań wokół tych błędów nie wytrzymuje sprawdzenia. Katalog `bin/` obok `src/` z Listingu 6.2 nie jest dla Cargo żadnym celem (*target*): plik w nim nie jest kompilowany, a `cargo run` drukuje `Hello, world!` — błąd o brakującym `main` i komunikat „could not determine which binary to run” pojawiają się dopiero w `src/bin/`. Ścieżek bezwzględnych (*absolute paths*) z `crate::` nie trzeba używać, bo `super::product::Product` działa równie dobrze. Ramka obwiniająca Rusta 1.80 myli się w kierunku: Listing 6.34 nie kompiluje się także na rustc 1.79.0, bo ścieżka `product::category::Category` przechodzi przez prywatny moduł, a naprawia to re-eksport, nie przedrostek `crate::`. Słowa „unresolved module” pochodzą z rust-analyzera, nie z rustc, który mówi `E0583` „file not found for module” i podaje **dwie** dopuszczalne ścieżki pliku. Rodzic (*parent module*) sięga do publicznych elementów dziecka bez przeszkód — niedostępne są tylko prywatne. A `pub(self)` to nie „tylko ten moduł”, lecz ten moduł **i jego potomkowie**, bo tak w Ruście brzmi słowo „prywatne”.

**Szukaj po polsku:** moduły w Ruście · pakiet, skrzynia, moduł · re-eksport `pub use` · widoczność `pub(crate)` `pub(self)` `pub(super)` · `rust E0603 module is private` · `rust E0583 file not found for module` · `rust E0451 private field` · `cargo run could not determine which binary` · `rust default-run`
