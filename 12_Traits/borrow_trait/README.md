# `Borrow`: look up an owned key with a borrowed one

**Level:** 201 · working knowledge

**One line:** `Borrow<Q>` says an owned value can lend itself out as a `&Q` — a `String` as a `&str` — **and** that the two compare and hash identically; the second half is the promise that lets `HashMap<String, V>::get` take a `&str` without building a `String` to search with.

```rust
use std::collections::HashMap;

fn main() {
    let mut seats: HashMap<String, u32> = HashMap::new();
    seats.insert(String::from("Ada"), 3);
    println!("{:?}", seats.get("Ada"));   // Some(3)
}
```

The map owns `String` keys, and the lookup never makes one.

## The trait, in full

```rust
pub trait Borrow<Borrowed: ?Sized> {
    fn borrow(&self) -> &Borrowed;
}
```

One method, and the same shape as [`AsRef::as_ref` ↗](https://doc.rust-lang.org/std/convert/trait.AsRef.html). The difference is not in the signature; it is a sentence in [the docs ↗](https://doc.rust-lang.org/std/borrow/trait.Borrow.html) that no compiler checks: `Eq`, `Ord` and `Hash` must be equivalent for borrowed and owned values, so `x.borrow() == y.borrow()` gives the same result as `x == y`.

`Borrow` is not in the prelude. `name.borrow()` without `use std::borrow::Borrow;` is `E0599`, and its `help:` line names the import.

## What `get` asks for

The signature on 1.98.0:

```rust
pub fn get<Q>(&self, k: &Q) -> Option<&V>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized,
```

With `K = String` and a `&str` argument, `Q` is `str`. The map hashes the `str` to find the bucket, then compares `stored_key.borrow() == k`. That works only because a `String` hashes exactly like the `str` it holds — the promise, in use. `?Sized` is there because `str` is [unsized](../../14_Strings/str_is_unsized/README.md), and so are most borrowed forms.

## Who implements it

From the implementor list for 1.98.0, stable impls only:

| Owned | Lends out |
|---|---|
| every `T` | `T` itself |
| `&T`, `&mut T` | `T` |
| [`String`](../../14_Strings/string_vs_str/README.md) | `str` |
| `Vec<T>`, `[T; N]` | `[T]` |
| `Box<T>`, [`Rc<T>`](../../18_Ownership/reference_counting/README.md), `Arc<T>` | `T` |
| [`PathBuf`](../../04_Files/path_and_pathbuf/README.md) | `Path` |
| `OsString` | `OsStr` |
| `CString` | `CStr` |
| [`Cow<'_, B>`](../../18_Ownership/clone_on_write/README.md) | `B` |

The first row is why `seats.get(&String::from("Ada"))` also compiles: `String: Borrow<String>`. Section 2 of the run below looks up a `PathBuf` key with a `&Path`, a `Vec<u8>` with a `&[u8]`, a `Box<str>` with a `&str`, and a `&str` key with a `str`.

## Why `ToOwned` needs it

[`ToOwned`](../to_owned/README.md) declares `type Owned: Borrow<Self>`. Going to the owned form is `to_owned`; `Borrow` is the guaranteed way back. That round trip is what lets a [`Cow<str>`](../../18_Ownership/clone_on_write/README.md) hand you a `&str` from either arm.

## Missing on purpose: `String` is not `Borrow<[u8]>`

A `String` can lend out its bytes, and does — through `AsRef<[u8]>`. It does not implement `Borrow<[u8]>`, because a `str` and a `[u8]` holding the same bytes hash differently. Section 3 of the run prints `false` for that comparison. So a byte-slice lookup on a `String`-keyed map is refused:

```text title="rustc 1.98.0 on bytes_key.rs — a byte-slice lookup on a String-keyed map"
error[E0277]: the trait bound `String: Borrow<[u8]>` is not satisfied
 --> bytes_key.rs:6:32
  |
6 |     println!("{:?}", seats.get(b"Ada".as_slice()));
  |                            --- ^^^^^^^^^^^^^^^^^ the trait `Borrow<[u8]>` is not implemented for `String`
  |                            |
  |                            required by a bound introduced by this call
  |
help: the trait `Borrow<[u8]>` is not implemented for `String`
      but trait `Borrow<str>` is implemented for it
 --> /rustc/88d9e12ae178fab0fb5cc050a94da85685d449ea/library/alloc/src/str.rs:229:0
  = help: for that trait implementation, expected `str`, found `[u8]`
note: required by a bound in `HashMap::<K, V, S, A>::get`
 --> /rustc/88d9e12ae178fab0fb5cc050a94da85685d449ea/library/std/src/collections/hash/map.rs:1035:4
```

That is the rule for choosing between the two: `AsRef<T>` for "give me a cheap view", `Borrow<T>` only when the view compares and hashes like the owner.

## The trap: an impl that breaks the promise compiles

The std docs' example is a key that compares and hashes ignoring ASCII case; the run below adds an `Ord` that does the same. Nothing stops you giving it `impl Borrow<str>` — the impl is one method returning `&self.0`, and the compiler accepts it. The map then answers wrong, and only for some keys.

Section 4 of the run puts `"apple"`, `"Banana"` and `"cherry"` in a `BTreeMap<CaseInsensitive, u32>`. The map orders them ignoring case: apple, Banana, cherry. A lookup by `&str` compares with `str`'s ordering, where every uppercase letter sorts before every lowercase one, so `"Banana"` is less than `"apple"` and the search stops at the first key:

| Lookup | Result |
|---|---|
| `stock.get(&CaseInsensitive("BANANA"))` | `Some(2)` |
| `stock.get("cherry")` | `Some(3)` |
| `stock.get("Banana")` — the exact key inserted | `None` |

A `HashMap` fails through the hash instead: section 4 shows the key's hash and the `str`'s hash differ, so a lookup searches the wrong place. It is not even consistently wrong. In a separate probe on 1.98.0, looking up the exact key inserted into 100,000 fresh maps found it 790 times and missed 99,210 — which one a run gets depends on the map's random seed, so a test can pass and the same line fail later.

The fix is to not write the `Borrow` impl. Offer `AsRef<str>` for access to the text, and look up with a `CaseInsensitive` value. A borrowed lookup without allocating needs an unsized `CaseInsensitiveStr` whose comparisons match, built the way [Implementing `ToOwned`](../implementing_to_owned/README.md) builds `AsciiStr`.

## The trap: `.borrow()` on an `Rc<RefCell<T>>`

With `Borrow` imported, the call you meant for `RefCell` finds the trait first:

```rust
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let scores = Rc::new(RefCell::new(vec![1, 2, 3]));
    let total: i32 = scores.borrow().iter().sum();
    println!("{total}");
}
```

```text title="rustc 1.98.0 on refcell_trap.rs — the snippet above"
error[E0282]: type annotations needed
 --> refcell_trap.rs:7:29
  |
7 |     let total: i32 = scores.borrow().iter().sum();
  |                             ^^^^^^
  |
help: try using a fully qualified path to specify the expected types
  |
7 -     let total: i32 = scores.borrow().iter().sum();
7 +     let total: i32 = <Rc<RefCell<Vec<_>>> as Borrow<Borrowed>>::borrow(&scores).iter().sum();
  |
```

[Method resolution](../method_resolution/README.md) tries `&Rc<RefCell<_>>` before it dereferences to `&RefCell<_>`. `Borrow::borrow` takes `&self`, so it matches at the earlier step and wins. `Rc` implements `Borrow` twice — as its pointee and, through the blanket impl, as itself — so the compiler cannot pick a `Borrowed` type. The error says nothing about `RefCell`, and the `help:` line keeps the `Borrow` call, with a placeholder `Borrowed` left for you to fill in.

Three fixes, and the first is usually right: drop the `use` line if nothing in the file needs it; write `(*scores).borrow()`, which starts the search at `RefCell`, where the inherent method beats the trait; or write `RefCell::borrow(&scores)`. Section 5 of the run uses the last two.

## If you are coming from another language

- **C++** — this is heterogeneous lookup: `std::map<std::string, int, std::less<>>` accepts a `std::string_view` in `find` since C++14, and the unordered containers since C++20 with a transparent hash. C++ makes you opt in per container; Rust turns it on for every map, for the pairs std declares. Both trust that the comparator and the hash agree, and neither checks.
- **Java** — `HashMap.get(Object key)` takes anything at all and relies on the `equals`/`hashCode` contract, which is the same promise `Borrow` makes. The difference is that Rust lists which borrowed types are allowed, so `get(b"Ada")` on a `String` map is a compile error instead of a silent `null`.
- **Python** — a `dict` looks up with any hashable key under the same `__eq__`/`__hash__` contract. `{"Ada": 3}.get(b"Ada")` is `None`, because `str` and `bytes` are different keys; Rust refuses the same lookup at compile time instead of returning nothing.

## Practice

**One lookup for every kind of key.** Write `count_for(map, key) -> u32`, returning `0` for a missing key, generic enough for all three calls: `count_for(&by_name, "Ada")` on a `HashMap<String, u32>`, `count_for(&by_path, Path::new("notes.txt"))` on a `HashMap<PathBuf, u32>`, and `count_for(&by_bytes, b"GET".as_slice())` on a `HashMap<Vec<u8>, u32>`. Then prove none of the calls allocated, with a counting global allocator.

Before you run it, say which bound breaks all three calls if you leave it off, and whether `count_for(&by_name, b"Ada".as_slice())` compiles.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:borrow_trait_kata -->
*[`borrow_trait_kata.rs`](examples/borrow_trait_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: one lookup for every kind of key, and a count proving that
//! none of the three calls built an owned key to search with.
//!
//!   rustc --edition 2024 borrow_trait_kata.rs -o /tmp/btk && /tmp/btk

use std::alloc::{GlobalAlloc, Layout, System};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

struct Counting;

static ALLOCS: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Relaxed);
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

/// `K: Borrow<Q>` lets the map compare its owned keys with the borrowed one.
/// `Q: Hash + Eq` is what the map needs to find the bucket from `key` alone.
/// `?Sized` because all three borrowed forms below — `str`, `Path`, `[u8]` —
/// are unsized. Delete it and the calls stop compiling, with E0277: "the size
/// for values of type `str` cannot be known at compilation time".
fn count_for<K, Q>(map: &HashMap<K, u32>, key: &Q) -> u32
where
    K: Borrow<Q> + Hash + Eq,
    Q: Hash + Eq + ?Sized,
{
    map.get(key).copied().unwrap_or(0)
}

fn main() {
    let by_name: HashMap<String, u32> = HashMap::from([(String::from("Ada"), 3)]);
    let by_path: HashMap<PathBuf, u32> = HashMap::from([(PathBuf::from("notes.txt"), 7)]);
    let by_bytes: HashMap<Vec<u8>, u32> = HashMap::from([(b"GET".to_vec(), 12)]);

    let before = ALLOCS.load(Relaxed);
    let name = count_for(&by_name, "Ada");
    let path = count_for(&by_path, Path::new("notes.txt"));
    let bytes = count_for(&by_bytes, b"GET".as_slice());
    let missing = count_for(&by_name, "Ben");
    let allocs = ALLOCS.load(Relaxed) - before;

    println!("1. One function, three kinds of key");
    println!("   String  keys, &str  lookup: {name}");
    println!("   PathBuf keys, &Path lookup: {path}");
    println!("   Vec<u8> keys, &[u8] lookup: {bytes}");
    println!("   a missing key:              {missing}");
    println!("   heap allocations across the four calls: {allocs}");

    println!();
    println!("2. The call that does not compile");
    // count_for(&by_name, b"Ada".as_slice());
    //   E0277: the trait bound `String: Borrow<[u8]>` is not satisfied.
    // A str and a [u8] with the same bytes hash differently, so std never
    // wrote that impl; String lends out its bytes through AsRef<[u8]> instead.
    println!("   count_for(&by_name, b\"Ada\".as_slice()) is E0277: String is not Borrow<[u8]>");
}
```
<!-- /source -->

<!-- output:borrow_trait_kata -->
*Verified output of [`borrow_trait_kata.rs`](examples/borrow_trait_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. One function, three kinds of key
   String  keys, &str  lookup: 3
   PathBuf keys, &Path lookup: 7
   Vec<u8> keys, &[u8] lookup: 12
   a missing key:              0
   heap allocations across the four calls: 0

2. The call that does not compile
   count_for(&by_name, b"Ada".as_slice()) is E0277: String is not Borrow<[u8]>
```
<!-- /output -->

</details>

## The verified output

<!-- output:borrow_trait -->
*Verified output of [`borrow_trait.rs`](examples/borrow_trait.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. A String key, looked up with a &str
   seats.get("Ada")                = Some(3)
   seats.get(&String::from("Ada")) = Some(3)   (T: Borrow<T> covers the owned form too)
   seats.get("ada")                = None   (str equality, so case counts)

2. Every owned/borrowed pair works the same way
   HashMap<PathBuf, _>.get(Path::new("notes.txt")) = Some(7)
   HashSet<Vec<u8>>.contains(b"GET".as_slice())    = true
   BTreeMap<Box<str>, _>.get("Ada")                = Some(3)
   HashMap<&str, _>.get("Ada")                     = Some(3)   (&T: Borrow<T>)

3. The promise: the borrowed form hashes like the owned one
   hash(String "Ada") == hash(&str "Ada")   : true
   hash(&str "Ada")   == hash(&[u8] b"Ada") : false
   so String is AsRef<[u8]> ([65, 100, 97]) but not Borrow<[u8]>

4. A key that breaks the promise compiles anyway
   hash(CaseInsensitive("Ada")) == hash("Ada") : false
   inserted "apple", "Banana", "cherry"
   stock.get(&CaseInsensitive("BANANA")) = Some(2)
   stock.get("cherry")                   = Some(3)
   stock.get("Banana")                   = None   <- the exact key inserted
   str orders "Banana" before "apple"; the map stored it after, and the search stopped at the first key.

5. The trap: with Borrow in scope, .borrow() on an Rc<RefCell<_>>
   (*scores).borrow()        sums to 6
   RefCell::borrow(&scores)  sums to 6
```
<!-- /output -->

## See also

- [`ToOwned`](../to_owned/README.md) — the trait whose `type Owned` is bound by this one
- [How to learn `ToOwned`](../how_to_learn_to_owned/README.md) — [step 7 of the path](../how_to_learn_to_owned/borrow_the_way_back/README.md) sends you here, after running a `Cow`'s read and write
- [Reading the `ToOwned` docs](../reading_the_to_owned_docs/README.md) — `Borrow` is dyn compatible and `ToOwned` is not, run side by side
- [`Cow`: borrow until somebody writes](../../18_Ownership/clone_on_write/README.md) — the type that needs both traits
- [`str::as_str`](../../14_Strings/str_as_str/README.md) — `AsRef`, the looser trait, and why it is not reflexive
- [Implementing `ToOwned` for your own type](../implementing_to_owned/README.md) — an unsized borrowed type whose `Borrow` impl keeps the promise
- [Method resolution](../method_resolution/README.md) — why the trait method beats `RefCell::borrow` in the second trap

## Sources

[`Borrow` in the standard library ↗](https://doc.rust-lang.org/std/borrow/trait.Borrow.html), whose description has the `HashMap` sketch and the case-insensitive key this page runs. [`HashMap::get` ↗](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.get) and [`BTreeMap::get` ↗](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.get) for the bounds. [`AsRef` ↗](https://doc.rust-lang.org/std/convert/trait.AsRef.html) for the comparison.

## Po polsku

`Borrow<Q>` mówi dwie rzeczy naraz. Pierwsza jest oczywista: wartość posiadana potrafi pożyczyć się jako `&Q` — `String` jako `&str`. Druga jest ważniejsza i nie sprawdza jej żaden kompilator: obie strony muszą się tak samo porównywać (`Eq`, `Ord`) i tak samo haszować (`Hash`). Dzięki tej obietnicy `HashMap<String, u32>::get("Ada")` przyjmuje zwykłe `&str` i nie buduje nowego `String` tylko po to, żeby coś wyszukać.

Pułapka numer jeden: implementację łamiącą tę obietnicę da się napisać i skompilować. Klucz porównywany bez względu na wielkość liter, z dopisanym `impl Borrow<str>`, znajduje `"cherry"`, ale nie znajduje `"Banana"` — dokładnie tego klucza, który został wstawiony. Dlatego `String` implementuje `AsRef<[u8]>`, a `Borrow<[u8]>` już nie: te same bajty jako `str` i jako `[u8]` dają różne hasze. Zasada wyboru: `AsRef`, gdy wystarczy tani widok; `Borrow`, gdy widok porównuje się i haszuje tak jak właściciel.

Pułapka numer dwa: po `use std::borrow::Borrow;` wywołanie `.borrow()` na `Rc<RefCell<T>>` trafia w metodę z cechy (*trait*), a nie w `RefCell::borrow`, bo wyszukiwanie metody sprawdza `&Rc<…>`, zanim zejdzie do `RefCell`. Kompilator zgłasza wtedy `E0282` bez słowa o `RefCell`. Najprostsza naprawa: usunąć niepotrzebny `use` albo napisać `(*scores).borrow()`.

**Szukaj po polsku:** cecha Borrow w Ruscie · wyszukiwanie w HashMap po &str · `rust Borrow vs AsRef` · `rust HashMap get &str String key` · `rust RefCell borrow type annotations needed`
