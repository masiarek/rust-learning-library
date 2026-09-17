# *Rust in Action*, chapter 6: pointer claims, run

[Pointers](../README.md) › **Claims, run**

**Level:** 201 · a companion to the section

**One line:** Chapter 6 of *Rust in Action* ("Memory") gets the three-way split between addresses, pointers and references right, and gets alignment, the second word of a wide pointer and the size of `Option` wrong; one of its listings is undefined behaviour. Each claim below is checked on rustc 1.98.0 — by the program on this page, by the compiler's refusal, or by Miri.

The book is Tim McNamara, *Rust in Action* (Manning, 2021). Its code is on GitHub at [rust-in-action/code ↗](https://github.com/rust-in-action/code/tree/1st-edition/ch6), whose README says pull requests are not accepted for copyright reasons, so no listing is reproduced here. Each program below is a separate test of one claim, and names the listing it checks.

## The sidebar on references, pointers and addresses (§6.1)

| # | The book says, in short | Verdict on 1.98.0 | Checked on |
|---|---|---|---|
| 1 | a memory address is a number that refers to one byte, an abstraction of assembly languages | the number, yes; it comes from the CPU and the OS's virtual memory, which assembly spells out | [Address, pointer, reference](../address_pointer_reference/README.md#an-address-is-a-number) |
| 2 | a pointer is an address that points to a value of some type | yes, and it also carries **provenance**: an equal address is not an equal pointer | [Address, pointer, reference](../address_pointer_reference/README.md#a-pointer-is-more-than-its-address-provenance) |
| 3 | for a type with no known size, a reference is a pointer and an integer | a length for `[T]` and `str`; a **vtable pointer** for `dyn Trait` | section 3 below, [Wide pointers](../wide_pointers/README.md) |
| 4 | a compiler creating a pointer to an `i32` can verify 4 bytes of integer are there | for a reference; `0x1000 as *const i32` compiles in safe code and nothing is checked | section 4 below |
| 5 | the programmer is responsible for validity when a type's size is unknown at compile time | for raw pointers; the book's own third benefit of references says they keep the length for you | [Wide pointers](../wide_pointers/README.md) |
| 6 | references always refer to valid data | in safe code, yes — std defines a reference as aligned, non-null and pointing at a valid `T` | [Address, pointer, reference](../address_pointer_reference/README.md#a-reference-adds-the-promises) |
| 7 | references are aligned to multiples of `usize` | **no** — to `align_of::<T>()`, which is 1 for `u8`; the book's own printed addresses end in `…48a` | section 2 below, [Alignment](../aligned_to_the_referent/README.md) |
| 8 | CPUs are slower with unaligned memory | a hardware remark, true of some CPUs; in Rust a misaligned reference is undefined behaviour on every CPU | [Alignment](../aligned_to_the_referent/README.md#what-happens-if-you-build-one-anyway) |
| 9 | types include padding so references to them are not slow | padding is real; it exists so each field is aligned, which validity requires, not only speed | [Alignment](../aligned_to_the_referent/README.md#padding-is-how-a-struct-keeps-its-fields-aligned) |
| 10 | the difference between addresses and the other two is type information | yes, at compile time; at run time a thin pointer is one word whatever its type | [Address, pointer, reference](../address_pointer_reference/README.md#a-pointer-adds-the-type) |

## Around the sidebar

Claims 11 to 14 are from §6.1 itself; 15 and 16 are from notes written beside the chapter, and are checked here because they are about the same types. Claim 17 is from §6.2.2.

| # | Claim | Verdict | Checked on |
|---|---|---|---|
| 11 | null pointer optimization makes an `Option<T>` occupy 0 bytes | **no** — `Option<&u8>` is 8 bytes, the same as `&u8`; the tag costs 0 *extra* bytes only when the inner type has a niche, and `Option<u64>` is 16 | section 1 below, [Nullable pointers](../../17_Option_and_Result/nullable_pointers/README.md) |
| 12 | pointers are memory addresses represented as integers of type `usize` | same width for a thin pointer; not the same thing — use `addr()`, since the Reference calls pointer-to-integer transmutes "currently undecided" | [Address, pointer, reference](../address_pointer_reference/README.md#a-pointer-is-more-than-its-address-provenance) |
| 13 | the address space is a façade provided by the OS and the CPU | yes: virtual addresses; the recorded x86-64 stack addresses in this library are 12 hex digits | [What an address shows](../../18_Ownership/what_an_address_shows/README.md) |
| 14 | in Rust, pointers are most often met as `&T` and `&mut T` | those are references — pointers in the Reference's umbrella sense, not raw pointers | [Address, pointer, reference](../address_pointer_reference/README.md#three-words-and-what-each-adds) |
| 15 | raw pointers are not subject to the borrowing rules | not **checked** by them; breaking the aliasing rules through raw pointers is still undefined behaviour, and Miri reports it | [Raw pointers](../raw_pointers/README.md#no-borrow-checker-and-the-rules-still-apply) |
| 16 | `*const i32` is the primitive type std documents as `pointer` | yes — std's `pointer` page, "Raw, unsafe pointers" | [Raw pointers](../raw_pointers/README.md) |
| 17 | thin pointers, such as raw pointers, are a single `usize` wide | a raw pointer to a sized type, yes; `*const [u8]` and `*const dyn Debug` are 16 bytes | [Wide pointers](../wide_pointers/README.md#a-raw-pointer-is-wide-too) |

## Listings 6.1 and 6.2: what the addresses show

Listing 6.1 prints three values with `{:p}`: an integer and references to two static byte arrays. Figure 6.2 says a reference is 4 bytes on a 32-bit CPU and 8 on a 64-bit one; that is `size_of::<usize>()`, and section 5 prints 8.

The addresses themselves are different on every run, which is why [no example in this library prints one](../../18_Ownership/what_an_address_shows/README.md#why-no-example-in-this-library-prints-one). Two things in them are still worth reading. The second array starts 10 bytes after the first in the book's run (`…480`, `…48a`), in listing 6.2's printed run (`…c830`, `…c83a`) and in a reader's run on x86-64 macOS (`0x101b9a6e1`, `0x101b9a6eb`) — the linker placed them back to back, which nothing guarantees. And none of the three shows alignment to 8, which is claim 7.

Listing 6.2 adds sizes, and they are the ones section 5 checks: `usize` 8, `&[u8; 10]` 8, `Box<[u8]>` 16, `[u8; 10]` 10, `[u8; 11]` 11. The reference is thin because the array length is in its type; the box is wide because it is not. Figure 6.3 gives `b` a length field and an address field — that is the `String` of listing 6.3, which the book says the figure resembles more closely, not the `&[u8; 10]` of listing 6.1.

## Listing 6.3: a `String` that frees a static

Listing 6.3 decodes the two arrays. Its `String` half comes down to this:

```rust
static NAME: [u8; 10] = *b"carrytowel";

fn main() {
    let owned = unsafe {
        let first = &NAME as *const u8 as *mut u8;
        String::from_raw_parts(first, 10, 10)
    };
    println!("{owned}");
}
```

[`String::from_raw_parts` ↗](https://doc.rust-lang.org/std/string/struct.String.html#method.from_raw_parts) hands ownership of the buffer to the `String`, and its safety section warns against doing that unless the bytes were "originally allocated by the Rust standard library's allocator". A `static` was not, and when `owned` goes out of scope its `Drop` hands the static's address to the allocator. The line prints; then the process dies:

```text title="Real run — x86-64 macOS 26.6, rustc 1.98.0, plain rustc and -O alike"
carrytowel
exit=134
```

```text title="Real run — Docker rust:1.98-slim, rustc 1.98.0, plain rustc and -O alike"
carrytowel
free(): invalid pointer
Aborted
exit=134
```

Exit 134 is SIGABRT. The `exit=` line is the shell's `echo "exit=$?"` after the run.

Miri finds the problem one step earlier, inside `Drop`, before any `free`: dropping the bytes makes a `&mut [u8]` to read-only memory.

```text title="Abridged — cargo +nightly miri run, nightly 2026-08-27, string_over_a_static.rs"
carrytowel
error: Undefined Behavior: constructing invalid value of type &mut [u8]: encountered mutable reference pointing to read-only memory
    = note: stack backtrace:
            0: std::ptr::drop_in_place::<[u8]>
            1: std::ptr::mut_ptr::<impl *mut [u8]>::drop_in_place
            2: <std::vec::Vec<u8> as std::ops::Drop>::drop
            3: std::ptr::drop_glue::<std::vec::Vec<u8>> - shim(Some(std::vec::Vec<u8>))
            4: std::ptr::drop_glue::<std::string::String> - shim(Some(std::string::String))
            5: main
                at src/bin/string_over_a_static.rs:9:1: 9:2
```

Neither half needs `unsafe` at all. Section 6 of the run reads the same bytes with `str::from_utf8(&NAME)`, which borrows, and `CStr::from_bytes_with_nul(&C_NAME)`, which checks the terminator — and the second compares equal to the C-string literal `c"thanksfish"`. The listing's `CStr::from_ptr(…).to_string_lossy()` is sound on its own and returns a `Cow::Borrowed` when the bytes are valid UTF-8, which they are. The [ToOwned reading list](../../12_Traits/how_to_learn_to_owned/to_owned_reading_list/README.md) already lists this listing under *read with care* for its `Cow`.

Three annotations on the listing, checked:

- **References cannot be cast directly to `*mut T`, hence the double cast.** True: `&NAME as *mut u8` is `E0606`. The double cast compiles, and writing through its result is refused by the `invalid_reference_casting` lint when rustc can see it — [Raw pointers](../raw_pointers/README.md#casting-away-const-is-allowed-writing-through-it-may-not-be) has both transcripts.
- **`c_char` is a type alias for `i8`.** On x86-64 and on every Apple target, yes. std's source (`core/src/ffi/primitives.rs`) makes it `u8` on aarch64 Linux, ARM, PowerPC, RISC-V and s390x, among others, following each platform's C ABI.
- **The conversion to `i8` works because the bytes stay under 128.** No conversion happens: a pointer cast relabels the address and touches no byte. Section 7 reads a byte of 200 through a `*const c_char`, and `to_bytes()` returns 200; only `to_string_lossy` cares, and replaces it with U+FFFD because it is not UTF-8.

## `memscan-1` to `memscan-3`

The chapter's memory-scanning listings read from address 0, then from address 1, then print the addresses of a static, a string literal, locals, boxes and a pointer to a dead local. On 1.98.0:

- **Address 0** panics in a plain `rustc` build with `null pointer dereference occurred` — a debug check newer than the book — and segfaults under `-O`.
- **Address 1** segfaults in both builds, on macOS and on Linux.
- **The pointer to a dead local** now draws the `dangling_pointers_from_locals` warning at compile time.
- **The `*const str` line** prints `Pointer { addr: …, metadata: 1 }` rather than a bare address, because `{:p}` on a wide pointer shows the length (section 8).

[Raw pointers](../raw_pointers/README.md#dangling-null-and-address-1) has the transcripts and the table of runs.

## The verified output

<!-- output:rust_in_action_chapter_6 -->
*Verified output of [`rust_in_action_chapter_6.rs`](examples/rust_in_action_chapter_6.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. "Option<T> occupies 0 bytes" (null pointer optimization)
   &u8                 8 bytes
   Option<&u8>         8 bytes
   Box<u8>             8 bytes
   Option<Box<u8>>     8 bytes
   *const u8           8 bytes
   Option<*const u8>  16 bytes
   u64                 8 bytes
   Option<u64>        16 bytes
   The Option costs 0 EXTRA bytes where the inner type has a niche;
   it never occupies 0 bytes, and without a niche it costs a word.

2. "References are aligned to multiples of usize"
   align_of::<u8>() = 1, align_of::<usize>() = 8
   two neighbouring &u8, one byte apart; one is not a multiple of 8: true
   A reference is aligned to align_of::<T>(), which for u8 is 1.

3. "A pointer and an integer" for dynamically sized types
   &[u8] second word is a length:     slice.len() = 4
   &dyn Debug second word is a vtable: size_of_val = 1 and 24
   The integer is right for slices and str; a trait object carries a
   pointer to a table of size, alignment, drop and methods instead.

4. "When a compiler creates a pointer to an i32, it can verify 4 bytes"
   0x1000 as *const i32 compiled; size_of::<i32>() = 4
   is_null() = false — and nothing checked that an i32 lives there.
   The size is known from the type. Whether the bytes exist is only
   guaranteed for a reference, never for a raw pointer.

5. Listing 6.2's sizes: a reference to an array, and a box of a slice
   usize 8   &[u8; 10] 8   Box<[u8]> 16   [u8; 10] 10   [u8; 11] 11
   boxed holds a copy, not the static: same bytes true, same address false
   &[u8; 10] is thin (the 10 is in the type); Box<[u8]> is wide (the 11
   moved into the pointer). Neither is a String's three words.

6. Listing 6.3's two strings, without from_raw_parts
   str::from_utf8(&NAME)               "carrytowel"   borrowed, nothing to free
   CStr::from_bytes_with_nul(&C_NAME)  "thanksfish"   checks the NUL is last
   equal to the literal c"thanksfish": true
   The book builds a String over the static with from_raw_parts, and
   that String then frees memory no allocator gave it. See the page.

7. "The conversion to c_char works because we stay under 128"
   bytes read through a *const c_char: [104, 200, 105]
   to_string_lossy():                  "h�i"
   A pointer cast converts no byte. 200 came through as 200; only the
   UTF-8 decoding in to_string_lossy cared, and replaced it with U+FFFD.

8. Printing a pointer to a str with {:p}, as memscan-3 does
   the text starts with "Pointer { addr: " true
   and ends with "metadata: 1 }"        true
   A *const str is wide, and {:p} on rustc 1.98.0 prints both words,
   where the book's run printed a bare address.
```
<!-- /output -->

Miri reports no undefined behaviour in this program; it notes the `0x1000 as *const i32` cast, which is never read.

## Where this leaves the chapter

Read §6.1 and §6.2 for the vocabulary — the reference/pointer/raw pointer naming rule is a good one, and this section follows it — and for the figures of an address space. Take alignment, the second word of a wide pointer, and the size of `Option` from the pages above instead. Run no listing that calls `from_raw_parts` on memory it did not allocate. The smart-pointer building blocks of §6.2.3 are checked in [Smart pointer claims, run](../../41_Smart_Pointers/smart_pointer_claims_checked/README.md#from-rust-in-action-623), including the `core::ptr::Shared` type the book names, which no longer exists.

## See also

- [Pointers](../README.md) — the section these claims were checked for
- [What `Cow` explanations get wrong, run](../../12_Traits/how_to_learn_to_owned/cow_claims_checked/README.md) — the same kind of page, for `Cow`
- [Books](../../10_Resources/books/README.md) — where *Rust in Action* sits among the other Rust books
- [`String::from_raw_parts`](../../14_Strings/string_methods/string_from_raw_parts/README.md) — the contract listing 6.3 breaks

## Sources

*Rust in Action*, Tim McNamara, Manning 2021, ch. 6 "Memory", §6.1, §6.2 and §6.2.2, and listings 6.1–6.3; the chapter's code in [rust-in-action/code, branch 1st-edition ↗](https://github.com/rust-in-action/code/tree/1st-edition/ch6). std's [`String::from_raw_parts` ↗](https://doc.rust-lang.org/std/string/struct.String.html#method.from_raw_parts), [`CStr` ↗](https://doc.rust-lang.org/std/ffi/struct.CStr.html) and [`c_char` ↗](https://doc.rust-lang.org/std/ffi/type.c_char.html). Runs: rustc 1.98.0 on x86-64 macOS 26.6 and in Docker `rust:1.98-slim`; Miri on nightly 2026-08-27.

## Po polsku

Rozdział 6 książki *Rust in Action* („Memory”) dobrze rozdziela **adres**, **wskaźnik** i **referencję** i proponuje sensowną regułę nazewnictwa, którą przejmuje ten dział. Myli się jednak w kilku konkretach, które warto znać przed lekturą:

- referencja jest wyrównana do `align_of::<T>()`, a nie do wielokrotności `usize` — adresy wydrukowane w samej książce kończą się na `…48a`;
- drugie słowo szerokiego wskaźnika to długość tylko dla `[T]` i `str`; dla `dyn Trait` to wskaźnik do vtable;
- `Option<&T>` nie zajmuje 0 bajtów, tylko tyle co `&T` (8), a `Option<u64>` zajmuje 16;
- listing 6.3 buduje `String` na tablicy `static` przez `String::from_raw_parts`, a `Drop` próbuje ją zwolnić — to niezdefiniowane zachowanie: na macOS proces kończy się sygnałem SIGABRT, na Linuksie komunikatem `free(): invalid pointer`, a Miri wskazuje `&mut` do pamięci tylko do odczytu. Te same bajty da się odczytać bez `unsafe`: `str::from_utf8` i `CStr::from_bytes_with_nul`;
- rzutowanie wskaźnika na `*const c_char` niczego nie konwertuje, a `c_char` jest `i8` tylko na części platform.

**Szukaj po polsku:** Rust in Action rozdział 6 · wyrównanie referencji · `String::from_raw_parts` na statycznej tablicy · `rust null pointer optimization Option size`
