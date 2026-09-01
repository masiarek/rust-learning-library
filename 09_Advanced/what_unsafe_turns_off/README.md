# What `unsafe` turns off

**Level:** 301 · deep dive

**One line:** `unsafe` grants five specific powers and turns off nothing else — the borrow checker, type checking, lifetimes and move semantics are all still on inside the block, which is why "unsafe Rust" is not "C with Rust syntax".

The list is short enough to memorise:

| Inside `unsafe` you may | |
|---|---|
| dereference a raw pointer | `*p` |
| call an `unsafe fn` | including every FFI function |
| implement an `unsafe trait` | `Send`, `Sync`, and your own |
| read a field of a `union` | [what a union is](../what_a_union_is/README.md) |
| access or modify a mutable `static` | and in edition 2024, even `&COUNT` is refused |

That is all of it. Every other rule in the language applies unchanged.

## Making a raw pointer is safe; using one is not

```rust
fn main() {
    let x = 5u32;
    let p: *const u32 = &x;      // safe — no unsafe needed
    println!("{}", p.is_null());  // false
    println!("{}", unsafe { *p }); // 5
}
```

Creating a pointer cannot break anything, so it is not guarded. **Dereferencing** it is where a promise gets made, and that is the operation the keyword sits on. Without the block it is `E0133`, with a note worth reading: *"raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior"*.

## What the block is actually for

```rust
fn split_at_mut(v: &mut [u32], mid: usize) -> (&mut [u32], &mut [u32]) {
    let len = v.len();
    assert!(mid <= len, "mid must be within the slice");
    let ptr = v.as_mut_ptr();
    // SAFETY: mid <= len, checked directly above, so the two ranges are
    // disjoint and both lie inside the one allocation we hold a &mut to.
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```

Safe Rust cannot express this: the borrow checker sees two `&mut` borrows of one slice and refuses, because it cannot know the ranges are disjoint. The `assert!` is the proof it lacks. std's own `slice::split_at_mut` is written the same way, which is the general shape — **a safe API with a small unsafe core and a written-down reason.**

## `unsafe fn` and `unsafe {}` are two different jobs

| | Says |
|---|---|
| `unsafe { … }` | *I have checked the contract here* |
| `unsafe fn` | *calling me has a contract — read the `# Safety` doc* |

In edition 2024 the body of an `unsafe fn` is **no longer implicitly an unsafe block**, so the two are finally separate. Omit the inner block and rustc says so, in the sentence that explains the whole change:

```text
warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
note: an unsafe function restricts its caller, but its body is safe by default
  = note: `#[warn(unsafe_op_in_unsafe_fn)]` (part of `#[warn(rust_2024_compatibility)]`) on by default
```

## The trap: the unit of review is the module, not the block

`split_at_mut` is sound **because of the `assert!`** — safe code, three lines above. Delete it and the unsafe block is unchanged, its SAFETY comment still there, and now unsound.

So an `unsafe` block's correctness can depend on any safe code that can reach the same private state, and that is every line in its module. Two things follow, and both are actionable:

- **Keep a module containing `unsafe` small** — small enough to read in one sitting, because that module *is* the audit.
- **Write the `// SAFETY:` comment**, naming the invariant, so the person deleting an assert three lines away has something to notice.

The kata below builds a type whose unchecked read is safe because a private index is always in range — and then puts a perfectly ordinary `pub fn` in the same module that breaks it, with no `unsafe` at the call site.

## And what it still does not buy you

```rust
fn main() {
    let mut v = vec![1u32, 2, 3];
    unsafe {
        // let a = &mut v; let b = &mut v;   // still E0499
        v.push(4);
    }
    println!("{v:?}");   // [1, 2, 3, 4]
}
```

Inside an `unsafe` block, a second `&mut` is still `E0499`, a use after move is still `E0382`, and a reference still cannot outlive its referent. `unsafe` relaxes not one rule of the borrow checker: it moves five specific obligations from the compiler to you, and leaves everything else exactly where it was.

## If you are coming from another language

