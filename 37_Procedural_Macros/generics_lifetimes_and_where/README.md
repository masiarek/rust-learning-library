# Generics, lifetimes and `where`

**Level:** 301 · deep dive

**One line:** A derive on `struct Wrapper<'a, T: Clone, const N: usize> where T: PartialEq` has to repeat every parameter, bound and `where` clause in its `impl`. `Generics::split_for_impl` hands back the three pieces, `make_where_clause` adds a bound of your own — and the usual bound, `T: Trait` for every type parameter, is the one `#[derive(Debug)]` adds too, and it is wrong for a `T` that only sits in `PhantomData<T>`.

## A derive that repeats the generics

One crate, one `Debug` derive written four ways. The `fmt` body is the same in all four; only the `impl` line and its bounds differ:

<!-- file:demo/debug_generics/src/lib.rs -->
```rust title="demo/debug_generics/src/lib.rs"
//! One `Debug` derive, written four ways. The body of `fmt` is the same in
//! all four; they differ only in the `impl` line and its bounds, which is
//! where a derive on a generic type succeeds or fails.

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Field, Fields, Generics, parse_macro_input, parse_quote};

/// Right: repeats the generics, and bounds every type parameter by `Debug`.
#[proc_macro_derive(DebugBoundingParams)]
pub fn debug_bounding_params(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    bound_each_type_param(&mut input.generics);
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (ident, body) = (&input.ident, body(&input));
    quote!(impl #impl_generics ::core::fmt::Debug for #ident #ty_generics #where_clause #body)
        .into()
}

/// Right, differently: repeats the generics, and bounds every field's type.
#[proc_macro_derive(DebugBoundingFields)]
pub fn debug_bounding_fields(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    bound_each_field_type(&mut input);
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (ident, body) = (&input.ident, body(&input));
    quote!(impl #impl_generics ::core::fmt::Debug for #ident #ty_generics #where_clause #body)
        .into()
}

/// Wrong: `impl Debug for Wrapper`, as if `Wrapper` took no parameters.
#[proc_macro_derive(DebugIgnoringGenerics)]
pub fn debug_ignoring_generics(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let (ident, body) = (&input.ident, body(&input));
    quote!(impl ::core::fmt::Debug for #ident #body).into()
}

/// Wrong: `DebugBoundingParams` with `#where_clause` left out of the `impl`.
#[proc_macro_derive(DebugForgettingWhere)]
pub fn debug_forgetting_where(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    bound_each_type_param(&mut input.generics);
    let (impl_generics, ty_generics, _where_clause) = input.generics.split_for_impl();
    let (ident, body) = (&input.ident, body(&input));
    quote!(impl #impl_generics ::core::fmt::Debug for #ident #ty_generics #body).into()
}

/// `T: Debug` for every type parameter `T`. Lifetimes and consts need none.
fn bound_each_type_param(generics: &mut Generics) {
    // Collected first: `type_params` borrows `generics`, `make_where_clause` changes it.
    let params: Vec<_> = generics.type_params().map(|p| p.ident.clone()).collect();
    for param in params {
        let predicate = parse_quote!(#param: ::core::fmt::Debug);
        let where_clause = generics.make_where_clause(); // creates one if there is none
        where_clause.predicates.push(predicate);
    }
}

/// `FieldType: Debug` for every field, whatever the field's type mentions.
fn bound_each_field_type(input: &mut DeriveInput) {
    let types: Vec<_> = fields(input).map(|field| field.ty.clone()).collect();
    for ty in types {
        let predicate = parse_quote!(#ty: ::core::fmt::Debug);
        let where_clause = input.generics.make_where_clause();
        where_clause.predicates.push(predicate);
    }
}

/// `{ fn fmt(..) { f.debug_struct("Name").field("a", &self.a)...finish() } }`
fn body(input: &DeriveInput) -> proc_macro2::TokenStream {
    let name = input.ident.to_string();
    let members: Vec<_> = fields(input).filter_map(|f| f.ident.as_ref()).collect();
    let labels = members.iter().map(|member| member.to_string());
    quote!({
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_struct(#name) #(.field(#labels, &self.#members))* .finish()
        }
    })
}

/// The named fields. Other shapes are the subject of every_struct_and_enum_shape.
fn fields(input: &DeriveInput) -> impl Iterator<Item = &Field> {
    let Data::Struct(data) = &input.data else {
        panic!("this derive supports structs only");
    };
    let Fields::Named(named) = &data.fields else {
        panic!("this derive supports named fields only");
    };
    named.named.iter()
}
```
<!-- /file -->

`DebugBoundingParams` on a struct with one of each kind of generic parameter, and a `where` clause:

<!-- file:demo/generics_app/src/main.rs -->
```rust title="demo/generics_app/src/main.rs"
use debug_generics::DebugBoundingParams;

/// A lifetime, a bounded type parameter, a const parameter and a `where` clause.
#[derive(DebugBoundingParams)]
struct Wrapper<'a, T: Clone, const N: usize>
where
    T: PartialEq,
{
    label: &'a str,
    items: [T; N],
}

fn main() {
    let evens = Wrapper {
        label: "evens",
        items: [2, 4, 6],
    };
    let words = Wrapper {
        label: "words",
        items: ["foo", "bar"],
    };
    println!("{evens:?}");
    println!("{words:?}");
}
```
<!-- /file -->

<!-- cargo:generics_split_for_impl -->
*Verified output of `cargo run -q -p generics_app` — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
Wrapper { label: "evens", items: [2, 4, 6] }
Wrapper { label: "words", items: ["foo", "bar"] }
```
<!-- /cargo -->

What the derive wrote. `cargo expand -p generics_app`, from the separately installed `cargo-expand`, shows the same; this key was recorded with the compiler command that tool wraps, named in the caption:

<!-- cargo:generics_split_for_impl_expanded -->
*Verified output of `cargo rustc -q -p generics_app --profile=check -- -Zunpretty=expanded` — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
#![feature(prelude_import)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use debug_generics::DebugBoundingParams;

/// A lifetime, a bounded type parameter, a const parameter and a `where` clause.
struct Wrapper<'a, T: Clone, const N : usize> where T: PartialEq {
    label: &'a str,
    items: [T; N],
}
impl<'a, T: Clone, const N : usize> ::core::fmt::Debug for Wrapper<'a, T, N>
    where T: PartialEq, T: ::core::fmt::Debug {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Wrapper").field("label",
                    &self.label).field("items", &self.items).finish()
    }
}

