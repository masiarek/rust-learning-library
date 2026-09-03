# Temporary lifetime extension

**Level:** 301 · deep dive

**One line:** A temporary dies at the semicolon, except after a short list of shapes in a `let` that quietly keep it to the end of the block instead — and a function call is not on the list.

```rust
fn main() {
    let held = &String::from("kept");   // the temporary lives to the closing brace
    println!("{held}");                 // kept
}
```

Nothing owns that `String`. It is a temporary, and the ordinary rule would drop it at the `;` — leaving `held` pointing at freed memory, which is `E0716`. Instead the `&` in a `let` **extends** it: the value is given the same lifetime as `held` itself.

## The rule

The default is narrow and easy to state: **a temporary is dropped at the end of the statement that made it.** That is why `let _ = Guard::acquire();` releases the guard before the next line, which is the trap [`Drop` and RAII](../../12_Traits/drop_and_raii/README.md) opens on.

Extension applies only in a `let` (and a `static`/`const`), and only when the initializer is an **extending expression**:

| Initializer | Extended | Why |
|---|---|---|
| `&make()` / `&mut make()` | yes | a borrow expression |
| `Holder { r: &make() }` | yes | struct literal, extending operand |
| `(&make(), 1)` · `[&make()]` | yes | tuple and array literals likewise |
| `&make().field` | yes | field access chains through |
| `&pairs()[0]` | yes | indexing chains through |
| `make().name()` | **no** | a method call |
| `hold(&make())` | **no** | a function call |

The list is syntactic, not semantic. The compiler decides by looking at the shape of the expression you wrote, which is why the next section's two lines behave differently while doing the same thing.

## The pair that differs by nothing visible

```rust
let inline = Holder { r: &Noisy("H") };   // compiles — struct literal extends
// let called = hold(&Noisy("W"));        // E0716 — a call does not
```

```text title="Abridged — real rustc output for not_extended.rs"
error[E0716]: temporary value dropped while borrowed
  --> not_extended.rs:11:24
   |
11 |     let called = hold(&Noisy("W"));
   |                        ^^^^^^^^^^ - temporary value is freed at the end of this statement
   |                        |
   |                        creates a temporary value which is freed while still in use
12 |     println!("{} {}", inline.r.0, called.r.0);
   |                                   ---------- borrow later used here
   |
help: consider using a `let` binding to create a longer lived value
```

`hold` does nothing but build the same `Holder`. Wrapping the construction in a function is what removes the extension, and the `help:` line is the fix: give the temporary a name of its own, on its own line.

## Extending one field keeps the whole value

`&pair().first` does not extract a field and drop the rest. The `Pair` temporary is extended entire, and both its fields go out of scope together at the closing brace. Section 3 of the transcript below shows both drops arriving at once.

## A `match` holds its scrutinee through every arm

This is where the rule stops being trivia:

```rust
match registry.lock().unwrap().state() {   // guard alive for the whole match
    State::Ready => registry.lock().unwrap().start(),   // ...deadlock
    _ => {}
}
```

Temporaries in a scrutinee live until the end of the `match`, arms included. The same expression in a `let` releases at the `;`:

```rust
let state = registry.lock().unwrap().state();   // guard dropped here
```

**Edition 2024 changed the `if let` half of this**: the scrutinee's temporaries are now dropped *before* the `else` block runs, where in edition 2021 they lived to the end of the whole `if`/`else`. Same program, two editions:

```text title="Abridged — real runs of iflet_edition.rs under each edition"
--- 2024 ---            --- 2021 ---
before                  before
   [drop IF]               else branch
   else branch             [drop IF]
after                   after
```

## When there is nothing to extend

`&Quiet("Q")` needs no extension at all: with no `Drop` impl and no interior mutability, the value is **promoted** into an anonymous `static`, and the reference is `&'static`. Annotating it is the proof:

```rust
let promoted: &'static Quiet = &Quiet("Q");   // compiles: promotion, not extension
```

That is a different mechanism with a different rule — see [`const` and `static`](../../27_Modules/const_and_static/README.md). It matters here because it hides the lesson: an experiment written with a plain data struct will appear to extend everything, and only gains a `Drop` impl's honesty once there is something to drop.

## If you are coming from another language

- **C++.** This is the one place the two languages agree closely, and the C++ rule is older: binding a temporary to a `const T&` (or a `T&&`) extends its lifetime to the reference's. The traps are the same shape too — extending through a function return does not work in either language, and C++'s `const std::string& s = f().name();` dangles for the reason this page's `hold(&make())` does. What Rust adds is that the failure is `E0716` at compile time rather than a value that reads fine in testing.
- **Python.** Nothing corresponds, because nothing is destroyed at a syntactic boundary: a temporary lives while any reference to it does, and CPython's refcount decides that at run time. The Python habit to unlearn is `with` — `with lock:` makes the region explicit and visible, where Rust's guard is released by an invisible rule about *where the expression sat*. When you want the C++/Rust temporary to behave like a `with` block, give it a name.
- **ABAP.** The closest thing is the lifetime of an inline-declared result or a `NEW`-created object inside an expression: garbage collection decides, and there is no statement boundary at which anything is released. So the mistake this page is about cannot happen — and neither can the guarantee. `CLEANUP` and explicit `FREE` exist because ABAP has no deterministic destructor at all; Rust has one, and this page is the list of exceptions to *when* it fires.

