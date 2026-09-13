# Reading bytes

**Level:** 201 · working knowledge

**One line:** A dump, a checksum and a type sniffer read `[u8]` through `Read`, never `str` — and the loop they share has one rule the 101 page never met: `read` may return fewer bytes than the buffer holds, on a pipe especially and not only at the end, so a sixteen-byte row is *accumulated*, never assumed.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- Why `read_line` is the wrong tool: a `String` promises UTF-8 and a PNG does not keep it ([the failure on the 101 page](../reading_stdin/README.md#the-one-failure-you-will-actually-meet)), and a `0a` in a binary is not a line end
- The short read: `read(&mut buf)` returning `Ok(3)` into a 16-byte buffer mid-stream, and the accumulating loop that makes a full row anyway; `read_exact` as the version that fills or fails, and why its `UnexpectedEof` is the wrong error for a last row
- `read_to_end` into a `Vec<u8>` when the whole thing fits, and `bytes()` as the per-byte iterator you should not use for speed
- `Read::take(n)` to bound a reader, `Read::chain` to put a sniffed prefix back in front of the rest, and `io::copy` into `io::sink` as the skip that works on a pipe where `Seek` cannot
- The row itself: offset as `u64`, sixteen `{:02x}`, and a text gutter that is `is_ascii_graphic() || b' '` per byte and never a UTF-8 decode — the column that [cannot mislead because it never had an opinion ↗](https://masiarek.github.io/encodings-learning-library/11_Tools/hexdump/index.html#what-it-decides-about-your-text)
- The last row on files of 15, 16 and 17 bytes, and what `hexdump` and `xxd` each do with it
- `from_utf8_lossy` and `str::from_utf8` on the bytes afterwards, when the question *is* whether they are text — [`from_utf8` and its lossy twin ↗](https://masiarek.github.io/encodings-learning-library/05_Rust/from_utf8_and_lossy/index.html)

## The trap it exists for

A row builder written as `reader.read(&mut row)?` followed by `print_row(&row)` prints correct output for every file on the author's disk and wrong output the first time the input is a pipe from a slow producer: rows of three and five bytes, each padded as if it were the end. The bug is invisible in tests that use `&[u8]`, because a slice never short-reads. The test that finds it is a reader that returns one byte per call.

## See also

- [A file or stdin](../a_file_or_stdin/README.md) — where the `impl Read` came from
- [A file is bytes; a `String` is a promise](../../04_Files/a_file_is_bytes/README.md) — the same distinction, on the file side
- [Meet the byte](../../19_Numbers/meet_the_byte/README.md) — `u8`, hex, and `from_le_bytes` for the day the dump groups bytes into words
- [Writing a file inspector](../writing_a_file_inspector/README.md) — the tool these loops add up to
- [Reading a hex dump ↗](https://masiarek.github.io/encodings-learning-library/01_Bits_and_Bytes/reading_a_hex_dump/index.html) — the three columns, and a ten-line `xxd` in Python to compare against
- [`std::io::Read` ↗](https://doc.rust-lang.org/std/io/trait.Read.html)

## Po polsku

Zrzut szesnastkowy, suma kontrolna i rozpoznawanie typu pliku czytają `[u8]` przez cechę `Read`, nigdy `str` — bo `String` obiecuje UTF-8, a PNG tej obietnicy nie dotrzymuje, i bo bajt `0a` w pliku binarnym nie kończy żadnego wiersza. Wspólna pętla tych narzędzi ma jedną regułę, której strona 101 nie spotkała: `read` **może zwrócić mniej bajtów, niż mieści bufor**, zwłaszcza na potoku i nie tylko na końcu danych, więc szesnastobajtowy wiersz zrzutu trzeba *zbierać*, a nie zakładać. `read_exact` wypełnia bufor albo zwraca `UnexpectedEof` — i to jest zły błąd dla ostatniego, krótszego wiersza. Do tego trzy narzędzia z `std`, które załatwiają to, czego potok nie umie: `take(n)` ogranicza czytnik, `chain` wstawia z powrotem prefiks zużyty na rozpoznanie typu, a `io::copy` do `io::sink` przeskakuje bajty tam, gdzie `Seek` nie zadziała. Pułapka: pętla `read` + `print_row` działa na każdym pliku z dysku autora i psuje się na pierwszym powolnym potoku, a testy na `&[u8]` tego nie wykryją, bo wycinek nigdy nie czyta krótko.

**Szukaj po polsku:** czytanie pliku binarnego w Ruscie · zrzut szesnastkowy własnego autorstwa · `rust Read short read loop` · `rust read_exact UnexpectedEof` · `rust Read::take chain`