fn main() {
    let evens = Wrapper { label: "evens", items: [2, 4, 6] };
    let words = Wrapper { label: "words", items: ["foo", "bar"] };
    { ::std::io::_print(format_args!("{0:?}\n", evens)); };
    { ::std::io::_print(format_args!("{0:?}\n", words)); };
}
```
<!-- /cargo -->

The `impl` line, read left to right:

- `impl<'a, T: Clone, const N : usize>` — every parameter, declared again, with its inline bounds. The lifetime and the const parameter came along without a line of code for either.
- `for Wrapper<'a, T, N>` — the same parameters used as arguments: names only, no bounds.
- `where T: PartialEq, T: ::core::fmt::Debug` — the struct's own `where` clause, then the bound `bound_each_type_param` pushed onto it. `T` needs `Debug` because `[T; N]` is only `Debug` when `T` is ([`core/src/array/mod.rs` at 1.98.0 ↗](https://github.com/rust-lang/rust/blob/1.98.0/library/core/src/array/mod.rs#L355)). `'a` and `N` need nothing.

## The three pieces

`split_for_impl` does not build an `impl`; it returns three values that print as those three fragments. `syn` and `quote` run on [`proc-macro2`](../proc_macro2_makes_it_testable/README.md), which works outside the compiler, so an ordinary program can print them. The struct is `generics_app`'s, with defaults added:

<!-- file:demo/generic_pieces/src/main.rs -->
```rust title="demo/generic_pieces/src/main.rs"
//! `split_for_impl` outside a macro. `syn` and `quote` run on `proc-macro2`,
//! which works in an ordinary program, so the three pieces can be printed.

use quote::quote;
use syn::{DeriveInput, parse_quote};