## The verified output

<!-- output:temporary_lifetimes -->
*Verified output of [`temporary_lifetimes.rs`](examples/temporary_lifetimes.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. `&` in a let extends; anything else drops at the semicolon
   before both lets
      [drop B]
   after  both lets       -> A is A, B was B
   leaving the block
      [drop A]

2. A struct literal is an extending expression too
   Holder built inline    -> H
   leaving the block
      [drop H]

3. Extending one field keeps the WHOLE temporary alive
   made a Pair            -> P1 + P2
   &pair().first          -> P1
   leaving the block      -> both fields are still here
      [drop P1]
      [drop P2]

4. A match holds its scrutinee's temporaries through every arm
   entering the match
   inside the arm         -> M is still alive
      [drop M]
   left the match

5. ...and the same expression in a let does not
      [drop S]
   after the let          -> S was returned, S is already gone

6. An `if let` releases its scrutinee BEFORE the else — since edition 2024
   entering the if let
      [drop IF]
   else branch            -> IF is already gone
   left the if let

7. With no `Drop` and no interior mutability, there is nothing to extend
   &Quiet(..) as &'static -> Q
   the annotation is the proof: rvalue static promotion, not extension
```
<!-- /output -->

## See also

- [`Drop`, and what RAII buys](../../12_Traits/drop_and_raii/README.md) — the `let _ =` trap, which is this rule with no extension applied
- [`const` and `static`](../../27_Modules/const_and_static/README.md) — promotion, the mechanism that hides this one
- [Scope is about names](../scope_is_about_names/README.md) — the three things "out of scope" is asked to mean
- [Lifetime annotations](../lifetime_annotations/README.md) — `'a` names the relationship that extension is quietly satisfying

## Sources

The Reference on [temporary lifetime extension ↗](https://doc.rust-lang.org/reference/destructors.html#temporary-lifetime-extension) and [temporary scopes ↗](https://doc.rust-lang.org/reference/destructors.html#temporary-scopes); the `if let` change is [RFC 3212 ↗](https://rust-lang.github.io/rfcs/3212-if-let-temporary-scope.html), shipped in the [2024 edition ↗](https://doc.rust-lang.org/edition-guide/rust-2024/temporary-if-let-scope.html). The `E0716` transcript and both edition runs are real compiles of the files they name.

## Po polsku

Domyślna reguła jest wąska: **wartość tymczasowa ginie na końcu instrukcji**, czyli przy średniku. Wyjątkiem jest `let`, w którym po prawej stronie stoi tak zwane *wyrażenie przedłużające* — `&coś`, `&mut coś`, literał struktury, krotki albo tablicy zawierający taki operand, a także łańcuch dostępu do pola lub indeksu. Wtedy wartość tymczasowa dostaje ten sam czas życia co zmienna i dożywa końca bloku. Decyzja zapada po **kształcie zapisu**, a nie po tym, co kod robi — i właśnie dlatego `Holder { r: &Noisy("H") }` się kompiluje, a `hold(&Noisy("W"))`, które buduje dokładnie to samo, kończy się błędem `E0716`.

Reguła przestaje być ciekawostką w `match`. Wartości tymczasowe ze *scrutinee* żyją przez całe dopasowanie, razem z ramionami — więc `match rejestr.lock().unwrap().state()` trzyma blokadę także w środku ramion, i wywołanie `lock()` w ramieniu to zakleszczenie. Ta sama treść w `let` zwalnia blokadę na średniku. W edycji 2024 zmieniła się połowa tej historii dotycząca `if let`: wartości tymczasowe ze scrutinee są teraz zwalniane **przed** blokiem `else`, a nie po całym `if`/`else` jak w edycji 2021.

Ostatnia rzecz, która potrafi zamaskować całą lekcję: jeśli typ nie ma `Drop` ani wnętrza modyfikowalnego, to `&Quiet("Q")` w ogóle nie wymaga przedłużania — wartość zostaje **wypromowana** do anonimowego `static`, a referencja jest `&'static`. Eksperyment napisany na zwykłej strukturze danych będzie więc wyglądał tak, jakby przedłużało się wszystko. Dopiero implementacja `Drop` mówi prawdę o tym, kiedy cokolwiek naprawdę ginie.

**Szukaj po polsku:** czas życia wartości tymczasowej · przedłużenie czasu życia · zwolnienie na średniku · `rust temporary lifetime extension` · `rust E0716`
