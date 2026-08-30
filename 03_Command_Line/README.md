# Command line

The front door. Everything in [Foundations](../01_Foundations/README.md) happens inside a program that somebody has already started; this section is about the handful of strings they typed to start it, and about proving the program does the right thing with them.

**These pages are stubs** — outlines waiting for a runnable example. See the [Errors](../02_Errors/README.md) section for what that means and how a page graduates.

| Lesson | Level | What it will teach |
|---|---|---|
| [Command-line arguments](command_line_arguments/README.md) | 101 | `env::args()` — an iterator whose first item is the program's own name, and the filename that is not valid UTF-8 |
| [Flags by hand](flags_by_hand/README.md) | 201 | What a flag actually is, ten lines that parse one, and the eleventh line where you start rewriting `clap` |
| [Deriving a parser with `clap`](clap_derive/README.md) | 201 | A struct becomes the whole interface: parsing, `--help`, `--version`, and the error message for a bad flag |
| [Testing a command](testing_a_command/README.md) | 201 → 301 | Unit tests prove a function; only running the binary proves the program — status, streams, and assertions that are not brittle |
| [The `Default` trait](the_default_trait/README.md) | 101 → 201 | The value a type takes when nobody said — and `..Default::default()`, which is how an options struct grows a field |
| [Arguments and the environment](arguments_and_environment/README.md) | 201 | Which inputs belong on the command line, which belong in the environment, and why the 2024 edition made `set_var` `unsafe` |

## Po polsku

Ta sekcja opisuje wiersz poleceń (*command line*) — te kilka łańcuchów znaków, które ktoś wpisał, żeby uruchomić program, oraz sposób na udowodnienie, że program robi z nimi to, co trzeba. Polskie nazewnictwo jest tu rozdwojone: Microsoft tłumaczy *command line* jako „wiersz polecenia”, a *switch* jako „przełącznik”, podczas gdy potocznie mówi się „linia poleceń” i „flaga” — i żadna z tych fraz nie prowadzi do odpowiedzi o Ruście, bo zapytanie `rust wiersz poleceń` wysyła prosto do poradników o `cmd.exe`. Polski czytelnik trafia przy tym na jedną pułapkę wcześniej niż angielski: nazwa pliku z ogonkami, na przykład `wyniki_ąćę.txt`, przekazana programowi w konsoli Windows nie musi być poprawnym UTF-8, a `std::env::args()` właśnie w takim przypadku panikuje — dlatego lekcja o argumentach zaczyna się od `args_os()` i typu `OsString`, a nie od wygodnego `String`. Reszta sekcji to podział, który warto mieć w głowie od początku: co jest argumentem, co flagą, co zmienną środowiskową (*environment variable*) — i dlaczego dopiero uruchomienie samego binarium, a nie test jednostkowy funkcji, sprawdza program jako całość.

**Szukaj po polsku:** argumenty wiersza poleceń · zmienne środowiskowe · przełączniki i flagi · `rust std::env::args` · `rust args_os OsString`
