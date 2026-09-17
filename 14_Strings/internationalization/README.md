# Internationalization

**Level:** 201 · working knowledge

**One line:** `std` has no locale — `to_uppercase` is the same everywhere, `{}` formats `1234.5` identically in every country, and `<` sorts by code point — so translating messages, plural rules, number formats and collation all come from crates.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- What `std` deliberately does without a locale: `format!`, `to_uppercase`, `cmp` on `str` — each shown producing the same output regardless of the machine's settings
- Messages and plurals: Project Fluent (`fluent`), `gettext-rs`, `rust-i18n` — how each stores translations and how each handles "1 file" versus "2 files" in a language with more plural forms
- Numbers, dates and currencies: ICU4X (the `icu` crate) with CLDR data, and `num-format` for the narrow case
- Collation: why sorting German or Polish words by `<` is wrong, and ICU4X's collator — link [comparing and sorting text](../comparing_strings/README.md)
- Case: the Turkish dotted and dotless `i`, and why `to_lowercase` cannot know which you meant
- Where the locale comes from on each OS, and why a server should take it from the request rather than from `LANG`

## The trap it exists for

Formatting user-facing numbers with `format!("{:.2}")` and concatenating translated fragments (`"You have " + n + " messages"`). The first ignores the decimal separator, and the second hard-codes English word order and plural rules into every language.

## Where this sits

[The string crates](../string_crates/README.md) lists the crates by job. This page is the i18n job specifically. [Comparing and sorting text](../comparing_strings/README.md) is where byte order stops being enough.

## See also

- [The string crates](../../14_Strings/string_crates/README.md) — `icu` and its neighbours, by job
- [Comparing and sorting text](../../14_Strings/comparing_strings/README.md) — why `<` is not collation
- [The format mini-language](../../14_Strings/the_format_language/README.md) — what `{:.2}` does, the same everywhere
- [Four lengths](../../14_Strings/four_lengths/README.md) — the length a translated string will have on screen
- [STRINGS.md](../../STRINGS.md) — every strings lesson in reading order

## If you are coming from another language

- **Java.** `Locale` is built in and consulted implicitly — the Java text library's [format follows the locale ↗](https://masiarek.github.io/java-text-learning-library/03_Locale/format_follows_the_locale/) and [case is locale-sensitive ↗](https://masiarek.github.io/java-text-learning-library/03_Locale/case_is_locale_sensitive/) show the bugs that implicit default causes, which Rust avoids by having no default at all.
- **C.** `setlocale` changes `printf` and `strcoll` for the whole process; the C library's [ICU4C ↗](https://masiarek.github.io/c-learning-library/03_Strings/unicode_text_with_icu4c/) page is the explicit alternative, and ICU4X is its Rust successor.
- **Python.** `locale` and `gettext` are in the standard library, and `str.lower` is locale-independent like Rust's. The Python library's [lowercasing is not folding ↗](https://masiarek.github.io/python-learning-library/01_Text_and_Bytes/lowercasing_is_not_folding/) page covers the case half.
- **Encodings.** The encodings library's [locale and LC_CTYPE ↗](https://masiarek.github.io/encodings-learning-library/06_Terminal/locale_and_lc_ctype/) and [sorting and collation ↗](https://masiarek.github.io/encodings-learning-library/07_Real_Data/sorting_and_collation/) are the terminal and data halves of the same question.

## Po polsku

Biblioteka standardowa Rusta nie ma pojęcia ustawień regionalnych (*locale*): `format!` wypisze `1234.5` tak samo w Polsce i w USA, a `<` posortuje „ćma” za „dom”, bo porównuje kody znaków, a nie polski alfabet. Tłumaczenia komunikatów, formy liczby mnogiej, formaty liczb i sortowanie językowe (*collation*) dają dopiero skrzynie — `fluent`, `icu` (ICU4X).

**Szukaj po polsku:** internacjonalizacja · lokalizacja aplikacji · sortowanie polskich znaków · `rust i18n fluent` · `icu4x collator rust`
