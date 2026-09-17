# Borrowing something forever: `&'a mut Thing<'a>`

**Level:** 201 → 301 · working knowledge

**One line:** `&'a mut Node<'a>` gives one name to two lifetimes: how long you borrow the node, and how long the text inside it lives. `&mut` will not let the compiler shorten the second, so the borrow lasts as long as the node. After the call you cannot print it, call a method on it, borrow it, move it, or even drop it if it has a destructor. Two names fix it.

```rust
#[derive(Debug)]
struct Node<'a>(&'a str);

fn rename(node: &mut Node<'_>, name: &'static str) { // two lifetimes: the borrow ends when rename returns
    node.0 = name;
}

fn main() {
    let leaf = String::from("leaf");
    let mut node = Node(&leaf);
    rename(&mut node, "twig");
    rename(&mut node, "branch");
    println!("{node:?}"); // Node("branch")
}
```

This page runs the claims in quinedot's [*Borrowing something forever* ↗](https://quinedot.github.io/rust-learning/pf-borrow-forever.html) on rustc 1.98.0. All of them hold. The one about destructors is more exact than it sounds, and [its companion on self-referential structs](../self_referential_structs/README.md) has one claim that does not hold.

## Two lifetimes, one name

Spell the signature out and there are two questions in it:

```rust
struct Node<'a>(&'a str);

fn rename_forever<'a>(node: &'a mut Node<'a>, name: &'static str) {
    //                       ^^             ^^
    //   how long rename_forever        how long the text the
    //   borrows the node               node points at stays valid
    node.0 = name;
}
```

The text is `leaf`, which lives for the whole of `main`. Writing `'a` in both places says the borrow of the node lasts exactly as long as the lifetime inside the node's type. `rename` above answers the two questions separately: the elided borrow ends with the call, and `'_` is whatever the text's lifetime happens to be.

## Why one name locks it

Behind a shared reference the compiler would quietly shrink `'a`. A `Node<'long>` can stand in wherever a `Node<'short>` is wanted, so `&'a Node<'a>` gets an `'a` just long enough for the call. That substitution is *covariance*.

Behind `&mut` it stops. `&mut T` is **invariant** in `T`: writing through the pointer could store a shorter-lived reference into a slot whose type promises a longer one, so no substitution happens in either direction ([What `&'a T` claims](../what_a_reference_claims/README.md#the-direction-reverses-behind-mut) shows the refusal). So `'a` must be exactly the lifetime in `node`'s type, and that lifetime covers every later use of `node`. The exclusive borrow is given that same `'a`, so every later use of `node` falls inside a borrow that is still live.

## What the lock forbids

Each row is a separate program: `rename_forever(&mut node, "twig")`, then one more line.