- **Python.** The nearest thing is `ctypes` or a C extension: a boundary where the interpreter's guarantees stop and you are responsible instead. What does not transfer is the *scale* — in Python that boundary is a whole extension module, and in Rust it is five operations inside a block, with the rest of the language still checking you. The other useful comparison is `__slots__`-style micro-optimisation: reaching for `unsafe` for speed, before measuring, is the same mistake, and the cost is worse here because the failure is undefined behaviour rather than a wrong answer.
- **ABAP.** There is no `unsafe`, and the closest analogue is `ASSIGN … CASTING` on a field symbol, or `SYSTEM-CALL` — places where the type system stops checking and a wrong assumption corrupts memory rather than raising. The transferable habit is the one good ABAP developers already have around `ASSIGN`: check `sy-subrc`, keep the block tiny, and comment what you assumed about the source field's length. That is precisely the `// SAFETY:` discipline, and this page's argument is that the comment should name an invariant somebody else could break rather than describing what the line does.
- **C.** Every line is what an `unsafe` block is here, which is the whole point of the comparison: Rust's version is opt-in, greppable, and small. The one genuine surprise for a C programmer is that the borrow checker keeps running inside the block — `unsafe` is not an escape hatch to C semantics, it is five extra verbs.
- **C#.** `unsafe` and `fixed` are the direct counterparts, including the block syntax and the requirement to opt in at the project level. C#'s version disables the GC's ability to move an object; Rust's disables nothing at all.

---

## The verified output

