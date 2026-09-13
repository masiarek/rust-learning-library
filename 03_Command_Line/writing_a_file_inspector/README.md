# Writing a file inspector: the questions before the first line

**Level:** 201 → 301 · a project brief, not a lesson

**One line:** `xxd`, `hexdump`, `od`, `strings` and `file` are one program with five opinions — take some bytes from a file or from standard input, decide what they are, print a view — and every decision the finished tool makes is a question you can ask before writing it; this page is those questions, the page in this library or its [encodings sibling ↗](https://masiarek.github.io/encodings-learning-library/) that answers each, and the concepts nobody here has written yet.

There is no program on this page, on purpose. [Reading a line from standard input](../reading_stdin/README.md) is where the code starts; this is where the *design* starts, and the two habits it is trying to hand you are older than Rust — ask what the input is before asking what to do with it, and ask what you can measure before asking what to print.

---

## 1. Where do the bytes come from?

The tools you are imitating all accept a filename *and* a pipe, and behave differently on each in ways their man pages do not lead with.

- **Which spellings do you accept?** `tool FILE`, `tool < FILE`, `cat FILE | tool`, `tool -`, and `tool` alone. The first hands you a path; the other four hand you file descriptor 0 and nothing else. [A file or stdin](../a_file_or_stdin/README.md) is the page for making one function serve both.
- **What do you lose when it is stdin?** A name, a size, a position you can move, and everything `stat` knows. `file -` prints `/dev/stdin: ASCII text` because it has no other name to print; `xxd -s 100` on a pipe has to read and throw away 100 bytes, because a pipe cannot be rewound. Decide what the tool prints in the name column when there is no name, and whether `--skip` on a pipe is an error or a slow read.
- **Is anyone typing?** `tool` alone at an interactive prompt: `xxd` waits silently for Ctrl-D, which reads as a hang to anyone who did not mean it. [`IsTerminal` ↗](https://doc.rust-lang.org/std/io/trait.IsTerminal.html) is the one question that tells the two apart, and [A pipe is not a terminal ↗](https://masiarek.github.io/encodings-learning-library/06_Terminal/pipe_is_not_a_terminal/index.html) measures what else changes when the answer is no. Print a usage line, a `reading from stdin — Ctrl-D ends it` note on stderr, or nothing: choose, and say so in `--help`.
- **The same program, two behaviours by input shape** is not hypothetical. Apple's `strings` reads a named file's sections and reads *all* of stdin, so `strings a.out` and `strings - < a.out` give different answers — [measured in the encodings library ↗](https://masiarek.github.io/encodings-learning-library/11_Tools/strings/index.html). Whatever your tool does, do it the same way for both.
- **Several files?** `hexdump a b` is one stream with cumulative offsets and no boundary marker; `xxd` takes one file. `cat a - b` puts stdin *between* two files, and it works. Decide whether offsets restart, and whether a filename header is printed when there is more than one.

## 2. Bytes, or text?

The 101 page reads lines into a `String`, and that is exactly the wrong unit for this tool.

- **A `String` promises UTF-8**, so `read_line` refuses a PNG at its first byte that does not decode — [the one failure you will actually meet](../reading_stdin/README.md#the-one-failure-you-will-actually-meet), and [A file is bytes; a `String` is a promise](../../04_Files/a_file_is_bytes/README.md). A dump reads `[u8]` through `Read`, never `str`: [Reading bytes](../reading_bytes/README.md).
- **A line is not a unit either.** A binary has `0a` bytes that are not line ends, and a 4 GiB file with no `0a` at all is one "line". Sixteen bytes is the row `xxd` and `hexdump -C` chose, and it is a *display* decision, not a property of the file.
- **`read` may hand you fewer bytes than you asked for**, on a pipe especially, and not only at the end. A row builder that assumes sixteen arrive at once prints wrong rows on a slow pipe and right ones on a file; `read_exact` fills the buffer or fails with `UnexpectedEof` on a ten-byte file, which is the wrong error for the last row. The loop that accumulates a row across short reads is the heart of the tool.
- **What does the last row look like?** `hexdump` zero-pads the block and prints spaces for units past the end, which is [where every dump's trailing spaces come from ↗](https://masiarek.github.io/encodings-learning-library/11_Tools/hexdump/index.html#the-man-page-and-the-rules-the-presets-hide); `xxd` pads with spaces to keep the text column aligned. A file of 15, 16 and 17 bytes are three different last rows, and all three go in the tests.
- **Offsets.** `usize` is the length of a slice in memory; the offset into a 5 GiB stream is a `u64`, and on a 32-bit target those differ. Eight hex digits (`xxd`, `hexdump -C`), seven (bare `hexdump`), or octal (`od`'s default) — [`od` reads types, not bytes ↗](https://masiarek.github.io/encodings-learning-library/11_Tools/od/index.html) is the page on why a dump's offset base is a decision that surprises people.
- **If you group bytes into words, which end comes first?** Bare `hexdump` prints `6163` for a file that starts `61 62`, because it read a 16-bit number in CPU order — [the swap, at length ↗](https://masiarek.github.io/encodings-learning-library/11_Tools/hexdump/index.html). In Rust the question is `u16::from_le_bytes` against `from_be_bytes`, and [Meet the byte](../../19_Numbers/meet_the_byte/README.md) is where those live. A dump that never groups never has to answer.
- **The text gutter is ASCII and only ASCII.** `0x20..=0x7e` prints itself and everything else is a dot: `u8::is_ascii_graphic() || b == b' '`, decided per byte, and never a UTF-8 decode — the column [cannot mislead you because it never had an opinion ↗](https://masiarek.github.io/encodings-learning-library/11_Tools/hexdump/index.html#what-it-decides-about-your-text). `od -a` [invents names for bytes it cannot draw ↗](https://masiarek.github.io/encodings-learning-library/06_Terminal/inspecting_a_file/index.html) and is the cautionary tale.

## 3. Binary, or text — and says who?

[Binary is a verdict, not a property ↗](https://masiarek.github.io/encodings-learning-library/06_Terminal/binary_or_text/index.html): nothing in a file says *binary*, so every tool that says it ran a test, and they run different ones.

- **Which test does your inspector run, and does it say?** A `00` byte in the first block is what `git`, `diff` and `grep` look for — and [`diff` looks at the whole file on macOS and the first block on Linux ↗](https://masiarek.github.io/encodings-learning-library/11_Tools/diff_and_cmp/index.html), so the same file gets two verdicts. Valid UTF-8 from start to end is a stronger test and a slower one. A BOM is three bytes of evidence. `file` reads the first 64 KiB and [reports what an encoding looks like, not what it is ↗](https://masiarek.github.io/encodings-learning-library/06_Terminal/file_guesses/index.html).
- **If it is text, what is worth counting?** Lines — where `wc -l` and `grep -c ''` [disagree by one on a file with no trailing newline ↗](https://masiarek.github.io/encodings-learning-library/06_Terminal/trailing_newline/index.html). Line endings — `\n`, `\r\n`, a lone `\r`, or a mix, which [RFC 1212](../../14_Strings/rfc_1212_line_endings/README.md) is about. The longest line, the control characters present by name (`ht`, `lf`, `nul`, `del` — `hexdump`'s `%_u` names them), and whether any byte above `7f` decodes.
- **If it decodes as UTF-8, is that an answer?** [`us-ascii` is a statement no experiment could contradict, and `iso-8859-1` is a proof of a negative ↗](https://masiarek.github.io/encodings-learning-library/06_Terminal/file_guesses/index.html) — a Latin-2 file of Polish text is valid Latin-1, valid Windows-1250 and invalid UTF-8 all at once. Report the test's result, not a name for the encoding, unless the name is what the test proved. In Rust the test is [`from_utf8` and its lossy twin ↗](https://masiarek.github.io/encodings-learning-library/05_Rust/from_utf8_and_lossy/index.html).
- **The first two bytes decide whether it runs at all**: a BOM in front of `#!` [makes the kernel refuse the file ↗](https://masiarek.github.io/encodings-learning-library/06_Terminal/the_first_two_bytes/index.html), so an inspector that shows the first row in hex shows the bug that `cat` hides.

## 4. What kind of file is it?

A path names one of seven kinds of thing, and `file` answers a different question from all of them.

- **The seven kinds, and the first character of `ls -l`:** `-` regular, `d` directory, `l` symbolic link, `p` FIFO, `s` socket, `c` character device, `b` block device. In Rust, `fs::metadata(path)?.file_type()` answers three of them (`is_file`, `is_dir`, `is_symlink`) and [`FileTypeExt` ↗](https://doc.rust-lang.org/std/os/unix/fs/trait.FileTypeExt.html) the other four on Unix — and `metadata` follows a symlink where `symlink_metadata` does not, so on a Mac `/tmp` is a directory to the first and a link to the second. No page in this library covers this yet: [Seven kinds of file](../../04_Files/seven_kinds_of_file/README.md) is the stub for it.
- **What does your tool do with each?** `File::open` on a directory *succeeds* on Unix, and the first `read` fails with `IsADirectory` — so the check has to be on the read, not on the open. A FIFO blocks until somebody writes. `/dev/zero` never ends, so `tool /dev/zero` without a `--length` is `yes` with extra steps. A socket cannot be opened this way at all. A missing file and an empty file are [different answers, and only one is an error](../../04_Files/missing_is_not_empty/README.md).
- **`file`'s answer is one of four**, and [File type is four questions ↗](https://masiarek.github.io/encodings-learning-library/06_Terminal/file_type_is_four_questions/index.html) separates them: *how is it stored* (the inode, above), *how do I run it* (mode bits and the first two bytes), *what is in it* (magic numbers — what `file` reads), *what did the user mean* (the extension). Decide which your inspector answers, and print the question beside the answer.
- **On a Mac, two more questions**, neither of which the seven kinds or `file` sees: does the path carry extended attributes (`xattr -l`, where `com.apple.quarantine` is the one that stops a download from running), and what type does Launch Services think it is (`mdls -name kMDItemContentType`)? An `.app` is a directory to every Unix tool and a program to the Finder.
- **Magic numbers are a table, and the table is data.** `89 50 4e 47` at offset 0 is PNG; `ustar` at offset 257 is tar; `CD001` at offset 32769 is an ISO image — so "read the first sixteen bytes" is not enough for the third, and the order of the rules matters when a ZIP is also a DOCX, a JAR and an APK. [`libmagic` ↗](https://www.darwinsys.com/file/) is that table with thirty years of rules; [wiza ↗](https://github.com/qjerome/magic-rs) and [libmagic-rs ↗](https://github.com/EvilBit-Labs/libmagic-rs) are two Rust readings of it, on [the shelf](../../11_Unix/byte_tools/README.md). Decide how many signatures your table holds, and whether a match is a *verdict* or a *guess* in the output.

## 5. The first bytes, the first lines

Every inspector needs a bounded read, and the bound is a design decision.

- **In the shell, four spellings of the same sixteen bytes:** `head -c 16 f | xxd`, `xxd -l 16 f`, `hexdump -n 16 -C f`, `od -N 16 -An -tx1 f` — [measured side by side on Feeding stdin](../../11_Unix/feeding_stdin/README.md#the-first-few-bytes-and-the-first-few-lines), where `od` is the one that pads. Lines: `head -3` and `sed -n '1,3p'`.
- **In Rust:** [`Read::take(n)` ↗](https://doc.rust-lang.org/std/io/trait.Read.html#method.take) bounds any reader, `read_exact` into a `[u8; 16]` fills it or fails, and `lines().take(3)` stops the line iterator — [Reading lines efficiently](../../04_Files/reading_lines_efficiently/README.md) is the page on what each costs.
- **A pipe cannot be rewound**, so if the type-sniffing step consumed the first sixteen bytes, the dump still has to show them. Keep the prefix in a `Vec<u8>` and [`Read::chain` ↗](https://doc.rust-lang.org/std/io/trait.Read.html#method.chain) it back in front of the rest; skipping is `io::copy` from `take(n)` into `io::sink()`. On a real file `Seek` does both for free, and no page here covers `Seek` yet.
- **How many bytes are enough?** `file` reads 64 KiB and [a file that is UTF-8 from byte 65536 onward reports as ASCII ↗](https://masiarek.github.io/encodings-learning-library/06_Terminal/file_guesses/index.html). Any bound you pick has a file that fools it; the honest output says what the bound was.

## 6. How do you test the content?

- **The function takes `impl Read` and `impl Write`.** Then a test hands it `&[u8]` and a `Vec<u8>` — no temporary file, no process, no keyboard — which is the whole of [`Read` and `Write`](../../12_Traits/read_and_write/README.md) and the move the [101 page's example](../reading_stdin/README.md#how-this-page-is-checked) already makes. [Temporary directories in tests](../../04_Files/temp_dirs_in_tests/README.md) is for the day a test needs a real path after all.
- **Which oracle?** The obvious test compares your dump against `xxd`'s. The encodings library measured what that oracle cannot promise: [`od` pads on macOS and not on Linux ↗](https://masiarek.github.io/encodings-learning-library/11_Tools/od/index.html), [`xxd -e`'s padding changed between two xxd versions ↗](https://masiarek.github.io/encodings-learning-library/11_Tools/xxd/index.html), `hexdump -c` changes notation with the locale, and `file`'s English wording moved between releases while `--mime-type` did not. `hexdump -C` is the one view byte-identical across platforms and locales, so if a dump tool is your oracle, that is the flag.
- **The inputs to keep forever:** empty; one byte; 15, 16 and 17 bytes; 64 zero bytes (does your tool squeeze repeated rows into a `*`, and is that *off* by default, since [it destroys data that will be parsed ↗](https://masiarek.github.io/encodings-learning-library/11_Tools/hexdump/index.html)?); a `00` in the first row and one 200 KiB in; invalid UTF-8; `\r\n`; no trailing newline; a directory; a FIFO; a missing file; a name that is not UTF-8. Each is one assertion, and [what a test asserts](../../28_Testing/what_a_test_asserts/README.md) is worth reading before writing the first.
- **The binary is a separate test.** Unit tests prove the function; only running the program proves that `tool -` and `tool FILE` reach it, that the exit status is non-zero on a missing file, and which stream the complaint went to — [Testing a command](../testing_a_command/README.md).

## 7. The other end of the pipe

- **stdout is the answer, stderr is everything else**, and the exit status is the only part a script reads — [Standard error, and exit status](../../02_Errors/stderr_and_exit_status/README.md). A dump that prints `warning:` on stdout has just become data.
- **Thousands of rows through `println!`** lock stdout per call; a `BufWriter` around `stdout().lock()` is the fix, and it needs a `flush` before `process::exit`.
- **`tool big.bin | head`** closes the pipe after ten rows, and Rust's runtime ignores `SIGPIPE`, so the eleventh `println!` *panics* with `failed printing to stdout: Broken pipe` — measured for this page on 1.98.0, and the reason a command-line tool writes with `writeln!` and matches on `ErrorKind::BrokenPipe`. No page covers it yet: [Broken pipe](../../02_Errors/broken_pipe/README.md) is the stub.
- **Colour only when stdout is a terminal**, which is `IsTerminal` again, on fd 1 this time; `hexyl` does it and so does `ls`. `--color=never` is the flag for the reader who pipes.

## Concepts still missing

What the questions above need, and where each stands. A stub has a URL and an outline and no example yet; *none* means the page has not been started.

| Concept | Status | Page |
|---|---|---|
| Reading stdin line by line | **written** | [Reading a line from standard input](../reading_stdin/README.md) |
| One function for a file or stdin; `-`; `IsTerminal` | stub | [A file or stdin](../a_file_or_stdin/README.md) |
| `Read`, short reads, `take`, `chain`, the sixteen-byte row | stub | [Reading bytes](../reading_bytes/README.md) |
| The seven kinds of file, `metadata`, `FileTypeExt`, `symlink_metadata` | stub | [Seven kinds of file](../../04_Files/seven_kinds_of_file/README.md) |
| `SIGPIPE`, `BrokenPipe`, and `println!` in a pipeline | stub | [Broken pipe](../../02_Errors/broken_pipe/README.md) |
| A keypress without Enter; a prompt that does not echo | stub | [Raw mode and passwords](../raw_mode_and_passwords/README.md) |
| A read you can give up on | stub | [Reading without blocking](../reading_without_blocking/README.md) |
| `Seek`, and skipping on a pipe | none | belongs beside [Reading bytes](../reading_bytes/README.md) |
| A magic-number table, and the order of its rules | none | the tool question is answered on [`file` guesses ↗](https://masiarek.github.io/encodings-learning-library/06_Terminal/file_guesses/index.html) |
| Byte order, `from_le_bytes` | partly | [Meet the byte](../../19_Numbers/meet_the_byte/README.md); [Byte order and the BOM ↗](https://masiarek.github.io/encodings-learning-library/03_Encodings/byte_order_and_bom/index.html) |
| The binary-or-text verdict, in Rust | written, next door | [Binary is a verdict ↗](https://masiarek.github.io/encodings-learning-library/06_Terminal/binary_or_text/index.html), *In Rust* section |
| Arguments, flags, `clap` | stubs | [Command-line arguments](../command_line_arguments/README.md), [Flags by hand](../flags_by_hand/README.md), [Deriving a parser with `clap`](../clap_derive/README.md) |
| Running the binary in a test | stub | [Testing a command](../testing_a_command/README.md) |
| The shell side: making, feeding and looking at the bytes | **written** | [Feeding stdin](../../11_Unix/feeding_stdin/README.md) |
| The tools this one imitates, and what to install | **written** | [Byte tools](../../11_Unix/byte_tools/README.md) |

## Terms

Seven words the tools on [the shelf](../../11_Unix/byte_tools/README.md) use as if everyone knew them. Each is also in the [glossary](../../GLOSSARY.md).

- **Magic number** — bytes at a known offset that identify a format: `89 50 4e 47` is PNG. What `file` reads and the extension does not.
- **Polyglot file** — one file that is valid in two formats at once (a ZIP that is also a JAR, a GIF that is also JavaScript). The reason a signature table's *order* is part of its answer.
- **Hex viewer / hex editor** — a viewer prints the bytes (`xxd`, `hexyl`); an editor writes them back (`xxd -r`, ImHex, 010 Editor). The column you are most tempted to edit in a dump is [the one `xxd -r` throws away ↗](https://masiarek.github.io/encodings-learning-library/11_Tools/xxd/index.html).
- **Disassembler / decompiler** — a disassembler turns machine code into assembly, one instruction per line (`radare2`, `objdump`); a decompiler guesses the C that produced it (Ghidra). Both start from the bytes a dump shows.
- **Firmware image** — a file that is a whole device's flash: a boot loader, a kernel, a filesystem and padding, end to end. `binwalk` finds the boundaries by scanning for magic numbers.
- **Homoglyph** — two characters that look alike and are not (`а` U+0430 against `a` U+0061). Invisible in the text column, obvious in the hex.
- **BiDi override** — a Unicode control (U+202E and its family) that reverses the displayed order of what follows, so source code can *read* differently from how it *runs*. `straudit` and `uni` find them; a dump shows them as `e2 80 ae`.

## If you are coming from another language

**Python.** The tool is `sys.stdin.buffer` (bytes, not `sys.stdin`, which decodes) or `open(path, 'rb')`, `os.stat` with the `stat.S_ISFIFO` family for the seven kinds, and `bytes.hex(' ')` for the middle column — [the ten-line `dump()` in the encodings library ↗](https://masiarek.github.io/encodings-learning-library/01_Bits_and_Bytes/reading_a_hex_dump/index.html) is the whole of `xxd` minus the flags. The question that does not transfer is the short read: `f.read(16)` on a regular file returns sixteen bytes or the end, and Python programmers rarely meet the pipe that returns three. And `print()` under `| head` raises `BrokenPipeError`, which is the same event with a Python name.

**ABAP.** `OPEN DATASET ... IN BINARY MODE` and `READ DATASET ... INTO xstr` are the bytes; `xstring` is the `Vec<u8>`, and the debugger's hex view is the dump. What is absent is the whole of section 1 — there is no standard input, no pipe, and no `-` — and section 4's seven kinds, since an application server file is regular or it is not there. The nearest thing to a magic number is `cl_abap_zip` refusing a file that does not start `50 4b`, and it is the same idea: the format is decided by the bytes, not by the name.

## See also

- [Reading a line from standard input](../reading_stdin/README.md) — the first line of the program this page declines to write
- [Feeding stdin](../../11_Unix/feeding_stdin/README.md) — `printf`, pipes, redirects and `xxd -r`: how the bytes get to fd 0, and how to look at them before your program does
- [Byte tools](../../11_Unix/byte_tools/README.md) — `xxd`, `hexdump`, `od`, `strings`, `file`, and the tools that replace them, with what each is written in
- [Inspecting a file ↗](https://masiarek.github.io/encodings-learning-library/06_Terminal/inspecting_a_file/index.html) — the same tools inside a session, and the one column that is the file
- [`hexdump` is a format engine ↗](https://masiarek.github.io/encodings-learning-library/11_Tools/hexdump/index.html) · [`xxd` is the dump you can put back ↗](https://masiarek.github.io/encodings-learning-library/11_Tools/xxd/index.html) · [`od` reads types, not bytes ↗](https://masiarek.github.io/encodings-learning-library/11_Tools/od/index.html) · [`strings` has a printable set, not an encoding ↗](https://masiarek.github.io/encodings-learning-library/11_Tools/strings/index.html) · [`file` guesses ↗](https://masiarek.github.io/encodings-learning-library/06_Terminal/file_guesses/index.html)
- [`std::io::Read` ↗](https://doc.rust-lang.org/std/io/trait.Read.html) · [`std::fs::FileType` ↗](https://doc.rust-lang.org/std/fs/struct.FileType.html) · [`std::io::ErrorKind` ↗](https://doc.rust-lang.org/std/io/enum.ErrorKind.html)

## Po polsku

Ta strona celowo nie zawiera programu. `xxd`, `hexdump`, `od`, `strings` i `file` to jeden program z pięcioma opiniami — weź bajty z pliku albo ze standardowego wejścia, zdecyduj, czym są, wypisz widok — a każda decyzja gotowego narzędzia jest pytaniem, które można zadać przed napisaniem pierwszej linijki. Siedem grup pytań powyżej idzie w kolejności, w jakiej bajty przechodzą przez program: skąd pochodzą (plik czy potok — i co tracisz, gdy to potok: nazwę, rozmiar, możliwość przewinięcia), czy to bajty, czy tekst (zrzut szesnastkowy czyta `[u8]`, nigdy `String`, bo `String` obiecuje UTF-8, a PNG tej obietnicy nie składa), kto orzeka, że plik jest binarny (to werdykt testu, nie własność pliku — bajt `00`, poprawne UTF-8 i BOM to trzy różne testy z trzema różnymi odpowiedziami), jakiego rodzaju to plik (siedem rodzajów uniksowych, pierwsza litera `ls -l`, i cztery osobne pytania, na które `file`, rozszerzenie, bity trybu i i-węzeł odpowiadają każde po swojemu), ile bajtów wystarczy obejrzeć, jak przetestować zawartość bez klawiatury i bez pliku tymczasowego (funkcja bierze `impl Read`, test podaje `&[u8]`), i co się dzieje na drugim końcu potoku (`tool big.bin | head` zamyka potok, a `println!` w Ruście wtedy **panikuje**, bo środowisko uruchomieniowe ignoruje `SIGPIPE`).

Polski czytelnik ma tu jeden przykład pod ręką, którego angielski nie ma: plik z polskim tekstem zapisany w Latin-2 albo w stronie kodowej 1250 jest jednocześnie poprawnym Latin-1, poprawnym Windows-1250 i niepoprawnym UTF-8 — więc narzędzie, które wypisuje *nazwę kodowania*, zgaduje, a to, które wypisuje *wynik testu*, mówi prawdę. Tabela „Concepts still missing” wylicza, które z potrzebnych stron w tej bibliotece już są, które są tylko szkicami (*stubs*) z gotowym adresem, a których nie ma wcale — `Seek` i tablica liczb magicznych to dwa największe braki.

**Szukaj po polsku:** zrzut szesnastkowy pliku · liczba magiczna pliku · plik binarny a tekstowy · rodzaje plików w Uniksie · `rust impl Read hex dump` · `rust BrokenPipe println panic` · `rust FileType is_fifo`
