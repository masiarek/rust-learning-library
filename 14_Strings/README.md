# Strings

**One line:** Text in Rust is one pattern met over and over — an *owner* and a *view* — and almost every surprise in this section comes from the same place: the owner keeps its bytes on the heap, and those bytes are UTF-8, so an index into them is a byte offset rather than a character.

Two types do nearly all the work. `String` owns its text and can grow it; `&str` is a borrowed window onto text somebody else owns — including the binary itself, which is where a literal lives. A function takes `&str` because it only needs to read; a struct field owns a `String` because it has to outlive the call that built it.

The rest of the section is what follows from the bytes underneath. `len()` counts bytes, not characters. `s[0]` does not compile. `+` insists on an owned value on its left. And a slice whose endpoint lands inside a character panics at run time rather than at compile time — the one place strings ask you to be careful rather than letting the compiler be careful for you.

| Lesson | Level | What it teaches |
|---|---|---|
| [`String` vs `&str`](string_vs_str/README.md) | 101 → 201 | The owner and the view — a literal lives in the *binary*, not the stack; `&String` coerces to `&str` for free; and why parameters take `&str` while fields own `String` |
| [String slices](string_slices/README.md) | 101 → 201 | A view is a pointer and a length — the stale byte index it replaces, the `E0502` that keeps it honest, `&s[..5]`, and the one way a slice panics: an index inside a character |
| [The anatomy of a `String`](anatomy_of_a_string/README.md) | 101 → 201 | Three words on the stack, bytes on the heap — `len` is what you have, `capacity` is what you paid for, growth doubles, and the borrow checker's rule against a view held across a `push_str` |
| [Making a `String`](making_a_string/README.md) | 101 → 201 | Five spellings for one conversion — and why you implement `Display`, never `ToString`, plus the `.to_string()` on a `String` that is a silent clone |
| [Concatenating strings](concatenating_strings/README.md) | 101 → 201 | `format!`, `+` and `join` — the single `Add` impl behind all of it, and the three error codes you get from putting the wrong side on the left |
| [Building a `String`](building_a_string/README.md) | 101 → 201 | `push_str`, `push`, and the `+` that eats its left operand — `format!` vs `write!` in a loop, and why `truncate` panics where a slice does |
| [Meet the `char`](meet_the_char/README.md) | 101 → 201 | One Unicode scalar, four bytes as a value, 1–4 inside a `String` — why `.len()` is not "how many characters", `s[0]` refuses to compile, and `'ß'.to_uppercase()` returns *two* letters |
| [Raw strings, escapes and the literal prefixes](raw_strings_and_escapes/README.md) | 101 → 201 | Every way to write text in source — `r"…"` turns the escapes off, `b"…"` drops the UTF-8 promise, `c"…"` adds the NUL — and `"C:\temp\new"` is two bytes shorter than it looks |
| [RFC 69 — how Rust got `b'A'`](rfc_69_byte_literals/README.md) | 201 | Where that table came from — two pages in 2014, the alternatives it rejected, its three unresolved questions all since answered, and the pattern syntax that changed meaning underneath its one example |
| [Walking a `String`](walking_a_string/README.md) | 101 → 201 | Three item types and the split family — splits are the gaps between matches, `char_indices()` is not `chars().enumerate()`, and `split_whitespace()` silently shortens a row |
| [`&'static str`](static_str/README.md) | 201 | On a literal it is the same type as `&str` — where the annotation starts refusing things, `const` vs `static`, and the three ways a `String` really can yield one |
| [Six kinds of string](six_kinds_of_string/README.md) | 201 | `OsString`, `CString` and friends are not five more inventions — three promises about the bytes, each owned or borrowed, and narrowing is where a promise gets checked |
| [`str` is unsized](str_is_unsized/README.md) | 201 | Why you never hold a `str`, only a pointer to one — the size that belongs to the value rather than the type, the fat pointer's second word, `?Sized` as a *relaxation*, and the struct field that makes a whole struct unsized |
| [Inside a `Split`](inside_a_split/README.md) | 201 → 301 | Why `println!("{:?}", s.split(":"))` prints a struct and not your pieces — the plan read field by field, the pattern that picks the searcher, and the one bool that is `split_terminator` |

## The method reference

The lessons above teach the ideas. When you know which method you want and need to know how it behaves at the edges — what it panics on, what it allocates, which of the four pattern shapes it accepts — there is a page for every one:

| reference | pages | what is in it |
|---|---|---|
| [`str` methods](str_methods/README.md) | 83 | Everything about *reading* text: searching, splitting, trimming, case, parsing, and the byte-offset panics |
| [`String` methods](string_methods/README.md) | 42 | Everything that needs *ownership*: building, growing, removing, capacity, and handing the allocation away |

Every page carries the signature, the stability line, the trap the method is usually involved in, and a complete runnable program whose printed output is checked by CI — so nothing on them is a claim about what Rust does, only a record of what it did.