<!-- output:what_unsafe_turns_off -->
*Verified output of [`what_unsafe_turns_off.rs`](examples/what_unsafe_turns_off.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The five things `unsafe` lets you do, and nothing else
   a. dereference a raw pointer
   b. call an `unsafe fn`
   c. implement an `unsafe trait`  (Send, Sync, and your own)
   d. read a field of a `union`
   e. access or modify a mutable `static`
   That is the whole list. Everything else — the borrow checker,
   type checking, lifetimes on references, move semantics — is still
   on inside an `unsafe` block. It is not "turn off Rust".

2. Making a raw pointer is safe; using one is not
   let p: *const u32 = &x;   <- safe, no unsafe needed
   p.is_null() = false
   *p                        <- E0133 without an unsafe block
   unsafe { *p } = 5
   Creating a pointer cannot break anything. DEREFERENCING one is
   where the promise is made, which is why that is the operation the
   keyword guards.

3. What the block is actually for
   split_at_mut(&mut scores, 3) -> two &mut into one array
   after writing through both: [50, 3, 0, 40, 2, 1]
   Safe Rust cannot express this: the borrow checker sees two &mut
   borrows of the same slice and refuses, because it cannot know the
   ranges are disjoint. The `assert!` above is the proof it lacks —
   and the SAFETY comment says which invariant the block depends on.
   (std has this as slice::split_at_mut, written the same way.)

4. `unsafe fn` versus `unsafe {}` — two different jobs
   unsafe { … }  I have checked the contract here.
   unsafe fn      CALLING me has a contract; read the # Safety doc.
   In edition 2024 the body of an `unsafe fn` is no longer implicitly
   an unsafe block, so the two are finally separate. Omit the inner
   block and rustc says so, with the sentence that explains the whole
   change: "an unsafe function restricts its caller, but its body is
   safe by default" — the `unsafe_op_in_unsafe_fn` lint, warn by
   default under rust_2024_compatibility.
   get_unchecked(&scores, 1) = 3

5. The unit of review is the module, not the block
   `split_at_mut` is sound because of the `assert!` three lines above
   the block — safe code. Delete the assert, and the unsafe block is
   unchanged and now unsound. So an `unsafe` block's correctness can
   depend on any safe code that can reach the same private state,
   which is every line in its module.
   Two consequences worth acting on: keep modules containing `unsafe`
   SMALL, and write a `// SAFETY:` comment naming the invariant, so
   the next reader knows what they must not break.

6. And what it still does not buy you
   v = [1, 2, 3] — inside an unsafe block, `let a = v; let b = v;` is
   still E0382 use-after-move; `&mut v` twice is still E0499; a
   reference still cannot outlive its referent. `unsafe` does not
   relax one rule of the borrow checker. It moves five specific
   obligations from the compiler to you, and leaves the rest.
```
<!-- /output -->

## Practice

**The safe line the unsafe block depends on.** Write `split_at_mut` yourself and check it against std's on the same array. Then hand it a `mid` past the end: the `assert!` turns that into a panic, so catch it and print the message. Now say what the *same block* does if you delete the assert — and why there is no output to show for that case.

Then build a small type with a private index and an unchecked read, so that the read is safe to call because two other methods maintain `head < len`. Add an ordinary `pub fn set_head(&mut self, head: usize)` in the same module. Call it with a valid value, note that no `unsafe` was needed at the call site, and then say what one different literal would do. Give three fixes and pick one.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:what_unsafe_turns_off_kata -->
*[`what_unsafe_turns_off_kata.rs`](examples/what_unsafe_turns_off_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: the safe line the unsafe block depends on.
//!
//!   rustc --edition 2024 what_unsafe_turns_off_kata.rs -o /tmp/uk && /tmp/uk

mod ring {
    /// A fixed ring buffer whose reads skip the bounds check.
    pub struct Ring {
        slots: [u32; 4],
        head: usize,   // INVARIANT: head < slots.len()
    }

    impl Ring {
        pub fn new(slots: [u32; 4]) -> Self {
            Ring { slots, head: 0 }
        }

        /// Safe: it maintains the invariant itself.
        pub fn advance(&mut self) {
            self.head = (self.head + 1) % self.slots.len();
        }

        /// The unchecked read the whole type exists for.
        pub fn current(&self) -> u32 {
            // SAFETY: `head` is always < slots.len(), maintained by `new` and
            // `advance`, the only two places that write it.
            unsafe { *self.slots.as_ptr().add(self.head) }
        }

        /// The door. A SAFE function, in the same module, that can break the
        /// invariant the unsafe block above depends on.
        pub fn set_head(&mut self, head: usize) {
            self.head = head;
        }

        /// What it should have been.
        pub fn try_set_head(&mut self, head: usize) -> bool {
            if head < self.slots.len() {
                self.head = head;
                true
            } else {
                false
            }
        }
    }
}

use ring::Ring;

fn split_at_mut(v: &mut [u32], mid: usize) -> (&mut [u32], &mut [u32]) {
    let len = v.len();
    assert!(mid <= len, "mid must be within the slice");
    let ptr = v.as_mut_ptr();
    // SAFETY: mid <= len, checked directly above, so the two ranges are
    // disjoint and both lie inside the one allocation we hold a &mut to.
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn caught(f: impl FnOnce() + std::panic::UnwindSafe) -> String {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let r = std::panic::catch_unwind(f);
    std::panic::set_hook(hook);
    match r {
        Ok(()) => "(no panic)".into(),
        Err(e) => e
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| e.downcast_ref::<&str>().map(|s| (*s).to_string()))
            .unwrap_or_else(|| "(non-string panic)".into()),
    }
}

fn main() {
    println!("1. Your split_at_mut, checked against std's");
    let mut a = [5u32, 3, 0, 4, 2, 1];
    let mut b = a;
    let (l1, r1) = split_at_mut(&mut a, 3);
    let (l2, r2) = b.split_at_mut(3);
    println!("   mine: {l1:?} | {r1:?}");
    println!("   std : {l2:?} | {r2:?}");
    println!("   agree: {}", l1 == l2 && r1 == r2);

    println!();
    println!("2. The safe line the unsafe block depends on");
    println!("   split_at_mut(&mut a, 99) -> {}", caught(|| {
        let mut v = [1u32, 2, 3];
        let _ = split_at_mut(&mut v, 99);
    }));
    println!("   The assert turned a bad argument into a panic. DELETE it and the");
    println!("   same unsafe block — unchanged, still with its SAFETY comment —");
    println!("   builds a slice out of memory the allocation does not own. That is");
    println!("   undefined behaviour, not a panic and not a wrong number: the");
    println!("   Reference lists producing an invalid value as UB, and there is no");
    println!("   defined outcome to demonstrate.");

    println!();
    println!("3. The same shape, one module wide");
    let mut r = Ring::new([10, 20, 30, 40]);
    println!("   current() = {}", r.current());
    r.advance();
    println!("   after advance(), current() = {}", r.current());
    println!("   `current` reads without a bounds check, and is SAFE to call,");
    println!("   because `head < 4` is maintained by `new` and `advance`.");

    println!();
    println!("4. And the door in the same module");
    r.set_head(0);   // 0 happens to be in range; 99 would compile identically
    println!("   `pub fn set_head(&mut self, head: usize)` is a safe function that");
    println!("   assigns head directly. set_head(0) is called above and is fine —");
    println!("   `current()` = {} — and `r.set_head(99)` compiles exactly the same,", r.current());
    println!("   needs no unsafe at the call site, and makes the NEXT `current()`");
    println!("   read out of bounds. The unsafe block is unchanged and now unsound,");
    println!("   and the difference between the two calls is one literal.");
    println!("   So `set_head` is wrong in one of two ways, and both fixes are one");
    println!("   line:");
    println!("     - make it private, or");
    println!("     - make it check: try_set_head(9) = {}, try_set_head(2) = {}",
             r.try_set_head(9), r.try_set_head(2));
    println!("   current() after try_set_head(2) = {}", r.current());
    println!("   Marking it `unsafe fn` is the third option, and the honest one");
    println!("   when the caller genuinely can know more than the type does.");

    println!();
    println!("5. What this makes the unit of review");
    println!("   Not the `unsafe` block: every line that can reach the same");
    println!("   private state. `slots` and `head` are private, so the audit is");
    println!("   the MODULE — and that is the argument for keeping a module that");
    println!("   contains `unsafe` small enough to read in one sitting.");
    println!("   The `// SAFETY:` comment is the other half: it names the");
    println!("   invariant, so the person deleting an assert three lines away has");
    println!("   something to notice.");
}
```
<!-- /source -->

<!-- output:what_unsafe_turns_off_kata -->
*Verified output of [`what_unsafe_turns_off_kata.rs`](examples/what_unsafe_turns_off_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Your split_at_mut, checked against std's
   mine: [5, 3, 0] | [4, 2, 1]
   std : [5, 3, 0] | [4, 2, 1]
   agree: true

2. The safe line the unsafe block depends on
   split_at_mut(&mut a, 99) -> mid must be within the slice
   The assert turned a bad argument into a panic. DELETE it and the
   same unsafe block — unchanged, still with its SAFETY comment —
   builds a slice out of memory the allocation does not own. That is
   undefined behaviour, not a panic and not a wrong number: the
   Reference lists producing an invalid value as UB, and there is no
   defined outcome to demonstrate.

3. The same shape, one module wide
   current() = 10
   after advance(), current() = 20
   `current` reads without a bounds check, and is SAFE to call,
   because `head < 4` is maintained by `new` and `advance`.

4. And the door in the same module
   `pub fn set_head(&mut self, head: usize)` is a safe function that
   assigns head directly. set_head(0) is called above and is fine —
   `current()` = 10 — and `r.set_head(99)` compiles exactly the same,
   needs no unsafe at the call site, and makes the NEXT `current()`
   read out of bounds. The unsafe block is unchanged and now unsound,
   and the difference between the two calls is one literal.
   So `set_head` is wrong in one of two ways, and both fixes are one
   line:
     - make it private, or
     - make it check: try_set_head(9) = false, try_set_head(2) = true
   current() after try_set_head(2) = 30
   Marking it `unsafe fn` is the third option, and the honest one
   when the caller genuinely can know more than the type does.

5. What this makes the unit of review
   Not the `unsafe` block: every line that can reach the same
   private state. `slots` and `head` are private, so the audit is
   the MODULE — and that is the argument for keeping a module that
   contains `unsafe` small enough to read in one sitting.
   The `// SAFETY:` comment is the other half: it names the
   invariant, so the person deleting an assert three lines away has
   something to notice.
