# The strings map

**Level:** reference · the map

**One line:** Text in Rust is one pattern — an *owner* and a *view* — met four times: in the two everyday types, in the memory they manage, in the characters they encode, and in the wider family for text that keeps different promises. This page is the door to every lesson about it, in the order the questions come up.

**If you just want to know which type to write, read [`String` vs `&str`](14_Strings/string_vs_str/README.md)** — the tables below are the route, not the explanation.

Same rule as the other maps: order is presentation, so it lives here rather than in folder names — [STRUCTS.md](STRUCTS.md) explains why in full, and [OPTION.md](OPTION.md) and [SHADOWING.md](SHADOWING.md) follow it too.

---

## The territory, as a picture

```mermaid
flowchart LR
    S["text in Rust<br/>one pattern: an OWNER and a VIEW"]

    S --> TY["THE TWO TYPES<br/>String owns, &str looks"]
    S --> MEM["MEMORY<br/>what the owner is made of"]
    S --> ENC["ENCODING<br/>what the bytes mean"]
    S --> USE["MAKING & EDITING<br/>getting one, growing one"]
    S --> FAM["THE FAMILY<br/>three promises, one pattern"]

    TY --> T1["a literal is &'static str —<br/>in the binary, not the stack"]
    TY --> T2["deref coercion:<br/>&String becomes &str, free"]
    TY --> T3["parameters take &str,<br/>fields own String"]
    TY --> T4["String moves, &str copies"]
    TY --> T5["a slice is ptr + len;<br/>its indices are BYTES"]
    TY --> T6["&str and &'static str:<br/>on a literal, one type"]

    MEM --> M1["ptr / len / capacity on the stack,<br/>bytes on the heap"]
    MEM --> M2["growth doubles;<br/>with_capacity pre-pays"]
    MEM --> M3["a Vec of u8<br/>that promises UTF-8"]

    ENC --> E1["char: one Unicode scalar,<br/>4 bytes as a value"]
    ENC --> E2["in a String: 1-4 UTF-8 bytes,<br/>so len() counts bytes"]
    ENC --> E3["bytes, chars, UTF-16, graphemes —<br/>four answers to 'how long'"]
    ENC --> E4["walking it: u8, char, &str —<br/>splits are the gaps between matches"]

    USE --> U1["to_string comes free<br/>from Display"]
    USE --> U2["two &str will not add —<br/>+ needs an owned left"]
    USE --> U3["push_str grows in place;<br/>format! borrows everything"]
    USE --> U4["every edit is byte-indexed,<br/>so every edit can panic"]

    FAM --> F1["OsString / &OsStr —<br/>whatever the OS handed you"]
    FAM --> F2["CString / &CStr —<br/>text bound for C"]
    FAM --> F3["PathBuf / &Path —<br/>an OsString with path smarts"]
```

## Start here

