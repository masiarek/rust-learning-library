# Address, pointer, reference

[Pointers](../README.md) › **Address, pointer, reference**

**Level:** 201 · working knowledge

**One line:** An address is a number, a pointer is an address with a type, and a reference is a pointer with promises the compiler checks — `&x`, `&x as *const T` and `.addr()` hold the same number, and the three types differ only in what they let you do with it.

```rust
let x: u32 = 42;
let r: &u32 = &x;          // reference: aligned, not null, valid, alive, not aliased by a &mut
let p: *const u32 = r;     // raw pointer: the same address, no promises
let a: usize = p.addr();   // address: the number, nothing else
```

## Three words, and what each adds

| Word | Rust type | Adds | Made from any number in safe code? | Read in safe code? |
|---|---|---|---|---|
| **memory address** | `usize` | — a count of bytes from zero | yes, it is a number | no, it is not a pointer |
| **pointer**, or **raw pointer** | [`*const T`, `*mut T` ↗](https://doc.rust-lang.org/std/primitive.pointer.html) | the **type** of what is there, and **provenance** | yes: `0x1000 as *const u32` | no, reading needs `unsafe` |
| **reference** | [`&T`, `&mut T` ↗](https://doc.rust-lang.org/std/primitive.reference.html) | aligned, not null, a valid `T`, a lifetime, and no `&mut` beside it | no | yes |

*Rust in Action* gives the same three definitions in a sidebar in chapter 6, §6.1, and opens §6.2 by turning them into a naming rule for its own text: say **reference** where the compiler's guarantees apply, **pointer** for the more primitive thing where you are responsible, and **raw pointer** where that unsafety should be spelled out. Rust's documentation draws the same line. The Reference's chapter [Pointer types ↗](https://doc.rust-lang.org/reference/types/pointer.html) lists references, then raw pointers — "pointers without safety or liveness guarantees" — then smart pointers, and std files `*const T` under "Raw, unsafe pointers".

The umbrella word is *pointer*, and this library uses it that way: a reference **is** a pointer, and so are [`Box`](../../26_Collections/the_box/README.md) and [`Rc`](../../18_Ownership/reference_counting/README.md), which are the subject of [Smart pointers](../../41_Smart_Pointers/README.md).

## An address is a number

A memory address numbers one byte. The book calls addresses an abstraction of assembly languages; the numbers themselves come from the CPU and the operating system, which give each process its own *virtual* address space. `usize` is wide enough to number all of that space, not the RAM installed — the recorded runs on [What an address shows](../../18_Ownership/what_an_address_shows/README.md) are 12 hex digits long, 48 bits, on a machine whose `usize` has 64.

A `usize` has lost two things a pointer carries, and the next two sections are those two things.

## A pointer adds the type

Section 2 of the run below reads the same address twice. Through `*const u32` a read takes 4 bytes and gives `707406378`; through `p.cast::<u8>()` it takes 1 byte and gives `42`. The type says how many bytes and what they mean.

The type exists only in the compiler. `*const u8` and `*const [u64; 1000]` are both 8 bytes at run time, one word, the address.

And nothing checks that the type is true. `0x1000 as *const u32` compiles in safe code with no warning. The book says the compiler can verify that a pointer to an `i32` has 4 bytes of integer behind it; that holds for a **reference**, where the bytes exist by construction, and not for a raw pointer, where only the size is known.

## A pointer is more than its address: provenance

Two pointers can hold the same address and still not be interchangeable:

```rust
fn main() {
    let x = 42u32;
    let p = &raw const x;
    let bare = std::ptr::without_provenance::<u32>(p.addr());
    println!("{}", bare == p);          // true
    println!("{}", unsafe { *bare });   // undefined behaviour
}
```

`==` on raw pointers compares addresses, so the first line prints `true`. The second read is undefined behaviour, because `bare` was built from a number and carries no permission to touch `x`. std's [pointer docs ↗](https://doc.rust-lang.org/std/ptr/index.html#provenance) call that permission **provenance**: the memory a pointer is allowed to access. A native build printed `42` for that read, which is what undefined behaviour often looks like. [Miri ↗](https://github.com/rust-lang/miri) reports it:

```text title="Abridged — cargo +nightly miri run, nightly 2026-08-27, no_provenance.rs"
true
error: Undefined Behavior: memory access failed: attempting to access 4 bytes, but got 0x213c0[noalloc] which is a dangling pointer (it has no provenance)
 --> src/bin/no_provenance.rs:6:29
  |
6 |     println!("{}", unsafe { *bare });   // undefined behaviour
  |                             ^^^^^ Undefined Behavior occurred here
```

That is why the address comes out through `p.addr()` and goes back in through `p.with_addr(a)`, which keeps `p`'s provenance — both stable since 1.84.0. The Reference says the meaning of transmuting a pointer into an integer type is "currently undecided" ([bit validity ↗](https://doc.rust-lang.org/reference/types/pointer.html#bit-validity)), so a `transmute` from pointer to `usize` is not the way to get the number.

## A reference adds the promises

std's [`reference` ↗](https://doc.rust-lang.org/std/primitive.reference.html) page defines a reference as a pointer assumed to be aligned, not null, and pointing at a valid `T`. The borrow checker adds two more. Each promise has a page:

| A `&T` promises | Where it is shown |
|---|---|
| aligned to `align_of::<T>()` | [A reference is aligned to its referent](../aligned_to_the_referent/README.md) |
| not null | [Nullable pointers](../../17_Option_and_Result/nullable_pointers/README.md) |
| points at a valid, initialized `T` | [Uninitialized reads](../../31_C_and_Cpp/uninitialized_reads/README.md) |
| the referent outlives every use | [What `&'a T` claims](../../18_Ownership/what_a_reference_claims/README.md) |
| no `&mut` to the same place is live | [Borrowing](../../18_Ownership/borrowing/README.md) |

Going from a raw pointer back to a reference, `unsafe { &*p }`, is the one step where you make all five yourself — [Raw pointers](../raw_pointers/README.md) is about that step.

## The verified output

<!-- output:address_pointer_reference -->
*Verified output of [`address_pointer_reference.rs`](examples/address_pointer_reference.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. One number, three types
   r as *const u32 == p           true
   r as *const u32 as usize == a  true
   size_of: &u32 8, *const u32 8, usize 8
   Same number, same width. The difference is what each type lets you do.

2. What a pointer adds to an address: the type of what is there
   read as u32 through p                 707406378
   read as u8 through p.cast::<u8>()     42
   size_of::<*const u8>()          = 8
   size_of::<*const [u64; 1000]>() = 8
   Same address, 4 bytes read or 1. The type says how many bytes to
   read and what they mean, and it exists only in the compiler: at run
   time a pointer to one byte and a pointer to 8,000 are both one word.

3. What an address alone has lost: provenance
   bare.addr() == p.addr()        true
   bare == p                      true
   Equal as numbers and equal under ==, which compares addresses only.
   Reading through `bare` is still undefined behaviour: it was made from
   a number, so it carries no permission to touch x. Miri reports it as
   a pointer that "has no provenance". This program never reads it.

4. A raw pointer can be made from nothing
   0x1000 as *const u32  -> is_null() false, addr() 0x1000
   Making it compiled with no unsafe and no warning. The compiler knows
   the type, and nothing about whether 4 bytes of u32 sit at 0x1000.

5. What a reference adds to a pointer: promises the compiler holds you to
   std: a reference is a pointer assumed to be aligned, not null, and
   pointing at a valid value of its type. The borrow checker adds that
   it outlives every use, and that a &mut to the same place is not live.
   unsafe { &*p } points where r does: true
   Turning a pointer back into a reference is the step that needs unsafe:
   the compiler cannot check the promises, so you make them.
```
<!-- /output -->

Miri reports no undefined behaviour in this program; it notes the `0x1000 as *const u32` cast, which is never read.

## If you are coming from another language

- **C.** A C pointer is Rust's raw pointer: an address and a type, with nothing checked about either — C has no reference type at all. What transfers is the vocabulary (`uintptr_t` is the address, `int *` the pointer). What is new is that the checked kind exists, is the default, and is the one every safe API takes; the C kind is still there, spelled `*const T`, and reading through it needs `unsafe`. The C learning library's [Reading the memory map ↗](https://masiarek.github.io/c-learning-library/02_Decompiling/reading_the_memory_map/index.html) shows the address ranges a C program's bytes are loaded at — the numbers every pointer on this page holds.
- **C++.** `T&` is the neighbour: it cannot be null or reseated. But its lifetime and aliasing are conventions, not checks — a `T&` to a destroyed local compiles with at most a warning. A Rust `&T` carries the same address and the two promises C++ leaves to you.
- **Python.** Every name is a reference, and every reference is counted, so there is no raw pointer to misuse outside `ctypes`. `id(x)` in CPython is documented as the object's address — a number, like `.addr()`, that you cannot turn back into the object. The closest Rust type to a Python reference is [`Rc`](../../18_Ownership/reference_counting/README.md), not `&T`.
- **ABAP.** `REF TO` data references are counted by the runtime, so they cannot dangle; `FIELD-SYMBOLS` are closer to a raw pointer, and `ASSIGN … CASTING` is a pointer cast — the same bytes read as another type, with the same absence of checks.

## See also

- [Wide pointers](../wide_pointers/README.md) — when the pointer is two words, and what the second one holds
- [What an address shows](../../18_Ownership/what_an_address_shows/README.md) — which address `&x` gives you, and why no example prints one
- [What `unsafe` turns off](../../09_Advanced/what_unsafe_turns_off/README.md) — making a raw pointer is safe; dereferencing it is one of five powers
- [*Rust in Action*, chapter 6, run](../rust_in_action_chapter_6/README.md) — the book's definitions, claim by claim
- [Smart pointers](../../41_Smart_Pointers/README.md) — the third kind the Reference lists

## Sources

[Pointer types ↗](https://doc.rust-lang.org/reference/types/pointer.html) in the Reference; std's [`reference` ↗](https://doc.rust-lang.org/std/primitive.reference.html), [`pointer` ↗](https://doc.rust-lang.org/std/primitive.pointer.html) and the [`std::ptr` module docs ↗](https://doc.rust-lang.org/std/ptr/index.html) on provenance; *Rust in Action* (Tim McNamara, Manning 2021), ch. 6 "Memory", §6.1 and §6.2. The Miri transcript is from `cargo +nightly miri run` on nightly 2026-08-27; the native read is rustc 1.98.0 on x86-64 macOS.

## Po polsku

**Adres** to liczba, **wskaźnik** to adres z typem, a **referencja** to wskaźnik z obietnicami, które sprawdza kompilator. `&x`, `&x as *const T` i `.addr()` przechowują tę samą liczbę; różnią się tym, na co pozwala ich typ.

Wskaźnik dodaje do adresu dwie rzeczy. Pierwsza to **typ** — ile bajtów przeczytać i jak je rozumieć — i istnieje on tylko w kompilatorze: `*const u8` i `*const [u64; 1000]` mają po 8 bajtów. Druga to **pochodzenie** (*provenance*): pozwolenie na dostęp do konkretnej pamięci. Dwa wskaźniki o tym samym adresie mogą być równe według `==`, a odczyt przez jeden z nich i tak jest niezdefiniowanym zachowaniem, co pokazuje Miri. Dlatego adres wyjmuje się przez `addr()`, a nie przez `transmute`.

Referencja dodaje obietnice: wyrównanie, brak nulla, poprawną wartość, czas życia i brak równoległego `&mut`. Surowy wskaźnik da się zrobić z dowolnej liczby w bezpiecznym kodzie; referencji — nie. Powrót od wskaźnika do referencji (`unsafe { &*p }`) to moment, w którym te obietnice składasz ty, a nie kompilator. Książka *Rust in Action* zamienia ten podział w regułę nazewnictwa: „referencja” tam, gdzie gwarancje daje kompilator, „wskaźnik” tam, gdzie odpowiadasz sam.

**Szukaj po polsku:** wskaźnik a referencja w Ruście · surowy wskaźnik · adres pamięci · `rust pointer provenance` · `rust reference vs raw pointer`
