# Files

The filesystem is the first thing outside your program that you are likely to talk to, and the first place Rust's insistence on types starts paying rent: a path is not a string, a missing file is not an empty one, and a handle that goes out of scope closes itself.

**Two of these pages are finished; the other four are stubs** — outlines waiting for a runnable example, written so the arc has a shape and a permanent URL before the prose exists. A page graduates by gaining an `examples/` program and losing its stub notice, which is the same promise as everywhere else here: [no page claims something a program has not printed](../CONTRIBUTING.md). A file example has a second obstacle to clear before it can be checked against a recorded answer key — the path differs per machine, and the directory may hold leftovers from an earlier run — and the two finished pages clear it the same way: write only inside a directory made under `std::env::temp_dir()` and named with the process id, print byte counts, error kinds and contents but never a path, and remove the directory at the end.

| Lesson | Level | What it teaches |
|---|---|---|
| [Opening a file](opening_a_file/README.md) | 201 | `open`, `create` and `OpenOptions` — three doors and one decision the type system will not make for you: `create` empties the file at open, `append` does not create, and the error names the problem but not the file |
| [A file is bytes; a `String` is a promise](a_file_is_bytes/README.md) | 201 | `write_all` takes `&[u8]`, so a `&str` is widened for free; `read_to_string` checks UTF-8 once at the door and refuses with `InvalidData`; `write!` is two traits; and `include_str!` moves the check to compile time |
| [`Path` and `PathBuf`](path_and_pathbuf/README.md) | 201 | The same split as `&str` and `String`, plus the `join` that throws your path away |
| [Reading lines efficiently](reading_lines_efficiently/README.md) | 201 | One allocation for the file, one per line, or none per line — and when each is the right answer |
| [Missing is not empty](missing_is_not_empty/README.md) | 201 | *"The file is not there"* and *"the file is there and empty"* are different answers, and only one is an error |
| [Temporary directories in tests](temp_dirs_in_tests/README.md) | 201 → 301 | A test that writes to a fixed path cannot run twice at once — and the fix deletes itself if you drop the handle |

The string half of this story — [`String` against `&str`](../14_Strings/string_vs_str/README.md), and [the slice that panics mid-character](../14_Strings/string_slices/README.md) — is in [Strings](../14_Strings/README.md), where it belongs. The seam between the two, where a `&str` becomes bytes on the way out and bytes have to earn their way back in, is [A file is bytes](a_file_is_bytes/README.md).

## Po polsku

System plików to zwykle pierwsza rzecz poza programem, z którą program rozmawia, i pierwsze miejsce, w którym upór Rusta przy typach zaczyna na siebie zarabiać: ścieżka (*path*) ma własny typ i nie jest łańcuchem znaków, brak pliku to co innego niż plik pusty, a uchwyt do pliku (*file handle*) zamyka się sam, gdy wychodzi z zasięgu. Ten ostatni punkt zaskakuje najczęściej — nie ma tu ani `close()`, ani bloku `finally`, bo zamknięcie pliku to zwykłe wypuszczenie zasobu, czyli dokładnie to, co polskie materiały o C++ nazywają RAII. Dwie strony tego rozdziału są gotowe, cztery pozostałe to wciąż szkice — a powód jest ten sam dla wszystkich: każdy przykład w tej bibliotece jest w CI porównywany z zapisanym wzorcowym wyjściem, a wszystko, co dotyka systemu plików, z natury różni się między maszynami. Gotowe strony radzą sobie z tym jednakowo: piszą wyłącznie do katalogu utworzonego pod `std::env::temp_dir()` i nazwanego numerem procesu, wypisują liczby bajtów, rodzaje błędów i zawartość, ale nigdy ścieżkę, i na końcu ten katalog usuwają.

**Szukaj po polsku:** obsługa plików w Ruscie · ścieżka a łańcuch znaków · wypuszczenie zasobu RAII · `rust std::fs` · `rust file closed when dropped`
