# Raw strings, escapes and byte strings

**Level:** 101 → 201 · working knowledge

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** Four prefixes decide what the compiler does with the characters between your quotes — none, `r`, `b`, and `br` — and the one you reach for most is the one that turns a Windows path or a regex back into what you actually typed.

---

## What this page has to answer

- The escape set: `\n`, `\t`, `\\`, `\"`, `\0`, `\u{1F600}` and `\x41` — and why `\x` stops at 0x7F while `\u{}` does not.
- `r"…"` and the `#` counting trick: `r#"he said "hi""#`, and what to do when the text itself contains `"#`.
- `b"…"` is a `&[u8; N]`, not a string — no UTF-8 promise, which is the point, and what that means for the type you get back.
- The line continuation `\` at end of line, and the leading-whitespace problem it does *not* solve (no `dedent` in `std`).
- Where raw strings are genuinely the right call: regexes, Windows paths, embedded JSON — and where they merely hide a `\n` you needed.

## See also

- [Meet the `char`](../meet_the_char/README.md)
- [`&'static str`](../static_str/README.md)
- [Six kinds of string](../six_kinds_of_string/README.md)
- [STRINGS.md](../../STRINGS.md) — the map this page is a gap in
- [Strings: links, books and videos](../resources/README.md) — where to read about it in the meantime

## Po polsku

Z tej listy jeden szczegół dotyczy polskiego tekstu bezpośrednio: `\x` kończy się na `\x7F`, czyli dokładnie na ASCII, a każda polska litera diakrytyczna leży wyżej — `"\xF3"` się nie skompiluje, `ó` zapisuje się jako `"\u{F3}"`, a `ł` jako `"\u{142}"`. W łańcuchu bajtowym `b"…"` jest jednocześnie luźniej i ciaśniej: `\xNN` sięga tam do `\xFF`, ale wpisanie `ł` wprost jest błędem (*non-ASCII character in byte string literal*), bo `b"…"` to `&[u8; N]` bez żadnej obietnicy UTF-8 — i właśnie po to istnieje. Surowy łańcuch `r"…"` przydaje się natomiast tam, gdzie ukośnik ma zostać ukośnikiem: `"C:\temp\nowe"` to w rzeczywistości `C:`, tabulator, `emp`, nowa linia i `owe`, podczas gdy `r"C:\temp\nowe"` jest tym, co widać.

**Szukaj po polsku:** surowe łańcuchy znaków · sekwencje ucieczki · polskie znaki diakrytyczne w kodzie · `rust raw string literal` · `rust byte string literal b""`
