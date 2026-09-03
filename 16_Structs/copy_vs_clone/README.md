# `Copy` vs `Clone`

**Level:** 101 → 201 · working knowledge

**One line:** `Clone` is a method you call. `Copy` changes what `let b = a;` means. A struct is never `Copy` by accident.

```rust
let b = a;   // moves or copies. Nothing here says which; the type decides.
```

| | `Clone` | `Copy` |
|---|---|---|
| how you use it | `a.clone()` — you write it | nothing; `=` stops moving |
| what it may do | run your code, allocate | a bit-for-bit `memcpy`, always |
| after `let b = a;` | `a` is **moved** — dead | `a` is **copied** — alive |
| visible in the source | yes | **no** |

The last row is why they are separate traits. `Copy` hides duplication, which is only safe when duplication is trivial.

---

## `Copy` is not "cheap to duplicate"

It is **"duplicating is just a `memcpy`, and afterwards nobody owns anything extra."**

One `String` field disqualifies the whole struct. Copying it bit-for-bit duplicates the *pointer*: two owners, two frees. Not discouraged — unrepresentable.

## "`Copy` is shallow, `Clone` is deep" — both halves are false

A framing borrowed from languages where the choice really is how far a copy reaches. It gets both traits backwards.

**`Copy` is not a reference to the original.** It is a `memcpy`, so the two values are separate memory that happens to hold the same bits. If `let two = one;` produced an alias, editing `two` would change `one` — that is Python's `b = a`, and it is what `Copy` exists *not* to be.

```rust
let mut two = one;      // Copy
two.y = 999;   // one.y is still 100
```

**`Clone` is not a promise of depth.** It is a promise of an independently *owned* value, and how far that reaches is the type's business. `Rc::clone` copies a pointer and bumps a count — the shallowest clone there is, and the idiomatic one in any shared-ownership code:

```rust
let counted = Rc::clone(&shared);   // Rc::ptr_eq(&shared, &counted) is true
let independent = (*shared).clone();  // this is the one that duplicates the data
```

`String::clone` allocates, `Rc::clone` does not, `&T::clone` copies a pointer — one trait, three depths. Nothing in the trait says which you get, so read the type, not the method name. ([`ToOwned`](../../12_Traits/to_owned/README.md) is where that `Rc` trap bites hardest.)

The performance claim rides along with it — *`Copy` is faster than `Clone`, because cloning allocates.* A derived `Clone` on a `Copy` type is the same `memcpy` the `=` was already doing, and `one.clone()` on a `Point` allocates nothing. Allocation is a property of `String` and `Vec`, not of either trait.

Depth is not the axis. **Who asks** is: the compiler, silently, at every `=`; or you, in writing, at one call site.

## Three refusals, three codes

```text
error[E0277]: the trait bound `P: Clone` is not satisfied     // Copy requires Clone
error[E0204]: the trait `Copy` cannot be implemented for this type
  |  struct P { s: String }
  |         ^   --------- this field does not implement `Copy`
error[E0184]: `Copy` not allowed on types with destructors    // Drop runs once per value
```

Always `#[derive(Clone, Copy)]` together. `E0204` points at the offending field, which is the fastest way to see what a type owns.

`rustc --explain E0204` then gets its own rule wrong, in both directions: *"The `Copy` trait is implemented by default only on primitive types. If your type only contains primitive types, you'll be able to implement `Copy` on it. Otherwise, it won't be possible."* A `&String` field is `Copy` and `String` is not primitive; a struct whose only field is another `Copy` struct is `Copy` and contains no primitive either. And going the other way, a struct holding one `u32` plus a `Drop` impl contains nothing *but* a primitive and still cannot be `Copy` — that is `E0184`, the third code above. The rule is **every field `Copy`, no destructor, and you opted in**; "primitive" is a description of the common case that leaked into the definition.

## All-`Copy` fields is not enough

```rust
#[derive(Debug, Clone)]   // no Copy
struct Counter { count: u32 }
```

One `u32`, still moves. You opt in, not the fields. `Copy` is a promise to callers that adding a `String` later would silently break.