```
<!-- /output -->

</details>

---

## See also

- [What a union is](../what_a_union_is/README.md) — one of the five powers, in full
- [When the type checker is wrong](../../20_Compilers/when_the_type_checker_is_wrong/README.md) — the same word one layer up: soundness as a property of `rustc` rather than of your module, and the thirty times it did not hold
- [The global allocator](../the_global_allocator/README.md) — an `unsafe trait` implemented for real
- [Borrowing](../../18_Ownership/borrowing/README.md) — the rules that stay on inside the block
- [Arrays and slices](../../26_Collections/arrays_and_slices/README.md) — what `from_raw_parts_mut` is building
- [Modules and visibility](../../27_Modules/modules_and_visibility/README.md) — why the module is the audit unit
- [`const` and `static`](../../27_Modules/const_and_static/README.md) — `static mut`, and what replaced it

## Sources

[Unsafe Operations ↗](https://doc.rust-lang.org/rust-by-example/unsafe.html) in Rust by Example; the Reference's [behavior considered undefined ↗](https://doc.rust-lang.org/reference/behavior-considered-undefined.html) for what is actually at stake, and the edition guide's [unsafe_op_in_unsafe_fn ↗](https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html), which rustc's own note links to. [The Rustonomicon ↗](https://doc.rust-lang.org/nomicon/) is the long version of this page.

## Po polsku

Samo słowo `unsafe` jest pułapką akurat dla polskiego czytelnika: „niebezpieczny” brzmi jak ostrzeżenie przed czymś groźnym, a angielskie *unsafe* znaczy tu tylko tyle, że kompilator przestaje **sprawdzać** — ciężar dowodu przechodzi na ciebie, ale ani jedna reguła języka nie znika. To najczęstsze polskie nieporozumienie wokół tego słowa: `unsafe` **nie** wyłącza borrow checkera. Wewnątrz bloku drugie `&mut` to wciąż `E0499`, użycie po przeniesieniu własności to wciąż `E0382`, a referencja nadal nie może przeżyć tego, co pożyczyła. „Unsafe Rust” to nie „C ze składnią Rusta” — to pięć konkretnych uprawnień (dereferencja surowego wskaźnika, wywołanie `unsafe fn`, implementacja `unsafe trait`, odczyt pola unii, dostęp do mutowalnego `static`) i nic ponadto.

Podział między utworzeniem wskaźnika a jego użyciem jest tu ten sam, co przy uniach: **utworzenie surowego wskaźnika jest bezpieczne, użycie go nie**. `let p: *const u32 = &x;` nie wymaga żadnego bloku, bo samo utworzenie wskaźnika niczego nie psuje; dopiero `*p` jest obietnicą — i bez `unsafe` dostajesz `E0133`. Warto też rozdzielić dwie rzeczy, które po polsku łatwo się zlewają w jedno „niebezpieczne”: blok `unsafe { … }` mówi „sprawdziłem tutaj umowę”, a `unsafe fn` mówi „wywołanie mnie ma umowę, przeczytaj sekcję `# Safety`”. W edycji 2024 ciało `unsafe fn` **przestało** być domyślnie blokiem `unsafe` (lint `unsafe_op_in_unsafe_fn`), więc starsze polskie tutoriale opisują w tym miejscu stan nieaktualny; rustc streszcza całą zmianę jednym zdaniem: *„an unsafe function restricts its caller, but its body is safe by default”*.

