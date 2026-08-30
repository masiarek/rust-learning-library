# Calling C — the call is free, the data is not

**Level:** 301 · deep dive

**One line:** *"No overhead FFI"* is true and is a claim about the **call**: `extern "C"` compiles to an ordinary call instruction, with no marshalling layer and no runtime to start first. It says nothing about the **data**, and the data is where the bill arrives — a `&str` cannot cross as it stands, because Rust strings carry a length and C strings carry a terminator.

Every list of Rust's selling points has a line like *seamless C interop* or *no overhead FFI* — [Benefits of Rust](../../00_Start_Here/benefits_of_rust/README.md) has it from Google's twenty, [Measured claims](../../00_Start_Here/measured_claims/README.md) has it from cheats.rs's six. Both are right, and both are narrower than they read. This page is the narrowing, with a program that does the whole round trip.

It needs no build script, no bindings crate, and **no C compiler**, which is itself part of the lesson: `std` already links the platform C library, so a declaration is all the setup there is.

## The whole setup is a declaration

```rust
use std::ffi::c_int;

unsafe extern "C" {
    safe fn abs(n: c_int) -> c_int;
}
```

That is it. `extern "C"` picks the C calling convention — which arguments go in which registers, who cleans up the stack — and the linker resolves `abs` against libc the same way it resolves everything else. There is no glue object, no generated shim, and nothing at run time: see [The linker](../../20_Compilers/the_linker/README.md) for what is actually doing the work.

Compare what the same call costs elsewhere. Java needs a JNI stub and a `System.loadLibrary`; Python's `ctypes` builds an argument tuple and converts each value at run time; C# writes a `[DllImport]` and lets the marshaller decide what your struct becomes. Rust writes down the signature and calls it. **That** is the claim, and it holds.

## Edition 2024 moved the promise to where it is made

Write the block the way every pre-2024 tutorial shows it and the compiler stops you:

```text
error: extern blocks must be unsafe
 --> old_style.rs:1:1
  |
1 | extern "C" { fn abs(n: i32) -> i32; }
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to 1 previous error
```

The old rule was that declaring a foreign function was safe and *calling* it was unsafe. The new rule is better, and the reason is worth internalising: **the declaration is the promise.** Nothing anywhere checks your signature against the function that actually ships in libc. Declare `fn strlen(s: *const c_char) -> u8` and the compiler believes you; the program is then wrong at the ABI level, at every call site, with no diagnostic available at any of them. That mistake is made once, in the block — so the block is where the `unsafe` belongs.

The same edition adds the other half, which is the part most tutorials have not caught up with. Inside an `unsafe extern` block, an individual function may be marked **`safe`**:

```rust
unsafe extern "C" {
    safe fn abs(n: c_int) -> c_int;          // callable with no unsafe block
    fn strlen(s: *const c_char) -> usize;    // still needs one
}
```

`abs` takes an integer. There is no precondition for a caller to get wrong, so demanding an `unsafe` block at every call site was noise that trained people to write the keyword without thinking. `strlen` takes a raw pointer and trusts you that a zero byte is coming. The rule that falls out is a good one to hold on to: **a foreign function taking a raw pointer is never `safe`** — the promise it needs is exactly the one Rust cannot make on your behalf. What `unsafe` does and does not switch off is [its own page](../what_unsafe_turns_off/README.md).

## The data is the bill

A Rust `&str` is a pointer and a length, side by side, with no terminator. A C string is a pointer, and the length is *wherever the first zero byte turns up*. Those are not two encodings of the same thing; they are two different data structures, and neither can be reinterpreted as the other for free.

So every string that crosses gets copied:

```rust
let c_name = CString::new("Ferris")?;   // allocates 7 bytes, copies 6, appends the 0
```

Six bytes in, seven bytes out, one heap allocation. Do that in a loop over a million rows and the "no overhead" line has stopped describing your program — not because the calls cost anything, but because you are rebuilding your data on every one of them.

Then, having thrown the length away, you pay to find it again. `strlen` walks the bytes; `str::len` reads a field. The example below prints both answering `6`, which is the whole shape of the boundary in one line: the information Rust had, discarded at the border, then bought back at O(n).

**And the conversion can fail** — the part no slogan mentions. `CString::new` returns a `Result`, because a Rust string may legally contain a zero byte and a C string may not:

```text
CString::new refused: interior NUL at byte 3
```

For text read from a file, a socket, or a user, that is a real error path and not a theoretical one. `.unwrap()` there is the same decision as anywhere else — see [`unwrap` is a to-do](../../02_Errors/unwrap_is_a_todo/README.md).

## Coming back: two promises, neither checkable

