# Missing is not empty

**Level:** 201 · working knowledge

**One line:** *"There is no file yet"* and *"the file is there and has nothing in it"* are different facts, and a program that treats the first as an error cannot run for the first time.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- Matching on [`ErrorKind::NotFound` ↗](https://doc.rust-lang.org/std/io/enum.ErrorKind.html) specifically, rather than on "an error happened"
- The first-run case: no file means an empty collection, not a failure — and where that decision belongs, which is not in the function that reads bytes
- The other half: a file the user *named explicitly* and that does not exist **is** an error, because they told you it existed
- An empty file is a valid file: zero bytes parses to zero records, and any code path that treats that as suspicious will be wrong on day one
- Where `unwrap_or_default` earns its place here, and where it hides the distinction instead

## The trap it exists for

Both of these are one-line decisions, and both defaults are wrong half the time. Treating `NotFound` as fatal breaks the first run; treating every read error as "start empty" means a permissions problem or a corrupt disk silently **deletes the user's data** on the next save. The kind matters, and it is the only thing that does.

## See also

- [Opening a file](../opening_a_file/README.md) — where the `ErrorKind` comes from
- [`unwrap_or_default`](../../17_Option_and_Result/unwrap_or_default/README.md) — the fallback that is right here and dangerous next door
- [Six kinds of zero](../../17_Option_and_Result/six_kinds_of_zero/README.md) — the same lesson in another domain: empty has more than one reason

## Po polsku

Po polsku obie sytuacje wpadają w to samo zdanie — „nic tam nie ma” znaczy zarówno „pliku nie ma”, jak i „plik jest, tylko pusty” — a to są dwa różne fakty i tylko jeden z nich jest błędem. Dlatego w obsłudze błędów nie pytamy, *czy* błąd wystąpił, tylko *jakiego jest rodzaju*: dopasowanie wzorca na `e.kind()` z `ErrorKind::NotFound` oddziela pierwsze uruchomienie programu, gdy pliku z danymi jeszcze nie ma i pusta kolekcja jest poprawną odpowiedzią, od braku uprawnień albo uszkodzonego nośnika. Pokusą jest tu jednolinijkowiec `unwrap_or_default()`, który zamienia na wartość domyślną **każdy** błąd odczytu: jest na miejscu, gdy naprawdę chodzi o „pliku jeszcze nie ma”, i jest katastrofą, gdy plik istnieje, lecz nie dało się go przeczytać — program rusza z pustymi danymi i przy najbliższym zapisie po cichu kasuje dorobek użytkownika. Odwrotny domyślny wybór jest równie zły: uznanie `NotFound` za błąd krytyczny sprawia, że program nie potrafi wystartować ani razu.

**Szukaj po polsku:** brak pliku a pusty plik · obsługa błędów wejścia/wyjścia · rodzaj błędu io · `rust ErrorKind::NotFound` · `rust match io::Error kind`
