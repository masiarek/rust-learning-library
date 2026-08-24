# Strings

**Level:** reference · the map

**One line:** Text in Rust is one pattern — an *owner* and a *view* — met four times: in the two everyday types, in the memory they manage, in the characters they encode, and in the wider family for text that keeps different promises. This page is the door to every lesson about it, in the order the questions come up.

**If you just want to know which type to write, read [`String` vs `&str`](01_Foundations/string_vs_str/README.md)** — the tables below are the route, not the explanation.

Same rule as the other maps: order is presentation, so it lives here rather than in folder names — [STRUCTS.md](STRUCTS.md) explains why in full, and [OPTION.md](OPTION.md) and [SHADOWING.md](SHADOWING.md) follow it too.

---

## The territory, as a picture

```mermaid
flowchart LR
    S["text in Rust<br/>one pattern: an OWNER and a VIEW"]

    S --> TY["THE TWO TYPES<br/>String owns, &str looks"]
    S --> MEM["MEMORY<br/>what the owner is made of"]
    S --> ENC["ENCODING<br/>what the bytes mean"]
    S --> FAM["THE FAMILY<br/>three promises, one pattern"]
    S --> ED["BUILDING & EDITING<br/>still missing, listed below"]

    TY --> T1["a literal is &'static str —<br/>in the binary, not the stack"]
    TY --> T2["deref coercion:<br/>&String becomes &str, free"]
    TY --> T3["parameters take &str,<br/>fields own String"]
    TY --> T4["String moves, &str copies"]

    MEM --> M1["ptr / len / capacity on the stack,<br/>bytes on the heap"]
    MEM --> M2["growth doubles;<br/>with_capacity pre-pays"]
    MEM --> M3["a Vec of u8<br/>that promises UTF-8"]

    ENC --> E1["char: one Unicode scalar,<br/>4 bytes as a value"]
    ENC --> E2["in a String: 1-4 UTF-8 bytes,<br/>so len() counts bytes"]
    ENC --> E3["bytes, chars, graphemes —<br/>three answers to 'how long'"]

    FAM --> F1["OsString / &OsStr —<br/>whatever the OS handed you"]
    FAM --> F2["CString / &CStr —<br/>text bound for C"]
    FAM --> F3["PathBuf / &Path —<br/>an OsString with path smarts"]
```

## Start here

| # | Lesson | Level | The question it answers |
|---|---|---|---|
| 1 | [`String` vs `&str`](01_Foundations/string_vs_str/README.md) | 101 → 201 | Which of the two types do I write here — and why does every parameter want `&str`? |
| 2 | [The anatomy of a `String`](01_Foundations/anatomy_of_a_string/README.md) | 101 → 201 | What the owner *is*: three words on the stack, bytes on the heap, and a capacity that is not the length |
| 3 | [Meet the `char`](01_Foundations/meet_the_char/README.md) | 101 → 201 | What the bytes encode — why `.len()` is not "how many characters", and why `s[0]` refuses to compile |
| 4 | [Six kinds of string](01_Foundations/six_kinds_of_string/README.md) | 201 | Why `OsString` and `CString` exist, and the one owned/borrowed pattern all six types repeat |

## The lessons strings lean on

Strings are the worked example half the library's ownership pages already use, so the deep explanations live there:

| Lesson | Level | What it settles for strings |
|---|---|---|
| [Ownership and moves](01_Foundations/ownership_and_moves/README.md) | 101 | The `E0382` a moved `String` produces — in full, with a value that announces its own death |
| [`Copy` vs `Clone`](01_Foundations/copy_vs_clone/README.md) | 101 → 201 | Why `&str` copies freely, and one `String` field makes a whole struct move |
| [Borrowing](01_Foundations/borrowing/README.md) | 101 → 201 | The rule that refuses a view held across a `push_str` |
| [How to learn lifetimes](01_Foundations/how_to_learn_lifetimes/README.md) | 201 | Why "own `String`, clone when stuck" is legitimate advice while `&str` fields wait |
| [Meet the byte](01_Foundations/meet_the_byte/README.md) | 101 → 201 | The unit `len` counts in — this map's encoding arc is what those bytes *mean* |
| [What is a ballot, in memory?](01_Foundations/representing_a_ballot/README.md) | 201 | `String` fields chosen inside a real struct design |
| [`Path` and `PathBuf`](04_Files/path_and_pathbuf/README.md) | 201 | The family's honorary pair, in full — a **stub** for now |

## Still missing

Named honestly, because a map that only lists what exists is a map of the wrong territory. Each becomes a page once it has a runnable example worth reading — rough order, not a promise:

- **Building and editing** — `push` vs `push_str`, `insert`, `truncate`, `pop`, and why `+` consumes its left operand while `format!` borrows everything
- **Parsing out of a string** — `.parse()`, the turbofish, and `FromStr` as the trait behind both
- **Searching and splitting** — `find`/`contains`, `split` and its many siblings, `trim`, `lines`, and the `Pattern` type that unifies them
- **Slicing that panics** — `&s[a..b]` off a char boundary at runtime, `get` as the total version, and where the panic actually points
- **`Cow<str>`** — the maybe-owned string: borrow when unchanged, allocate only when you must
- **The third owned form** — `Box<str>`, `Rc<str>`, `Arc<str>`: frozen text without the capacity word, and when the small saving matters
- **Raw strings and escapes** — `r"…"`, `r#"…"#`, `\u{…}`, and byte strings `b"…"`
- **Comparing and sorting** — `Ord` on strings is byte order, which is not human order; case folding vs `to_lowercase`
- **String APIs worth copying** — `impl AsRef<str>`, `Into<String>`, and when a signature should take `impl Display`

If you want one of these next, that is the list to point at.

## Looking a term up

[GLOSSARY.md](GLOSSARY.md) defines the vocabulary these pages use — string slice, string literal, deref coercion, capacity, Unicode scalar value, grapheme cluster, the `OsString` and `CString` pairs — and every entry links the page that explains it properly.