| # | Lesson | Level | The question it answers |
|---|---|---|---|
| 1 | [`String` vs `&str`](14_Strings/string_vs_str/README.md) | 101 → 201 | Which of the two types do I write here — and why does every parameter want `&str`? |
| 2 | [String slices](14_Strings/string_slices/README.md) | 101 → 201 | What a view actually is: the stale index it replaces, the `E0502` that keeps it honest, and the one way `&s[a..b]` panics |
| 3 | [The anatomy of a `String`](14_Strings/anatomy_of_a_string/README.md) | 101 → 201 | What the owner *is*: three words on the stack, bytes on the heap, and a capacity that is not the length |
| 4 | [Making a `String`](14_Strings/making_a_string/README.md) | 101 → 201 | `to_string` vs `to_owned` vs `String::from` vs `into` vs `format!` — and why `Display` is the only one you implement |
| 5 | [Concatenating strings](14_Strings/concatenating_strings/README.md) | 101 → 201 | How to join two pieces of text — and why `+` insists on an owned `String` on the left, which is the whole of `E0369`/`E0308`/`E0368` |
| 6 | [Building a `String`](14_Strings/building_a_string/README.md) | 101 → 201 | `push_str`, `push`, the `+` that consumes its left operand, and `format!` vs `write!` inside a loop |
| 7 | [Meet the `char`](14_Strings/meet_the_char/README.md) | 101 → 201 | What the bytes encode — why `.len()` is not "how many characters", and why `s[0]` refuses to compile |
| 8 | [Raw strings, escapes and the literal prefixes](14_Strings/raw_strings_and_escapes/README.md) | 101 → 201 | Every way to write text in source — `r"…"`, `b"…"`, `c"…"`, `\x`, `\u{…}` — and why `"C:\temp\new"` is two bytes shorter than it looks |
| 9 | [Walking a `String`](14_Strings/walking_a_string/README.md) | 101 → 201 | Three item types and the split family — and why `split(' ')` and `split_whitespace()` disagree about empty fields |
| 10 | [`&'static str`](14_Strings/static_str/README.md) | 201 | Is it different from `&str`? On a literal, no — and the claim that a `String` can never yield one is false |
| 11 | [String parameters worth copying](14_Strings/string_api_design/README.md) | 201 → 301 | Which signature makes the caller pay — `&str`, `impl AsRef<str>`, `impl Into<String>` or a `Cow` return — counted in allocations rather than asserted |
| 12 | [Six kinds of string](14_Strings/six_kinds_of_string/README.md) | 201 | Why `OsString` and `CString` exist, and the one owned/borrowed pattern all six types repeat |
| 13 | [`str` is unsized](14_Strings/str_is_unsized/README.md) | 201 | Why you never hold a `str`, only a pointer to one — the size that belongs to the value, the fat pointer's second word, and `?Sized` as a relaxation |
| 14 | [Inside a `Split`](14_Strings/inside_a_split/README.md) | 201 → 301 | Why printing `s.split(":")` with `{:?}` gives a struct full of `crit_pos` and `byteset` instead of `["a", "b", "c"]` — and what every field of it means |
| 15 | [The third owned form](14_Strings/boxed_str/README.md) | 201 → 301 | `Box<str>`, `Rc<str>`, `Arc<str>`: the capacity word dropped, and the interning move that makes a repeated column stop paying per row |
| 16 | [Four lengths, and which one the other system means](14_Strings/four_lengths/README.md) | 201 | One string, four counts — and why a name that fits a `VARCHAR(12)` can bounce off an `nvarchar(12)` |
| 17 | [Searching without splitting](14_Strings/searching_a_string/README.md) | 101 → 201 | Is it there, where is it, does it start like this — and why one trait makes a `char`, a `&str` and a closure the same argument |
| 18 | [Replacing part of a string](14_Strings/replacing_in_a_string/README.md) | 101 → 201 | `replace` hands back a new `String`; `replace_range` and `retain` are the two that edit in place, and a chain of replaces is not a substitution table |
| 19 | [`str::as_str`: the method that was stabilized and taken back](14_Strings/str_as_str/README.md) | 201 → 301 | Why `as_str` works on a `String` and is `E0658` on everything else that owns text — and what an inherent method added to `str` or `[T]` does to the crates that already extend them |
| 20 | [Splitting on nothing](14_Strings/splitting_on_nothing/README.md) | 201 | Why `"abc".split("")` yields `["", "a", "b", "c", ""]` — and why the empty string is the one that splits into two pieces |
| 21 | [Comparing and sorting text](14_Strings/comparing_strings/README.md) | 201 | What `==` and `sort()` actually compare — and why the answer is reproducible on every machine and still wrong for every reader |
| 22 | [Parsing out of a string](14_Strings/parsing_a_string/README.md) | 101 → 201 | Text on the way in — what names the type, what the string is allowed to look like, and what the failure will tell you |
| 23 | [The format mini-language](14_Strings/the_format_language/README.md) | 201 | Text on the way out — the whole grammar, and why a format spec is a request an impl is free to ignore |
| 24 | [When the UTF-8 invariant broke](14_Strings/when_the_invariant_broke/README.md) | 301 | Has `std`'s own string code ever been wrong about memory — and what did safe code have to do to break it? |
| 25 | [Wrong, but not unsafe](14_Strings/wrong_but_not_unsafe/README.md) | 201 → 301 | And has it ever simply returned the wrong answer, with nothing unsafe anywhere near it? |

## Starting from nothing

The table above is ordered by the question you arrive with. If you arrive with none — new to Rust, or new to its text — this is the other order: by what each step needs from the one before. Ownership first, then the two types, then what they cost, then programs.

What the whole route builds, in four lines:

```text
String   owned, growable, on the heap: a Vec<u8> that promises UTF-8
&str     borrowed and read-only: points into a String, the binary, or anything holding text
str      unsized: never held directly, only behind a pointer (&str, Box<str>, Rc<str>)
"text"   a literal is a &'static str, and its bytes are in the binary
```

| Step | Read | What you can say afterwards |
|---|---|---|
| 0 | [Ownership and moves](18_Ownership/ownership_and_moves/README.md) · [Borrowing](18_Ownership/borrowing/README.md) · [Stack and heap](18_Ownership/stack_and_heap/README.md) · [`Copy` vs `Clone`](16_Structs/copy_vs_clone/README.md) | Who frees a value, and why a `String` moves where a `&str` copies |
| 1 | [`String` vs `&str`](14_Strings/string_vs_str/README.md) | A parameter takes `&str`, and a `&String` coerces to it for free because `String: Deref<Target = str>` |
| 2 | [The anatomy of a `String`](14_Strings/anatomy_of_a_string/README.md) · [`&'static str`](14_Strings/static_str/README.md) | A `String` is pointer, length and capacity with its bytes on the heap; a literal's bytes are in the binary |
| 3 | [String slices](14_Strings/string_slices/README.md) · [`str` is unsized](14_Strings/str_is_unsized/README.md) | A `&str` is a fat pointer — address and length — into bytes someone else owns, and `&s[a..b]` panics off a character boundary |
| 4 | [Making a `String`](14_Strings/making_a_string/README.md) · [Concatenating strings](14_Strings/concatenating_strings/README.md) · [Building a `String`](14_Strings/building_a_string/README.md) | The ways from `&str` to `String`, and why `+` needs an owned left |
| 5 | [Meet the `char`](14_Strings/meet_the_char/README.md) · [Four lengths](14_Strings/four_lengths/README.md) | A character is 1 to 4 bytes in UTF-8, so `.len()` counts bytes |
| 6 | [`Cow`: borrow until somebody writes](18_Ownership/clone_on_write/README.md) · [`String::from_utf8_lossy`](14_Strings/string_methods/string_from_utf8_lossy/README.md) | A function that allocates only sometimes, and std's own example of one |
| 7 | [`str` methods](14_Strings/str_methods/README.md) · [`String` methods](14_Strings/string_methods/README.md) | The reference, once you know what you are looking for |
| 8 | [String parameters worth copying](14_Strings/string_api_design/README.md) | Which signature to write — `&str` in, `String` out, a `String` field — with the allocations counted |

Every step but 7, the reference you look things up in, has katas to type; [KATAS.md](KATAS.md) groups them by subject.

## The lessons strings lean on

Strings are the worked example half the library's ownership pages already use, so the deep explanations live there:

| Lesson | Level | What it settles for strings |
|---|---|---|
| [Ownership and moves](18_Ownership/ownership_and_moves/README.md) | 101 | The `E0382` a moved `String` produces — in full, with a value that announces its own death |
| [`Copy` vs `Clone`](16_Structs/copy_vs_clone/README.md) | 101 → 201 | Why `&str` copies freely, and one `String` field makes a whole struct move |
| [Borrowing](18_Ownership/borrowing/README.md) | 101 → 201 | The rule that refuses a view held across a `push_str` |
| [How to learn lifetimes](18_Ownership/how_to_learn_lifetimes/README.md) | 201 | Why "own `String`, clone when stuck" is legitimate advice while `&str` fields wait |
| [Meet the byte](19_Numbers/meet_the_byte/README.md) | 101 → 201 | The unit `len` counts in — this map's encoding arc is what those bytes *mean* |
| [What is a record, in memory?](16_Structs/representing_a_record/README.md) | 201 | `String` fields chosen inside a real struct design |
| [`Path` and `PathBuf`](04_Files/path_and_pathbuf/README.md) | 201 | The family's honorary pair, in full — a **stub** for now |
| [A file is bytes; a `String` is a promise](04_Files/a_file_is_bytes/README.md) | 201 | Where the promise is dropped and where it is checked — `write_all` takes `&[u8]`, `read_to_string` refuses non-UTF-8 with `InvalidData` and leaves your `String` untouched, and `include_str!` runs the same check at compile time |
| [`Cow`: borrow until somebody writes](18_Ownership/clone_on_write/README.md) | 201 | The maybe-owned string: borrow when the text needs no change, allocate only on the write |
| [The global allocator](09_Advanced/the_global_allocator/README.md) | 301 | Where the bytes on the heap come from — and how to count a `String`'s allocations instead of inferring them from `capacity()` |