`CStr::from_ptr` turns a `*const c_char` back into something Rust can read, and it is `unsafe` for two separate reasons that are easy to collapse into one:

1. **A zero byte exists ahead of that pointer.** If it does not, the read runs off the end of the allocation — [a buffer overrun](../../31_C_and_Cpp/buffer_overruns/README.md), with C's consequences rather than Rust's.
2. **The memory outlives the borrow.** `from_ptr` hands back a `&CStr` with *whatever lifetime the caller asks for*, because a raw pointer carries none. The compiler will happily infer one that outlives the buffer, and then you have a use-after-free that the borrow checker had no way to see — the one bug in [C and C++](../../31_C_and_Cpp/use_after_free/README.md) that Rust's guarantee normally rules out, reintroduced by hand at the boundary.

The second is the one that bites, because it fails silently and only sometimes. The habit that prevents it: name the owner, keep it alive, and never return a `&CStr` from a function that owns the buffer it points into.

## A struct needs `#[repr(C)]`, or it is not a layout

Rust's default struct layout is [explicitly not guaranteed ↗](https://doc.rust-lang.org/nomicon/repr-rust.html). The compiler may reorder fields to pack them, and it does. Two structs whose fields match, one in Rust and one in C, are therefore not the same bytes — and passing one across the boundary is undefined behaviour even though every field type lines up.

```rust
#[repr(C)]
struct Point { x: c_int, y: c_int }   // now it is what C would build
```

`#[repr(C)]` is a request for C's layout rules: declaration order, C's padding, C's alignment. It is also the reason [`what_a_union_is`](../what_a_union_is/README.md) carries the attribute — a union's whole point is that you know where the bytes are.

## Going the other way

Export a Rust function under a name C's linker can find:

```rust
#[unsafe(no_mangle)]
pub extern "C" fn rust_double(x: c_int) -> c_int { x * 2 }
```

Two things there. `extern "C"` on a definition means *use C's calling convention*, which is the mirror of the declaration above. And `no_mangle` turns off the name mangling that normally makes symbols unique — in edition 2024 it is spelled `#[unsafe(no_mangle)]`, because taking a fixed global symbol name is a claim that nothing else in the final binary wants it, and the linker's answer to a collision is not always an error.

The symbol really is there afterwards, which `nm` will show you — on macOS, with the leading underscore that platform adds:

```text
$ nm calling_c | grep rust_double
0000000100001ce0 T _rust_double
```

What that does *not* prove is that C can call it, which needs a C compiler and a linker invocation this library deliberately does not require. The example below calls it from Rust; treat that as evidence the function exists and is correct, and the `nm` line as evidence the symbol is exported.

## If you are coming from another language

- **C or C++** — this is the least surprising FFI you have ever used, because there is no FFI: `extern "C"` is your own header declaration written in Rust syntax, and the same header-lies-about-the-implementation bug is available in exactly the same way. What is new is the ownership question at every pointer. C hands you a `char *` and the documentation says who frees it; Rust makes you encode that answer in a type, and `CStr` versus `CString` **is** that answer — borrowed versus owned.
- **Python** — `ctypes` is the closest thing you have met, and the difference is when the work happens. `ctypes` inspects and converts arguments at run time, per call; Rust resolves the signature at compile time and the call is then free. What survives unchanged is the encoding tax: `s.encode()` in Python and `CString::new(s)` in Rust are the same allocation and copy, for the same reason.
- **Java** — JNI is a whole subsystem, with a stub per function, a `JNIEnv *` threaded through, and explicit local-reference management. None of that exists here. Nor does the escape hatch: JNI can ask the JVM for help at run time, and Rust's boundary has no run time to ask.
- **ABAP** — the nearest thing is an RFC destination to a non-SAP system, and the mental model transfers better than you would expect: the *call* is cheap and the *conversion* is where the cost and the failures live. A `CString::new` that refuses an interior NUL is the same shape of problem as a field that will not fit the target's type — the boundary rejecting data that was perfectly legal on your side.

## The verified output

