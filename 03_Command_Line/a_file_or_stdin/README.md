# A file or stdin

**Level:** 201 · working knowledge

**One line:** `tool FILE` and `cat FILE | tool` should reach the same function, which they do the moment it takes `impl Read` and `main` decides once — `File::open(path)?` or `io::stdin().lock()`, boxed as one `Box<dyn Read>` — and the three things the pipe cannot give you are a name, a size and a way back.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The convention every Unix tool follows: no argument means stdin, and a lone `-` means stdin *even in a list of files*, so `cat a - b` works
- `Box<dyn Read>` as the one type both sources become, and why a `match` returning two different types is the first compile error on this page
- `Read` against `BufRead`: `File` is the first and needs a `BufReader`, `StdinLock` is already the second, and the parameter type decides which the caller must supply
- What is missing on stdin — `metadata`, a length, `Seek` — and what the tool prints in the name column when there is no name (`file -` says `/dev/stdin`)
- [`IsTerminal` ↗](https://doc.rust-lang.org/std/io/trait.IsTerminal.html) on fd 0: `tool` typed alone at a prompt, and the choice between waiting silently, printing usage, and saying `reading from stdin, Ctrl-D to end` on stderr
- The same program behaving differently by input shape — Apple's `strings` reads a named file's sections and all of stdin ([measured ↗](https://masiarek.github.io/encodings-learning-library/11_Tools/strings/index.html)) — and making sure yours does not
- Filenames that are not UTF-8, which is why the argument is an `OsString` before it is a path — [Command-line arguments](../command_line_arguments/README.md)

## The trap it exists for

A tool tested only with `tool FILE` reads the whole file into memory with `fs::read`, and works. The first user who runs `cat huge.log | tool` waits for the pipe to close before seeing any output — or never sees it, because the writer on the other end is waiting for the tool to say something first. A function that takes `impl Read` and streams cannot make that mistake; one that takes a `&Path` cannot avoid it.

## See also

- [Reading a line from standard input](../reading_stdin/README.md) — the 101 page this one generalises
- [Reading bytes](../reading_bytes/README.md) — what to do with the reader once you have it, when the content is not text
- [Feeding stdin](../../11_Unix/feeding_stdin/README.md) — the five ways bytes reach fd 0 from the shell, and `test -t 0`
- [Writing a file inspector](../writing_a_file_inspector/README.md) — the questions this page's answers serve
- [`Read` and `Write`](../../12_Traits/read_and_write/README.md) — the trait that makes the two sources one type
- [Opening a file](../../04_Files/opening_a_file/README.md) — the `File::open` half

## Po polsku

`tool PLIK` i `cat PLIK | tool` powinny trafiać do tej samej funkcji — i trafiają, gdy tylko przyjmuje ona `impl Read`, a `main` decyduje raz: `File::open(ścieżka)?` albo `io::stdin().lock()`, opakowane w jeden typ `Box<dyn Read>`. Konwencja jest uniksowa i stara: brak argumentu znaczy „czytaj standardowe wejście”, a samotny myślnik `-` znaczy to samo nawet w środku listy plików. Potok (*pipe*) nie da ci trzech rzeczy, które plik daje za darmo — nazwy, rozmiaru i możliwości cofnięcia się — więc narzędzie musi zdecydować, co wypisze w kolumnie z nazwą (`file -` mówi `/dev/stdin`) i czy `--skip` na potoku jest błędem, czy powolnym czytaniem. Pułapka, dla której ta strona istnieje: narzędzie testowane tylko na `tool PLIK` wczytuje cały plik przez `fs::read` i działa, a pierwszy użytkownik, który uruchomi `cat ogromny.log | tool`, czeka na zamknięcie potoku, zanim zobaczy cokolwiek — albo nie doczeka się wcale.

**Szukaj po polsku:** czytanie z pliku albo ze standardowego wejścia · myślnik jako nazwa pliku · `rust Box<dyn Read> stdin or file` · `rust IsTerminal stdin`
