# Every borrowed-forever error, and its fix

Beside [Borrowing something forever](../borrowing_forever/README.md) and [A struct that points into itself](../self_referential_structs/README.md) · the warnings, not the errors: [Lints around borrowing forever](../borrowing_forever_lints/README.md)

**Level:** 201 → 301 · a reference, by symptom

**One line:** Fourteen compiler errors that turn up around `&'a mut Thing<'a>` and structs that borrow from themselves. For each: the code that causes it, what rustc 1.98.0 prints, the mistake in a few sentences, and a fix. The broken code must fail and the fix must compile: [`check_fences.py`](../../tools/check_fences.py) holds both on every build.

The transcripts were recorded from rustc 1.98.0 with `--crate-type lib --emit=metadata`, the way the fences are checked, from a file named in each title. Every fix was built the same way and printed nothing, not even a warning. The transcripts are not regenerated on each commit the way an example's output is, so a later rustc can reword one. The fences are checked, so none of them can silently start compiling.

Eight codes cover all fourteen. Read the last line of a transcript first: *"borrow later used here"* names the use that keeps the lock alive, and in this family that use is often a line you did not write, such as the closing brace that runs a destructor.

## Find the error

| # | Code | What rustc says | The mistake |
|---|---|---|---|
| 1 | `E0106` | [missing lifetime specifier](#1-a-str-field-with-no-lifetime) | A `&str` field with no lifetime |
| 2 | `E0726` | [implicit elided lifetime not allowed here](#2-impl-snek-for-a-sneka) | `impl Snek` for a `Snek<'a>` |
| 3 | `E0502` | [cannot borrow `node` as immutable because it is also borrowed as mutable](#3-printing-the-node-after-a-mut-nodea) | Printing the node after `&'a mut Node<'a>` |
| 4 | `E0502` | [cannot borrow `node` as immutable because it is also borrowed as mutable](#4-calling-a-self-method-after-a-mut-nodea) | Calling a `&self` method after `&'a mut Node<'a>` |
| 5 | `E0505` | [cannot move out of `node` because it is borrowed](#5-moving-the-node-after-a-mut-nodea) | Moving the node after `&'a mut Node<'a>` |
| 6 | `E0499` | [cannot borrow `node` as mutable more than once at a time](#6-calling-the-a-mut-nodea-function-twice) | Calling the `&'a mut Node<'a>` function twice |
| 7 | `E0597` | [`loud` does not live long enough](#7-a-type-with-drop-behind-a-mut-thinga) | A type with `Drop` behind `&'a mut Thing<'a>` |
| 8 | `E0502` | [cannot borrow `snek.borrowed` as immutable because it is also borrowed as mutable](#8-reading-a-field-after-bitea-mut-self) | Reading a field after `bite(&'a mut self)` |
| 9 | `E0502` | [cannot borrow `*self` as mutable because it is also borrowed as immutable](#9-bite-handing-back-a-mut-self) | `bite` handing back `&'a mut Self` |
| 10 | `E0505` | [cannot move out of `snek` because it is borrowed](#10-moving-a-struct-built-in-place) | Moving a struct built in place |
| 11 | `E0505` | [cannot move out of `snek` because it is borrowed](#11-moving-it-after-pointing-the-field-elsewhere) | Moving it after pointing the field elsewhere |
| 12 | `E0502` | [cannot borrow `snek.owned` as mutable because it is also borrowed as immutable](#12-changing-the-owned-field) | Changing the owned field |
| 13 | `E0713` | [borrow may still be in use when destructor runs](#13-a-drop-impl-on-a-self-referential-struct) | A `Drop` impl on a self-referential struct |
| 14 | `E0505`, `E0515` | [cannot move out of `snek` because it is borrowed](#14-a-constructor-that-returns-one) | A constructor that returns one |

## Putting a borrowed field in

### 1. A `&str` field with no lifetime

```rust,compile_fail
pub struct Snek {
    pub owned: String,
    pub borrowed: &str,
}
```

```text title="rustc 1.98.0 on field_without_lifetime.rs"
error[E0106]: missing lifetime specifier
 --> field_without_lifetime.rs:3:19
  |
3 |     pub borrowed: &str,
  |                   ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ pub struct Snek<'a> {
2 |     pub owned: String,
3 ~     pub borrowed: &'a str,
  |
```

**The mistake.** A reference field has to name how long the text it points at lives, so the struct takes a lifetime parameter. rustc's help adds one, and that compiles. It is the right fix when the text lives *outside* the struct. When `borrowed` is meant to point into `owned`, the `'a` it adds is where entries 8 to 14 begin.

**The fix.** If the text is outside the struct, take rustc's help as written. If it is inside, store a position.

```rust
use std::ops::Range;

pub struct Snek {
    pub owned: String,
    pub borrowed: Range<usize>, // where in `owned`, not a reference into it
}
```

**Read:** [Self-referential structs](../self_referential_structs/README.md#the-working-answer-store-where-not-a-reference), [Lifetime annotations](../lifetime_annotations/README.md)

### 2. `impl Snek` for a `Snek<'a>`

```rust,compile_fail
pub struct Snek<'a> {
    pub owned: String,
    pub borrowed: &'a str,
}

impl Snek {
    pub fn borrowed_len(&self) -> usize {
        self.borrowed.len()
    }
}
```

```text title="rustc 1.98.0 on impl_without_lifetime.rs"
error[E0726]: implicit elided lifetime not allowed here
 --> impl_without_lifetime.rs:6:6
  |
6 | impl Snek {
  |      ^^^^ expected lifetime parameter
  |
help: indicate the anonymous lifetime
  |
6 | impl Snek<'_> {
  |          ++++
```

**The mistake.** Once the struct has a lifetime parameter, every mention of the type needs one, `impl` headers included. Nothing here needs a *name* for it.

**The fix.** `'_` says "some lifetime, I will not refer to it". Write `impl<'a> Snek<'a>` only when a method signature has to say `'a`, and then read entry 8 before giving `self` that same `'a`.

```rust
pub struct Snek<'a> {
    pub owned: String,
    pub borrowed: &'a str,
}

impl Snek<'_> {
    pub fn borrowed_len(&self) -> usize {
        self.borrowed.len()
    }
}
```

**Read:** [Lifetime annotations](../lifetime_annotations/README.md)

## `&'a mut Thing<'a>`

### 3. Printing the node after `&'a mut Node<'a>`

```rust,compile_fail
#[derive(Debug)]
pub struct Node<'a>(pub &'a str);

pub fn rename<'a>(node: &'a mut Node<'a>, name: &'static str) {
    node.0 = name;
}

pub fn demo() {
    let leaf = String::from("leaf");
    let mut node = Node(&leaf);
    rename(&mut node, "twig");
    println!("{node:?}");
}
```

```text title="rustc 1.98.0 on print_after_forever.rs"
error[E0502]: cannot borrow `node` as immutable because it is also borrowed as mutable
  --> print_after_forever.rs:12:16
   |
11 |     rename(&mut node, "twig");
   |            --------- mutable borrow occurs here
12 |     println!("{node:?}");
   |                ^^^^
   |                |
   |                immutable borrow occurs here
   |                mutable borrow later used here
```

**The mistake.** One name, `'a`, for two lifetimes. `&mut T` is invariant in `T`, so `'a` cannot shrink to fit the call: the exclusive borrow has to last as long as `'a`, and `'a` has to cover every later use of `node`. The `println!` is a later use, made while the borrow is still live.

**The fix.** Two lifetimes: the borrow of the node (elided) and the text inside it (`'_`). The borrow now ends when `rename` returns.

```rust
#[derive(Debug)]
pub struct Node<'a>(pub &'a str);

pub fn rename(node: &mut Node<'_>, name: &'static str) {
    node.0 = name;
}

pub fn demo() {
    let leaf = String::from("leaf");
    let mut node = Node(&leaf);
    rename(&mut node, "twig");
    println!("{node:?}");
}
```

**Read:** [Borrowing forever](../borrowing_forever/README.md#why-one-name-locks-it), [What `&'a T` claims](../what_a_reference_claims/README.md#the-direction-reverses-behind-mut)

### 4. Calling a `&self` method after `&'a mut Node<'a>`

```rust,compile_fail
#[derive(Debug)]
pub struct Node<'a>(pub &'a str);

impl Node<'_> {
    pub fn name_len(&self) -> usize {
        self.0.len()
    }
}

pub fn rename<'a>(node: &'a mut Node<'a>, name: &'static str) {
    node.0 = name;
}

pub fn demo() {
    let leaf = String::from("leaf");
    let mut node = Node(&leaf);
    rename(&mut node, "twig");
    println!("{}", node.name_len());
}
```

```text title="rustc 1.98.0 on method_after_forever.rs"
error[E0502]: cannot borrow `node` as immutable because it is also borrowed as mutable
  --> method_after_forever.rs:18:20
   |
17 |     rename(&mut node, "twig");
   |            --------- mutable borrow occurs here
18 |     println!("{}", node.name_len());
   |                    ^^^^
   |                    |
   |                    immutable borrow occurs here
   |                    mutable borrow later used here
```

**The mistake.** A method call takes `&node`, which is a shared borrow made while the exclusive one is still live. It is the same error as entry 3, reached without naming `node` in a macro.

**The fix.** The same one-line signature change as entry 3.

```rust
#[derive(Debug)]
pub struct Node<'a>(pub &'a str);

impl Node<'_> {
    pub fn name_len(&self) -> usize {
        self.0.len()
    }
}

pub fn rename(node: &mut Node<'_>, name: &'static str) {
    node.0 = name;
}

pub fn demo() {
    let leaf = String::from("leaf");
    let mut node = Node(&leaf);
    rename(&mut node, "twig");
    println!("{}", node.name_len());
}
```

**Read:** [Borrowing forever](../borrowing_forever/README.md#what-the-lock-forbids)

### 5. Moving the node after `&'a mut Node<'a>`

```rust,compile_fail
#[derive(Debug)]
pub struct Node<'a>(pub &'a str);

pub fn rename<'a>(node: &'a mut Node<'a>, name: &'static str) {
    node.0 = name;
}

pub fn demo() {
    let leaf = String::from("leaf");
    let mut node = Node(&leaf);
    rename(&mut node, "twig");
    let nodes = vec![node];
    println!("{nodes:?}");
}
```

```text title="rustc 1.98.0 on move_after_forever.rs"
error[E0505]: cannot move out of `node` because it is borrowed
  --> move_after_forever.rs:12:22
   |
10 |     let mut node = Node(&leaf);
   |         -------- binding `node` declared here
11 |     rename(&mut node, "twig");
   |            --------- borrow of `node` occurs here
12 |     let nodes = vec![node];
   |                      ^^^^
   |                      |
   |                      move out of `node` occurs here
   |                      borrow later used here
   |
note: if `Node<'_>` implemented `Clone`, you could clone the value
  --> move_after_forever.rs:2:1
   |
 2 | pub struct Node<'a>(pub &'a str);
   | ^^^^^^^^^^^^^^^^^^^ consider implementing `Clone` for this type
...
11 |     rename(&mut node, "twig");
   |                 ---- you could clone this value
```

**The mistake.** Moving a value while it is borrowed would leave the borrow pointing at the old place. The borrow is still live here, because `nodes` holds a `Node<'a>` and is used on the next line, and `'a` is the length of the borrow.

**The fix.** The same signature change as entry 3.

```rust
#[derive(Debug)]
pub struct Node<'a>(pub &'a str);

pub fn rename(node: &mut Node<'_>, name: &'static str) {
    node.0 = name;
}

pub fn demo() {
    let leaf = String::from("leaf");
    let mut node = Node(&leaf);
    rename(&mut node, "twig");
    let nodes = vec![node];
    println!("{nodes:?}");
}
```

**Read:** [Borrowing forever](../borrowing_forever/README.md#what-the-lock-forbids), [Borrowed state](../borrowed_state/README.md)

### 6. Calling the `&'a mut Node<'a>` function twice

```rust,compile_fail
#[derive(Debug)]
pub struct Node<'a>(pub &'a str);

pub fn rename<'a>(node: &'a mut Node<'a>, name: &'static str) {
    node.0 = name;
}

pub fn demo() {
    let leaf = String::from("leaf");
    let mut node = Node(&leaf);
    rename(&mut node, "twig");
    rename(&mut node, "branch");
}
```

```text title="rustc 1.98.0 on call_twice_forever.rs"
error[E0499]: cannot borrow `node` as mutable more than once at a time
  --> call_twice_forever.rs:12:12
   |
11 |     rename(&mut node, "twig");
   |            --------- first mutable borrow occurs here
12 |     rename(&mut node, "branch");
   |            ^^^^^^^^^
   |            |
   |            second mutable borrow occurs here
   |            first borrow later used here
```

**The mistake.** The first call's exclusive borrow lasts as long as the node, so the second `&mut node` is a second exclusive borrow of the same value. In a loop rustc words it as "mutably borrowed here in the previous iteration of the loop".

**The fix.** The same signature change. When the function also returns a reference, decide which lifetime it borrows from: [the kata](../borrowing_forever/README.md#practice) is that decision.

```rust
#[derive(Debug)]
pub struct Node<'a>(pub &'a str);

pub fn rename(node: &mut Node<'_>, name: &'static str) {
    node.0 = name;
}

pub fn demo() {
    let leaf = String::from("leaf");
    let mut node = Node(&leaf);
    rename(&mut node, "twig");
    rename(&mut node, "branch");
}
```

**Read:** [Borrowing forever](../borrowing_forever/README.md#practice), [Reborrowing](../reborrowing/README.md)

### 7. A type with `Drop` behind `&'a mut Thing<'a>`

```rust,compile_fail
pub struct Loud<'a>(pub &'a str);

impl Drop for Loud<'_> {
    fn drop(&mut self) {
        println!("dropping {}", self.0);
    }
}

pub fn touch<'a>(loud: &'a mut Loud<'a>) {
    loud.0 = "touched";
}

pub fn demo() {
    let leaf = String::from("leaf");
    let mut loud = Loud(&leaf);
    touch(&mut loud);
}
```

```text title="rustc 1.98.0 on drop_after_forever.rs"
error[E0597]: `loud` does not live long enough
  --> drop_after_forever.rs:16:11
   |
15 |     let mut loud = Loud(&leaf);
   |         -------- binding `loud` declared here
16 |     touch(&mut loud);
   |           ^^^^^^^^^ borrowed value does not live long enough
17 | }
   | -
   | |
   | `loud` dropped here while still borrowed
   | borrow might be used here, when `loud` is dropped and runs the `Drop` code for type `Loud`
```

**The mistake.** There is no later line at all, and it still fails. `drop` takes `&mut self` at the closing brace, which is a use of `loud` that the exclusive borrow still covers. Without a `Drop` impl the brace uses nothing, which is why quinedot's `Node` compiles and this does not.

**The fix.** The same signature change. Wrapping the value in `ManuallyDrop` or leaking it with `Box::leak` also compiles, because no destructor will ever run, which is rarely what you want.

```rust
pub struct Loud<'a>(pub &'a str);

impl Drop for Loud<'_> {
    fn drop(&mut self) {
        println!("dropping {}", self.0);
    }
}

pub fn touch(loud: &mut Loud<'_>) {
    loud.0 = "touched";
}

pub fn demo() {
    let leaf = String::from("leaf");
    let mut loud = Loud(&leaf);
    touch(&mut loud);
}
```

**Read:** [Borrowing forever](../borrowing_forever/README.md#a-destructor-is-a-use), [Scope is about names](../scope_is_about_names/README.md)

## A struct that points into itself

### 8. Reading a field after `bite(&'a mut self)`

```rust,compile_fail
pub struct Snek<'a> {
    pub owned: String,
    pub borrowed: &'a str,
}

impl<'a> Snek<'a> {
    pub fn bite(&'a mut self) {
        self.borrowed = &self.owned;
    }
}

pub fn demo() {
    let mut snek = Snek { owned: String::from("hiss"), borrowed: "" };
    snek.bite();
    println!("{}", snek.borrowed);
}
```

```text title="rustc 1.98.0 on use_after_bite.rs"
error[E0502]: cannot borrow `snek.borrowed` as immutable because it is also borrowed as mutable
  --> use_after_bite.rs:15:20
   |
14 |     snek.bite();
   |     ---- mutable borrow occurs here
15 |     println!("{}", snek.borrowed);
   |                    ^^^^^^^^^^^^^
   |                    |
   |                    immutable borrow occurs here
   |                    mutable borrow later used here
```

**The mistake.** `bite` compiles: this is the one method signature that can store `&self.owned` in `self.borrowed`. It works by borrowing `snek` exclusively for the whole of `'a`, the same lock as entries 3 to 7, so the struct it made self-referential can never be read again.

**The fix.** Borrow when asked instead of storing the borrow: a method returning `&str` is tied to one call, not to the struct's life.

```rust
pub struct Snek {
    pub owned: String,
}

impl Snek {
    pub fn borrowed(&self) -> &str {
        &self.owned
    }
}

pub fn demo() {
    let snek = Snek { owned: String::from("hiss") };
    println!("{}", snek.borrowed());
}
```

**Read:** [Self-referential structs](../self_referential_structs/README.md#through-a-method-bitea-mut-self)

### 9. `bite` handing back `&'a mut Self`

```rust,compile_fail
pub struct Snek<'a> {
    pub owned: String,
    pub borrowed: &'a str,
}

impl<'a> Snek<'a> {
    pub fn bite(&'a mut self) -> &'a mut Self {
        self.borrowed = &self.owned;
        self
    }
}
```

```text title="rustc 1.98.0 on bite_returns_mut.rs"
error[E0502]: cannot borrow `*self` as mutable because it is also borrowed as immutable
 --> bite_returns_mut.rs:9:9
  |
6 | impl<'a> Snek<'a> {
  |      -- lifetime `'a` defined here
7 |     pub fn bite(&'a mut self) -> &'a mut Self {
8 |         self.borrowed = &self.owned;
  |         ---------------------------
  |         |               |
  |         |               immutable borrow occurs here
  |         assignment requires that `self.owned` is borrowed for `'a`
9 |         self
  |         ^^^^ mutable borrow occurs here
```

**The mistake.** quinedot's guide says the struct stays usable through a reborrow returned from the method. That reborrow cannot be exclusive: `self.owned` is already shared-borrowed for `'a`, and an exclusive `&'a mut Self` would overlap it. The error is inside the method, before any caller exists.

**The fix.** Hand back a shared reborrow. The caller can read through it for as long as it likes, and can never write to the struct again.

```rust
pub struct Snek<'a> {
    pub owned: String,
    pub borrowed: &'a str,
}

impl<'a> Snek<'a> {
    pub fn bite(&'a mut self) -> &'a Self {
        self.borrowed = &self.owned;
        self
    }
}
```

**Read:** [Self-referential structs](../self_referential_structs/README.md#through-a-method-bitea-mut-self)

### 10. Moving a struct built in place

```rust,compile_fail
pub struct Snek<'a> {
    pub owned: String,
    pub borrowed: &'a str,
}

pub fn demo() {
    let mut snek = Snek { owned: String::from("hiss"), borrowed: "" };
    snek.borrowed = &snek.owned;
    let sneks = vec![snek];
    println!("{}", sneks[0].borrowed);
}
```

```text title="rustc 1.98.0 on move_self_referential.rs"
error[E0505]: cannot move out of `snek` because it is borrowed
 --> move_self_referential.rs:9:22
  |
7 |     let mut snek = Snek { owned: String::from("hiss"), borrowed: "" };
  |         -------- binding `snek` declared here
8 |     snek.borrowed = &snek.owned;
  |                     ----------- borrow of `snek.owned` occurs here
9 |     let sneks = vec![snek];
  |                      ^^^^
  |                      |
  |                      move out of `snek` occurs here
  |                      borrow later used here
  |
help: consider cloning the value if the performance cost is acceptable
  |
8 |     snek.borrowed = &snek.owned.clone();
  |                                ++++++++
```

**The mistake.** Assigning `snek.borrowed = &snek.owned` compiles with no `&mut Snek` anywhere, which quinedot's guide says is impossible. The price is the move: `borrowed` holds the address of `owned` in its old place, and a move could leave it pointing at nothing, so rustc refuses the move. (The `String`'s heap bytes would not move, but the borrow checker does not reason about heaps.)

**The fix.** Keep the owner outside the struct. The struct of views moves freely, because what it points at stays where it is.

```rust
pub struct View<'a> {
    pub borrowed: &'a str,
}

pub fn demo() {
    let owned = String::from("hiss");
    let view = View { borrowed: &owned };
    let views = vec![view];
    println!("{}", views[0].borrowed);
}
```

**Read:** [Self-referential structs](../self_referential_structs/README.md#built-in-place-no-a-mut-needed), [What an address shows](../what_an_address_shows/README.md)

### 11. Moving it after pointing the field elsewhere

```rust,compile_fail
pub struct Snek<'a> {
    pub owned: String,
    pub borrowed: &'a str,
}

pub fn demo() {
    let mut snek = Snek { owned: String::from("hiss"), borrowed: "" };
    snek.borrowed = &snek.owned;
    println!("{}", snek.borrowed);
    snek.borrowed = "elsewhere";
    let moved = snek;
    println!("{}", moved.borrowed);
}
```

```text title="rustc 1.98.0 on move_after_repointing.rs"
error[E0505]: cannot move out of `snek` because it is borrowed
  --> move_after_repointing.rs:11:17
   |
 7 |     let mut snek = Snek { owned: String::from("hiss"), borrowed: "" };
   |         -------- binding `snek` declared here
 8 |     snek.borrowed = &snek.owned;
   |                     ----------- borrow of `snek.owned` occurs here
...
11 |     let moved = snek;
   |                 ^^^^
   |                 |
   |                 move out of `snek` occurs here
   |                 borrow later used here
   |
help: consider cloning the value if the performance cost is acceptable
   |
 8 |     snek.borrowed = &snek.owned.clone();
   |                                ++++++++
```

**The mistake.** Repointing `borrowed` changes the value, not the type. `snek` and `moved` share one type, `Snek<'a>`, and the borrow `&snek.owned` was given that same `'a`. `moved` is used on the last line, so `'a` reaches that line, and the borrow is still live at the move.

**The fix.** A range borrows nothing, so there is no lifetime to outlast.

```rust
use std::ops::Range;

pub struct Snek {
    pub owned: String,
    pub borrowed: Range<usize>,
}

pub fn demo() {
    let mut snek = Snek { owned: String::from("hiss"), borrowed: 0..4 };
    println!("{}", &snek.owned[snek.borrowed.clone()]);
    snek.borrowed = 1..4;
    let moved = snek;
    println!("{}", &moved.owned[moved.borrowed.clone()]);
}
```

**Read:** [Self-referential structs](../self_referential_structs/README.md#built-in-place-no-a-mut-needed)

### 12. Changing the owned field

```rust,compile_fail
pub struct Snek<'a> {
    pub owned: String,
    pub borrowed: &'a str,
}

pub fn demo() {
    let mut snek = Snek { owned: String::from("hiss"), borrowed: "" };
    snek.borrowed = &snek.owned;
    snek.owned.push_str("!");
    println!("{}", snek.borrowed);
}
```

```text title="rustc 1.98.0 on mutate_self_referential.rs"
error[E0502]: cannot borrow `snek.owned` as mutable because it is also borrowed as immutable
  --> mutate_self_referential.rs:9:5
   |
 8 |     snek.borrowed = &snek.owned;
   |                     ----------- immutable borrow occurs here
 9 |     snek.owned.push_str("!");
   |     ^^^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
10 |     println!("{}", snek.borrowed);
   |                    ------------- immutable borrow later used here
```

**The mistake.** `push_str` may reallocate, which would leave `borrowed` dangling. That is the bug the rule exists for: many readers or one writer, and `borrowed` is still reading.

**The fix.** Private fields and one method that changes `owned` and recomputes the range. The range can still go stale if some other method forgets to update it, but a stale range at worst panics or returns the wrong slice, and nothing dangles.

```rust
use std::ops::Range;

pub struct Snek {
    owned: String,
    borrowed: Range<usize>,
}

impl Snek {
    pub fn new(owned: String) -> Snek {
        let borrowed = 0..owned.len();
        Snek { owned, borrowed }
    }

    pub fn push_str(&mut self, more: &str) {
        self.owned.push_str(more);
        self.borrowed = 0..self.owned.len(); // the one place that may invalidate it fixes it
    }

    pub fn borrowed(&self) -> &str {
        &self.owned[self.borrowed.clone()]
    }
}

pub fn demo() {
    let mut snek = Snek::new(String::from("hiss"));
    snek.push_str("!");
    println!("{}", snek.borrowed());
}
```

**Read:** [Self-referential structs](../self_referential_structs/README.md#practice), [How to learn lifetimes](../how_to_learn_lifetimes/README.md#practice)

### 13. A `Drop` impl on a self-referential struct

```rust,compile_fail
pub struct Snek<'a> {
    pub owned: String,
    pub borrowed: &'a str,
}

impl Drop for Snek<'_> {
    fn drop(&mut self) {
        println!("dropping {}", self.owned);
    }
}

pub fn demo() {
    let mut snek = Snek { owned: String::from("hiss"), borrowed: "" };
    snek.borrowed = &snek.owned;
}
```

```text title="rustc 1.98.0 on drop_self_referential.rs"
error[E0713]: borrow may still be in use when destructor runs
  --> drop_self_referential.rs:14:21
   |
14 |     snek.borrowed = &snek.owned;
   |                     ^^^^^^^^^^^
15 | }
   | -
   | |
   | here, drop of `snek` needs exclusive access to `snek.owned`, because the type `Snek<'_>` implements the `Drop` trait
   | borrow might be used here, when `snek` is dropped and runs the `Drop` code for type `Snek`
   |
   = note: consider using a `let` binding to create a longer lived value
```

**The mistake.** Entry 7 from the inside. `drop` gets `&mut self` while `borrowed` still holds a shared borrow of `owned`, and nothing after the assignment ends that borrow. The error code differs, E0713 rather than E0597, because the borrow is of a field inside the value being dropped.

**The fix.** With a range, `drop` borrows nothing that is already borrowed.

```rust
use std::ops::Range;

pub struct Snek {
    pub owned: String,
    pub borrowed: Range<usize>,
}

impl Drop for Snek {
    fn drop(&mut self) {
        println!("dropping {}", &self.owned[self.borrowed.clone()]);
    }
}

pub fn demo() {
    let snek = Snek { owned: String::from("hiss"), borrowed: 0..4 };
    println!("{}", snek.owned);
}
```

**Read:** [Self-referential structs](../self_referential_structs/README.md#built-in-place-no-a-mut-needed), [The drop flag](../the_drop_flag/README.md)

### 14. A constructor that returns one

```rust,compile_fail
pub struct Snek<'a> {
    pub owned: String,
    pub borrowed: &'a str,
}

pub fn new<'a>(owned: String) -> Snek<'a> {
    let mut snek = Snek { owned, borrowed: "" };
    snek.borrowed = &snek.owned;
    snek
}
```

```text title="rustc 1.98.0 on constructor_self_referential.rs"
error[E0505]: cannot move out of `snek` because it is borrowed
 --> constructor_self_referential.rs:9:5
  |
6 | pub fn new<'a>(owned: String) -> Snek<'a> {
  |            -- lifetime `'a` defined here
7 |     let mut snek = Snek { owned, borrowed: "" };
  |         -------- binding `snek` declared here
8 |     snek.borrowed = &snek.owned;
  |                     ----------- borrow of `snek.owned` occurs here
9 |     snek
  |     ^^^^
  |     |
  |     move out of `snek` occurs here
  |     returning this value requires that `snek.owned` is borrowed for `'a`
  |
help: consider cloning the value if the performance cost is acceptable
  |
8 |     snek.borrowed = &snek.owned.clone();
  |                                ++++++++

error[E0515]: cannot return value referencing local data `snek.owned`
 --> constructor_self_referential.rs:9:5
  |
8 |     snek.borrowed = &snek.owned;
  |                     ----------- `snek.owned` is borrowed here
9 |     snek
  |     ^^^^ returns a value referencing data owned by the current function
```

**The mistake.** What people want when they reach for a self-referential struct, and the one thing safe Rust will not do with one. Returning `snek` moves it out of the function (E0505), and the value returned would reference `snek.owned`, a local of the function (E0515). No lifetime written on `new` can help, because the text does not exist before the call.

**The fix.** Return the owner and a range into it. If the borrowed part is expensive to recompute and a range cannot describe it, the crates in [the reading list](../borrowing_forever_resources/README.md#crates-that-build-one-for-you) generate the `unsafe` code that makes this shape sound.

```rust
use std::ops::Range;

pub struct Snek {
    pub owned: String,
    pub borrowed: Range<usize>,
}

pub fn new(owned: String) -> Snek {
    let borrowed = 0..owned.len();
    Snek { owned, borrowed }
}
```

**Read:** [Self-referential structs](../self_referential_structs/README.md#what-people-actually-want), [A stack slot is reused](../a_stack_slot_is_reused/README.md)

## See also

- [Borrowing something forever](../borrowing_forever/README.md) — why one lifetime name written twice locks a value for the rest of its life
- [A struct that points into itself](../self_referential_structs/README.md) — the three ways safe Rust builds one, and the constructor it refuses
- [Lints around borrowing forever](../borrowing_forever_lints/README.md) — what the compiler only warns about, and what nothing flags
- [Reading on borrowing forever](../borrowing_forever_resources/README.md) — where each error is explained at length
- [Borrowed state](../borrowed_state/README.md) — `E0505` and `E0506` from the owner's side, the family entries 5, 10 and 11 belong to
- [Every `ToOwned` error, and its fix](../../12_Traits/how_to_learn_to_owned/to_owned_errors/README.md) — the same kind of page for `ToOwned`, `Clone` and `Cow`
- [rustc's error index ↗](https://doc.rust-lang.org/error_codes/error-index.html) — the long explanation behind each code, also printed by `rustc --explain E0713`

## Po polsku

Czternaście błędów kompilatora wokół `&'a mut Thing<'a>` i struktur, które pożyczają same z siebie. Przy każdym: kod, który go wywołuje, dokładny komunikat rustc 1.98.0, na czym polega pomyłka i poprawka, która się kompiluje. Wszystkie sprowadzają się do ośmiu kodów: `E0106` i `E0726` (brakujący czas życia), `E0502`, `E0499` i `E0505` (pożyczenie, które wciąż trwa) oraz `E0597`, `E0713` i `E0515` (destruktor albo zwrot wartości wskazującej na zmienną lokalną).

Wskazówka do czytania: zacznij od ostatniej linii komunikatu. *„borrow later used here”* pokazuje użycie, które trzyma blokadę, a w tej rodzinie błędów bywa to linia, której wcale nie napisałeś, na przykład zamykająca klamra, w której działa destruktor.

**Szukaj po polsku:** `rust E0502` · `rust E0505 cannot move out because it is borrowed` · `rust E0713` · `rust E0515 self-referential`
