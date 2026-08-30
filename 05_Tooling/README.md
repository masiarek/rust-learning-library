# Tooling

The toolchain rather than the language: `cargo`, `rustc`'s flags, build profiles, and the parts of a working day that are spent waiting rather than writing.

The toolchain pages have a map of their own — [**TOOLCHAIN.md**](../TOOLCHAIN.md) puts them in reading order and sorts them by the problem you actually have.

These pages are not about making your program better. They are about the loop you sit inside all day — edit, build, run — and they earn their place because that loop is where Rust asks the most patience of you.

| Lesson | Level | What it teaches |
|---|---|---|
| [A throwaway that needs a crate](scratch_with_a_crate/README.md) | 101 | `cargo new` + `cargo add` + `cargo run` — what *"you might be missing a crate named `rand`"* really means, and the folder, manifest and test runner the three commands write for you |
| [A tree of practice projects](practice_workspace/README.md) | 201 | Forty exercise folders want the same four config files — a workspace shares them from the root, and `cargo new` writes the opt-in for you, so there is no script to maintain |
| [Adding a dependency](cargo_dependencies/README.md) | 101 → 201 | `search`, `info`, `add` — and the fact that `rayon = "1.12.0"` is a *range*, not the version you got |
| [bacon](bacon/README.md) | 101 → 201 | A pane that re-runs check, clippy or the tests on every save — the cheapest tool here, and where `watchexec` wins instead |
| [cargo-nextest](nextest/README.md) | 201 | A process per test rather than a thread, so a test that *aborts* is one failure and not a lost run — and the doctests it silently stops running |
| [Choosing an editor](editors/README.md) | reference | Every editor but RustRover is a front end for the same `rust-analyzer`, so the choice is what the window costs you before it shows you a type — with the pros and cons of six of them, and one verified way the do-it-yourself path fails silently |
| [Commit on green](commit_on_green/README.md) | 201 | `savepoint` commits on the one transition that matters — red to green — and why that is neither `git stash` nor anything to do with GitHub; plus what squashing is, and `--soft` vs `--hard` |
| [Compile times](compile_times/README.md) | 201 | A build is four phases, and each optimization reaches exactly one — reduced debug info, the parallel front end, Cranelift, and why a saving is never portable |
| [devenv](devenv/README.md) | 201 | What a Nix development environment buys — and the ladder of cheaper tools it sits on top of, so you can tell which rung your project is actually standing on |
| [Formatting](formatting/README.md) | 101 → 201 | Hand the whitespace argument to `rustfmt` — and learn which of your IDE's *two* Rust formatters just ran, because a selection and a whole file do not go through the same one |
| [Neovim with LazyVim](neovim_setup/README.md) | 201 | A verified Rust setup in four commands — and the two independent ways it installs perfectly, looks healthy, and never starts a language server |
| [Nightly by default](nightly/README.md) | 201 | `rustup default nightly` changes the compiler for every project on the machine, and is the one toolchain choice recorded nowhere |
| [Pinning the toolchain](pinning_the_toolchain/README.md) | 201 | Which `rustc` verified the answer keys — nothing here says, and `rust-toolchain.toml` is the four-line file that makes the laptop and CI agree on purpose |
| [RustRover Code Vision](rustrover_code_vision/README.md) | 101 → 201 | The grey `1 usage · 1 implementation` line above every declaration — the right-click that hides one metric, the checkbox that hides all of them, and the three toggles that are not the same toggle |
| [RustRover setup](rustrover_setup/README.md) | 101 → 201 | Make clippy the on-the-fly linter, teach the run configurations which workspace package you meant, stop the documentation and Build panels misbehaving, find a light theme when the three bundled ones are not enough, and the things the IDE needs no help with |
| [rustup](rustup/README.md) | 101 → 201 | The `rustc` on your `PATH` is a 154-byte shim, and the five-rung rule it uses to pick the real one |
| [Scaffolding a practice tree](scaffolding/README.md) | 201 | What a setup script should write — the workspace root, `.idea/` run configurations, and a `doctor` for the cross-file invariants no single tool owns — and the one thing it must not template |
| [Strict clippy lints](strict_lints/README.md) | 201 | Denying `unwrap`, `panic` and indexing turns runtime aborts into compile errors — and rejects `n + 1` along the way |
| [What MCP is](what_mcp_is/README.md) | 201 | JSON-RPC on a pipe — a whole MCP server in dependency-free Rust, the `println!` that corrupts one, and what *Always allow (`rustrover:*`)* actually grants |

The one tooling page that is a *prerequisite* rather than a refinement lives in Foundations instead: [running a scratch program](../15_First_Programs/rustc_without_cargo/README.md), which is how you run anything in this library at all.

## Planned

Rough order, not a promise:

- **`cargo test`, and the three kinds of test** — unit, integration, and doc tests; what each one can see, and which file it belongs in
- **Clippy** — the lints worth arguing with, and `#[allow]` as a comment that the compiler checks
- **Workspaces** — one `target/`, one lockfile, many crates, and the split that actually speeds a build up
- **`cargo add` and semver** — what a caret requirement really permits, and what `cargo update` is allowed to do to you

## Po polsku

Ten dział nie uczy języka, tylko **łańcucha narzędzi** (*toolchain*) — czyli tego, w czym programista siedzi przez cały dzień: edytuj, zbuduj, uruchom. Rozróżnienie jest ważne, bo polskojęzyczne materiały o Ruscie opisują niemal wyłącznie sam język (Tour of Rust po polsku urywa się na rozdziale 5. i o `cargo` nie mówi nic), a pytania, które realnie zatrzymują początkującego, brzmią raczej „dlaczego to się buduje trzy minuty” i „skąd wziąć ten crate”. Odpowiedzi na nie są tutaj, nie w rozdziale o własności.

Dla kogoś, kto przychodzi z C++ albo z Pythona, największą niespodzianką bywa to, jak **mało** jest tu narzędzi do wyboru. Jedno polecenie `cargo` robi to, co tam rozkłada się na make lub CMake, pip, venv, pytest i generator dokumentacji: buduje, uruchamia testy, pobiera zależności, składa dokumentację. Nie ma pliku budowania, który trzeba by napisać ręcznie, i nie ma decyzji „którego menedżera pakietów użyć” — jest `Cargo.toml`. Cenę za to płaci się w jednym miejscu, w czasie kompilacji, i dlatego kilka stron w tym dziale ([Compile times](compile_times/README.md), [bacon](bacon/README.md), [cargo-nextest](nextest/README.md)) dotyczy wyłącznie skracania oczekiwania, a nie pisania lepszego kodu.

Praktyczna uwaga na start: szukanie po polsku prawie nic tu nie da. Komunikaty `cargo` i `rustup` są po angielsku, nazwy podkomend są angielskimi słowami i to ich trzeba używać w wyszukiwarce — polskie „kompilacja Rusta wolno działa” zwróci garść wpisów blogowych, a `rust slow compile times` zwróci odpowiedź. Jeśli nie wiesz, od której strony zacząć, mapa [TOOLCHAIN.md](../TOOLCHAIN.md) układa je według problemu, który faktycznie masz, a nie według poziomu trudności.

**Szukaj po polsku:** łańcuch narzędzi · zależności w Cargo · `the cargo book` · `rust slow compile times` · `rustup manage toolchains`
