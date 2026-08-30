# Six kinds of string

**Level:** 201 · working knowledge

**One line:** `String`, `&str`, `OsString`, `&OsStr`, `CString`, `&CStr` — six names, one pattern: three different *promises about the bytes*, each in an owned and a borrowed form. You write the first pair daily, meet the second at every filename, and the third only at a C boundary.

"Rust has six string types" is a standing complaint, and as a count it is generous — `PathBuf`, `Cow<str>`, `Box<str>` and friends push it into double digits. The complaint dissolves once you stop counting *types* and start counting *promises*. There are three, and you already know the owned/borrowed split from [`String` vs `&str`](../string_vs_str/README.md); the rest is that split, three times:

| what the bytes promise | owned | borrowed | you meet it |
|---|---|---|---|
| valid UTF-8, checked at the door | [`String` ↗](https://doc.rust-lang.org/std/string/struct.String.html) | [`&str` ↗](https://doc.rust-lang.org/std/primitive.str.html) | all ordinary text |
| whatever the OS handed you | [`OsString` ↗](https://doc.rust-lang.org/std/ffi/struct.OsString.html) | [`&OsStr` ↗](https://doc.rust-lang.org/std/ffi/struct.OsStr.html) | filenames, env vars, `args_os` |
| no NUL inside, one NUL at the end | [`CString` ↗](https://doc.rust-lang.org/std/ffi/struct.CString.html) | [`&CStr` ↗](https://doc.rust-lang.org/std/ffi/struct.CStr.html) | calling C |
| the OS row, plus path smarts | [`PathBuf` ↗](https://doc.rust-lang.org/std/path/struct.PathBuf.html) | [`&Path` ↗](https://doc.rust-lang.org/std/path/struct.Path.html) | every file API |
| nothing at all — just bytes | [`Vec<u8>` ↗](https://doc.rust-lang.org/std/vec/struct.Vec.html) | [`&[u8]` ↗](https://doc.rust-lang.org/std/primitive.slice.html) | I/O buffers |

The last two rows are the honorary members: [`Path` is an `OsStr` that knows about `/`](../../04_Files/path_and_pathbuf/README.md), and `Vec<u8>` is what remains when no promise is made.

---

## Why one type cannot do it

Each promise is real, and they conflict:

- A **Linux filename** may be any bytes except NUL and `/` — nothing says UTF-8. A **Windows** filename is UTF-16 that may contain unpaired surrogates, which valid UTF-8 cannot represent.
- A **C string** ends at the first NUL, so it cannot *contain* one — while a `String` happily can (`"a\0b"` is three fine chars).
- A **`String`** guarantees valid UTF-8 to every method on it — the guarantee [`.chars()` and friends](../meet_the_char/README.md) lean on.

Force all text through `String` and a real filename either crashes your program or gets silently mangled. Rust's answer is to make the difference a *type*, so text of unknown pedigree cannot reach the functions that assume pedigree. The complaint is real; the alternative was worse.

## Narrowing is where the promise gets checked

Widening — toward fewer guarantees — is free and silent: any `&str` is a fine `OsString`. Narrowing — toward more — is exactly where a check must happen, so the signature says so:

```rust
let tidy = OsString::from("results.yaml");
tidy.to_str()              // Some("results.yaml") — it happened to be UTF-8

let wild = OsStr::from_bytes(&[b'b', b'v', 0xFF, b'.', b'y']);   // a legal Unix filename
wild.to_str()              // None — String would have to lie
wild.to_string_lossy()     // "bv�.y" — data lost, visibly, on purpose

CString::new("STAR\0vote") // Err: nul byte found in provided data at position: 4
```

That `Option` is not friction, it is the honest answer to "is this filename printable text?" — a question with a real *no*. The `to_string_lossy` escape hatch replaces what will not convert with `�`, which is the right tool for a log line and the wrong one for a path you intend to reopen.

## The everyday consequence

The complaint's practical half — *"some functions return `&str` and you need to convert to `String`"* — is the owned/borrowed split, not the family. `&str → String` is `.to_string()` (one allocation, asked for out loud); `&String → &str` is free by deref coercion. If a signature keeps fighting you, the fix is usually [taking `&str` in parameters and owning `String` in fields](../string_vs_str/README.md) — the family's other four types only enter when the OS or C does.

## If you are coming from another language

**Python.** Python has the same three worlds — it just checks them at different moments.

| Python | | Rust |
|---|---|---|
| `str` vs `bytes` | the text/bytes split | `String` vs `Vec<u8>` |
| `os.fsdecode` / PEP 383 | undecodable filename bytes smuggled into `str` as lone surrogates | `OsString` — a separate type instead of smuggling |
| `open("f.txt")` just works | until a weird filename reaches production | `Path` APIs take `OsStr`, so the weird name never crosses into text |
| `ctypes.c_char_p(b"hi")` | you remember the NUL rules | `CString::new` refuses an interior NUL with an error |

What changes: Python's surrogateescape keeps one string type by hiding the problem inside values that crash on `.encode()` later; Rust surfaces the same problem as a type, at the boundary, where the `None` still means something.

**ABAP.** SAP controls its own world end to end, which is why this problem never bit you there.

| ABAP | | Rust |
|---|---|---|
| `string` vs `xstring` | text vs raw bytes | `String` vs `Vec<u8>` |
| one system codepage, kernel-enforced | text is *always* decodable | no such luck: the OS promises nothing |
| RFC/iDoc conversion at the boundary | the kernel converts for you | `to_str()` / `to_string_lossy()` — you choose the failure mode |

What changes: the boundary types exist because a Rust program does not get to assume a managed landscape — it meets the filesystem raw, so the conversion step ABAP's kernel does invisibly becomes a visible `Option` in your code.

---

## Practice

**Three arrivals, three types.** Text you built yourself, a filename from the OS, a string headed into a C library: name the right type for each, out loud.

Then break both promises on purpose: forge a non-UTF-8 filename with `OsStr::from_bytes` (unix-only import, which is rather the point) and read what `to_str()` and `to_string_lossy()` each do with it; feed `CString::new` an interior NUL and read the error — including *which byte* it names. Finish by narrowing a well-behaved `CString` back to `&str` and note which checked conversions answer `Option` and which answer `Result`.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:six_kinds_of_string_kata -->
*[`six_kinds_of_string_kata.rs`](examples/six_kinds_of_string_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: three arrivals, three types — then break both promises
//! on purpose and read the refusals.
//!
//!   rustc --edition 2024 six_kinds_of_string_kata.rs -o /tmp/sixk && /tmp/sixk

use std::ffi::{CString, OsStr};

fn main() {
    println!("Round 1 — route each arrival to its type");
    println!("   text you built yourself      -> String    (UTF-8 is yours to promise)");
    println!("   a filename from the OS       -> OsString  (the OS promised nothing)");
    println!("   a string headed into C code  -> CString   (C stops at the first NUL)");

    println!("\nRound 2 — break the UTF-8 promise");
    use std::os::unix::ffi::OsStrExt;
    let filename = OsStr::from_bytes(&[b'c', b'a', b's', b'e', 0xF5, b'.', b'y', b'a', b'm', b'l']);
    println!("   a real, legal Unix filename: {filename:?}");
    match filename.to_str() {
        Some(s) => println!("   to_str() -> Some({s:?})"),
        None => println!("   to_str() -> None      <- not UTF-8; String would have to lie"),
    }
    println!("   to_string_lossy() -> {:?}   <- the byte is gone, and says so", filename.to_string_lossy());

    println!("\nRound 3 — break the NUL promise");
    match CString::new("tally\0sheet") {
        Ok(c) => println!("   unexpectedly fine: {c:?}"),
        Err(e) => {
            println!("   CString::new(\"tally\\0sheet\") -> Err");
            println!("   the error names the byte: {e}");
            println!("   C would have read only {:?} — Rust refuses instead", "tally");
        }
    }

    println!("\nRound 4 — and the promise that always holds");
    let c = CString::new("tally").unwrap();
    let back = c.to_str();
    println!("   a CString of plain ASCII is also valid UTF-8: to_str() = {back:?}");
    println!("   every narrowing that checks out hands you the &str view for free");
}
```
<!-- /source -->

<!-- output:six_kinds_of_string_kata -->
*Verified output of [`six_kinds_of_string_kata.rs`](examples/six_kinds_of_string_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Round 1 — route each arrival to its type
   text you built yourself      -> String    (UTF-8 is yours to promise)
   a filename from the OS       -> OsString  (the OS promised nothing)
   a string headed into C code  -> CString   (C stops at the first NUL)

Round 2 — break the UTF-8 promise
   a real, legal Unix filename: "case\xF5.yaml"
   to_str() -> None      <- not UTF-8; String would have to lie
   to_string_lossy() -> "case�.yaml"   <- the byte is gone, and says so

Round 3 — break the NUL promise
   CString::new("tally\0sheet") -> Err
   the error names the byte: nul byte found in provided data at position: 5
   C would have read only "tally" — Rust refuses instead

Round 4 — and the promise that always holds
   a CString of plain ASCII is also valid UTF-8: to_str() = Ok("tally")
   every narrowing that checks out hands you the &str view for free
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:six_kinds_of_string -->
*Verified output of [`six_kinds_of_string.rs`](examples/six_kinds_of_string.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The pattern: every pair is String-and-&str again
   owned      borrowed   what the bytes promise
   String     &str       valid UTF-8, always
   OsString   &OsStr     whatever the OS handed you
   CString    &CStr      no NUL inside, one NUL at the end
   PathBuf    &Path      an OsString that knows about '/'
   Vec<u8>    &[u8]      nothing at all — just bytes

2. OsStr: the honest type for filenames
   to_str() on a UTF-8 name     = Some("results.yaml")
   to_str() on a non-UTF-8 one  = None
   to_string_lossy()            = "bv�.y"   <- data lost, visibly

3. CString: the contract C needs
   CString::new("STAR")        = "STAR"
   CString::new("STAR\0vote")  = Err: nul byte found in provided data at position: 4

4. Path: an OsStr that knows the shape of a path
   file_stem() = Some("results")
   extension() = Some("yaml")
   parent()    = Some("04_Approval/cases")

5. Widening is free; narrowing returns an Option
   &str -> OsString -> to_str() = Some("turnout.csv")
   narrowing is where a promise gets CHECKED — which is why the
   cheap direction never asks, and the checked one answers with
   an Option (OsStr) or a Result naming the fault (CStr)
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 14_Strings/six_kinds_of_string/examples/six_kinds_of_string.rs -o /tmp/six && /tmp/six
```

## See also

- [STRINGS.md](../../STRINGS.md) — the map: every string lesson, in reading order
- [`String` vs `&str`](../string_vs_str/README.md) — the owned/borrowed split this family repeats
- [Meet the `char`](../meet_the_char/README.md) — what the UTF-8 promise buys
- [`Path` and `PathBuf`](../../04_Files/path_and_pathbuf/README.md) — the honorary pair, in full (a stub for now)
- [std docs — `std::ffi` ↗](https://doc.rust-lang.org/std/ffi/index.html), where `OsString` and `CString` live and the encodings are spelled out
