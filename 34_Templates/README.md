# Templates

**Level:** reference · files to copy

**One line:** Files you copy into a project rather than read — each folder holds the templates, the script that applies them, and a page saying what every line is for.

| Folder | What is in it |
|---|---|
| [Automation](automation/README.md) | Two lint profiles for a standalone Cargo project — **training**, very strict except for unused bindings, and **production**, where every warning is an error — and [`lint_profiles.py`](automation/lint_profiles.py), which applies them, checks them, and keeps the path in `.cargo/config.toml` current |

## A template or a workspace?

A template **copies** configuration; a [workspace](../05_Tooling/practice_workspace/README.md) **shares** it. Where there is a workspace, sharing wins: change the root once and every member follows, which is the case [Scaffolding a practice tree](../05_Tooling/scaffolding/README.md) makes at length. The files here are for a project with nothing to share from — RustRover's *New Project*, a `cargo new` outside any tree. Every template here comes with a `check`, because a copy starts going stale the day it is made.

## Po polsku

*Szablon* kopiuje konfigurację, a *workspace* ją współdzieli — i tam, gdzie workspace jest, współdzielenie wygrywa. Pliki w tym dziale są dla projektu, który nie ma skąd niczego dziedziczyć: nowego projektu z RustRovera albo `cargo new` poza jakimkolwiek drzewem. Dlatego każdy szablon ma swoje `check` — kopia zaczyna się starzeć w dniu, w którym powstała.

**Szukaj po polsku:** szablon projektu Rust · `cargo new` szablon · `Cargo.toml` konfiguracja lintów