| After the call you write | rustc 1.98.0 says | Entry |
|---|---|---|
| `println!("{node:?}");` | `E0502` cannot borrow `node` as immutable because it is also borrowed as mutable | [3](../borrowing_forever_errors/README.md#3-printing-the-node-after-a-mut-nodea) |
| `node.name_len()` | `E0502`, the same message | [4](../borrowing_forever_errors/README.md#4-calling-a-self-method-after-a-mut-nodea) |
| `let r = &node;` | `E0502`, the same message | |
| `let nodes = vec![node];` | `E0505` cannot move out of `node` because it is borrowed | [5](../borrowing_forever_errors/README.md#5-moving-the-node-after-a-mut-nodea) |
| `rename_forever(&mut node, "branch");` | `E0499` cannot borrow `node` as mutable more than once at a time | [6](../borrowing_forever_errors/README.md#6-calling-the-a-mut-nodea-function-twice) |
| nothing at all, when the type has a `Drop` impl | `E0597` `loud` does not live long enough | [7](../borrowing_forever_errors/README.md#7-a-type-with-drop-behind-a-mut-thinga) |

For a type with no destructor, the call itself compiles. quinedot's `example_1(&mut node_a)` is fine because `node_a` is never named again, which is why this bug survives until the second use gets written.

The compiler can even suggest the pattern. Given `fn skip_first(t: &mut &mut [i32]) { *t = &mut t[1..]; }`, rustc 1.98.0's `help:` offers `fn skip_first<'a>(t: &'a mut &'a mut [i32])`. The function then compiles and its caller gets this page's `E0502` ([Re-pointing a slice](../references/repointing_a_slice/README.md#why-not-t-mut-t1) has both transcripts and the fix).

## What still works

**The borrow you handed over.** If the function gives it back, you keep going through it. Section 2 of the run below renames through `handle` and prints `handle`. `locked` itself is gone for good. This is what quinedot means by "except via that borrow", and the handle is a [reborrow](../reborrowing/README.md).

**The shared shape, for a type like `Node`.** `&'a Node<'a>` is covariant, so `'a` shrinks to the call and section 3 calls `look` twice and prints the node afterwards. When the type is not covariant, because it holds a `Cell` or a `&mut`, `'a` cannot shrink and the shared borrow lasts as long as the value too. Reading still works, since any number of shared borrows may overlap, but moving it is `E0505`, `&mut` is `E0502`, and a `Drop` impl is `E0597` (run on a `struct Slot<'a>(Cell<&'a str>)`). quinedot's next page, [`&'a Struct<'a>` and covariance ↗](https://quinedot.github.io/rust-learning/pf-shared-nested.html), covers that case.

## A destructor is a use

With no `Drop` impl, the closing brace does nothing with the node, and a node that is never named again compiles. Add a `Drop` impl and the brace calls `drop(&mut self)`, a use that the forever-borrow still covers:

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

The exact version of quinedot's claim: `&'a mut Thing<'a>` on a type with a destructor compiles **only when that destructor will never run**. Section 4 shows one of the two ways, `ManuallyDrop`. The other, `Box::leak`, compiles too, and neither is a fix.

## The whole verified run

<!-- output:borrowing_forever -->
*Verified output of [`borrowing_forever.rs`](examples/borrowing_forever.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. &mut Node<'_>: the borrow ends at the call
   renamed twice, then printed: Node("branch")

2. &'a mut Node<'a>: the borrow lasts as long as the node
   everything goes through the handle now: Node("branch")
   `locked` itself may not be named again: printing it, calling a
   method, taking & or &mut, or moving it are all refused.

3. &'a Node<'a>: shared, so the same shape costs nothing
   look = 4, look again = 4
   and still usable: Node("leaf")

4. A destructor needs the value at the closing brace
   &mut Loud<'_> compiles, and the drop is allowed to run:
   drop runs for Loud("touched")
   &'a mut Loud<'a> compiles only when no drop will run,
   here inside ManuallyDrop, whose destructor never calls Loud's.
   `kept` is locked like `locked` above: not even kept.0 may be read.
```
<!-- /output -->

## quinedot's claims, run

| The guide says | Run on 1.98.0 |
|---|---|
| `&'a mut Thing<'a>` takes an exclusive borrow for the rest of the thing's validity | **True.** Every later use is refused, with the codes in the table above. |
| You can do it once and it is OK | **True**, as long as the thing is never named again and has no destructor. |
| You cannot call methods on it, take another reference, move it or print it | **True:** `E0502`, `E0502`, `E0505`, `E0502`. |
| …except via that borrow | **True.** Return the borrow and use the handle (section 2). |
| You cannot call a non-trivial destructor on it; with one, the code will not compile | **True**, and exact: `E0597` whenever the destructor would run, and it compiles only inside `ManuallyDrop` or after `Box::leak`, where it never does. |
| Avoid `&'a mut Thing<'a>` | **Good advice, and nothing enforces it.** With every clippy group switched on, no lint names the pattern ([lints page](../borrowing_forever_lints/README.md#what-no-lint-catches)), and a rustc `help:` line can propose it (above). |

## If you are coming from another language

- **A reader–writer lock** (Go `sync.RWMutex`, Java `ReentrantReadWriteLock`, C++ `std::shared_mutex`). `&` and `&mut` follow the same rule as that lock: many readers or one writer ([Read-write lock, a stub in the Concurrency library ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/synchronization/read_write_lock/index.html)). `&'a mut Thing<'a>` is taking the write lock and never releasing it. In those languages the next reader blocks forever at run time. In Rust the program does not compile, the "lock" costs nothing at run time, and nothing is ever actually held.
- **C and C++.** A signature cannot say "keeps exclusive access to its argument for as long as the argument exists", so you would write it in a comment, and callers who did not read it would find out at run time. Rust can say it, and does whenever the same lifetime name is written twice, which is usually by accident.
- **Python.** Any number of names can reach an object and any of them can change it. The nearest thing to this lock is the runtime check `RuntimeError: dictionary changed size during iteration`, where one user of the dict locks out another. The [Python library's crosswalk ↗](https://masiarek.github.io/python-learning-library/CROSSWALK.html#mutable-and-immutable) puts the two rules side by side under *two names for one buffer*.
- **ABAP.** A lock set by an `ENQUEUE_…` function module that nothing `DEQUEUE`s. The entry stays in the lock table (SM12) until the program or its update task ends, and every other user who asks for that object gets a foreign-lock error. The difference is when you find out: SM12 shows you the stuck entry in production, and rustc refuses the build.

## Practice

**Unlock the cursor.** This cursor hands out one word at a time, and the loop does not compile:

```rust
struct Cursor<'a> {
    text: &'a str,
    pos: usize,
}

impl<'a> Cursor<'a> {
    fn next_word(&'a mut self) -> Option<&'a str> {
        let rest = &self.text[self.pos..];
        let start = rest.find(|c: char| !c.is_whitespace())?;
        let len = rest[start..].find(char::is_whitespace).unwrap_or(rest.len() - start);
        self.pos += start + len;
        Some(&rest[start..start + len])
    }
}

fn main() {
    let text = String::from("borrow it forever");
    let mut cursor = Cursor { text: &text, pos: 0 };
    let mut words = Vec::new();
    while let Some(word) = cursor.next_word() { // E0499: mutably borrowed in the previous iteration
        words.push(word);
    }
    println!("{words:?}"); // ["borrow", "it", "forever"]
}
```

Change the signature of `next_word`, and nothing else in the loop, so it prints the three words. Then check that the words outlive the cursor: `drop(cursor)` and print them afterwards. Try eliding both lifetimes first (`&mut self` and `Option<&str>`). It is still `E0499`, and working out why is the kata: where do the words actually borrow from?

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:borrowing_forever_kata -->
*[`borrowing_forever_kata.rs`](examples/borrowing_forever_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: a word cursor whose `next_word` can be called in a loop, and
//! whose words outlive the cursor. The fix is one lifetime moved: the result
//! borrows from the text (`'a`), not from the cursor.
//!
//!   rustc --edition 2024 borrowing_forever_kata.rs -o /tmp/bfk && /tmp/bfk
//!   rustc --edition 2024 --test borrowing_forever_kata.rs -o /tmp/bfkt && /tmp/bfkt

struct Cursor<'a> {
    text: &'a str,
    pos: usize,
}

impl<'a> Cursor<'a> {
    fn new(text: &'a str) -> Cursor<'a> {
        Cursor { text, pos: 0 }
    }

    /// Was `fn next_word(&'a mut self) -> Option<&'a str>`: the first call
    /// borrowed the cursor for all of `'a`, so the second was E0499.
    /// Eliding both (`&mut self -> Option<&str>`) still fails in the loop,
    /// because each word then borrows the cursor and the Vec keeps them.
    /// The words come out of `self.text`, which is `&'a str`, so say so.
    fn next_word(&mut self) -> Option<&'a str> {
        let text: &'a str = self.text;
        let rest = &text[self.pos..];
        let start = rest.find(|c: char| !c.is_whitespace())?;
        let len = rest[start..].find(char::is_whitespace).unwrap_or(rest.len() - start);
        self.pos += start + len;
        Some(&rest[start..start + len])
    }
}

fn words(text: &str) -> Vec<&str> {
    let mut cursor = Cursor::new(text);
    let mut found = Vec::new();
    while let Some(word) = cursor.next_word() {
        found.push(word);
    }
    found // the cursor is dropped here; the words are not tied to it
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collects_every_word() {
        assert_eq!(words("borrow it forever"), ["borrow", "it", "forever"]);
        assert_eq!(words("  spaced   out  "), ["spaced", "out"]);
        assert!(words("").is_empty());
    }

    #[test]
    fn cursor_is_usable_between_calls() {
        let mut cursor = Cursor::new("one two");
        assert_eq!(cursor.next_word(), Some("one"));
        assert_eq!(cursor.pos, 3);
        assert_eq!(cursor.next_word(), Some("two"));
        assert_eq!(cursor.next_word(), None);
    }
}

fn main() {
    println!("1. The loop that was E0499");
    let text = String::from("borrow it forever");
    let found = words(&text);
    println!("   words({text:?}) = {found:?}");

    println!();
    println!("2. The cursor is still yours between calls");
    let mut cursor = Cursor::new("  spaced   out  ");
    let first = cursor.next_word();
    println!("   first = {first:?}, cursor.pos = {}", cursor.pos);
    let second = cursor.next_word();
    println!("   second = {second:?}, cursor.pos = {}", cursor.pos);
    drop(cursor);
    println!("   after drop(cursor), both words still print: {first:?} {second:?}");

    println!();
    println!("3. Three signatures, run");
    println!("   fn next_word(&'a mut self) -> Option<&'a str>  E0499 in the loop");
    println!("   fn next_word(&mut self) -> Option<&str>        E0499 in the loop");
    println!("   fn next_word(&mut self) -> Option<&'a str>     compiles, as above");
}
```
<!-- /source -->

<!-- output:borrowing_forever_kata -->
*Verified output of [`borrowing_forever_kata.rs`](examples/borrowing_forever_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The loop that was E0499
   words("borrow it forever") = ["borrow", "it", "forever"]

2. The cursor is still yours between calls
   first = Some("spaced"), cursor.pos = 8
   second = Some("out"), cursor.pos = 14
   after drop(cursor), both words still print: Some("spaced") Some("out")

3. Three signatures, run
   fn next_word(&'a mut self) -> Option<&'a str>  E0499 in the loop
   fn next_word(&mut self) -> Option<&str>        E0499 in the loop
   fn next_word(&mut self) -> Option<&'a str>     compiles, as above
```
<!-- /output -->

</details>

## See also

- [A struct that points into itself](../self_referential_structs/README.md) — the same lock, reached from inside a struct by `fn bite(&'a mut self)`
- [Every borrowed-forever error, and its fix](../borrowing_forever_errors/README.md) — fourteen errors, each with its transcript and a fix that compiles
- [Lints around borrowing forever](../borrowing_forever_lints/README.md) — the lints that push toward `Node<'_>`, and the pattern no lint names
- [Reading on borrowing forever](../borrowing_forever_resources/README.md) — quinedot, the Nomicon on variance, Jon Gjengset's `strtok` stream, and book sections
- [What `&'a T` claims](../what_a_reference_claims/README.md) — covariance, and why it stops behind `&mut`
- [What a lifetime does at the call site](../lifetimes_at_the_call_site/README.md) — one lifetime against two, from the caller's side
- [Borrowed state](../borrowed_state/README.md) — the lock read from the owner's side, `E0505` and `E0506`
- [A borrow is a loan](../references/a_borrow_is_a_loan/README.md#going-out-of-scope-is-a-use-too) — the loan model behind every row of the table above, and why a closing brace counts as a use
- [Re-pointing a slice](../references/repointing_a_slice/README.md#why-not-t-mut-t1) — `&'a mut &'a mut [i32]`, the same lock on a slice reference, suggested by rustc itself
- [References: the map](../references/README.md) — `&`, `&mut`, `ref` and `*`, the path this page branches off
- [Reborrowing](../reborrowing/README.md) — the handle a function hands back is a reborrow
- [Lifetime annotations](../lifetime_annotations/README.md) — writing `<'a>` and the elision rules that write `'_` for you
- [How to learn lifetimes](../how_to_learn_lifetimes/README.md) — its first amendment, "don't put references in struct fields", is how most people meet `Thing<'a>`

## Po polsku

`&'a mut Node<'a>` nadaje **jedną nazwę dwóm czasom życia**: temu, jak długo pożyczasz węzeł, i temu, jak długo żyje tekst, na który węzeł wskazuje. Za `&mut` kompilator nie może skrócić `'a` (typ `&mut T` jest **inwariantny** względem `T`), więc pożyczenie trwa tyle, ile życie samego węzła. Po takim wywołaniu nie wydrukujesz węzła, nie wywołasz na nim metody, nie pożyczysz go, nie przeniesiesz, a jeśli typ ma `Drop`, program w ogóle się nie skompiluje, bo destruktor też jest użyciem.

Naprawa to jedna zmiana w sygnaturze: dwie nazwy zamiast jednej, najczęściej po prostu `fn rename(node: &mut Node<'_>)`. Żaden lint w rustc ani w clippy nie wskazuje wzorca `&'a mut Thing<'a>`, więc trzeba go rozpoznawać samemu, zwykle po błędzie `E0499` przy drugim wywołaniu.

**Szukaj po polsku:** pożyczanie na zawsze · inwariancja `&mut` · wariancja czasów życia · `rust borrow forever` · `rust &'a mut self` · `rust invariance`
