# Interior mutability

**Level:** 301 · deep dive

**One line:** `Cell` and `RefCell` let you write through a `&T` — not by defeating the borrow rules but by **moving the check from compile time to run time**, which buys the shapes the static checker cannot prove and costs you a panic instead of an error.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The rule being bent: `&T` means shared, and shared means read-only — *except* through `UnsafeCell`, which is the one place in the language where that is not true and which both types are built on
- `Cell<T>`: no references handed out at all, so nothing to check. `get` copies out, `set` copies in, `replace`/`take` move. Requires `Copy` for `get`, and is therefore the cheap one
- `RefCell<T>`: hands out real references and counts them — `borrow()` and `borrow_mut()` return guards, and a violated rule is a **panic**, not a compile error
- Where each is right: `Cell` for a counter or a flag, `RefCell` for a graph node or a cache holding something not `Copy`
- `Rc<RefCell<T>>` — the shared-mutable pair, what it costs, and why it is the single most common shape in Rust code that started life as a Python or Java design
- Single-threaded only: neither is `Sync`, so the multi-threaded answer is `Mutex`, which is the same idea with the check made blocking rather than panicking
- What it does **not** buy: no cycle collection, and an `Rc<RefCell<T>>` cycle still leaks

## The trap it exists for

`already borrowed: BorrowMutError` is a runtime panic in code that compiles perfectly, and it fires on a path the tests may not take — typically a method that calls another method on the same `RefCell` while a guard is still alive, often through a callback. The static checker would have caught the same mistake as an error; you gave that up when you reached for the type.

## See also

- [Borrowing](../../18_Ownership/borrowing/README.md) — the compile-time rule this page moves
- [`Rc`: the clone that copies a pointer](../../18_Ownership/reference_counting/README.md) — the other half of `Rc<RefCell<T>>`
- [Lock poisoning](../mutex_poisoning/README.md) — the threaded counterpart, and what its `Result` is telling you
- [What `unsafe` turns off](../what_unsafe_turns_off/README.md) — the `UnsafeCell` underneath, and why a safe API over a small unsafe core is the pattern
- [`RefCell` ↗](https://doc.rust-lang.org/std/cell/struct.RefCell.html) · [Comprehensive Rust: Interior Mutability ↗](https://google.github.io/comprehensive-rust/borrowing/interior-mutability.html)

## Po polsku

Mutowalność wewnętrzna (*interior mutability*) niczego nie omija — **przenosi sprawdzanie reguł pożyczania z czasu kompilacji do czasu działania programu**. Przymiotnik „wewnętrzna” dotyczy wnętrza typu, a nie referencji: `&Cell<T>` i `&RefCell<T>` to wciąż zwykłe referencje współdzielone, tylko że typ, na który wskazują, bierze na siebie odpowiedzialność za bezpieczną zmianę. `Cell<T>` nie wydaje żadnych referencji (`get` kopiuje wartość na zewnątrz, `set` do środka), więc nie ma czego pilnować; `RefCell<T>` wydaje prawdziwe referencje i je zlicza — i to jest ta droższa połowa.

Pułapka jest też nazewnicza. Metody nazywają się `borrow()` i `borrow_mut()`, ale to **nie** jest pożyczanie w sensie kontrolera pożyczeń (*borrow checker*) — to zwykłe wywołanie metody, a złamanie reguły nie kończy się błędem kompilacji, tylko paniką `already borrowed: BorrowMutError` na ścieżce, po której testy mogą nigdy nie przejść. Kod się kompiluje, a program pada. Warto o tym pamiętać zwłaszcza przy `Rc<RefCell<T>>` — to najczęstszy kształt w kodzie przenoszonym z Pythona czy Javy, gdzie „współdzielony i mutowalny obiekt” był po prostu domyślnym sposobem myślenia. Żaden z tych typów nie jest `Sync`, więc w wielu wątkach odpowiednikiem jest `Mutex`: ten sam pomysł, tylko z czekaniem zamiast paniki.

**Szukaj po polsku:** mutowalność wewnętrzna · referencja współdzielona · `rust Cell vs RefCell` · `already borrowed BorrowMutError` · `rust Rc RefCell pattern`