## Still written as outlines

Named honestly, because a map that only lists what exists is a map of the wrong territory. Each of these is a **stub** — a real page at a real URL, carrying the questions the finished lesson has to answer and the links to its neighbours, but with no runnable example behind it yet. That is the boundary this library draws: a stub states what it does not know, and nothing on it has been through the check that backs every other claim here.

| The page | Level | What it will answer |
|---|---|---|
| [When `String` is too slow](14_Strings/when_string_is_too_slow/README.md) | 301 | `with_capacity` in anger, `smallstr` / `smartstring`, and avoiding a `format!` that a literal would do |

A stub graduates by gaining an `examples/` program, losing its notice, and — if it earned one on the way — a row in [KATAS.md](KATAS.md). If you want one of these next, that is the table to point at.

## Every method, one page each

The lessons above are the ideas; this is the reference to reach for once you know what you are looking for.

| reference | pages | covers |
|---|---|---|
| [`str` methods](14_Strings/str_methods/README.md) | 83 | Reading text — searching, splitting, trimming, case, parsing, escapes, and the two byte-offset panics |
| [`String` methods](14_Strings/string_methods/README.md) | 42 | Owning text — building, growing, removing, capacity, and giving the allocation away |

Each page states the signature and the stability, explains what the method does and the mistake it is usually part of, and ends with a complete program plus its verified output. `String` derefs to `str`, so every method on the first list works on a `String` too.

## Where else to learn this

[Strings: links, books and videos](14_Strings/resources/README.md) collects the outside sources — the Book's ch. 8.2, *Programming Rust* ch. 17, Easy Rust ch. 14 and its video, the essays, the two forum threads worth reading whole — including [*`String` vs `str`, why?* ↗](https://users.rust-lang.org/t/string-vs-str-why/61334), the source of the `StringBuf` reading on the [`String` vs `&str`](14_Strings/string_vs_str/README.md#the-names-hide-the-pattern) page — and the external exercise sets (rustlings' `strings` and `conversions`) that map onto these lessons.

[The string crates](14_Strings/string_crates/README.md) is the crates half: fifteen crates for the jobs `std` leaves out — graphemes, normalization, legacy encodings, bytes that are not UTF-8, short strings off the heap — each beside the lesson here that covers what `std` does instead.

## Looking a term up

[GLOSSARY.md](GLOSSARY.md) defines the vocabulary these pages use — string slice, string literal, deref coercion, capacity, Unicode scalar value, grapheme cluster, the `OsString` and `CString` pairs — and every entry links the page that explains it properly.

## Po polsku

Tekst w Ruscie to jeden wzorzec spotykany cztery razy: **właściciel i widok**. `String` posiada, `&str` ogląda — i ta sama para wraca przy `Vec<T>` / `&[T]`, przy `PathBuf` / `Path`, przy `OsString` / `OsStr`. Kto zobaczy ten wzorzec raz, przestaje uczyć się każdej z tych par osobno.

Dla polskiego czytelnika ten dział jest ważniejszy niż dla angielskiego, i to z jednego konkretnego powodu: **każda z liter ą ć ę ł ń ó ś ź ż zajmuje w UTF-8 dwa bajty**. Dlatego `"Łódź".len()` daje 7, a nie 4; dlatego `&s[0..1]` potrafi wywołać panikę o granicy znaku; i dlatego cała rodzina metod z `ascii` w nazwie po cichu nic nie robi na polskim tekście — `to_ascii_uppercase` nie ruszy `ł`, a `split_ascii_whitespace` nie widzi twardej spacji, którą polska typografia stawia po spójnikach jednoliterowych. Angielski czytelnik może przejść przez ten dział i nigdy się na to nie natknąć; polski natknie się pierwszego dnia.

Nazewnictwo, którego trzymamy się w całej bibliotece: `String` to **łańcuch znaków**, `&str` to **wycinek łańcucha**, a `char` to **znak** — przy czym jest to punkt kodowy Unicode, a nie bajt i nie „litera na ekranie".

**Szukaj po polsku:** łańcuchy znaków w Ruscie · `String` a `&str` · kodowanie UTF-8 · polskie znaki diakrytyczne · `rust char boundary panic`
