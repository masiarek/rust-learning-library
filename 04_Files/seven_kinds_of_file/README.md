# Seven kinds of file

**Level:** 201 · working knowledge

**One line:** A path names one of seven kinds of thing — regular file, directory, symbolic link, FIFO, socket, character device, block device — `ls -l` shows which in its first character, `fs::metadata` answers three of them and `FileTypeExt` the other four, and `File::open` on a directory *succeeds*, so the check that matters is on the first `read`.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The seven, with the `ls -l` letter for each (`-`, `d`, `l`, `p`, `s`, `c`, `b`) and one example path on a Mac and on Linux — `/dev/zero` is a character device on both, `/dev/disk0` a block device on one
- [`FileType` ↗](https://doc.rust-lang.org/std/fs/struct.FileType.html): `is_file`, `is_dir`, `is_symlink`, and why the three are not exhaustive; [`std::os::unix::fs::FileTypeExt` ↗](https://doc.rust-lang.org/std/os/unix/fs/trait.FileTypeExt.html) for `is_fifo`, `is_socket`, `is_char_device`, `is_block_device`
- `metadata` follows a symlink and `symlink_metadata` does not — measured for the inspector page: `/tmp` on macOS is a directory to the first and a link to the second
- `File::open` on a directory returns `Ok` on Unix and the first `read` fails with `ErrorKind::IsADirectory` — the reason a tool checks the kind before reading, or handles the read error, and never trusts the open
- What each kind does to a reader: a FIFO blocks until a writer arrives, `/dev/zero` never ends, a socket refuses `open` outright
- Size, permissions and modification time from the same `Metadata`, and which of them a pipe does not have
- What is *not* a kind: an `.app` bundle is a directory, a hard link is a regular file with two names, and a Windows junction is a reparse point that `is_symlink` reports as a link

## The trap it exists for

`if path.exists() { File::open(path)? }` passes for a directory, a FIFO and a device, and the failure moves to wherever the first read happens — often inside a library, with a message that names none of them. `exists()` answers one question; `file_type()` answers the one you meant.

## See also

- [Opening a file](../opening_a_file/README.md) — the open that succeeds on a directory
- [Missing is not empty](../missing_is_not_empty/README.md) — the two answers that both look like *nothing there*
- [`Path` and `PathBuf`](../path_and_pathbuf/README.md) — the name, before it is any of the seven
- [Writing a file inspector](../../03_Command_Line/writing_a_file_inspector/README.md) — why a tool needs all seven answers
- [File type is four questions ↗](https://masiarek.github.io/encodings-learning-library/06_Terminal/file_type_is_four_questions/index.html) — the inode is question one; `file`, the mode bits and the extension are the other three

## Po polsku

Ścieżka nazywa jedną z siedmiu rzeczy — zwykły plik, katalog, dowiązanie symboliczne, kolejkę FIFO, gniazdo, urządzenie znakowe albo urządzenie blokowe — a `ls -l` pokazuje którą w pierwszym znaku wiersza (`-`, `d`, `l`, `p`, `s`, `c`, `b`). `fs::metadata(ścieżka)?.file_type()` odpowiada na trzy z tych pytań (`is_file`, `is_dir`, `is_symlink`), a uniksowa cecha `FileTypeExt` na pozostałe cztery; `metadata` podąża za dowiązaniem, `symlink_metadata` nie, więc `/tmp` na Macu jest katalogiem dla pierwszej i dowiązaniem dla drugiej. Rzecz, dla której ta strona istnieje: `File::open` na katalogu **kończy się powodzeniem** w Uniksie, a dopiero pierwszy `read` zwraca `IsADirectory` — więc sprawdzenie `path.exists()` przed otwarciem niczego nie chroni, a błąd przenosi się tam, gdzie ktoś pierwszy raz czyta, często w cudzej bibliotece i z komunikatem, który nie nazywa żadnego z siedmiu rodzajów.

**Szukaj po polsku:** rodzaje plików w Uniksie · dowiązanie symboliczne a twarde · kolejka FIFO · `rust fs::metadata file_type` · `rust FileTypeExt is_fifo` · `rust IsADirectory`