The split follows the types: because `String` dereferences to `str`, every `str` method works on a `String` unchanged, which is why the borrowed side is twice the size.

## The lessons strings lean on

Strings are the worked example half the ownership pages already use, so the deep explanations live there and this section links them rather than moving them:

- [Ownership and moves](../18_Ownership/ownership_and_moves/README.md) — the `E0382` a moved `String` produces, in full
- [`Copy` vs `Clone`](../16_Structs/copy_vs_clone/README.md) — why `&str` copies freely, and one `String` field makes a whole struct move
- [Borrowing](../18_Ownership/borrowing/README.md) — the rule that refuses a view held across a `push_str`
- [How to learn lifetimes](../18_Ownership/how_to_learn_lifetimes/README.md) — why "own `String`, clone when stuck" is legitimate advice
- [Meet the byte](../19_Numbers/meet_the_byte/README.md) — the unit `len` counts in
- [`Path` and `PathBuf`](../04_Files/path_and_pathbuf/README.md) — the family's honorary pair, a **stub** for now

[STRINGS.md](../STRINGS.md) is the full map: the same lessons with the question each one answers, plus the six topics that are still outlines rather than lessons.

[Strings: links, books and videos](resources/README.md) is the reading list — the Book, *Programming Rust* ch. 17, Easy Rust, the essays, and the exercise sets outside this library that map onto these pages.

The bytes underneath are a library of their own. Everything this section takes as given — what UTF-8 *is*, why `ż` costs two bytes and an emoji four, and who checks that a run of bytes really is UTF-8 before `from_utf8` will hand back a `&str` — is the subject of the sibling [encodings learning library ↗](https://masiarek.github.io/encodings-learning-library/), which teaches the same ground in Python, C and the shell alongside Rust. [Validation is a boundary ↗](https://masiarek.github.io/encodings-learning-library/03_Encodings/validation_is_a_boundary/index.html) is the page that pairs with this chapter: it reads `core`'s own validator, and shows what the `&str` invariant buys that a C `char *` and a Python `str` do not — the check runs once, so `chars()` afterwards does not run it again.

## Po polsku

Tekst w Ruscie to dwa typy i jeden podział: `String` jest właścicielem swoich bajtów (łańcuch znaków, dane na stercie, można je rozbudowywać), a `&str` to tylko podgląd na cudzy tekst — wycinek łańcucha (*string slice*), czyli para „wskaźnik + długość”. Stąd praktyczna reguła całego działu: parametr funkcji bierze `&str`, bo chce tylko czytać, a pole struktury trzyma `String`, bo musi przeżyć wywołanie, które je stworzyło. Literał w kodzie nie leży ani na stosie, ani na stercie — siedzi w samym pliku wykonywalnym, dlatego ma typ `&'static str`.

Reszta niespodzianek bierze się z tego, że te bajty są w UTF-8 — i tu polski czytelnik ma trudniej niż angielski, bo trafia na nie od pierwszego dnia, a nie w egzotycznym przykładzie. Każde `ą ć ę ł ń ó ś ź ż` zajmuje **dwa** bajty, więc `"żółw".len()` daje 7, a nie 4. `s[0]` w ogóle się nie kompiluje (nie ma indeksowania po znakach), a `&s[0..1]` na słowie „żółw” kompiluje się znakomicie i **panikuje w czasie działania**: *end byte index 1 is not a char boundary; it is inside 'ż' (bytes 0..2 of string)*. To jedyne miejsce w tym dziale, gdzie kompilator nie pilnuje za ciebie — dlatego do liczenia „liter” służy `.chars().count()`, a do cięcia w bezpiecznych miejscach `char_indices()`. Uwaga na skróty myślowe: `char` to jeden skalar Unicode, a nie „jedna litera na ekranie”, więc emoji ze znacznikiem koloru skóry to nadal kilka `char`ów.

Sam dział jest zbudowany dwuwarstwowo i warto to wiedzieć, zanim zaczniesz szukać: **lekcje** (tabela na górze) tłumaczą pojęcia, a **dokumentacja metod** — 83 strony dla `str` i 42 dla `String` — odpowiada na pytanie „co ta metoda robi na brzegach, na czym panikuje, co alokuje”. Podział przebiega dokładnie po typach: `String` dereferencjonuje się do `str`, więc każda metoda `str` działa też na `String` i dlatego strona pożyczona jest dwa razy grubsza. Materiałów po polsku o łańcuchach jest niewiele — polskie tłumaczenie Tour of Rust kończy się na rozdziale 5, a tekst to rozdział 6 — więc do wyszukiwarki i tak wpisuje się angielskie hasła.

**Szukaj po polsku:** łańcuchy znaków w Ruscie · wycinek łańcucha · kodowanie UTF-8 a polskie znaki · `rust String vs &str` · `rust byte index is not a char boundary`