## Two ways to write it, and the bound the derive adds

`#[derive(Clone, Copy)]` is one of two spellings. The other is the two impls by hand — and a `Copy` type's `Clone` body is always the same three characters, because the compiler is already copying the bits:

```rust
impl Copy for MyStruct {}
impl Clone for MyStruct {
    fn clone(&self) -> Self { *self }   // there is nothing else it could be
}
```

On a *generic* type the two are not interchangeable. The derive writes a bound you did not:

```rust
#[derive(Clone, Copy)]        //  ->  impl<T: Copy> Copy for Derived<T>
struct Derived<'a, T>(&'a T);
```

That bound is about `T`, but the field is `&T`, and a shared reference copies whatever `T` is. So `Derived<'_, Message>` refuses to copy for a reason that appears nowhere in the code you wrote:

```text
error[E0382]: borrow of moved value: `d`
  move occurs because `d` has type `Derived<'_, Message>`, which does not implement the `Copy` trait
  note: derived `Clone` adds implicit bounds on type parameters
  help: consider manually implementing `Clone` to avoid undesired bounds
```

The compiler names the fix in its own `help`. Written by hand there is no bound to meet, and the same struct copies:

```rust
impl<T> Copy for Manual<'_, T> {}
impl<T> Clone for Manual<'_, T> { fn clone(&self) -> Self { *self } }
```

This is the general rule — [every derive bounds the type parameters](../../27_Modules/what_an_attribute_is/README.md) — landing where it costs most, because `Copy` is the one trait whose absence changes what `=` means. A `PhantomData` marker field hits it the same way, and for the same reason: it is duplicable regardless of the `T` it is standing in for.

## `&T` is `Copy`. `&mut T` is not

Both are one word wide, both are pointers, and only one of them duplicates:

```rust
let s = &n;      let s2 = s;   // s is still usable
let r = &mut m;  let r2 = r;   // r is MOVED
```

```text
error[E0382]: borrow of moved value: `r`
  move occurs because `r` has type `&mut u32`, which does not implement the `Copy` trait