<!-- output:calling_c -->
*Verified output of [`calling_c.rs`](examples/calling_c.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The call itself
   abs(-7) from C = 7   no unsafe block: declared `safe`
   (-7i32).abs()  = 7   same answer, no boundary crossed

2. The data is the part that costs
   &str    "Ferris"  6 bytes, length carried beside the pointer
   CString "Ferris"  7 bytes, length is wherever the 0 turns up

3. So C gets asked a question Rust already knew the answer to
   strlen(..) = 6   walks the bytes looking for the 0
   name.len() = 6   reads a field

4. And the conversion can fail
   CString::new refused: interior NUL at byte 3

5. Coming back the other way
   CStr::from_ptr(..) = "Ferris"
   ..borrowed from c_name, which must outlive it

6. And out again
   rust_double(21) = 42   exported to C as `rust_double`
```
<!-- /output -->

## See also

- [What `unsafe` turns off](../what_unsafe_turns_off/README.md) — the five powers, and why the audit unit is the module
- [What a union is](../what_a_union_is/README.md) — the other `#[repr(C)]` page, and where C's layout rules matter most
- [The linker](../../20_Compilers/the_linker/README.md) — what resolves `abs` against libc, and why no glue is needed
- [Use-after-free](../../31_C_and_Cpp/use_after_free/README.md) — the bug `CStr::from_ptr` lets back in, compiled and run
- [Benefits of Rust](../../00_Start_Here/benefits_of_rust/README.md) · [Measured claims](../../00_Start_Here/measured_claims/README.md) — the two lists this page is the footnote to
- [The Nomicon — repr(Rust) ↗](https://doc.rust-lang.org/nomicon/repr-rust.html) · [`CString` ↗](https://doc.rust-lang.org/std/ffi/struct.CString.html) · [`CStr::from_ptr` ↗](https://doc.rust-lang.org/std/ffi/struct.CStr.html#method.from_ptr) · [Edition guide — unsafe extern blocks ↗](https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-extern.html)

## Po polsku

Hasło *no overhead FFI* tłumaczy się na polski gładko — „zerowy narzut” — i właśnie dlatego myli. Zdanie jest prawdziwe o **wywołaniu**: `extern "C"` to zwykła instrukcja skoku, bez warstwy pośredniej, bez generowanego kodu sklejającego i bez środowiska uruchomieniowego, które trzeba najpierw wystartować. Nie mówi natomiast nic o **danych**, a to dane kosztują. Polski czytelnik ma tu jedną przewagę: jeśli pisałeś kiedyś RFC do systemu spoza SAP-a albo `ctypes` w Pythonie, znasz ten podział z praktyki — samo wywołanie jest tanie, a rachunek wystawia konwersja.

Sedno jest strukturalne, nie wydajnościowe. `&str` w Ruscie to **wskaźnik i długość obok siebie**, bez terminatora; łańcuch w C to wskaźnik, a długość jest tam, gdzie trafi się pierwsze zero. To nie są dwa kodowania tego samego — to dwie różne struktury danych, więc każdy łańcuch przekraczający granicę jest kopiowany: `CString::new("Ferris")` alokuje siedem bajtów, przepisuje sześć i dokłada zero. Potem, skoro długość została po drodze wyrzucona, `strlen` odkupuje ją chodząc po bajtach — w O(n) — podczas gdy `str::len` czytało po prostu pole. Cała granica mieści się w tej jednej obserwacji.

Dwie rzeczy, które w polskich omówieniach FFI pojawiają się najrzadziej. Po pierwsze, **konwersja może się nie udać**: `CString::new` zwraca `Result`, bo łańcuch Rusta może legalnie zawierać bajt zerowy, a łańcuch C nie może — przy danych z pliku, gniazda albo od użytkownika to realna ścieżka błędu, a nie ciekawostka. Po drugie, edycja 2024 przeniosła obietnicę tam, gdzie jest składana: blok `extern` musi być teraz `unsafe extern`, bo to **deklaracja jest obietnicą** — nic nie porównuje twojej sygnatury z prawdziwą funkcją z libc. Ta sama edycja pozwala oznaczyć pojedynczą funkcję słowem **`safe`** (jak `abs`, które bierze liczbę i nie ma czego zepsuć), a zasada praktyczna brzmi: funkcja przyjmująca surowy wskaźnik nigdy nie jest `safe`.

I ostrzeżenie na koniec, bo to jedyne miejsce w bezpiecznym Ruscie, gdzie wraca błąd z rozdziału o C. `CStr::from_ptr` oddaje referencję o **dowolnym** czasie życia, jakiego zażąda wywołujący — surowy wskaźnik żadnego nie niesie. Kompilator chętnie wywnioskuje taki, który przeżyje bufor, i masz use-after-free, którego borrow checker nie miał jak zobaczyć. Nazwij właściciela, utrzymaj go przy życiu i nie zwracaj `&CStr` z funkcji, która jest właścicielem wskazywanego bufora. Osobno pamiętaj o `#[repr(C)]`: domyślny układ pól struktury w Ruscie **nie jest gwarantowany** i kompilator faktycznie je przestawia, więc bez tego atrybutu „takie same pola” nie znaczy „takie same bajty”.

**Szukaj po polsku:** wywoływanie funkcji C z Rusta · konwersja łańcuchów znaków FFI · `rust unsafe extern block 2024` · `rust CString CStr difference` · `rust repr(C) layout`
