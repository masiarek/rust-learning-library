# Command line

The front door. Everything in [Foundations](../01_Foundations/README.md) happens inside a program that somebody has already started; this section is about the handful of strings they typed to start it, and about proving the program does the right thing with them.

**Most of these pages are stubs** — outlines waiting for a runnable example; see the [Errors](../02_Errors/README.md) section for what that means and how a page graduates. [Reading a line from standard input](reading_stdin/README.md) is finished, and [Writing a file inspector](writing_a_file_inspector/README.md) is a project brief rather than a lesson: the questions to ask before building the tool the input pages lead to.

| Lesson | Level | What it will teach |
|---|---|---|
| [Command-line arguments](command_line_arguments/README.md) | 101 | `env::args()` — an iterator whose first item is the program's own name, and the filename that is not valid UTF-8 |
| [Flags by hand](flags_by_hand/README.md) | 201 | What a flag actually is, ten lines that parse one, and the eleventh line where you start rewriting `clap` |
| [Deriving a parser with `clap`](clap_derive/README.md) | 201 | A struct becomes the whole interface: parsing, `--help`, `--version`, and the error message for a bad flag |
| [Testing a command](testing_a_command/README.md) | 201 → 301 | Unit tests prove a function; only running the binary proves the program — status, streams, and assertions that are not brittle |
| [The `Default` trait](the_default_trait/README.md) | 101 → 201 | The value a type takes when nobody said — and `..Default::default()`, which is how an options struct grows a field |
| [Arguments and the environment](arguments_and_environment/README.md) | 201 | Which inputs belong on the command line, which belong in the environment, and why the 2024 edition made `set_var` `unsafe` |

The third input a program has, after its arguments and its environment, is standard input — and it is the one the Rust Book reaches for on page two. These pages are the arc from the first `read_line` to a tool that reads anything:

| Lesson | Level | What it teaches |
|---|---|---|
| [Reading a line from standard input](reading_stdin/README.md) | 101 | `read_line` appends, keeps the newline and returns a count — `clear()` before, `trim()` after, and `Ok(0)` is the end; plus the `flush` a prompt needs and the one failure you will actually meet |
| [A file or stdin](a_file_or_stdin/README.md) | 201 | `tool FILE` and `cat FILE \| tool` from one function that takes `impl Read`, the `-` convention, and the three things a pipe cannot give you |
| [Reading bytes](reading_bytes/README.md) | 201 | `Read` instead of `read_line`, the short read a pipe hands you mid-stream, `take` and `chain`, and the sixteen-byte row a dump accumulates |
| [Raw mode and passwords](raw_mode_and_passwords/README.md) | 301 | A keypress without Enter and a prompt that does not echo: what the terminal driver was doing for you, and why `std` cannot switch it off |
| [Reading without blocking](reading_without_blocking/README.md) | 301 | A read you can give up on — a thread, a channel and `recv_timeout`, which is what `tokio::io::stdin` is underneath |
| [Writing a file inspector](writing_a_file_inspector/README.md) | 201 → 301 | Not a lesson: the questions to ask before writing an `xxd`, a `strings` or a `file` of your own, with the page that answers each and the concepts nobody has written yet |

The shell side of the same story — `printf`, pipes, redirects, and how to see the bytes before your program does — is [Feeding stdin](../11_Unix/feeding_stdin/README.md), and the tools an inspector imitates are on [the byte-tools shelf](../11_Unix/byte_tools/README.md).

## Po polsku

Ta sekcja opisuje wiersz poleceń (*command line*) — te kilka łańcuchów znaków, które ktoś wpisał, żeby uruchomić program, oraz sposób na udowodnienie, że program robi z nimi to, co trzeba. Polskie nazewnictwo jest tu rozdwojone: Microsoft tłumaczy *command line* jako „wiersz polecenia”, a *switch* jako „przełącznik”, podczas gdy potocznie mówi się „linia poleceń” i „flaga” — i żadna z tych fraz nie prowadzi do odpowiedzi o Ruście, bo zapytanie `rust wiersz poleceń` wysyła prosto do poradników o `cmd.exe`. Polski czytelnik trafia przy tym na jedną pułapkę wcześniej niż angielski: nazwa pliku z ogonkami, na przykład `wyniki_ąćę.txt`, przekazana programowi w konsoli Windows nie musi być poprawnym UTF-8, a `std::env::args()` właśnie w takim przypadku panikuje — dlatego lekcja o argumentach zaczyna się od `args_os()` i typu `OsString`, a nie od wygodnego `String`. Reszta sekcji to podział, który warto mieć w głowie od początku: co jest argumentem, co flagą, co zmienną środowiskową (*environment variable*) — i dlaczego dopiero uruchomienie samego binarium, a nie test jednostkowy funkcji, sprawdza program jako całość. Trzecim wejściem programu jest standardowe wejście (*standard input*), i to od niego zaczyna się druga tabela: od pierwszego `read_line` — który dopisuje do bufora, zostawia znak nowej linii i zwraca liczbę bajtów, a `Ok(0)` znaczy koniec — przez plik albo potok w jednej funkcji i czytanie bajtów zamiast wierszy, aż po pytania, które warto zadać przed napisaniem własnego `xxd` czy `file`.

**Szukaj po polsku:** argumenty wiersza poleceń · zmienne środowiskowe · przełączniki i flagi · `rust std::env::args` · `rust args_os OsString`
