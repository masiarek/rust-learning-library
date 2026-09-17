# Lints around borrowing forever

Beside [Borrowing something forever](../borrowing_forever/README.md) and [A struct that points into itself](../self_referential_structs/README.md) · the errors, not the warnings: [Every borrowed-forever error, and its fix](../borrowing_forever_errors/README.md)

**Level:** 201 · a reference, by lint

**One line:** Five rustc and clippy lints that touch lifetimes in structs and signatures, each with a program that triggers it, what the tool prints, a silent version, and when the lint is right. Then the part that matters most: no lint, in any group, flags `&'a mut Thing<'a>` or a struct that borrows from itself.

Every transcript was recorded on rustc 1.98.0 and clippy 0.1.98 by running the file shown with only that lint switched on. The lines dropped are the documentation link and the note naming the flag. Each silent version was run with that lint plus `clippy::all`, `clippy::pedantic` and `rust_2018_idioms`, and printed nothing. Both programs of every pair are compiled on every build by [`check_fences.py`](../../tools/check_fences.py). Clippy itself is not re-run, so a later clippy can reword a warning.

## Turning them on

rustc's warn-by-default lints and clippy's default groups need nothing: `cargo clippy` prints them. The opt-in ones take a flag or a `Cargo.toml` entry:

```sh
cargo clippy -- -W rust_2018_idioms -W clippy::elidable_lifetime_names -W clippy::needless_pass_by_ref_mut
```

```toml
[lints.rust]
rust_2018_idioms = { level = "warn", priority = -1 }

[lints.clippy]
elidable_lifetime_names = "warn"
needless_pass_by_ref_mut = "warn"
```

## Find the lint

