# The `move` keyword

**Level:** 201 · working knowledge

**One line:** `move` decides **where the captured values live** — inside the closure rather than behind a reference to your scope — and it answers a lifetime question, not a "which `Fn` trait" one.

```rust
use std::thread;

fn main() {
    let rows = vec![5u32, 3, 0, 4];
    let handle = thread::spawn(move || rows.iter().sum::<u32>());
    println!("{}", handle.join().unwrap());   // 12
}
```

Delete the `move` and this stops compiling. The thread may still be running when `main` returns, so a closure holding `&rows` could outlive `rows` — and the compiler says so in as many words, with the fix attached.

## What the default is

Without `move`, a closure captures **the least it can get away with**: a shared reference if the body only reads, a mutable one if the body writes, and by value only if the body actually moves the value out. That is why a plain reading closure is 8 bytes — one pointer — and its `move` twin is 24, the whole `String`.

So `move` is not "capture by value instead of nothing". It is *"capture by value even where a reference would have done"*.

## The two errors that ask for it

**Returning a closure.** A closure built inside a function and handed back cannot hold a reference to that function's locals:

```text title="Abridged — real rustc output, without the file-and-line header and one note block"
error[E0373]: closure may outlive the current function, but it borrows `name`, which is owned by the current function
  |
3 |     || format!("hello, {name}")
  |     ^^                  ---- `name` is borrowed here
  |     |
  |     may outlive borrowed value `name`
  |
help: to force the closure to take ownership of `name` (and any other referenced variables), use the `move` keyword
  |
3 |     move || format!("hello, {name}")
  |     ++++
```

