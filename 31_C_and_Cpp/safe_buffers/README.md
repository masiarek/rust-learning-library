# Safe Buffers — C++ adopts the slice

**Level:** 201 · for C and C++ programmers

**One line:** Clang's answer to buffer overruns has three parts — every buffer carries its length, every index is checked, and indexing a raw pointer is flagged unless you fence it off — which are Rust's slice, Rust's bounds check and Rust's `unsafe` block, reached from the other side.

Apple proposed the model as [*RFC: C++ Buffer Hardening* ↗](https://discourse.llvm.org/t/rfc-c-buffer-hardening/65734) in October 2022, and Clang documents it as [C++ Safe Buffers ↗](https://clang.llvm.org/docs/SafeBuffers.html). It is the spatial-safety half of Yitzhak Mandelbaum's C++Now 2026 talk, [*A Path to Practically Safe C++* ↗](https://www.youtube.com/watch?v=fi6csDXvve0), which reports where Google's rollout stands.

## The program

```cpp
#include <cstddef>
#include <memory>
#include <span>
#include <vector>

int sum_raw(const int* p, std::size_t n) {
    int s = 0;
    for (std::size_t i = 0; i < n; ++i) s += p[i];   // index a raw pointer
    return s;
}

int sum(std::span<const int> xs) {
    int s = 0;
    for (int x : xs) s += x;                          // no index at all
    return s;
}

int main() {
    std::vector<int> v = {1, 2, 3};
    std::span<const int> forged(v.data(), 2);         // a length nobody checked
    auto owned = std::make_unique<int[]>(3);          // an owner with no length
    owned[2] = sum_raw(v.data(), v.size()) + sum(v) + sum(forged);
    return owned[2];
}
```

Every index here is in bounds. The question is which lines *could* go out of bounds without anything noticing.

## What Clang says

```text title="Real output — clang 23.1.0 on Compiler Explorer, -std=c++20 -Wunsafe-buffer-usage"
<source>:8:46: warning: unsafe buffer access [-Wunsafe-buffer-usage]
    8 |     for (std::size_t i = 0; i < n; ++i) s += p[i];   // index a raw pointer
      |                                              ^
<source>:8:46: note: pass -fsafe-buffer-usage-suggestions to receive code hardening suggestions
<source>:20:26: warning: the two-parameter std::span construction is unsafe as it can introduce mismatch between buffer size and the bound information [-Wunsafe-buffer-usage-in-container]
   20 |     std::span<const int> forged(v.data(), 2);         // a length nobody checked
      |                          ^
<source>:22:5: warning: direct access using operator[] on std::unique_ptr<T[]> is unsafe due to lack of bounds checking [-Wunsafe-buffer-usage-in-unique-ptr-array-access]
   22 |     owned[2] = sum_raw(v.data(), v.size()) + sum(v) + sum(forged);
      |     ^~~~~~~~
<source>:23:12: warning: direct access using operator[] on std::unique_ptr<T[]> is unsafe due to lack of bounds checking [-Wunsafe-buffer-usage-in-unique-ptr-array-access]
   23 |     return owned[2];
      |            ^~~~~~~~
4 warnings generated.
```

| Flagged | Why | Apple clang 21 |
|---|---|---|
| `p[i]`, indexing a raw pointer | a pointer carries no length, so the index has nothing to be checked against | flagged |
| `span(v.data(), 2)` | the length was typed in rather than taken from the buffer | flagged |
| `owned[2]` on a `unique_ptr<int[]>` | the owner never stored its length | not flagged — that check arrived in Clang 22 |

Not flagged: `sum(v)`, and the range-`for` inside `sum`, which never indexes. A `span` built from one container's own `data()` and `size()` passes too — Clang recognises the pair. On Apple's clang 21, `-Wall -Wextra` print none of these; `-Wunsafe-buffer-usage` has to be asked for by name.

## At run time — a hardened standard library

The warnings find the lines. Stopping a bad index while the program runs is the other half: libc++'s [hardening modes ↗](https://libcxx.llvm.org/Hardening.html), one macro, after which `operator[]` checks its bound.

```cpp
#include <cstdio>
#include <vector>
int main(int argc, char**) {
    std::vector<int> v = {1, 2, 3};
    std::size_t i = argc + 2;       // 3 at run time, one past the end
    std::printf("%d\n", v[i]);
    std::puts("still running");
}
```

Real runs — Apple clang 21.0.0, x86_64 macOS, `-std=c++20 -O0`, run with no arguments, exit status read from the shell:

| `_LIBCPP_HARDENING_MODE` | What it printed | Exit |
|---|---|---|
| not set | `0`, then `still running` | 0 |
| `_LIBCPP_HARDENING_MODE_FAST` | nothing | 132 — `SIGILL`, a trap instruction |
| `_LIBCPP_HARDENING_MODE_EXTENSIVE` | nothing | 132 — `SIGILL` |
| `_LIBCPP_HARDENING_MODE_DEBUG` | `libc++ Hardening assertion __n < size() failed: vector[] index out of bounds`, after the header's path | 134 — `SIGABRT` |

The `DEBUG` build of a `std::span` stops the same way, with `span<T>::operator[](index): index out of range`. A `std::make_unique<int[]>(3)` written at index 3 under the same `DEBUG` mode printed `no check fired` and exited 0: there is no length for the check to compare against.

Google turned hardened libc++ on across its production services and [measured it ↗](https://security.googleblog.com/2024/11/retrofitting-spatial-safety-to-hundreds.html): an average **0.30%** performance cost, **more than 1,000** bugs found, and a **30%** drop in the baseline rate of segmentation faults. The check itself is the compare-and-branch every Rust index already does — [what it costs](../buffer_overruns/README.md#what-the-check-costs) there.

## The owner with no length

```text title="Real output — sizeof in bytes, Apple clang 21.0.0, x86_64 macOS"
unique_ptr<int>   8
unique_ptr<int[]> 8
span<int>         16
string_view       16
vector<int>       24
```

An owned array is one word, so after `make_unique` returns the length exists nowhere — which is why Clang 22 flags its `operator[]`, and why hardening cannot help it. The talk's fix is an owning type that keeps the length: Google's is `UniqueArray`, and Chromium's public one is [`base::HeapArray` ↗](https://chromium.googlesource.com/chromium/src/+/main/base/containers/heap_array.h). Rust's `Box<[T]>` has been that type all along — [two words, pointer and length](../../26_Collections/arrays_and_slices/README.md), in the first lines of the example below.

## The rules, side by side

| The Safe Buffers rule | C++, with Clang and libc++ | Rust |
|---|---|---|
| Every buffer carries its length | `std::span`, `std::string_view`, the containers; a new owning type for arrays | `&[T]`, `&str`, `Vec<T>`, `Box<[T]>` |
| Every index is checked | libc++ hardening — a build mode | always; `get` when you would rather ask |
| No indexing a raw pointer | `-Wunsafe-buffer-usage` — a warning you enable | a raw pointer has no `[]` at all (`E0608`), and `ptr.add` is `unsafe` |
| No forging a view from a pointer and a length | the two-parameter `span` constructor is flagged | `slice::from_raw_parts` is an `unsafe fn` (`E0133`) |
| A place to vouch for it anyway | `#pragma clang unsafe_buffer_usage begin` … `end` | an `unsafe` block, with a `// SAFETY:` comment |
| Iterators and C strings | in the talk's proposed spatial-safety mode, pointer iterators are banned by default outside range-`for`, and direct C-string operations are banned outright | a slice iterator carries its end and `&str` its length; `CStr` is the null-terminated one, for FFI |

## What Rust does instead

```rust
let owned: Box<[i32]> = vec![1, 2, 3].into_boxed_slice();
println!("{:?}", owned.get(3));                                   // None
let forged: &[i32] = unsafe { std::slice::from_raw_parts(ptr, len) }; // you vouch, in writing
```

<!-- output:safe_buffers -->
*Verified output of [`safe_buffers.rs`](examples/safe_buffers.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Box<i32>   1 word
Box<[i32]> 2 words
&[i32]     2 words
owned.get(3)  -> None
sum(&v)       -> 6
sum(forged)   -> 6
```
<!-- /output -->

`sum(&v)` takes the buffer as one value. Rewriting a `(pointer, length)` parameter pair into that shape is what the talk calls **spanification**, and Google's tooling had landed more than a thousand such changes by the time of the talk. The forged slice is the one way to assemble a view from parts: inside `unsafe`, with the reason written above it.

## The refusal

Leave out the `unsafe`, and the forge does not compile:

```text title="Real rustc output — from_raw_parts.rs, --edition 2024"
error[E0133]: call to unsafe function `std::slice::from_raw_parts` is unsafe and requires unsafe block
 --> from_raw_parts.rs:3:21
  |
3 |     let s: &[i32] = std::slice::from_raw_parts(v.as_ptr(), v.len());
  |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
  |
  = note: consult the function's documentation for information on how to avoid undefined behavior
```

That is `-Wunsafe-buffer-usage-in-container` with two differences: it is an error, and no flag turns it off — only the word `unsafe`, which [`grep` finds](../../09_Advanced/what_unsafe_turns_off/README.md).

## C, and the seam between them

The talk's last spatial slide is about C, which almost every C++ binary contains:

- **Fixed-size arrays.** `-fsanitize=array-bounds -fsanitize-trap=array-bounds` checks indexes into an array whose size the compiler knows. The C-array version of the program above — index 3 into `int a[3]` — printed `-641269726` in a plain build and stopped with `SIGILL` under the trap.
- **Flexible array members.** `int data[] __attribute__((counted_by(len)));` names the field that holds the count, and the same check then covers `data[i]`: a write to index 3 of a three-element packet trapped too.
- **Pointers.** [`-fbounds-safety` ↗](https://clang.llvm.org/docs/BoundsSafety.html) is Apple's annotation language for the bounds of C pointers — the talk's "part 1" for C. Apple's clang 21 on this Mac accepts it for C and refuses it for C++; Homebrew's Clang 21.1.8 does not know the flag.

Rust meets the same seam wherever it [calls C](../../09_Advanced/calling_c/README.md): the C side hands over a pointer and a length, and `from_raw_parts` is where you vouch that they belong together.

## If you are coming from another language

- **C++** — If you already reach for `std::span`, `.at()` and the Core Guidelines' ban on pointer arithmetic, Safe Buffers is that practice turned into a compiler flag, and Rust is the same flag with no off switch. What stays different is the default: C++ is adopting these one warning, one build mode and one migration at a time, while in Rust the unchecked form is the one you have to spell — `get_unchecked`, `unsafe`.
- **Python** — You have never indexed a raw pointer: `list`, `bytes` and `memoryview` carry their length and raise `IndexError`. The C layer underneath already does what Safe Buffers asks of C++ — the buffer protocol hands out a `Py_buffer` holding the pointer *and* the length together — and a `memoryview` is the Python object closest to a slice.
- **ABAP** — Internal tables are addressed by index or key through the runtime, which checks: `READ TABLE … INDEX` past the end sets `sy-subrc` instead of reading the next row. With no address arithmetic in the language, ABAP never had this class of bug to retrofit — the route Rust took too, with `unsafe` as the marked door for the times you need an address.

## Practice

**Spanify the docs' own example.** Clang's Safe Buffers guide opens on a function like this one, and flags it:

```cpp
int get_last_element(int *pointer, size_t size) {
  return pointer[size - 1];
}
```

Port it to Rust twice: once literally — an `unsafe fn` taking a raw pointer and a length — and once the Safe Buffers way, taking the buffer as one value. Say what each does when the buffer is empty.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:safe_buffers_kata -->
*[`safe_buffers_kata.rs`](examples/safe_buffers_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: spanify the Safe Buffers docs' own example.
//!
//!   rustc --edition 2024 safe_buffers_kata.rs -o /tmp/k && /tmp/k

/// The literal port of `int get_last_element(int *pointer, size_t size)`.
///
/// # Safety
/// `ptr` must point at `size` initialised `i32`s, and `size` must not be zero.
unsafe fn get_last_raw(ptr: *const i32, size: usize) -> i32 {
    // SAFETY: the caller promised `size` elements starting at `ptr`, size > 0.
    unsafe { *ptr.add(size - 1) }
}

/// The spanified version: the buffer arrives as one value, length included.
fn get_last(xs: &[i32]) -> Option<i32> {
    xs.last().copied()
}

fn main() {
    let v = vec![1, 2, 3];

    println!("THE LITERAL PORT");
    // SAFETY: the pointer and the length come from the same live Vec.
    let last = unsafe { get_last_raw(v.as_ptr(), v.len()) };
    println!("  get_last_raw(ptr, 3) = {last}");
    println!("  At size 0, `size - 1` is the bug. In C++ the size_t wraps to SIZE_MAX");
    println!("  and pointer[SIZE_MAX] is undefined behaviour. This port panics on the");
    println!("  subtraction in a debug build -- and under --release it wraps too, and");
    println!("  ptr.add() is undefined behaviour exactly as in C++. `unsafe` did not");
    println!("  remove the bug; it labelled the function it lives in.");
    println!();

    println!("THE SPANIFIED VERSION");
    println!("  get_last(&v)      = {:?}", get_last(&v));
    println!("  get_last(&[])     = {:?}", get_last(&[]));
    println!("  get_last(&v[..1]) = {:?}", get_last(&v[..1]));
    println!("  The length travels with the buffer, so no caller can pass a wrong one");
    println!("  and there is no subtraction to underflow: last() asks the slice.");

    assert_eq!(last, 3);
    assert_eq!(get_last(&[]), None);
}
```
<!-- /source -->

<!-- output:safe_buffers_kata -->
*Verified output of [`safe_buffers_kata.rs`](examples/safe_buffers_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
THE LITERAL PORT
  get_last_raw(ptr, 3) = 3
  At size 0, `size - 1` is the bug. In C++ the size_t wraps to SIZE_MAX
  and pointer[SIZE_MAX] is undefined behaviour. This port panics on the
  subtraction in a debug build -- and under --release it wraps too, and
  ptr.add() is undefined behaviour exactly as in C++. `unsafe` did not
  remove the bug; it labelled the function it lives in.

THE SPANIFIED VERSION
  get_last(&v)      = Some(3)
  get_last(&[])     = None
  get_last(&v[..1]) = Some(1)
  The length travels with the buffer, so no caller can pass a wrong one
  and there is no subtraction to underflow: last() asks the slice.
```
<!-- /output -->

</details>

## See also

- [Buffer overruns](../buffer_overruns/README.md) — the bug this is C++'s reply to, and what Rust's check costs
- [Arrays and slices](../../26_Collections/arrays_and_slices/README.md) — the length in the type, and the length in the value
- [What `unsafe` turns off](../../09_Advanced/what_unsafe_turns_off/README.md) — `from_raw_parts_mut`, used to build `split_at_mut` by hand
- [Lifetime safety in Clang](../lifetime_safety_in_clang/README.md) — the temporal half of the same talk
- [libc++ hardening modes ↗](https://libcxx.llvm.org/Hardening.html) — what each mode checks
- [Retrofitting spatial safety to hundreds of millions of lines of C++ ↗](https://security.googleblog.com/2024/11/retrofitting-spatial-safety-to-hundreds.html) — Google's measurements
- [C and C++](../README.md) — the nine bugs, and the other reply

## Po polsku

Bezpieczeństwo przestrzenne (*spatial safety*) znaczy tyle, że każdy dostęp do pamięci trafia w granice obiektu, do którego należy. W C i C++ psuje się ono z jednego powodu: wskaźnik nie niesie długości, więc indeksu nie ma z czym porównać. Model *C++ Safe Buffers* w Clangu odpowiada na to trzema regułami — bufor zawsze wędruje razem ze swoją długością (`std::span`, kontenery), każdy indeks sprawdza utwardzona biblioteka standardowa (*hardened libc++*), a indeksowanie surowego wskaźnika (*raw pointer*) jest ostrzeżeniem, dopóki nie otoczysz go pragmą `unsafe_buffer_usage`.

W Ruście to trzy rzeczy, które już znasz: wycinek (*slice*) `&[T]` to adres i długość w jednej wartości, każdy indeks jest sprawdzany, a złożenie wycinka z adresu i długości (`slice::from_raw_parts`) wymaga bloku `unsafe`. Pomysł jest ten sam; różni się to, co jest domyślne. W C++ włączasz to ostrzeżeniem i trybem kompilacji, w Ruście nie da się tego wyłączyć — można najwyżej świadomie podpisać się pod wyjątkiem.

Jeden wyjątek tłumaczy całość: `std::unique_ptr<int[]>` ma 8 bajtów, sam wskaźnik, więc po `make_unique` długość nie istnieje nigdzie i żadne utwardzanie jej nie sprawdzi. Rustowy `Box<[i32]>` ma dwa słowa: wskaźnik i długość. Google wprowadza dla C++ osobny typ z długością, a automatyczne przepisywanie par (wskaźnik, długość) na `span` nazywa *spanification* — tego słowa szukaj po angielsku, bo polskiego odpowiednika nie ma.

**Szukaj po polsku:** bezpieczeństwo pamięci · przepełnienie bufora · sprawdzanie granic tablicy · `C++ Safe Buffers` · `-Wunsafe-buffer-usage` · `libc++ hardening` · `rust slice bounds check`
