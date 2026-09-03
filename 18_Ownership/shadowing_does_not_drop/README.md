# A shadow does not drop

**Level:** 201 · working knowledge

**One line:** Shadowing takes away a **name**, not a **value** — the shadowed `String` is still alive, still owned, and still borrowable, which is why a reference taken before the shadow keeps working after it.

[Shadowing and `unwrap`](../../17_Option_and_Result/shadowing_and_unwrap/README.md) covers what shadowing is *for*. This page covers what it does to the value underneath, because "the new one hides the old" leaves the interesting question unasked: what happened to the old one?

Nothing happened to it. That is the whole lesson, and everything below is the evidence.

---

## The program that raises the question

```rust
fn main() {
    let s = String::from("something important");
    let keep = &s;  // borrow the first `s`
    let s = 5;      // shadow it with an i32
    println!("s = {s}, but the String is still there: {keep}");
}
```

```text
s = 5, but the String is still there: something important
```

Two things had to be true for that line to print. The `String` was not **moved** — no ownership changed hands. And it was not **dropped** — its buffer is still allocated and still holds those bytes. All that changed is that the name `s` now means something else, so the `String` has become unreachable *by name* while remaining perfectly reachable through `keep`.

The name and the storage are separate things. Shadowing operates on the first one only.

## Where the value actually dies

You do not have to take that on trust. Give the value a `Drop` that prints and the question stops being a matter of opinion:

```rust
struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) { println!("  DROP {}", self.0); }
}

{
    let item = Noisy("first  — the shadowed one");
    let keep = &item;
    let item = Noisy("second — the shadow");
    println!("  both alive: keep -> {}, item -> {}", keep.0, item.0);
}
```

```text
  both alive: keep -> first  — the shadowed one, item -> second — the shadow
  DROP second — the shadow
  DROP first  — the shadowed one
```

Nothing drops at the `let`. Both values die at the closing brace, in **reverse declaration order** — so the shadow dies *first* and the value it hid dies *last*. The shadowed value outlived the thing that shadowed it.

## The cost: a shadowed value cannot be released early

This is the practical consequence, and it runs opposite to the intuition that shadowing "replaces" something.

```rust
let buffer = Noisy("the original — soon unreachable");
let buffer = Noisy("the shadow");
drop(buffer);   // drops the SHADOW: the name means the newest binding
```

```text
  DROP the shadow
  ...
  DROP the original — soon unreachable        ← at the closing brace, not before
```

Once shadowed, the original cannot be used, moved, **or dropped**, because every one of those needs a name and it no longer has one. So shadowing is not a way to release something early — it is the opposite: it removes the handle you would have needed. Shadow a 40 MB buffer with a summary of it near the top of a long function and the buffer is held for the whole function, with nothing in the code to suggest it.