fn main() {
    // The struct from generics_app, plus a default for each type and const parameter.
    let input: DeriveInput = parse_quote! {
        struct Wrapper<'a, T: Clone = String, const N: usize = 3>
        where
            T: PartialEq,
        {
            label: &'a str,
            items: [T; N],
        }
    };
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    println!("impl_generics  {}", quote!(#impl_generics));
    println!("ty_generics    {}", quote!(#ty_generics));
    println!("where_clause   {}", quote!(#where_clause));
}
```
<!-- /file -->

<!-- cargo:generics_three_pieces -->
*Verified output of `cargo run -q -p generic_pieces` — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
impl_generics  < 'a , T : Clone , const N : usize >
ty_generics    < 'a , T , N >
where_clause   where T : PartialEq ,
```
<!-- /cargo -->

| Piece | Goes | Keeps | Drops |
|---|---|---|---|
| `impl_generics` | after `impl` | every parameter, its inline bounds | defaults: `= String` and `= 3` are gone |
| `ty_generics` | after the type's name | the parameter names | bounds and defaults |
| `where_clause` | after the type, before `{` | the struct's `where` clause, as an `Option` | — |

Each is only as current as the `Generics` it was split from, so add bounds first and split after. The spacing (`< 'a , T : Clone`) is how `proc-macro2` prints tokens outside the compiler; the compiler reads the tokens, not the spaces.

If your macro reads a parameter's default rather than just passing it through, syn 3 moved it. syn 2's `TypeParam` has `eq_token: Option<Token![=]>` and `default: Option<Type>`; syn 3's has one field, `default: Option<(Token![=], Type)>`, and `ConstParam` changed the same way. `split_for_impl`, `make_where_clause`, `type_params`, and the way the three pieces print, are unchanged between syn 2.0.119 and 3.0.6.

## Refusals: leaving a piece out

### No generics at all

`DebugIgnoringGenerics` writes `impl Debug for Wrapper`:

<!-- file:demo/ignores_generics/src/main.rs -->
```rust title="demo/ignores_generics/src/main.rs"
use debug_generics::DebugIgnoringGenerics;

#[derive(DebugIgnoringGenerics)]
struct Wrapper<'a, T: Clone, const N: usize>
where
    T: PartialEq,
{
    label: &'a str,
    items: [T; N],
}

fn main() {
    let evens = Wrapper {
        label: "evens",
        items: [2, 4, 6],
    };
    println!("{evens:?}");
}
```
<!-- /file -->

<!-- cargo:generics_ignored -->
*Verified output of `cargo build -q -p ignores_generics`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
error[E0726]: implicit elided lifetime not allowed here
 --> ignores_generics/src/main.rs:4:8
  |
4 | struct Wrapper<'a, T: Clone, const N: usize>
  |        ^^^^^^^ expected lifetime parameter
  |
help: indicate the anonymous lifetime
  |
4 | struct Wrapper<'_><'a, T: Clone, const N: usize>
  |               ++++

error[E0107]: missing generics for struct `Wrapper`
 --> ignores_generics/src/main.rs:4:8
  |
4 | struct Wrapper<'a, T: Clone, const N: usize>
  |        ^^^^^^^ expected 2 generic arguments
  |
note: struct defined here, with 2 generic parameters: `T`, `N`
 --> ignores_generics/src/main.rs:4:8
  |
4 | struct Wrapper<'a, T: Clone, const N: usize>
  |        ^^^^^^^     -         --------------
help: add missing generic arguments
  |
4 | struct Wrapper<T, N><'a, T: Clone, const N: usize>
  |               ++++++

Some errors have detailed explanations: E0107, E0726.
For more information about an error, try `rustc --explain E0107`.
error: could not compile `ignores_generics` (bin "ignores_generics") due to 2 previous errors
```
<!-- /cargo -->

Two errors for one mistake: the lifetime is reported by itself (E0726), and E0107 counts the other two. Both point at `Wrapper` in the struct, because the only token on the `impl` line that came from the input is the struct's name, and it kept its span. That is also why the *help* lines splice `<'_>` and `<T, N>` into the struct's own line, where they make no sense: they are the edit rustc would suggest to someone who had typed `impl Debug for Wrapper` by hand. For a derive, the fix is `#impl_generics` and `#ty_generics`.

### The `where` clause dropped

`DebugForgettingWhere` adds the `Debug` bounds exactly as `DebugBoundingParams` does, then prints `impl #impl_generics ... #ty_generics` without `#where_clause`:

<!-- file:demo/forgets_where/src/main.rs -->
```rust title="demo/forgets_where/src/main.rs"
use debug_generics::DebugForgettingWhere;

#[derive(DebugForgettingWhere)]
struct Wrapper<'a, T: Clone, const N: usize>
where
    T: PartialEq,
{
    label: &'a str,
    items: [T; N],
}

fn main() {
    let evens = Wrapper {
        label: "evens",
        items: [2, 4, 6],
    };
    println!("{evens:?}");
}
```
<!-- /file -->

<!-- cargo:generics_where_forgotten -->
*Verified output of `cargo build -q -p forgets_where`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
error[E0277]: can't compare `T` with `T`
 --> forgets_where/src/main.rs:3:10
  |
3 | #[derive(DebugForgettingWhere)]
  |          ^^^^^^^^^^^^^^^^^^^^ no implementation for `T == T`
  |
note: required by a bound in `Wrapper`
 --> forgets_where/src/main.rs:6:8
  |
4 | struct Wrapper<'a, T: Clone, const N: usize>
  |        ------- required by a bound in this struct
5 | where
6 |     T: PartialEq,
  |        ^^^^^^^^^ required by this bound in `Wrapper`
  = note: this error originates in the derive macro `DebugForgettingWhere` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider further restricting type parameter `T` with trait `PartialEq`
  |
4 | struct Wrapper<'a, T: Clone + std::cmp::PartialEq, const N: usize>
  |                             +++++++++++++++++++++

error[E0277]: can't compare `T` with `T`
 --> forgets_where/src/main.rs:4:8
  |
4 | struct Wrapper<'a, T: Clone, const N: usize>
  |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `T == T`
  |
note: required by a bound in `Wrapper`
 --> forgets_where/src/main.rs:6:8
  |
4 | struct Wrapper<'a, T: Clone, const N: usize>
  |        ------- required by a bound in this struct
5 | where
6 |     T: PartialEq,
  |        ^^^^^^^^^ required by this bound in `Wrapper`
help: consider further restricting type parameter `T` with trait `PartialEq`
  |
4 | struct Wrapper<'a, T: Clone + std::cmp::PartialEq, const N: usize>
  |                             +++++++++++++++++++++

error[E0277]: `T` doesn't implement `Debug`
 --> forgets_where/src/main.rs:3:10
  |
3 | #[derive(DebugForgettingWhere)]
  |          ^^^^^^^^^^^^^^^^^^^^ the trait `Debug` is not implemented for `T`
  |
  = note: required for `[T; N]` to implement `Debug`
  = note: required for the cast from `&[T; N]` to `&dyn Debug`
  = note: this error originates in the derive macro `DebugForgettingWhere` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider further restricting type parameter `T` with trait `Debug`
  |
4 | struct Wrapper<'a, T: Clone + std::fmt::Debug, const N: usize>
  |                             +++++++++++++++++

For more information about this error, try `rustc --explain E0277`.
error: could not compile `forgets_where` (bin "forgets_where") due to 3 previous errors
```
<!-- /cargo -->

Everything that was in the `where` clause went missing together: the struct's own `T: PartialEq`, which `Wrapper<'a, T, N>` needs to be a valid type at all, and the added `T: Debug`, which the body needs. Tokens the derive wrote itself are spanned at the derive (`#[derive(DebugForgettingWhere)]`, line 3); tokens copied from the input keep theirs (line 4). The compiler's suggestion edits the user's struct; the bug is one missing `#where_clause` in the macro.

## The trap: which bound to add

`bound_each_type_param` adds `T: Debug` for every type parameter. That is what the compiler's own `#[derive(Debug)]` does, and it is wrong whenever `T` appears only inside a type that is `Debug` for every `T`. [`PhantomData<T>`](../../12_Traits/phantom_types/README.md) is the common case: its implementation is `impl<T: ?Sized> Debug for PhantomData<T>`, with no bound on `T` ([`core/src/fmt/mod.rs` at 1.98.0 ↗](https://github.com/rust-lang/rust/blob/1.98.0/library/core/src/fmt/mod.rs#L3142-L3146)).

<!-- file:demo/phantom_trap/src/main.rs -->
```rust title="demo/phantom_trap/src/main.rs"
use std::marker::PhantomData;

use debug_generics::DebugBoundingParams;

/// A unit of measure: a marker type, never constructed, with no `Debug`.
struct Meters;

/// `T` appears only inside `PhantomData`, which is `Debug` for every `T`.
#[derive(DebugBoundingParams)]
struct Length<T> {
    value: u32,
    unit: PhantomData<T>,
}

/// The same struct under the compiler's own derive.
#[derive(Debug)]
struct StdLength<T> {
    value: u32,
    unit: PhantomData<T>,
}

fn main() {
    let ours: Length<Meters> = Length {
        value: 5,
        unit: PhantomData,
    };
    let std: StdLength<Meters> = StdLength {
        value: 5,
        unit: PhantomData,
    };
    println!("{ours:?}");
    println!("{std:?}");
}
```
<!-- /file -->

<!-- cargo:generics_phantom_trap -->
*Verified output of `cargo build -q -p phantom_trap`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
error[E0277]: `Meters` doesn't implement `Debug`
  --> phantom_trap/src/main.rs:31:15
   |
31 |     println!("{ours:?}");
   |               ^^^^^^^^ `Meters` cannot be formatted using `{:?}` because it doesn't implement `Debug`
   |
   = help: the trait `Debug` is not implemented for `Meters`
help: the trait `Debug` is conditionally implemented for `Length<T>`
  --> phantom_trap/src/main.rs:9:10
   |
 9 | #[derive(DebugBoundingParams)]
   |          ^^^^^^^^^^^^^^^^^^^ unsatisfied requirement introduced here: `Meters: Debug`
note: required for `Length<Meters>` to implement `Debug`
  --> phantom_trap/src/main.rs:10:8
   |
 9 | #[derive(DebugBoundingParams)]
   |          ------------------- type parameter would need to implement `Debug`
10 | struct Length<T> {
   |        ^^^^^^^^^
   = help: consider manually implementing `Debug` to avoid undesired bounds
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the derive macro `DebugBoundingParams` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider annotating `Meters` with `#[derive(Debug)]`
   |
 6 + #[derive(Debug)]
 7 | struct Meters;
   |

error[E0277]: `Meters` doesn't implement `Debug`
  --> phantom_trap/src/main.rs:32:15
   |
32 |     println!("{std:?}");
   |               ^^^^^^^ `Meters` cannot be formatted using `{:?}` because it doesn't implement `Debug`
   |
   = help: the trait `Debug` is not implemented for `Meters`
help: the trait `Debug` is conditionally implemented for `StdLength<T>`
  --> phantom_trap/src/main.rs:16:10
   |
16 | #[derive(Debug)]
   |          ^^^^^
17 | struct StdLength<T> {
   |                  - unsatisfied requirement introduced here: `Meters: Debug`
note: required for `StdLength<Meters>` to implement `Debug`
  --> phantom_trap/src/main.rs:17:8
   |
16 | #[derive(Debug)]
   |          ----- in this derive macro expansion
17 | struct StdLength<T> {
   |        ^^^^^^^^^ - type parameter would need to implement `Debug`
   = help: consider manually implementing `Debug` to avoid undesired bounds
help: consider annotating `Meters` with `#[derive(Debug)]`
   |
 6 + #[derive(Debug)]
 7 | struct Meters;
   |

For more information about this error, try `rustc --explain E0277`.
error: could not compile `phantom_trap` (bin "phantom_trap") due to 2 previous errors
```
<!-- /cargo -->

The same error twice, once for this crate's derive and once for std's, down to the help line: *"consider manually implementing `Debug` to avoid undesired bounds"*. A `Length<Meters>` holds a `u32` and a zero-sized marker, and both are `Debug`; the bound on `T` asks for something the `fmt` body never uses.

### Bound the field types instead

`DebugBoundingFields` puts a bound on each field's type instead, whatever that type mentions:

<!-- file:demo/phantom_fixed/src/main.rs -->
```rust title="demo/phantom_fixed/src/main.rs"
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
```
<!-- /file -->

<!-- cargo:generics_phantom_fixed -->
*Verified output of `cargo run -q -p phantom_fixed` — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
Length { value: 5, unit: PhantomData<phantom_fixed::Meters> }
```
<!-- /cargo -->

<!-- cargo:generics_phantom_fixed_expanded -->
*Verified output of `cargo rustc -q -p phantom_fixed --profile=check -- -Zunpretty=expanded` — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
#![feature(prelude_import)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use std::marker::PhantomData;

use debug_generics::DebugBoundingFields;

/// A unit of measure: a marker type, never constructed, with no `Debug`.
struct Meters;

struct Length<T> {
    value: u32,
    unit: PhantomData<T>,
}
impl<T> ::core::fmt::Debug for Length<T> where u32: ::core::fmt::Debug,
    PhantomData<T>: ::core::fmt::Debug {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Length").field("value",
                    &self.value).field("unit", &self.unit).finish()
    }
}

fn main() {
    let length: Length<Meters> = Length { value: 5, unit: PhantomData };
    { ::std::io::_print(format_args!("{0:?}\n", length)); };
}
```
<!-- /cargo -->

`where u32: Debug, PhantomData<T>: Debug` asks exactly what the body needs, and `PhantomData<Meters>: Debug` holds. `u32: Debug` mentions no parameter at all, and the compiler accepts it.

### And where that breaks

A type whose field mentions the type itself:

<!-- file:demo/recursive_trap/src/main.rs -->
```rust title="demo/recursive_trap/src/main.rs"
use debug_generics::{DebugBoundingFields, DebugBoundingParams};

/// A linked list: the type of `next` mentions `List` itself.
#[derive(DebugBoundingFields)]
struct List<T> {
    value: T,
    next: Option<Box<List<T>>>,
}

/// The same list, bounded the other way.
#[derive(DebugBoundingParams)]
struct ParamList<T> {
    value: T,
    next: Option<Box<ParamList<T>>>,
}

fn main() {
    let by_params = ParamList {
        value: 1,
        next: None,
    };
    let by_fields = List {
        value: 1,
        next: None,
    };
    println!("{by_params:?}");
    println!("{by_fields:?}");
}
```
<!-- /file -->

<!-- cargo:generics_recursive_trap -->
*Verified output of `cargo build -q -p recursive_trap`, which fails on purpose — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
error[E0275]: overflow evaluating the requirement `List<i32>: Debug`
  --> recursive_trap/src/main.rs:27:15
   |
27 |     println!("{by_fields:?}");
   |               ^^^^^^^^^^^^^
   |
   = note: required for `Box<List<i32>>` to implement `Debug`
   = note: 2 redundant requirements hidden
   = note: required for `List<i32>` to implement `Debug`

For more information about this error, try `rustc --explain E0275`.
error: could not compile `recursive_trap` (bin "recursive_trap") due to 1 previous error
```
<!-- /cargo -->

`List<i32>: Debug` requires `Option<Box<List<i32>>>: Debug`, which requires `Box<List<i32>>: Debug`, which requires `List<i32>: Debug` — the question it started with, so the compiler gives up with E0275. `ParamList`, the same list under `DebugBoundingParams`, gets no error: its bound, `T: Debug`, asks only for `i32: Debug`.

So neither rule is right for every type. Bounding each type parameter refused `Length<Meters>`, which it should have accepted. Bounding each field's type accepted it, and then could not prove `List<i32>: Debug` at all.

## In the wild

- **`serde_derive`** bounds only the type parameters that appear in fields, walking each field's type, and skips `PhantomData` by name: *"Hardcoded exception, because PhantomData<T> implements Serialize and Deserialize whether or not T implements it"* ([`bound.rs` in 1.0.229 ↗](https://docs.rs/crate/serde_derive/1.0.229/source/src/bound.rs#132)). When the heuristic guesses wrong, `#[serde(bound = "...")]` on the container replaces the generated bounds with yours ([`ser.rs` ↗](https://docs.rs/crate/serde_derive/1.0.229/source/src/ser.rs#144)).
- **`darling`** has the same escape hatch for its own derives, `#[darling(bound = ...)]` ([`options/core.rs` in 0.24.1 ↗](https://docs.rs/crate/darling_core/0.24.1/source/src/options/core.rs#124)).
- **The compiler's `#[derive(Debug)]`** bounds every type parameter, as the recorded error shows. It declares no helper attributes (the by-hand page records *cannot find attribute* for one), so there is nothing to override; the help line's advice, a hand-written `impl`, is the way out.

## If you are coming from another language

- **C++.** A template is checked when it is used, not where it is written: `std::vector<Meters>` compiles for a `Meters` with no `operator==`, and only `v == v` fails. A Rust `impl` is checked where it is written, against its bounds, so a derive has to state its bounds up front and choose them, which is this page's whole problem. The closest C++ comes to a bounded `impl` is a C++20 `requires` clause on a member, `bool same(const Wrapper&) const requires std::equality_comparable<T>`: a member that exists only for some `T`.
- **Java.** A generic class cannot implement an interface only for some type arguments: `class Wrapper<T> implements Comparable<Wrapper<T>>` holds for every `T` or none. Rust's `impl<T: Debug> Debug for Wrapper<T>` is a *conditional* implementation — `Wrapper<i32>` is `Debug` and `Wrapper<Meters>` is not — and choosing that condition is what a derive's `where` clause does. `toString()` exists on every object, so Java never faces the `PhantomData` question.
- **Python.** No declaration exists to repeat: a `__repr__` that calls `repr()` on each field works for any values that support it, and fails at call time for any that do not. The Rust derive has to predict that condition as a bound before any value exists.

## See also

- [Where the bound goes](../../22_Generics/where_the_bound_goes/README.md) — why a bound belongs on the `impl` and not on the struct
- [Phantom types](../../12_Traits/phantom_types/README.md) — what `PhantomData<T>` is for
- [A generic recursive type](../../22_Generics/a_generic_recursive_type/README.md) — `Option<Box<Self>>`, the shape that defeats field-type bounds
- [Every struct and enum shape](../every_struct_and_enum_shape/README.md) — the other axis a derive has to cover
- [Parsing with `syn`](../parsing_with_syn/README.md) — `DeriveInput` and its `Generics`
- [Helper attributes by hand](../helper_attributes_by_hand/README.md) — the attribute-driven `Debug` derive, which ignores generics to stay short
- [`Generics::split_for_impl` in syn 3.0.6 ↗](https://docs.rs/syn/3.0.6/syn/struct.Generics.html#method.split_for_impl)

## Po polsku

Makro derive dla typu **generycznego** (*generic*) musi powtórzyć w bloku `impl` wszystkie **parametry** typu — czasy życia (*lifetimes*), parametry typów i stałych (*const generics*) — razem z ich **ograniczeniami** (*bounds*) i klauzulą `where`. `Generics::split_for_impl` zwraca trzy kawałki: `impl_generics` (po `impl`, z ograniczeniami, bez wartości domyślnych), `ty_generics` (po nazwie typu, same nazwy) i `where_clause`, a `make_where_clause` pozwala dopisać własne ograniczenie. Pominięcie parametrów daje E0726 i E0107, a pominięcie `#where_clause` — E0277.

Pułapka: najprostsza reguła, `T: Debug` dla każdego parametru typu, to ta sama, której używa wbudowane `#[derive(Debug)]`, i jest za ostra dla `PhantomData<T>` — `Length<Meters>` nie dostaje `Debug`, choć niczego od `Meters` nie potrzebuje. Ograniczenie typów pól (`PhantomData<T>: Debug`) rozwiązuje ten przypadek, ale na typie rekurencyjnym, takim jak lista `Option<Box<List<T>>>`, kończy się błędem E0275 (przepełnienie przy sprawdzaniu wymagania). `serde` stosuje heurystykę z wyjątkiem dla `PhantomData` i daje furtkę `#[serde(bound = "...")]`.

**Szukaj po polsku:** makra derive a typy generyczne · `split_for_impl` · `make_where_clause` · `rust derive PhantomData bound` · `perfect derive rust`