```

That is the [borrowing](../../18_Ownership/borrowing/README.md) rule restated as traits. `&T` may be duplicated because nobody may write through it; `&mut T` may not, because being the only one is the entire content of the type.

Most people never meet that error, because passing a `&mut` to a function **reborrows** instead of moving — `bump(r); bump(r);` compiles and `r` is alive afterwards. It is the plain `let` binding that moves. When you want the reborrow at a `let`, write it: `let r3 = &mut *r;`.

## Which to reach for

1. **A reference.** Most "I need `Copy`" is really "my signature should take `&`".
2. **`Copy`** for small plain data — ids, coordinates, counters — where `&` everywhere is noise.
3. **`.clone()`** last, justified. A real allocation, written where it happens.

[Struct update syntax](../struct_update/README.md) shows it sharpest: one `..base` line copies the `Copy` fields and moves the rest, out of the same value.

## If you are coming from another language

**Python.** Every name binds a reference, so `b = a` never duplicates and this page's question does not arise.

| Python | | Rust |
|---|---|---|
| `b = a` | a reference; both usable | a **move**; `a` is dead |
| `copy.copy` / `copy.deepcopy` | explicit | `.clone()` |
| — | no equivalent | `Copy` |

The move is the new idea. `Clone` is `deepcopy` renamed, and `Copy` names the types where a move and a copy are indistinguishable.

**ABAP.** Structure and internal-table assignment copies deeply, always — every ABAP type behaves like a Rust `Copy` type, so two-owners-of-one-allocation never happens.

| ABAP | | Rust |
|---|---|---|
| `ls_b = ls_a.` | copy | move, unless the type is `Copy` |
| `REF TO` / `CREATE OBJECT` | the exception | the default for anything owning data |
| pass a ref, agree not to touch the original | convention | enforced by the compiler |

`Copy` is how you say "small enough that no agreement is needed".

**C++.** The only one of the three whose default is the *opposite* of Rust's, which makes it the most useful and the most dangerous bridge.

| C++ | | Rust |
|---|---|---|
| `auto b = a;` | the copy constructor runs, however expensive | a **move**, unless the type is `Copy` |
| `std::move(a)` | opt *in* to moving | nothing to opt into — it is the default |
| a moved-from object | valid but unspecified: still usable, still destroyed | gone; using it is `E0382` and no destructor runs |
| `= delete` the copy constructor | how you forbid copying | simply do not implement `Clone` |

Two traps for a C++ reader. `std::move` does not move anything — it is a cast, and if the type has no move constructor it silently selects the copy constructor instead, so the expensive thing you were avoiding happens anyway and nothing says so. And a moved-from C++ object is still *there*: reading it is legal and meaningless. Rust's moved-from binding is not in a bad state, it is not in any state — which is why Rust needs no "valid but unspecified" concept at all, and why the failure is a compile error rather than a convention you have to remember.

---

## Practice

**One `E0382`, three fixes, and the field that removes one.** Write `Reading { sensor: u32, millivolts: u32 }`, a function taking it **by value**, call it twice.

1. Fix three ways — derive `Copy`, `.clone()` at the call site, take `&Reading`. Rank them by what each costs the *caller*.
2. Change `sensor` to `String` and retry all three. One is impossible: get the error, explain it in one sentence about heap allocations.
3. Predict which of `#[derive(Clone)]`, `#[derive(Copy)]`, `impl Drop` can coexist. Check.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:copy_vs_clone_kata -->
*[`copy_vs_clone_kata.rs`](examples/copy_vs_clone_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: one E0382, three fixes, and the field that forbids one of them.
//!
//!   rustc --edition 2024 copy_vs_clone_kata.rs -o /tmp/cvck && /tmp/cvck

// ---- Round 1: all fields Copy, so all three fixes are available -----------
#[derive(Debug, Clone, Copy)]
struct Reading {
    sensor: u32,
    millivolts: u32,
}

fn report(r: Reading) -> String { format!("s{} reads {} mV", r.sensor, r.millivolts) }
fn report_ref(r: &Reading) -> String { format!("s{} reads {} mV", r.sensor, r.millivolts) }

// ---- Round 2: one String field, and Copy is now impossible ----------------
#[derive(Debug, Clone)]
struct Named {
    sensor: String, // <- this one field forbids Copy for the whole struct
    millivolts: u32,
}

fn name_it(n: Named) -> String { format!("{} reads {} mV", n.sensor, n.millivolts) }
fn name_it_ref(n: &Named) -> String { format!("{} reads {} mV", n.sensor, n.millivolts) }

fn main() {
    println!("The bug: calling a by-value function twice.");
    println!("    let r = Reading {{ .. }};");
    println!("    report(r); report(r);");
    println!("    error[E0382]: use of moved value: `r`\n");

    println!("Fix 1 — derive Copy. The call sites do not change at all.");
    let r = Reading { sensor: 7, millivolts: 431 };
    println!("    {}", report(r));
    println!("    {}   <- same `r`, copied again", report(r));

    println!("\nFix 2 — clone at the call site. Visible, and it costs.");
    println!("    {}", report(r.clone()));

    println!("\nFix 3 — take a reference. Nothing is duplicated at all.");
    println!("    {}", report_ref(&r));

    println!("\nWhich to ship? Fix 3, then Fix 1, and Fix 2 last.");
    println!("  A reference asks for the least and copies nothing, so it is the");
    println!("  default. Copy is right for a small, plain-data value where the");
    println!("  `&` would be noise. Clone at a call site is the one to justify:");
    println!("  it is a real allocation, and it is often papering over a signature");
    println!("  that should have taken `&` in the first place.");

    println!("\nRound 2 — swap `sensor` to a String, and one fix disappears.");
    let n = Named { sensor: "thermistor".to_string(), millivolts: 431 };
    println!("    #[derive(Copy)] is now:");
    println!("      error[E0204]: the trait `Copy` cannot be implemented for this type");
    println!("        this field does not implement `Copy`");
    println!("    ...because Copy is a bit-for-bit duplicate, and duplicating a");
    println!("    String's pointer would give two owners of one allocation.");
    println!("    That is the double-free `Copy` exists to make unrepresentable.");
    println!("\n    So only two fixes remain:");
    println!("      clone     {}", name_it(n.clone()));
    println!("      reference {}", name_it_ref(&n));
    println!("      and `n` is still alive: {n:?}");

    println!("\nThe rule to carry away:");
    println!("  Copy is not 'cheap to duplicate' — it is 'duplicating is JUST a");
    println!("  memcpy, with nobody owning anything afterwards'. Any field that");
    println!("  owns a resource makes that false for the whole struct.");
}
```
<!-- /source -->

<!-- output:copy_vs_clone_kata -->
*Verified output of [`copy_vs_clone_kata.rs`](examples/copy_vs_clone_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
The bug: calling a by-value function twice.
    let r = Reading { .. };
    report(r); report(r);
    error[E0382]: use of moved value: `r`

Fix 1 — derive Copy. The call sites do not change at all.
    s7 reads 431 mV
    s7 reads 431 mV   <- same `r`, copied again

Fix 2 — clone at the call site. Visible, and it costs.
    s7 reads 431 mV

Fix 3 — take a reference. Nothing is duplicated at all.
    s7 reads 431 mV

Which to ship? Fix 3, then Fix 1, and Fix 2 last.
  A reference asks for the least and copies nothing, so it is the
  default. Copy is right for a small, plain-data value where the
  `&` would be noise. Clone at a call site is the one to justify:
  it is a real allocation, and it is often papering over a signature
  that should have taken `&` in the first place.

Round 2 — swap `sensor` to a String, and one fix disappears.
    #[derive(Copy)] is now:
      error[E0204]: the trait `Copy` cannot be implemented for this type
        this field does not implement `Copy`
    ...because Copy is a bit-for-bit duplicate, and duplicating a
    String's pointer would give two owners of one allocation.
    That is the double-free `Copy` exists to make unrepresentable.

    So only two fixes remain:
      clone     thermistor reads 431 mV
      reference thermistor reads 431 mV
      and `n` is still alive: Named { sensor: "thermistor", millivolts: 431 }

The rule to carry away:
  Copy is not 'cheap to duplicate' — it is 'duplicating is JUST a
  memcpy, with nobody owning anything afterwards'. Any field that
  owns a resource makes that false for the whole struct.
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:copy_vs_clone -->
*Verified output of [`copy_vs_clone.rs`](examples/copy_vs_clone.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The difference is what `let b = a;` MEANS
   Point is Copy: after `let _also = p;`, p is fine -> Point { x: 7, y: 431 }
   Message is not:  after `let moved = b;`, `b` is E0382
                    the value lives on as `moved` -> Message { author: "Ada", bytes: [5, 2, 0] }
   Same syntax. Different meaning. The TYPE decides.

2. Passing to a function is the same question
   consume_point(p) = 438 (x+y), and p survives -> Point { x: 7, y: 431 }
   consume_message(moved) = "Ada sent 3 bytes" and `moved` does not survive
   ...which is why `.clone()` is in that line at all.

3. All-Copy fields is NOT enough — you have to opt in
   Counter holds one u32 and still is not Copy, because it does
   not derive Copy. consume_counter(t) MOVES it:
   consume_counter(t) = 12
   `t` is now dead. Opting in is deliberate: making a type Copy
   is a promise to your callers that you cannot quietly take back.

4. Clone is a method you call; Copy is something the compiler does
   original  Message { author: "Ben", bytes: [4] }
   duplicate Message { author: "Ben", bytes: [4] }   <- a second heap allocation, on purpose
   `Copy` never allocates: it is a bit-for-bit copy, nothing else.

5. The three refusals, each with its own code
   impl Copy for P {} without Clone
     error[E0277]: the trait bound `P: Clone` is not satisfied
     -> `Copy` requires `Clone`. Always derive both together.
   #[derive(Copy)] on a struct holding a String
     error[E0204]: the trait `Copy` cannot be implemented for this type
       this field does not implement `Copy`
   #[derive(Copy)] on a struct that also impls Drop
     error[E0184]: `Copy` not allowed on types with destructors
     -> a destructor runs once per value; copies would run it twice.

6. "Copy is shallow, Clone is deep" — both halves are false
   after copying `one` into `two` and editing `two`:
     one  Point { x: 3, y: 100 }
     two  Point { x: 3, y: 999 }
     same address? false   <- Copy duplicates the BITS, it never aliases
   Rc::clone(&shared):
     same allocation? true   strong_count 2
     (*shared).clone() gives a separate Vec? true
   So `Clone` promises a value you may keep — not a depth.
   `Rc` clones a pointer, `String` clones a buffer, and a Copy
   type's derived clone is the same memcpy `=` already does:
     one.clone() = Point { x: 3, y: 100 }   (no allocation, nothing to free)
   The axis is not deep vs shallow. It is WHO ASKS:
     Copy  the compiler, silently, at every `=`
     Clone you, in writing, at one call site

7. Two ways to write it — and the one difference
   The derive is the simple one, and it writes a bound you did not:
     #[derive(Clone, Copy)] struct Derived<T>(&T);
       ->  impl<T: Copy> Copy for Derived<T>
   That bound is about T, but the field is `&T`, and a shared
   reference copies whatever T is. So the derived version refuses
   a Derived<Message> that would have been perfectly fine:
     error[E0382]: borrow of moved value: `d`
       move occurs because `d` has type `Derived<'_, Message>`,
       which does not implement the `Copy` trait
       note: derived `Clone` adds implicit bounds on type parameters
       help: consider manually implementing `Clone` to avoid
             undesired bounds
   Writing the two impls by hand drops the bound:
     impl<T> Copy for Manual<'_, T> {}
     impl<T> Clone for Manual<'_, T> { fn clone(&self) -> Self { *self } }
   Manual<Message> copies: "Dev" and "Dev"
   Derived<u32> copies:    431 and 431   <- the bound is met here
   And a Copy type's Clone body is always `*self`. There is
   nothing else it could be: the compiler already copies the bits.

8. `&T` is Copy. `&mut T` is not — and the call site hides it
   shared:  let s = &n; let s2 = s;   both alive -> 7 7
   unique:  let r = &mut m; let r2 = r;   r is MOVED
     error[E0382]: borrow of moved value: `r`
       move occurs because `r` has type `&mut u32`,
       which does not implement the `Copy` trait
   Most people never meet that error, because passing `r` to a
   function REBORROWS instead of moving:
     bump(r); bump(r);   -> 9
     &mut *r, bumped     -> 10
   Two references, one Copy and one not, for the ownership reason:
   `&T` may be duplicated because nobody may write through it.
```
<!-- /output -->

## See also

- [STRUCTS.md](../../STRUCTS.md) · [Struct update syntax](../struct_update/README.md) · [Ownership and moves](../../18_Ownership/ownership_and_moves/README.md) · [Borrowing](../../18_Ownership/borrowing/README.md)
- [`std::marker::Copy` — *How can I implement `Copy`?* ↗](https://doc.rust-lang.org/std/marker/trait.Copy.html) — where the manual-impl pair and the derived-bound caveat above come from, stated in three sentences; and [`rustc --explain E0204` ↗](https://doc.rust-lang.org/error_codes/E0204.html) for the `&mut T` line and the over-tight "primitive types" wording
- [LogRocket — disambiguating Rust traits: `Copy`, `Clone` and `Dynamic` ↗](https://blog.logrocket.com/disambiguating-rust-traits-copy-clone-dynamic/) — where the deep/shallow framing above comes from, stated outright: *`Copy`* "creates a shallow copy, a new reference to the original value", *`Clone`* "creates a deep copy". Section 6 of the run below is that sentence tested. Its code says `#[Derive(Copy, Clone)]`, which does not compile — the attribute is lowercase
- [`Rc`: the clone that copies a pointer](../../18_Ownership/reference_counting/README.md) — the clone that duplicates a pointer and a count, not the data; and [`Arc`](../../18_Ownership/sharing_across_threads/README.md) for the same thing across threads
- [`Cow`: borrow until somebody writes](../../18_Ownership/clone_on_write/README.md) — the clone deferred until a write actually needs one
- [The `move` keyword](../../23_Closures/the_move_keyword/README.md) — the `move` closure that copies instead of moving, because what it captured was `Copy`: it compiles, runs, and changes nothing outside itself

## Sources

### The same word in four books

Four Manning books from 2024 all explain `clone`, and no two of them mean the same thing by it. Together they cover the trait; one at a time, each reads like the definition.

| Book | Where `clone` lives | What it means, there |
|---|---|---|
| **Idiomatic Rust** — Brenden Matthews | §10.5 *Too many clones* (p. 216); ch. 9, *Immutability* (p. 189) | Both verdicts, from one author. Ch. 10 files it as an antipattern — a crutch for not thinking about ownership — then points at ch. 9, where he recommends it as the simple way to get immutable data. The rule he actually gives is *informed and deliberate*, not *avoid*. |
| **Learn Rust in a Month of Lunches** — David MacLeod | §2.8 *Copy types* (p. 42); §11.4 `Cow` (p. 215); §11.5 `Rc` (p. 219); §12.3 `Arc` (p. 249) | Cost, and the way out of it. The only one of the four that answers "I am cloning too much" with a *type* rather than with advice: reach for `Rc`, whose clone copies the pointer and nothing else. |
| **Code Like a Pro in Rust** — Brenden Matthews | ch. 5, *Working with memory* — §5.3 *Deep copying* (p. 97), §5.4 *Avoiding copies* (p. 99), §5.6 *Reference counting* (p. 103), §5.7 *Clone on write* (p. 106) | Depth. The sharpest statement of the deep-copy reading: a derived `Clone` recurses, so one `.clone()` on a `Vec` duplicates everything inside it. |
| **Write Powerful Rust Macros** — Sam Van Overmeire | §5.1.2 *Recreating the struct*; §6.4.5 *An alternative approach* | Getting rid of it. `clone` shows up because `parse_macro_input!` moves the `TokenStream` and the macro still has to give it back; the builder chapter then starts at *clone everything*, finds that this restricts the macro to `Clone` or `Copy` types, and ends by consuming `self` instead — needing no clone at all. |

Only half of that is about copying data. `Rc`, `Arc` and `Cow` are all here because they *avoid* the copy, and Van Overmeire's chapters end at a design that stops asking for one — so a reader who arrives at `clone` expecting a duplicate meets three chapters in a row where nothing is duplicated.

### Where a flattened summary goes wrong

Side-by-side comparisons of these four books circulate, and they get this exact seam wrong. One credits *Idiomatic Rust* with a detailed `LinkedList` example demonstrating deep copying. The book's `LinkedList` is `Rc<RefCell<ListItem<T>>>`, built in ch. 3 to teach `Iterator` and `IntoIterator`; every `.clone()` in it — on `cur_iter`, on `head`, on `next` — bumps a reference count and copies no list data at all. It is the *shallowest* clone in the book, and the summary reports it as the deepest.

That is not a careless reading so much as the predictable one: `.clone()` is spelled identically whichever it does, so a summary written from the method name alone has nothing to go on. MacLeod's chapter gives the fix in one line of style — write `Rc::clone(&item)` rather than `item.clone()`, so the call says which thing is being cloned. Same code, same cost, and no longer misreadable.

## Po polsku

Polski czytelnik przychodzi tu zwykle z parą pojęć „kopia płytka” i „kopia głęboka”, wyniesioną z kursów Javy, C# czy Pythona — i to jest dokładnie ta intuicja, którą ta strona rozbraja, bo obie połowy są tu fałszywe. `Copy` **nie jest** referencją do oryginału: po `let mut two = one;` i `two.y = 999;` pole `one.y` nadal wynosi `100`, bo skopiowane zostały bity, a nie wskaźnik na wspólną wartość. Aliasem jest właśnie pythonowe `b = a`, czyli to, czym `Copy` z założenia **nie** jest. `Clone` z kolei nie obiecuje żadnej głębokości, tylko wartość **posiadaną niezależnie**, a jak daleko to sięga, zależy od typu: `Rc::clone` kopiuje wskaźnik i zwiększa licznik w zliczaniu referencji (*reference counting*), `String::clone` alokuje nowy bufor na stercie, a `clone` na `&T` kopiuje sam wskaźnik. Jedna cecha (*trait*), trzy różne głębokości — więc czytaj typ, nie nazwę metody. Prawdziwa oś podziału to nie głębokość, tylko **kto pyta**: kompilator, po cichu, przy każdym `=`, albo ty, na piśmie, w jednym miejscu wywołania.

Stąd druga poprawka do popularnego skrótu: `Copy` nie znaczy „tanio się kopiuje”, tylko „duplikat to sam `memcpy`, i po nim nikt nie jest właścicielem niczego dodatkowego”. Jedno pole typu `String` dyskwalifikuje całą strukturę, bo skopiowanie bit po bicie zduplikowałoby **wskaźnik**: dwóch właścicieli jednej alokacji i dwa zwolnienia tej samej pamięci (*double free*). To nie jest odradzane — tego po prostu nie da się wyrazić, a `E0204` wskazuje palcem konkretne winne pole. Uwaga na `rustc --explain E0204`, które samo formułuje regułę błędnie, i to w obie strony (mówi o typach prymitywnych, choć `&String` jest `Copy`, a struktura z jednym `u32` i własnym `Drop` nie jest — to trzeci kod, `E0184`). Reguła brzmi: **każde pole `Copy`, żaden destruktor, i świadome zgłoszenie się**. Samo posiadanie wyłącznie pól `Copy` nie wystarcza — `Counter` z jednym `u32` nadal się przenosi, bo `Copy` jest obietnicą złożoną wywołującym, której potem nie da się po cichu cofnąć. A ponieważ `Copy` wymaga `Clone` (inaczej `E0277`), pisze się je zawsze razem: `#[derive(Clone, Copy)]`.

Osobna pułapka dotyczy typów generycznych: `derive` dopisuje ograniczenie, którego nie napisałeś. Dla `struct Derived<'a, T>(&'a T)` powstaje `impl<T: Copy> Copy for Derived<T>`, a warunek dotyczy `T`, choć polem jest `&T` — referencja współdzielona kopiuje się niezależnie od tego, co za `T` w niej siedzi. Efekt: `Derived<'_, Message>` odmawia kopiowania z powodu, którego nie widać w żadnej napisanej przez ciebie linijce, a kompilator sam podpowiada wyjście (*consider manually implementing `Clone` to avoid undesired bounds*). Napisane ręcznie — `impl<T> Copy for Manual<'_, T> {}` plus `fn clone(&self) -> Self { *self }` — nie ma żadnego ograniczenia do spełnienia. Ciało `clone` w typie `Copy` zawsze wygląda tak samo, bo nic innego nie mogłoby tam być: kompilator i tak kopiuje bity.

Na koniec para, którą warto zapamiętać jako regułę pożyczania przepisaną na `trait`y: `&T` jest `Copy`, a `&mut T` nie. Oba mają szerokość jednego wskaźnika, ale wyłączność to **cała treść** `&mut T`, więc duplikat byłby zaprzeczeniem typu. Większość ludzi nigdy nie zobaczy tego `E0382`, bo przekazanie `&mut` do funkcji **pożycza ponownie** (*reborrow*) zamiast przenosić — `bump(r); bump(r);` kompiluje się bez szemrania — przenosi dopiero zwykłe `let r2 = r;`. Gdy chcesz ponownego pożyczenia przy `let`, zapisz je wprost: `let r3 = &mut *r;`. A kolejność sięgania po rozwiązania jest odwrotna do odruchu: najpierw referencja (większość „potrzebuję `Copy`” to w rzeczywistości „sygnatura powinna brać `&`”), potem `Copy` dla małych, zwykłych danych, a `.clone()` na końcu i z uzasadnieniem.

**Szukaj po polsku:** kopia płytka a kopia głęboka · przeniesienie własności · zliczanie referencji · `rust Copy vs Clone trait` · `rust E0204 Copy cannot be implemented`