| Lint | From | On by default? | Fires on |
|---|---|---|---|
| [`mismatched_lifetime_syntaxes`](#mismatched_lifetime_syntaxes) | rustc | yes | `fn cursor(text: &str) -> Cursor`, one lifetime spelled two ways |
| [`needless_lifetimes`](#needless_lifetimes) | clippy · complexity | yes | an explicit lifetime on the outer reference that elision would supply, as in `&'b mut Node<'_>` |
| [`elided_lifetimes_in_paths`](#elided_lifetimes_in_paths) | rustc · `rust_2018_idioms` group | no | a type with a lifetime parameter written with none, as in `&mut Node` |
| [`elidable_lifetime_names`](#elidable_lifetime_names) | clippy · pedantic | no | a named lifetime used once, as in `fn rename<'a>(node: &mut Node<'a>)` |
| [`needless_pass_by_ref_mut`](#needless_pass_by_ref_mut) | clippy · nursery | no | a `&mut` parameter the body only reads, including `&'a mut Node<'a>` |

## `rustc`

### `mismatched_lifetime_syntaxes`

**rustc** · warn by default · fires on `fn cursor(text: &str) -> Cursor`, one lifetime spelled two ways

```rust
struct Cursor<'a> {
    text: &'a str,
    pos: usize,
}

fn cursor(text: &str) -> Cursor {
    Cursor { text, pos: 0 }
}

fn main() {
    let c = cursor("one two");
    println!("{} {}", c.text, c.pos); // one two 0
}
```

```text title="rustc --edition 2024 bad.rs — rustc 1.98.0"
warning: hiding a lifetime that's elided elsewhere is confusing
 --> bad.rs:6:17
  |
6 | fn cursor(text: &str) -> Cursor {
  |                 ^^^^     ^^^^^^ the same lifetime is hidden here
  |                 |
  |                 the lifetime is elided here
  |
  = help: the same lifetime is referred to in inconsistent ways, making the signature confusing
help: use `'_` for type paths
  |
6 | fn cursor(text: &str) -> Cursor<'_> {
  |                                ++++
```

**Silent:**

```rust
struct Cursor<'a> {
    text: &'a str,
    pos: usize,
}

fn cursor(text: &str) -> Cursor<'_> {
    Cursor { text, pos: 0 }
}

fn main() {
    let c = cursor("one two");
    println!("{} {}", c.text, c.pos); // one two 0
}
```

**When it is right.** Always right. The return type borrows from `text`, and `Cursor<'_>` says so where `Cursor` did not. This is the lint that points out a struct holding a reference is being returned, the first step toward every page here.

### `elided_lifetimes_in_paths`

**rustc · `rust_2018_idioms` group** · off by default · fires on a type with a lifetime parameter written with none, as in `&mut Node`

```rust
#[derive(Debug)]
struct Node<'a>(&'a str);

fn rename(node: &mut Node, name: &'static str) {
    node.0 = name;
}

fn main() {
    let leaf = String::from("leaf");
    let mut node = Node(&leaf);
    rename(&mut node, "twig");
    println!("{node:?}"); // Node("twig")
}
```

```text title="rustc --edition 2024 -W elided_lifetimes_in_paths bad.rs — rustc 1.98.0"
warning: hidden lifetime parameters in types are deprecated
 --> bad.rs:4:22
  |
4 | fn rename(node: &mut Node, name: &'static str) {
  |                      ^^^^ expected lifetime parameter
  |
help: indicate the anonymous lifetime
  |
4 | fn rename(node: &mut Node<'_>, name: &'static str) {
  |                          ++++
```

**Silent:**

```rust
#[derive(Debug)]
struct Node<'a>(&'a str);

fn rename(node: &mut Node<'_>, name: &'static str) {
    node.0 = name;
}

fn main() {
    let leaf = String::from("leaf");
    let mut node = Node(&leaf);
    rename(&mut node, "twig");
    println!("{node:?}"); // Node("twig")
}
```

**When it is right.** Right for any code that others read: `Node` hides that the type borrows, `Node<'_>` shows it. It is part of `rust_2018_idioms`; turn the whole group on as in [Turning them on](#turning-them-on).

## Clippy

### `needless_lifetimes`

**clippy · complexity** · warn by default · fires on an explicit lifetime on the outer reference that elision would supply, as in `&'b mut Node<'_>`

```rust
#[derive(Debug)]
struct Node<'a>(&'a str);

fn rename<'b>(node: &'b mut Node<'_>, name: &'static str) {
    node.0 = name;
}

fn main() {
    let leaf = String::from("leaf");
    let mut node = Node(&leaf);
    rename(&mut node, "twig");
    println!("{node:?}"); // Node("twig")
}
```

```text title="clippy-driver --edition 2024 -W clippy::needless_lifetimes bad.rs — clippy 0.1.98"
warning: the following explicit lifetimes could be elided: 'b
 --> bad.rs:4:11
  |
4 | fn rename<'b>(node: &'b mut Node<'_>, name: &'static str) {
  |           ^^         ^^
  |
help: elide the lifetimes
  |
4 - fn rename<'b>(node: &'b mut Node<'_>, name: &'static str) {
4 + fn rename(node: &mut Node<'_>, name: &'static str) {
  |
```

**Silent:**

```rust
#[derive(Debug)]
struct Node<'a>(&'a str);

fn rename(node: &mut Node<'_>, name: &'static str) {
    node.0 = name;
}

fn main() {
    let leaf = String::from("leaf");
    let mut node = Node(&leaf);
    rename(&mut node, "twig");
    println!("{node:?}"); // Node("twig")
}
```

**When it is right.** Always right in the shape shown. Note what it does *not* fire on: `&'a mut Node<'a>`. The name is used twice there, so it is not needless, and the one spelling on this page that is a bug is the one this lint goes quiet about.

### `elidable_lifetime_names`

**clippy · pedantic** · off by default · fires on a named lifetime used once, as in `fn rename<'a>(node: &mut Node<'a>)`

```rust
#[derive(Debug)]
struct Node<'a>(&'a str);

fn rename<'a>(node: &mut Node<'a>, name: &'static str) {
    node.0 = name;
}

fn main() {
    let leaf = String::from("leaf");
    let mut node = Node(&leaf);
    rename(&mut node, "twig");
    println!("{node:?}"); // Node("twig")
}
```

```text title="clippy-driver --edition 2024 -W clippy::elidable_lifetime_names bad.rs — clippy 0.1.98"
warning: the following explicit lifetimes could be elided: 'a
 --> bad.rs:4:11
  |
4 | fn rename<'a>(node: &mut Node<'a>, name: &'static str) {
  |           ^^                  ^^
  |
help: elide the lifetimes
  |
4 - fn rename<'a>(node: &mut Node<'a>, name: &'static str) {
4 + fn rename(node: &mut Node<'_>, name: &'static str) {
  |
```

**Silent:**

```rust
#[derive(Debug)]
struct Node<'a>(&'a str);

fn rename(node: &mut Node<'_>, name: &'static str) {
    node.0 = name;
}

fn main() {
    let leaf = String::from("leaf");
    let mut node = Node(&leaf);
    rename(&mut node, "twig");
    println!("{node:?}"); // Node("twig")
}
```

**When it is right.** A style lint, and a useful one here. `Node<'_>` shows at a glance that the text's lifetime is tied to nothing else in the signature, so there is no second `'a` to merge it with by accident. Noise when a name documents something, for example several parameters that deliberately share a lifetime.

### `needless_pass_by_ref_mut`

**clippy · nursery** · off by default · fires on a `&mut` parameter the body only reads, including `&'a mut Node<'a>`

```rust
#[derive(Debug)]
struct Node<'a>(&'a str);

fn name_len<'a>(node: &'a mut Node<'a>) -> usize {
    node.0.len()
}

fn main() {
    let leaf = String::from("leaf");
    let mut node = Node(&leaf);
    println!("{}", name_len(&mut node)); // 4
}
```

```text title="clippy-driver --edition 2024 -W clippy::needless_pass_by_ref_mut bad.rs — clippy 0.1.98"
warning: this parameter is a mutable reference but is not used mutably
 --> bad.rs:4:23
  |
4 | fn name_len<'a>(node: &'a mut Node<'a>) -> usize {
  |                       ^^^^----^^^^^^^^
  |                           |
  |                           help: consider removing this `mut`
  |
```

**Silent:**

```rust
#[derive(Debug)]
struct Node<'a>(&'a str);

fn name_len(node: &Node<'_>) -> usize {
    node.0.len()
}

fn main() {
    let leaf = String::from("leaf");
    let node = Node(&leaf);
    println!("{}", name_len(&node)); // 4
    println!("{node:?}"); // Node("leaf")
}
```

**When it is right.** Right whenever the body never writes, and on this page it does more than it says: dropping the `mut` turns `&'a mut Node<'a>` into `&'a Node<'a>`, which is covariant, so the forever-borrow disappears and the caller can print `node` again. It says nothing when the body writes, which is exactly when `&'a mut Node<'a>` is worst. Noise when the `&mut` is there on purpose, for a trait signature or an API that will write later. It is in *nursery*, so expect false positives, and do not rely on it.

## What no lint catches

This program contains all three things the lessons warn about: a function taking `&'a mut Node<'a>` that writes through it, `bite(&'a mut self)`, and a struct made self-referential in place.

```rust
#[derive(Debug)]
struct Node<'a>(&'a str);

fn rename<'a>(node: &'a mut Node<'a>, name: &'a str) {
    println!("renaming {:?} to {name}", node.0);
    node.0 = name;
}

struct Snek<'a> {
    owned: String,
    borrowed: &'a str,
}

impl<'a> Snek<'a> {
    fn bite(&'a mut self) -> &'a Self {
        self.borrowed = &self.owned;
        self
    }
}

fn main() {
    let leaf = String::from("leaf");
    let mut node = Node(&leaf);
    rename(&mut node, "twig"); // renaming "leaf" to twig

    let mut snek = Snek { owned: String::from("hiss"), borrowed: "" };
    let bitten = snek.bite();
    println!("{} {}", bitten.owned, bitten.borrowed); // hiss hiss

    let mut in_place = Snek { owned: String::from("rattle"), borrowed: "" };
    in_place.borrowed = &in_place.owned;
    println!("{}", in_place.borrowed); // rattle
}
```

```text title="clippy-driver --edition 2024 -W clippy::all -W clippy::pedantic -W clippy::nursery -W rust_2018_idioms -W unused no_lint.rs; echo exit $? — clippy 0.1.98"
exit 0
```

Nothing but the exit status. It compiles, and every group except `restriction` has nothing to say. Adding `-W clippy::restriction` produces warnings from `arbitrary_source_item_ordering`, `implicit_return`, `missing_docs_in_private_items`, `print_stdout`, `single_call_fn`, `single_char_lifetime_names`, `use_debug`, and the only one about lifetimes, `single_char_lifetime_names`, objects to the *name* `'a`, not to writing it twice.

So nothing flags `&'a mut Thing<'a>` in rustc or clippy, and rustc's own `help:` can propose it: for `*t = &mut t[1..]` on a `t: &mut &mut [i32]` it suggests `&'a mut &'a mut [i32]` ([Re-pointing a slice](../references/repointing_a_slice/README.md#why-not-t-mut-t1)). You find out when a later line uses the value and gets `E0502`, `E0499` or `E0505`, or when a `Drop` impl turns up and the brace gets `E0597`. The habit that replaces the lint: **when a lifetime name appears on both sides of a `&mut`, ask whether the two lifetimes really are one.**

## See also

- [Every borrowed-forever error, and its fix](../borrowing_forever_errors/README.md) — what the compiler refuses, where this page is what it only warns about
- [Borrowing something forever](../borrowing_forever/README.md#why-one-name-locks-it) — why the two lifetimes in `&'a mut Node<'a>` should have two names
- [Lifetime annotations](../lifetime_annotations/README.md) — the elision rules `needless_lifetimes` and `elidable_lifetime_names` apply
- [Strict clippy](../../05_Tooling/strict_lints/README.md) — turning whole lint groups on for a project, and what that costs
- [Lints around `ToOwned`](../../12_Traits/how_to_learn_to_owned/to_owned_lints/README.md) — the same kind of page for `clone`, `to_owned` and `Cow`

## Po polsku

Pięć lintów rustc i clippy związanych z czasami życia w strukturach i sygnaturach: dla każdego program, który go wywołuje, dokładny komunikat (rustc 1.98.0, clippy 0.1.98), wersja bez ostrzeżenia i kiedy lint ma rację. `mismatched_lifetime_syntaxes` i `needless_lifetimes` działają domyślnie; `elided_lifetimes_in_paths`, `elidable_lifetime_names` i `needless_pass_by_ref_mut` trzeba włączyć.

Najważniejsza jest sekcja o tym, czego **żaden** lint nie łapie: `&'a mut Thing<'a>`, metoda `bite(&'a mut self)` i struktura samoreferencyjna zbudowana przypisaniem przechodzą bez słowa przy włączonych wszystkich grupach clippy. O problemie dowiadujesz się dopiero z błędu `E0502`, `E0499` albo `E0505` przy następnym użyciu wartości.

**Szukaj po polsku:** `clippy needless_lifetimes` · `clippy elidable_lifetime_names` · `rust elided_lifetimes_in_paths` · `rust mismatched_lifetime_syntaxes`