Najważniejsza rzecz na tej stronie, a zarazem ta, która najrzadziej trafia do polskich omówień: **jednostką przeglądu jest moduł, a nie blok**. `split_at_mut` jest poprawne *dzięki* `assert!` trzy linijki wyżej — w zupełnie zwykłym, bezpiecznym kodzie. Skasuj ten `assert!`, a blok `unsafe` zostanie znak w znak taki sam, razem z komentarzem `// SAFETY:`, i od tej chwili będzie zły. Stąd dwie praktyczne konsekwencje: trzymaj moduł zawierający `unsafe` na tyle mały, żeby dało się go przeczytać za jednym posiedzeniem (bo to *on* jest audytem, nie blok), i pisz komentarz `// SAFETY:` tak, żeby nazywał **niezmiennik**, a nie opisywał, co robi linijka. Przy okazji jedna luka słownikowa, o której warto wiedzieć: polszczyzna nie ma ustalonego odpowiednika pary *sound / unsound* — „poprawny” gubi cały sens, którym jest „bezpieczne API nie pozwala bezpiecznemu kodowi wywołać niezdefiniowanego zachowania”. Tego pojęcia szukaj po angielsku.

**Szukaj po polsku:** surowe wskaźniki · niezdefiniowane zachowanie · niezmiennik · `rust unsafe superpowers` · `rust soundness unsafe` · `unsafe_op_in_unsafe_fn`