**Sending one to a thread.** The same error, with the reason spelled out one line lower — `note: function requires argument type to outlive 'static`. [`thread::spawn` ↗](https://doc.rust-lang.org/std/thread/fn.spawn.html) demands `F: Send + 'static` because it cannot know when the thread finishes, and `'static` here does not mean "lives forever": it means *"contains no borrow of anything that could end"*. An owned `Vec` qualifies. A `&Vec` borrowed from a local does not — only a borrow of something that itself lives for the whole program, such as a `&'static` literal, would.

That is the whole reason the keyword exists, and it is worth carrying: **`move` is the lifetime escape hatch**. Everywhere else it is optional, and the compiler infers the same thing.

## The trap: on a `Copy` type, `move` copies

`move` moves what is movable and copies what is `Copy`, and it announces neither. The result compiles, runs, and quietly does nothing:

```rust
fn main() {
    let mut total = 10;
    let mut add = move |n: i32| { total += n; total };
    println!("{} {}", add(1), add(1));   // 11 12
    println!("{}", total);               // 10  <- untouched
}
```

The closure got its own `i32`. Both numbers it printed are real, the outer `total` was never in the conversation, and there is no error and no warning. This is the same shape as [cloning to escape a mutation borrow](../../18_Ownership/how_to_learn_lifetimes/README.md): a copy is a perfectly legal thing to want, so nothing can flag it. If you meant to accumulate into the outer variable, drop the `move` — the closure then holds `&mut total` and the mutation lands.

## It captures the fields you named, not the whole value

Since the 2021 edition, a closure captures **individual fields**, not the variable they belong to. Measured, on a `Voter { name: String, ballot: Vec<u8> }`:

```text
size of Voter:       48 bytes
size of the closure: 24 bytes  (just the String field)
```

and the field the closure never mentioned is still yours afterwards. Before edition 2021 the whole `v` moved in, and the usual workaround was to bind the field to a local first (`let name = v.name;`) so the closure could capture that instead. Two consequences worth knowing: this is why `move` closures interfere with surrounding code less than older Rust books suggest — and why a `Drop` type's destructor can now run at a different time than it used to, which is the [disjoint capture ↗](https://doc.rust-lang.org/edition-guide/rust-2021/disjoint-capture-in-closures.html) migration the edition guide covers.

## What `move` does *not* do

It does not choose the closure's trait. A `move` closure that only reads is an `Fn` and is callable any number of times; a closure with no `move` is an `FnOnce` if its body hands a capture back. [The three closure traits](../three_closure_traits/README.md) has both, run side by side.

## When the original has to survive: clone, then move the clone

The idiom, and it is everywhere in threaded Rust:

```rust
let roster = vec![String::from("Ada"), String::from("Ben")];
let for_thread = roster.clone();
let handle = thread::spawn(move || for_thread.len());
println!("{}", handle.join().unwrap());   // 2
println!("{:?}", roster);                 // ["Ada", "Ben"]
```

The clone exists only to be moved, which is why it usually gets a name like `for_thread` and no other use. It is also the first `.clone()` on a page worth questioning, because *what* you clone decides what it costs: cloning a `Vec<String>` here duplicates every byte, while [cloning a reference-counted handle](../../18_Ownership/reference_counting/README.md) to the same data copies a pointer and a number. Neither is wrong; only one of them is free. [`Copy` vs `Clone`](../../16_Structs/copy_vs_clone/README.md) is the distinction, and [`Cow`](../../18_Ownership/clone_on_write/README.md) is the way out when most rows are never written to.

## If you are coming from another language

- **Python.** There is no `move`, and the default is Rust's *non*-`move` behaviour with the safety removed: a Python closure holds the enclosing variable, so it sees later rebinding and keeps the object alive — the reason `threading.Thread(target=lambda: use(rows))` is legal even if the enclosing frame returns first. Python gets away with it because the garbage collector will not free `rows` while the closure can reach it. Rust has no collector, so the same program has two possible meanings and the compiler makes you pick: borrow (fast, and the closure must not outlive the data) or `move` (independent, and the data goes with it). The Python idiom that most resembles `move` is the default-argument trick, `lambda r=rows: use(r)`, which snapshots the *binding* at definition time — but it still shares the same list object, so it is `move` on the name and not on the data. The closest thing to a real `move` is `copy.deepcopy` before the handoff, which is the clone-then-move idiom above, spelled by hand and unenforced.
- **ABAP.** `move` is the difference between `USING VALUE(iv_x)` and `USING iv_x` in a `FORM`, or between `IMPORTING VALUE(iv_x)` and plain `IMPORTING` on a method — pass by value versus pass by reference, a choice ABAP also makes at the boundary and also for lifetime reasons. Two things transfer badly. First, ABAP's default for `IMPORTING` is by reference *and* read-only, so the "borrow" case is familiar but the borrow checker's exclusivity rule has no counterpart; nothing stops the caller from changing the underlying field. Second, ABAP's by-value copy is exactly the `Copy` trap above: an internal table passed `VALUE(...)` and modified inside leaves the caller's table untouched, silently, which is the same bug in the same shape. What Rust adds is that this is only silent for `Copy` types — for a `String` or a `Vec` the compiler stops you from using the original at all, which is the case ABAP cannot detect. The `'static` requirement has no ABAP analogue, since a `PERFORM ... STARTING NEW TASK` marshals its parameters by value regardless.
- **C++.** `[=]` and `[&]` are the two halves of this decision, made per lambda, and `[x = std::move(x)]` is init-capture — the nearest thing to Rust's `move` at field granularity. The differences: C++ will happily let `[&]` outlive its scope and hand you a dangling reference at run time, which is exactly the E0373 above being caught at compile time instead; and `[=]` on a `std::vector` copies the vector, where Rust's `move` transfers it, so the C++ default is closer to the clone-then-move idiom than to `move` itself.

---

## The verified output

<!-- output:the_move_keyword -->
*Verified output of [`the_move_keyword.rs`](examples/the_move_keyword.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Without `move`, a closure borrows what it reads
   borrowed: Ada
   borrowed: Ada
   the original is still ours afterwards: "Ada"
   size of that closure: 8 bytes (a reference, not a String)

2. With `move`, the closure takes the value
   owned: Ben
   owned: Ben
   size of that closure: 24 bytes (the String itself)
   `owner` is no longer usable here — that use is E0382.

3. The trap: on a Copy type, `move` COPIES
   inside the closure:  11 then 12
   outside it, total is still 10
   nothing was moved: i32 is Copy, so the closure got its own copy,
   the outer `total` was never touched, and no error was raised.
   This is the one `move` bug that compiles, runs, and does nothing.

4. A `move` closure captures the FIELDS it uses, not the whole value
   voter: Cara
   size of Voter:       48 bytes
   size of the closure: 24 bytes  (just the String field)
   and the field it did not touch is still ours: [5, 3, 0]
   (edition 2021 changed this: before it, the whole `v` moved in.)

5. Two places `move` is not optional
   returned closure:  Hello, Ada
   without `move` that is E0373: the closure would outlive `greeting`.
   thread closure:    summed to 12
   without `move` that is E0373 too, and the note names the reason:
   `function requires argument type to outlive 'static`.

6. When the original has to survive: clone, then move the clone
   thread saw 2 rows
   and main still has its own: ["Ada", "Ben"]
   the clone exists only to be moved. That is the idiom, and it is
   also the first place a `.clone()` is worth questioning: cloning to
   satisfy `move` copies the data, cloning an Rc copies a pointer.
```
<!-- /output -->

---

## See also

- [The three closure traits](../three_closure_traits/README.md) — the classification `move` is often blamed for
- [What a closure is](../what_a_closure_is/README.md) — why the closure's size is the evidence for what it captured
- [Ownership and moves](../../18_Ownership/ownership_and_moves/README.md) — the rule `move` invokes, without the closure
- [How to learn lifetimes](../../18_Ownership/how_to_learn_lifetimes/README.md) — the `'static` bound above, and the clone-everything scaffold this page's last section is an instance of
- [`Copy` vs `Clone`](../../16_Structs/copy_vs_clone/README.md) — the trait that turns the move in section 3 into a silent copy
- [`&'static str` is not what most people think](../../14_Strings/static_str/README.md) — the other place `'static` means "no borrow that can end"

## Sources

[E0373 ↗](https://doc.rust-lang.org/error_codes/E0373.html), and the edition guide on [disjoint capture in closures ↗](https://doc.rust-lang.org/edition-guide/rust-2021/disjoint-capture-in-closures.html) for the field-granularity change measured above.

## Po polsku

Słowa kluczowego `move` się nie tłumaczy — przywołuje ono przeniesienie własności (*move*), ale samo zostaje po angielsku, jak każde słowo kluczowe. Warto natomiast wiedzieć, na jakie pytanie ono właściwie odpowiada, bo polskie streszczenia zwykle trafiają obok. Domyślnie domknięcie (*closure*) przechwytuje **jak najmniej się da**: referencję współdzieloną, jeśli tylko czyta, referencję mutowalną, jeśli zapisuje, a przez wartość dopiero wtedy, gdy ciało naprawdę wyprowadza wartość na zewnątrz. Dlatego czytające domknięcie waży 8 bajtów (sam wskaźnik), a jego bliźniak z `move` — 24, czyli cały `String`. `move` nie znaczy więc „przechwyć zamiast nie przechwytywać”, tylko **„przechwyć przez wartość nawet tam, gdzie wystarczyłaby referencja”** — i rozstrzyga pytanie o **czas życia**, a nie o to, którą z cech `Fn` / `FnMut` / `FnOnce` domknięcie dostanie. Tej drugiej rzeczy `move` nie ustala wcale.

Są dokładnie dwa miejsca, w których to słowo nie jest opcjonalne, i w obu kompilator sam dopisuje poprawkę. Pierwsze to **zwracanie domknięcia** z funkcji: `E0373`, *„closure may outlive the current function”* — domknięcie zbudowane w środku nie może trzymać referencji do zmiennych lokalnych tej funkcji. Drugie to `thread::spawn`, które żąda `F: Send + 'static`. I tu pilnuj tłumaczenia: statyczny czas życia (*`'static`*) **nie znaczy „żyje wiecznie”**, tylko „nie zawiera żadnego pożyczenia, które mogłoby się skończyć”. Posiadany na własność `Vec` to spełnia; `&Vec` pożyczony od lokalnej zmiennej — nie. Poza tymi dwoma przypadkami `move` jest tylko dopiskiem, a kompilator i bez niego wywnioskuje to samo.

Najgorsza pułapka działa jednak odwrotnie i po polsku wypada jeszcze gorzej niż po angielsku: `move` czyta się jak rozkaz „przenieś”, a na typie z cechą `Copy` **nic się nie przenosi — kopiuje się**, i to bez słowa ostrzeżenia. Domknięcie `move |n: i32| { total += n; total }` dostaje własny `i32`, w środku drukuje 11 i 12, a zewnętrzne `total` po wszystkim wynosi wciąż 10. Ani błędu, ani `warning` — kopia jest przecież rzeczą całkowicie legalną, więc nie ma czego zgłaszać. Jeśli chodziło o zliczanie do zmiennej z zewnątrz, lekarstwem jest **usunięcie** `move`: domknięcie weźmie wtedy `&mut total` i zmiana wyląduje tam, gdzie trzeba.

Ostatnia rzecz, przez którą starsze materiały wprowadzają w błąd: od edycji 2021 przechwytywane są **pojedyncze pola**, a nie cała zmienna. Widać to w liczbach z wydruku — `Voter` waży 48 bajtów, a domknięcie, które sięgnęło tylko po pole `name`, ma ich 24; pole, którego nie tknęło, zostaje twoje. Wszystko, co powstało wcześniej (a to spora część polskich tutoriali), opisuje stan, w którym do domknięcia wchodziło całe `v`, i zaleca obejście przez wcześniejsze `let name = v.name;`. Kiedy oryginał naprawdę musi przeżyć, idiom jest jeden: sklonuj i przenieś klon (`let for_thread = roster.clone();`). Warto tylko pamiętać, co się klonuje — kopia `Vec<String>` to kopia każdego bajtu, a klon `Arc` to wskaźnik i liczba.

**Szukaj po polsku:** słowo kluczowe move w Ruscie · przechwytywanie przez wartość · statyczny czas życia · `rust move closure E0373` · `rust closure 'static thread::spawn`
