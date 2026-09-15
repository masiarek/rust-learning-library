# Templates

**Level:** reference · files to copy

**One line:** Whole projects to copy — a relaxed **training** template and a strict **production** one, each with its `Cargo.toml`, `clippy.toml`, toolchain and a working example — plus a map of clippy's settings, and a script that applies either template to projects you already have.

| Page | What it is for |
|---|---|
| [Training](training/README.md) | Learning. Nothing stops `cargo run`; clippy's `pedantic` and `nursery` groups and 31 more lints warn about every line with a better form. The four unused-name lints are off, and `cargo strict` makes every warning an error |
| [Production](production/README.md) | Code that ships. Clippy's default groups, `pedantic` and 22 more lints at `deny`, `unsafe` forbidden, a release profile that keeps overflow checks, and `cargo ci` |
| [Clippy](clippy/README.md) | The nine lint groups with their sizes and defaults, where each setting lives, how to tell which setting a message came from, and the commands |
| [Automation](automation/README.md) | [`lint_profiles.py`](automation/lint_profiles.py): `new` from a template, `apply` a template to an existing project, `check` a folder of projects against one |

Both templates carry the same five everyday crates — `anyhow`, `rand`, `regex`, `serde`, `serde_json` — and production adds `clap` for arguments and `thiserror` for typed errors.

## A template or a workspace?

A template **copies** configuration; a [workspace](../05_Tooling/practice_workspace/README.md) **shares** it. Where there is a workspace, sharing wins: change the root once and every member follows, which is the case [Scaffolding a practice tree](../05_Tooling/scaffolding/README.md) makes at length. The templates here are for a project with nothing to share from — RustRover's *New Project*, a `cargo new` outside any tree — and `lint_profiles.py check` is there because a copy starts going stale the day it is made.

## Po polsku

Dwa gotowe projekty do skopiowania. **Trening** jest łagodny: nic nie blokuje `cargo run`, a clippy z grupami `pedantic` i `nursery` oraz 31 dodatkowymi lintami ostrzega przy każdej linijce, którą da się napisać lepiej; `cargo strict` zamienia te ostrzeżenia w błędy. **Produkcja** jest surowa: domyślne grupy clippy, `pedantic` i 22 linty na `deny`, `unsafe` zakazane, profil `release` z kontrolą przepełnień i `cargo ci` jako bramka. Strona **Clippy** to mapa grup i ustawień, a **Automation** opisuje skrypt, który zakłada projekt z szablonu albo nakłada szablon na istniejący.

*Szablon* kopiuje konfigurację, a *workspace* ją współdzieli — i tam, gdzie workspace jest, współdzielenie wygrywa. Szablony są dla projektu, który nie ma skąd niczego dziedziczyć: nowego projektu z RustRovera albo `cargo new` poza jakimkolwiek drzewem.

**Szukaj po polsku:** szablon projektu Rust · `Cargo.toml` linty · `clippy.toml` · `cargo new` szablon