The fix is to end its scope or move it, both of which the [Practice](#practice) below walks through.

## Two mechanisms that look alike

A same-scope shadow and an inner-block binding read almost identically and do different things:

```rust
let item = Noisy("outer");
{
    let item = Noisy("inner");   // a new binding in a new scope
}                                  // ← "inner" DIES here
println!("{}", item.0);          // "outer" was never touched
```

The inner binding is **destroyed** at its brace. A same-scope shadow merely goes nameless and lives on to the end of the enclosing scope. Both make the outer name unreachable for a stretch; only one of them frees anything.

## What the borrow checker is guaranteeing here

It is worth being precise about the promise, because it is narrower and more useful than "Rust is safe". Add a `drop` to the opening program and it stops compiling:

```text
error[E0505]: cannot move out of `s` because it is borrowed
 --> e0505.rs:4:10
  |
2 |     let s = String::from("something important");
  |         - binding `s` declared here
3 |     let keep = &s;
  |                -- borrow of `s` occurs here
4 |     drop(s);
  |          ^ move out of `s` occurs here
5 |     let s = 5;
6 |     println!("s = {s}, keep = {keep}");
  |                                ---- borrow later used here
```

The guarantee is not that shadowing is harmless. It is that **no reachable path frees the `String` while `keep` is still live** — and note the last line of that error, *"borrow later used here"*, which is the compiler working out how long `keep` matters and holding the value at least that long. The shadow on line 5 is not what it objects to; it never mentions it.

## What C and C++ do with the same program

Both halves of the Rust program have a counterpart, and both of them break — differently, which is what makes the comparison worth doing. Runnable versions of everything here sit beside this page: [`shadow.c`](c_comparison/shadow.c), [`shadow.cpp`](c_comparison/shadow.cpp), [`dangling.c`](c_comparison/dangling.c) and [`dangling.cpp`](c_comparison/dangling.cpp).

### C rejects the shadow outright

```c
char *s = strdup("something important");
char *keep = s;
int s = 5;      /* error: redefinition of 's' with a different type: 'int' vs 'char *' */
```

It is not the type change that does it, either — `int x = 1; int x = 2;` in one block is `error: redefinition of 'x'` just the same. **A name belongs to its block, once.** C++ answers identically.

To shadow at all you need new braces, and then both compilers treat it as a suspected mistake:

```c
{
    int s = 5;   /* warning: declaration shadows a local variable [-Wshadow] */
    printf("s = %d, but the string is still there: %s\n", s, keep);
}
```

That warning is the real difference in attitude. In C and C++, shadowing happens across a nesting boundary and is usually a bug. In Rust it happens inside one block, on purpose, and is idiomatic enough that `let x = x.trim().parse()?` is in the first chapter of the book.

### C++ gets the lifetime half right

`std::string` has a destructor, so C++ has genuine RAII and its values really do die at the closing brace. Run the `Noisy` demo from further up as a C++ struct and it prints the same order Rust does:

```text
  both alive: first  — declared first / second — declared second
  DROP second — declared second
  DROP first  — declared first
```

Reverse declaration order, at the brace. That is not a coincidence or an imitation — Rust's `Drop` and C++'s destructors are the same idea, and if you know RAII you already know most of what `Drop` does.

### Neither one checks the alias

Here is the half that has no Rust equivalent, because rustc rejects it:

```c
char *s = strdup("something important");
char *keep = s;
free(s);
printf("keep = %s\n", keep);       /* compiles clean under -Wall */
```

```cpp
const std::string* keep;
{
    std::string s = "something important";
    keep = &s;
}                                  // ~string() runs here
std::cout << *keep << "\n";        // compiles clean under -Wall
```

Both are undefined behaviour. Both produce **no diagnostic at all**. And on one run, on one machine, they printed this:

```text
C   ->  keep =
C++ ->  keep = something important
```

The C++ one returned the correct answer out of freed memory. That is the argument for the borrow checker in a single line: the failure mode is not "your program crashes", it is "your program is wrong and passes its test". Which of the two you get depends on the allocator, the optimiser, and what else has run since.

**These two programs are the only ones in this repository whose output is not a recorded answer key**, and they cannot be — undefined behaviour is by definition not reproducible, so pinning it would be asserting something the language does not promise. That limitation *is* the lesson, which is why they live outside the CI-verified [`examples/`](examples/shadowing_does_not_drop.rs) folder.

### To be fair to C and C++

They are not defenceless, and pretending otherwise would be cheating. Build either program with AddressSanitizer and both are caught immediately, with a better diagnosis than the phrase "undefined behaviour" suggests:

```text
dangling.c   ==> ERROR: AddressSanitizer: heap-use-after-free
                 READ of size 2 ... freed by thread T0 here: main dangling.c:17

dangling.cpp ==> ERROR: AddressSanitizer: stack-use-after-scope
                 Address ... is located in stack of thread T0 at offset 32
```

It even distinguishes the heap case from the stack case. So the honest comparison is not *checked* versus *unchecked* — it is **when**, and **on what**. ASan checks at run time, in a build you had to remember to make, on the inputs you actually ran. The borrow checker checks at compile time, in every build, on all inputs at once. The price Rust pays is that some correct programs are rejected too, and that price is real; the arrangement is a trade, not a free win.

## If you are coming from another language

- **Python.** `keep = s` then `s = 5` behaves almost exactly like the Rust program, and for a related reason: the name and the object are separate, and `keep` keeps the object alive. The mechanism is different — refcounting at run time versus a scope rule at compile time — and so is the failure you are protected from. Python cannot give you a dangling reference, but it pays for that with the counting; Rust proves it statically and the binary does no bookkeeping at all.
- **C / C++.** Covered in full above, and the short version is that C has no shadowing inside a block because there the name *is* the storage, while C++ has real destructors and so gets the *when does it die* half exactly right. What both are missing is any relation between how long `keep` lives and how long the value does.
- **ABAP.** There is no shadowing at all — a `DATA` name is one typed variable for the whole routine — and no scope-based destruction either, since a work area lives until the form ends and the garbage collector decides when an object dies. The nearest familiar shape is a field-symbol pointing at a table row after the table has been refreshed: same hazard as `keep`, and it surfaces as a dump at run time rather than an error at compile time.

---

## Practice

**The value you can no longer free.** Write a function that loads records into a buffer, shadows that name with a summary derived from it (a count, say), and then does a long stretch of unrelated work. Give the buffer a `Drop` that prints, and find out how long it actually stays alive.

Then make it die *before* the work starts — three ways, and decide which one you would ship.

Worth getting wrong on purpose: reach for `drop(buffer)` **after** the shadow and read carefully what got dropped. Then try it with a `usize` summary and again with a `String` one, and notice that whether the compiler warns you depends on the type of the thing you shadowed with.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:shadowing_does_not_drop_kata -->
*[`shadowing_does_not_drop_kata.rs`](examples/shadowing_does_not_drop_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: the value you can no longer free.
//!
//! A shadow removes a NAME. If the value behind that name is a 40 MB record
//! buffer and the shadow is a one-line summary of it, the buffer is still
//! there — held to the end of the scope, with nothing left that can reach it
//! to release it early. The instrument is the same as the lesson's: a value
//! that prints when it dies.
//!
//!   rustc --edition 2024 shadowing_does_not_drop_kata.rs -o /tmp/sdndk && /tmp/sdndk

struct Buffer {
    label: &'static str,
    bytes: Vec<u8>,
}

impl Buffer {
    fn load(label: &'static str) -> Self {
        Buffer { label, bytes: vec![5, 3, 0, 5, 4, 0, 2, 5] }
    }
    /// Borrows: the caller keeps the buffer.
    fn count_marked(&self) -> usize {
        self.bytes.iter().filter(|b| **b > 0).count()
    }
    /// Consumes: the buffer is moved in and dropped here.
    fn into_count_marked(self) -> usize {
        self.count_marked()
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        println!("    DROP {} ({} bytes freed)", self.label, self.bytes.len());
    }
}

fn long_unrelated_work(tag: &str) {
    println!("    ...doing {tag} work, during which nothing above is needed...");
}

fn banner(n: &str, title: &str) {
    println!("\n──── {n}: {title}");
}

fn main() {
    // ─────────────────────────────────────────────────────────── problem
    banner("The problem", "shadowed, so it cannot be released");
    {
        let records = Buffer::load("A: shadowed");
        let records = records.count_marked(); // usize now — buffer is nameless
        println!("    marked = {records}");
        long_unrelated_work("report");
        println!("    (the buffer is STILL alive right here)");
    }
    println!("      It was freed at the brace, after all the work — not after");
    println!("      the last line that needed it. Nothing was leaked; the");
    println!("      buffer was simply held far longer than the code implies.");

    // ─────────────────────────────────────────────────────────── 1
    banner("Fix 1", "drop it explicitly, BEFORE the shadow");
    {
        let records = Buffer::load("B: dropped early");
        let marked = records.count_marked();
        drop(records); // must happen while the name still means the buffer
        let records = marked;
        println!("    marked = {records}");
        long_unrelated_work("report");
    }
    println!("      Correct, and it relies on a human remembering. Move that");
    println!("      `drop` one line later and it silently drops the usize.");

    // ─────────────────────────────────────────────────────────── 2
    banner("Fix 2", "give it a scope that ends: the block expression");
    {
        let records = {
            let raw = Buffer::load("C: scoped");
            raw.count_marked()
        }; // raw dies HERE, structurally
        println!("    marked = {records}");
        long_unrelated_work("report");
    }
    println!("      The one to ship. The lifetime is stated by the shape of the");
    println!("      code rather than by a call you have to remember, and the");
    println!("      buffer is unreachable afterwards because it is out of scope,");
    println!("      not merely nameless.");

    // ─────────────────────────────────────────────────────────── 3
    banner("Fix 3", "hand it to something that consumes it");
    {
        let records = Buffer::load("D: moved in").into_count_marked();
        println!("    marked = {records}");
        long_unrelated_work("report");
    }
    println!("      Best when a real function already wants ownership: the move");
    println!("      makes the buffer the callee's problem, and it drops there.");

    // ─────────────────────────────────────────────────────────── 4
    banner("The trap", "drop() AFTER the shadow drops the wrong thing");
    {
        let records = Buffer::load("E: still held");
        // A String summary rather than a usize one — and that detail is the trap.
        let records = format!("{} marked", records.count_marked());
        drop(records); // a REAL drop, of the summary. Not a warning in sight.
        println!("    dropped `records`... and the buffer is still alive:");
        long_unrelated_work("report");
    }
    println!("      `drop` takes whatever the name means NOW, and after a shadow");
    println!("      that is the new binding. Here it freed a short String and");
    println!("      left the buffer untouched — it reads like a fix and is not");
    println!("      one.");
    println!("      Whether you get a warning depends on the shadow's type, which");
    println!("      is a thin thing to rely on. Shadow with a `usize` and rustc");
    println!("      catches it: `calls to std::mem::drop with a value that");
    println!("      implements Copy does nothing` (lint: dropping_copy_types).");
    println!("      Shadow with anything owned — a String, a Vec, a struct — and");
    println!("      the drop is genuine, so there is nothing for that lint to");
    println!("      say. The silent case is the one you will actually write.");

    println!("\n      The through-line: shadowing is a naming tool, not a memory");
    println!("      tool. When you want a value GONE, end its scope (fix 2) or");
    println!("      move it (fix 3); reach for `drop` only where neither shape");
    println!("      fits, and put it before the name changes meaning.");
}
```
<!-- /source -->

<!-- output:shadowing_does_not_drop_kata -->
*Verified output of [`shadowing_does_not_drop_kata.rs`](examples/shadowing_does_not_drop_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── The problem: shadowed, so it cannot be released
    marked = 6
    ...doing report work, during which nothing above is needed...
    (the buffer is STILL alive right here)
    DROP A: shadowed (8 bytes freed)
      It was freed at the brace, after all the work — not after
      the last line that needed it. Nothing was leaked; the
      buffer was simply held far longer than the code implies.

──── Fix 1: drop it explicitly, BEFORE the shadow
    DROP B: dropped early (8 bytes freed)
    marked = 6
    ...doing report work, during which nothing above is needed...
      Correct, and it relies on a human remembering. Move that
      `drop` one line later and it silently drops the usize.

──── Fix 2: give it a scope that ends: the block expression
    DROP C: scoped (8 bytes freed)
    marked = 6
    ...doing report work, during which nothing above is needed...
      The one to ship. The lifetime is stated by the shape of the
      code rather than by a call you have to remember, and the
      buffer is unreachable afterwards because it is out of scope,
      not merely nameless.

──── Fix 3: hand it to something that consumes it
    DROP D: moved in (8 bytes freed)
    marked = 6
    ...doing report work, during which nothing above is needed...
      Best when a real function already wants ownership: the move
      makes the buffer the callee's problem, and it drops there.

──── The trap: drop() AFTER the shadow drops the wrong thing
    dropped `records`... and the buffer is still alive:
    ...doing report work, during which nothing above is needed...
    DROP E: still held (8 bytes freed)
      `drop` takes whatever the name means NOW, and after a shadow
      that is the new binding. Here it freed a short String and
      left the buffer untouched — it reads like a fix and is not
      one.
      Whether you get a warning depends on the shadow's type, which
      is a thin thing to rely on. Shadow with a `usize` and rustc
      catches it: `calls to std::mem::drop with a value that
      implements Copy does nothing` (lint: dropping_copy_types).
      Shadow with anything owned — a String, a Vec, a struct — and
      the drop is genuine, so there is nothing for that lint to
      say. The silent case is the one you will actually write.

      The through-line: shadowing is a naming tool, not a memory
      tool. When you want a value GONE, end its scope (fix 2) or
      move it (fix 3); reach for `drop` only where neither shape
      fits, and put it before the name changes meaning.
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:shadowing_does_not_drop -->
*Verified output of [`shadowing_does_not_drop.rs`](examples/shadowing_does_not_drop.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── Step 1: The program that raises the question
  s = 5, but the String is still there: something important
      `keep` still reads it, so the String was neither moved nor
      freed. The shadow took away a NAME, not a value.

──── Step 2: Where the value actually dies
  both alive: keep -> first  — the shadowed one, item -> second — the shadow
      Nothing has dropped yet. Watch the brace:
  DROP second — the shadow
  DROP first  — the shadowed one
      Drop runs in REVERSE declaration order, so the shadow dies
      first and the value it hid dies last. The shadowed value
      outlived the thing that shadowed it.

──── Step 3: The cost: a shadowed value cannot be released early
  before the shadow, the original has a name: the original — soon unreachable
  DROP the shadow
  drop(buffer) took the shadow, as the name says it must.
      The original is still alive with no name left to reach
      it — it cannot be used, moved, or dropped until the brace:
  DROP the original — soon unreachable
      So shadowing is not a way to release something early. It is
      the opposite: it removes the handle you would have needed.

──── Step 4: An inner-block shadow ends; the outer value never moved
  inside the block -> inner
  DROP inner
  after the block  -> outer   the outer value was never touched
  DROP outer
      Two different mechanisms that look alike: the inner binding
      DIED at its brace, while a same-scope shadow merely goes
      nameless and lives to the end of the enclosing scope.

──── Step 5: What the borrow checker is guaranteeing while this happens
  s = 5, keep = something important
      Uncomment that `drop(s)` and rustc declines:
        error[E0505]: cannot move out of `s` because it is borrowed
      THAT is the guarantee. Not that shadowing is harmless, but
      that no reachable path frees the String while `keep` lives.

──── Step 6: The same shape in C, where the name IS the storage
  C rejects `int s = 5;` after `char *s` in one block outright:
        error: redefinition of 's' with a different type
  ...and it is not the type change. `int x = 1; int x = 2;` in one
  block is `error: redefinition of 'x'` too. A name belongs to its
  block, once. Shadowing needs new braces, and -Wshadow warns.
      C++ answers identically, and adds destructors — so a
      std::string really does die at its closing brace, the way
      Noisy does above. What neither language adds is Step 5:
      nothing checks that the alias dies before the value does.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 18_Ownership/shadowing_does_not_drop/examples/shadowing_does_not_drop.rs -o /tmp/sdnd && /tmp/sdnd
```

## Traps

- **Reading a shadow as a replacement.** It replaces a *name*. The old value is still allocated, still owned, and still dropping at the end of the scope — later than the shadow that hid it, not sooner.
- **Shadowing something large near the top of a long function.** The allocation is held to the closing brace with no handle left to release it. Bind it in an inner block, or move it into whatever consumes it.
- **Calling `drop(x)` after shadowing `x`.** It drops the new binding. If the shadow is `Copy` the [`dropping_copy_types` ↗](https://doc.rust-lang.org/rustc/lints/listing/warn-by-default.html) lint catches you; if it is a `String` or a `Vec` the drop is genuine, so nothing warns and the value you meant to free is untouched.
- **Expecting an inner-block shadow to behave like a same-scope one.** The inner binding is destroyed at its brace. The same-scope one is not destroyed at all until the enclosing scope ends.
- **Assuming rustc objected to the shadow.** When `E0505` or `E0502` arrives on a shadowed name, read the *"borrow later used here"* line: the complaint is about the borrow's extent, and the shadow is usually a bystander.

## See also

- [Shadowing and `unwrap`](../../17_Option_and_Result/shadowing_and_unwrap/README.md) — what shadowing is *for*, and the folklore that credits it for `Copy`'s work
- [When to shadow](../when_to_shadow/README.md) — where this page's mechanism turns into a rule: never shadow a value that holds a resource
- [Shadowing](../../SHADOWING.md) — the map of all three shadowing lessons and the pages that touch it
- [Ownership and moves](../ownership_and_moves/README.md) — who owes the free, and the `Drop` that makes this page's claims checkable
- [Borrowing](../borrowing/README.md) — `&T`, and the last-use rule that decides how long `keep` holds the value
- [The Rust Book, ch. 3.1 — Shadowing ↗](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing)
- [`std::mem::drop` ↗](https://doc.rust-lang.org/std/mem/fn.drop.html) — why "dropping" is just moving a value into a function that does nothing

## Po polsku

Przesłanianie zabiera **nazwę**, a nie **wartość**. Przesłonięty `String` dalej żyje, dalej jest posiadany i dalej można go pożyczać — dlatego referencja wzięta przed przesłonięciem działa również po nim.

To rozróżnienie ma konkretny koszt praktyczny: **przesłoniętej wartości nie da się zwolnić wcześniej**. Skoro nazwa zniknęła, nie ma jak wywołać na niej `drop()`, a wartość poczeka do końca bloku. Jeśli trzymała otwarty plik, blokadę albo połączenie, trzyma je dalej. Stąd rada, żeby nie przesłaniać niczego, co posiada zasób.

Dwa mechanizmy wyglądają tu podobnie i warto je rozdzielić po polsku: **przypisanie** (`s = String::from("nowy")`) *wypuszcza* starą wartość i wstawia nową w to samo miejsce — jeśli typ ma `Drop` z efektami ubocznymi, odpalą się właśnie tam. **Przesłonięcie** (`let s = …`) nie wypuszcza niczego; buduje obok drugie miejsce.

Dla kogoś z C++ to zaskoczenie w drugą stronę: tam wyjście nazwy z zasięgu i wywołanie destruktora to jedno zdarzenie. W Ruscie to dwie rzeczy, które zwykle zachodzą razem, ale nie muszą.

**Szukaj po polsku:** przesłanianie a wypuszczanie zasobów · kolejność wypuszczania · `rust drop order` · `rust shadowing does not drop`
