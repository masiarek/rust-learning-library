# Unix

The shell you run the compiler from.

Nothing in this section is Rust. It is here because two of the three tools on these pages *are* Rust — `fd` and `ripgrep` are among the most widely installed Rust programs in existence, and neither is a library or a framework. They are a hundred-line idea plus a fast engine, replacing a Unix utility that has shipped since the 1970s. If you want to know what Rust is actually used for outside of tutorials, `rg` is the honest answer.

[Tooling](../05_Tooling/README.md) is the loop *inside* a project — edit, build, run. This section is the minute before that loop starts: finding the file, finding the line, finding the command you ran yesterday.

| Lesson | Level | What it teaches |
|---|---|---|
| [Fuzzy finding with fzf](fuzzy_finding/README.md) | 101 → 201 | Three key bindings that replace typing a path with picking one — plus the shell integration `brew install` does not do for you, and the macOS key that silently breaks the third binding |
| [What the Rust rewrites bought](search_tools_in_rust/README.md) | 201 | `rg` answers in 0.02 s where `grep -r` takes 1.96 s on this repo — measured, then decomposed into the three separate things that gap is made of, only one of which is Rust |

## Planned

Rough order, not a promise:

- **`zoxide`** — a `cd` that learns which directories you actually use, and the shell hook that makes it work
- **`eza` and `delta`** — the other two Rust replacements worth the install, and the one place `delta` changes what `git` reports rather than how it looks
- **Reading a `PATH`** — why `which` and `command -v` can disagree, and what a shell function shadowing a real binary looks like from the inside

## Po polsku

Ten rozdział nie uczy Rusta, tylko powłoki (*shell*), z której uruchamiasz kompilator — a trafił do biblioteki dlatego, że na pytanie „co się właściwie pisze w Ruscie?” najuczciwszą odpowiedzią nie jest żaden framework webowy, tylko `fd` i `ripgrep`: narzędzia wiersza poleceń, które zastąpiły uniksowe `find` i `grep` i należą dziś do najczęściej instalowanych programów napisanych w tym języku. Polskie kursy Rusta pokazują zwykle sam język, znacznie rzadziej to, do czego bywa realnie używany, więc ta perspektywa potrafi zaskoczyć. Warto też trzymać osobno dwa rozdziały, które łatwo pomylić: `05_Tooling` opisuje pętlę *wewnątrz* projektu (edytuj — zbuduj — uruchom), a ten rozdział minutę wcześniejszą, zanim w ogóle znajdziesz plik, linię i polecenie wpisane wczoraj.

**Szukaj po polsku:** narzędzia wiersza poleceń w Ruscie · powłoka uniksowa · zamienniki grep i find · `ripgrep vs grep benchmark` · `rust cli tools`
