# devenv: what a Nix development environment buys, and what it costs

**Level:** 201 · working knowledge

**One line:** [devenv ↗](https://devenv.sh/) makes one file the complete definition of a project's development environment — compiler, CLI tools, system libraries, environment variables, git hooks and running databases alike — identical on every machine that opens the project; the entry fee is installing Nix, and whether that fee is worth paying is decided almost entirely by how far down that list your project actually reaches.

## What it is

devenv is built and maintained by [Cachix ↗](https://www.cachix.org/), and it describes itself as *"Fast, Declarative, Reproducible, and Composable Developer Environments."* Underneath it is [Nix ↗](https://nixos.org/) — a package manager whose central idea is that a package is a pure function of its inputs, so the same inputs give byte-identical outputs on any machine, and two projects can depend on incompatible versions of the same library without either one knowing.

Nix has had that capability for twenty years and has been kept from general use by its own learning curve: a lazy functional configuration language, a module system, and a vocabulary that assumes you already have it. devenv's contribution is a front end. You write a small number of high-level options, and it generates the Nix.

`devenv init` scaffolds three files — `devenv.nix` (the environment), `devenv.yaml` (where the inputs come from), and a `.gitignore`. Four commands cover ordinary use:

```sh
devenv shell             # enter the environment
devenv up                # start the background processes and services
devenv test              # build the environment and run its checks
devenv tasks run <task>  # run one declared task
```

## Reading the file

Taking the configuration block by block — this is the shape of the one on the devenv slide, which is a Rust environment:

```nix
languages.rust = {
  enable = true;
  channel = "nightly";
  components = [ "rustc" "cargo" "clippy" "rust-analyzer" ];
};
```

The toolchain, doing the job [`rust-toolchain.toml`](../pinning_the_toolchain/README.md) does — with one difference that is the whole argument for devenv: `rust-toolchain.toml` asks a rustup you already installed to fetch a version, while this *is* the installation. A machine with no Rust on it at all ends up in the same state as a machine that has had Rust for years.

```nix
packages = with pkgs; [ bacon cargo-seek cargo-nextest cargo-generate ];
```

The CLI tools. Note what this replaces: four `cargo install` invocations, each compiling from source, each landing in a shared `~/.cargo/bin` where they are one global version for every project on the machine. Here they are versioned per project and come prebuilt.

```nix
scripts.watcher.exec = ''
  watchexec -c -e rs "cargo clippy && cargo test && cargo run"
'';
```

A command that exists only inside this project, and carries its own dependency — `watchexec` is declared alongside it, so `watcher` cannot be on the `PATH` without the binary it calls also being there. That coupling is the part a `Makefile` or a shell alias cannot express.

```nix
env.DATABASE_URL = "postgres://user:pass@localhost/dbname";
enterShell = ''
  cargo update -n
'';
git-hooks.hooks.clippy.enable = true;
```

Environment variables, a hook that runs on entry, and a git hook installed by opening the project rather than by remembering to run an install step. Everything a `README.md` normally asks a new contributor to do by hand, done by the act of entering the directory.

## The slide is a montage — the real file is smaller

Worth separating, because the two are different documents. The configuration [Tris Oaten publishes ↗](https://namtao.com/rust) is correct, and notably plainer than the slide:

```nix
{ pkgs, lib, config, ... }:

{
  languages.rust = {
    enable = true;
    channel = "nightly";
    components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
  };

  packages = with pkgs; [ openssl ];

  scripts.watcher = {
    exec = ''
      RUSTFLAGS=-Awarnings watchexec -r --clear=reset -e rs --wrap-process=none "cargo run -q"
    '';
    packages = [ pkgs.watchexec ];
    description = "Rebuilds and runs app with supressed warnings";
  };

  # C LIBRARIES
  # env.LD_LIBRARY_PATH = lib.makeLibraryPath [
  #   pkgs.zlib
  # ];
}
```

It also needs one command the slide does not show, because `channel = "nightly"` is not something plain nixpkgs can serve:

```sh
devenv inputs add rust-overlay github:oxalica/rust-overlay --follows nixpkgs
```

Three differences from the slide are worth naming, because a reader's first instinct with a good config screenshot is to paste it:

**1. The real file opens `{ pkgs, lib, config, ... }:` and the slide does not.** A devenv module is a *function* of those arguments; without the header, `pkgs.watchexec` and `lib.makeLibraryPath` are unbound identifiers. This is the difference between a Nix module and a Nix attribute set, and it is the first thing to check when a pasted config fails on a name that is obviously right.

**2. The `LD_LIBRARY_PATH` block is commented out in the real file, and live on the slide** — which matters, because on the slide it lands above an `env = { … }` block and the two together are an error. Nix merges *attribute paths* freely (`a.b = 1; a.c = 2;` is one set with two keys), but mixing a path and a whole set on the same name is rejected, and, [notoriously, in only one order ↗](https://github.com/NixOS/nix/issues/2077):

```text
error: attribute 'x' at (string):1:12 already defined at (string):1:3
```

`{ x = { y = 3; }; x.q = 3; }` evaluates; the same two lines reversed do not. The slide is in the failing order. Pick one form per name — every line an `env.NAME = …`, or one `env = { … }` block — and the question never arises.

**3. The slide's `enterShell`, `git-hooks` and `DATABASE_URL` are feature demonstrations**, not part of the published environment. One version of the slide interpolates `config.env.GREET`, which nothing defines.

None of this is a criticism of the talk — a slide's job is to show surface area in one frame, and it does. It matters only because the published file is the one to copy.

## What you are actually buying

The useful way to decide is to find the rung your project is standing on. Each row solves everything above it, and costs more than everything above it:

| What you need | Cheapest thing that does it |
|---|---|
| One pinned compiler and its components | [`rust-toolchain.toml`](../pinning_the_toolchain/README.md) — four lines, no new tooling |
| …plus a few cargo CLI tools | `cargo install --locked`, or `cargo-binstall` for prebuilt ones |
| …plus system libraries (`zlib`, `openssl`, `protoc`, a C toolchain) | Nix starts genuinely winning — this is where `brew` and `apt` diverge and READMEs grow an OS-per-paragraph install section |
| …plus Postgres or Redis running on demand, per project, with no global daemon | devenv wins decisively; nothing else in the list is even competing |
| …plus the same environment in CI, in a container, and across a monorepo | devenv's composition and container generation are the point of the product |

The top two rows are where most Rust projects live, and they are fully served by tools that cost nothing to adopt. The bottom two are where devenv stops being a preference and starts being the obvious answer.

## What it costs

**The dependency is Nix, not devenv.** Installing devenv means installing Nix first, and on macOS that is not a small change: a multi-gigabyte `/nix` store on its own synthetic volume, a background daemon, a pool of build users, and shell initialisation edited in several files. It is a reversible change, but not a one-command one. devenv's own getting-started page also warns that macOS's ancient system Bash causes evaluation errors and suggests replacing it — the first hint that you are now administering a second package manager.

**And you are.** The `nixpkgs` in your lockfile is a distribution with its own release cadence, its own version lag, and its own occasional broken package, and when something is missing the fix is written in the Nix language rather than in Rust. That is a real skill with a real learning curve, and on the day a dependency will not build it is the only skill that helps.

**Everyone on the project pays it too.** An environment definition only reproduces an environment for people who have installed the thing that reads it. A contributor who wants to fix a typo now installs a package manager first. For a team, that is a one-time cost with an obvious payoff; for an open-source project hoping for drive-by contributions, it is a wall.

## So — should you use it?

**Yes, fairly clearly, if** your project needs services (a database, a queue, a cache) or non-Rust system libraries; or your team is polyglot and every language is currently pinned a different way; or onboarding a new machine is a day of following a README; or you are already running Nix and this is a nicer front end for it.

**No, fairly clearly, if** the honest answer to "what does this project need" is a compiler, a formatter, and two cargo tools. That is the top of the ladder, `rust-toolchain.toml` covers it, and installing a second package manager to solve it is a large amount of machinery pointed at a small amount of problem. The same applies when contributors are strangers rather than colleagues, and when nobody on the project wants to be the one who learns Nix — because eventually somebody has to be.

**The tell is services.** Everything above that row on the table has a cheap competitor, and nothing below it does.

## Before you commit to it

- **`devenv.lock` is the reproducibility, not `devenv.nix`.** The `.nix` file names inputs; the lockfile pins them to exact revisions. Commit it, and treat updating it as a reviewable event, exactly as with `Cargo.lock` or `uv.lock`.
- **2.0 broke things, and search results predate it.** In devenv 2.0 the `git-hooks` input is no longer included by default and must be declared in `devenv.yaml`; `pre-commit` was replaced by `prek`; a native process manager replaced process-compose; and `devenv build` now emits JSON. Configurations you find online — including the slide above — may assume 1.x.
- **CI has to install it too**, which is a cache-warming job of its own. The [`examples.yml` ↗](https://github.com/masiarek/rust-learning-library/blob/master/.github/workflows/examples.yml) workflow here takes seconds; a cold Nix evaluation does not.
- **Pair it with [`direnv` ↗](https://direnv.net/) or you will forget to enter the shell**, and then debug a "missing" tool that is merely one `devenv shell` away.
- **Nix itself is a moving target.** devenv's roadmap includes replacing its Nix evaluator with [Tvix ↗](https://tvix.dev/), so the layer you are adopting is under active reconstruction.

## If you are coming from another language

- **Python** — closest to `uv`, and the comparison is the clearest way to see the boundary. `uv` reproduces the interpreter and every Python dependency perfectly, and stops at the edge of the language: a wheel needing `libpq` still needs `libpq` from somewhere. devenv is what "somewhere" looks like when it is also declared. This library's documentation build already uses `uv`; nothing in it declares the C libraries underneath.
- **ABAP** — the environment is not something you configure, it is the system you logged into, administered centrally and identical for everyone by construction. devenv is an attempt to recover that property on a laptop, and the cost of recovering it is that somebody now has to write it down.

## Verdict for this library

Not worth it here, and the reason is a measurement rather than a preference. This repository needs `rustc` and Python 3.11+, and [what its answer keys are actually exposed to](../pinning_the_toolchain/README.md) is one unpinned compiler whose diagnostics never enter a recorded key in the first place. That is a top-of-the-ladder problem. A `rust-toolchain.toml` closes it in four lines; Nix would close it in several gigabytes, and would also mean a reader who wants to run one example installs a package manager to do it — for a *learning* library, that is the cost that settles it.

The tool is good and the idea is right. It is simply aimed at a bigger problem than this one.

## See also

- [Pinning the toolchain](../pinning_the_toolchain/README.md) — the four-line version of this page's problem, and the one this library actually needs
- [Compile times](../compile_times/README.md) — the other thing a development environment is silently costing you

## Sources

- [devenv.sh ↗](https://devenv.sh/) and its [getting started ↗](https://devenv.sh/getting-started/) guide — tagline, feature list, installation, and commands
- [Migrating to 2.0 ↗](https://devenv.sh/guides/migrating-to-2.0/) — the breaking changes above
- [NixOS/nix#2077 ↗](https://github.com/NixOS/nix/issues/2077) — the order-dependent attribute-merging error, with the exact message

---

*No generated output block on this page, deliberately: Nix is not installed on the machine this library is written on, so every transcript here would be one nobody ran. A page in this repository does not print output it has not verified — and that constraint is doing real work in a page whose subject is reproducibility.*

## Po polsku

devenv to odpowiedź na „u mnie działa” postawiona o poziom niżej, niż zwykle się ją stawia: jeden plik opisuje **całe** środowisko projektu — kompilator, narzędzia wiersza poleceń, biblioteki systemowe, zmienne środowiskowe, haki gita, a nawet działającą bazę danych — i każda maszyna, która otwiera ten projekt, dostaje dokładnie to samo. Pod spodem siedzi Nix, menedżer pakietów zbudowany wokół jednej myśli: pakiet jest **czystą funkcją swoich wejść**, więc te same wejścia dają identyczny wynik wszędzie. Nix ma tę własność od dwudziestu lat i od dwudziestu lat odstrasza własnym językiem konfiguracji; wkładem devenv jest przyjazna warstwa nad nim. Warto zauważyć jedną różnicę wobec [`rust-toolchain.toml`](../pinning_the_toolchain/README.md), bo to ona jest całym argumentem za devenv: tamten plik **prosi** zainstalowanego już `rustup` o konkretną wersję, a ten **jest** instalacją — maszyna bez śladu Rusta kończy w tym samym stanie co maszyna, która ma go od lat.

Najcenniejsze na tej stronie jest jednak nie to, jak devenv działa, tylko **jak zdecydować, czy go potrzebujesz**, bo polskie materiały o narzędziach rzadko podają koszt obok korzyści. Drabina wygląda tak: sam przypięty kompilator z komponentami — cztery linijki w `rust-toolchain.toml`, nic więcej nie trzeba; plus kilka narzędzi `cargo` — `cargo install --locked`; plus biblioteki systemowe (`openssl`, `zlib`, `protoc`) — tu Nix zaczyna naprawdę wygrywać, bo to jest dokładnie miejsce, w którym README puchnie o akapit na każdy system operacyjny; plus Postgres albo Redis uruchamiany na żądanie, osobno dla każdego projektu, bez globalnego demona — tu nic innego nawet nie startuje w tym wyścigu. **Rozstrzyga wiersz z usługami.** Wszystko powyżej ma tani odpowiednik, poniżej nie ma żadnego.

Koszt trzeba nazwać wprost, bo nie jest to koszt devenv, tylko **Nixa**. Na macOS instalacja oznacza kilkugigabajtowy magazyn `/nix` na własnym wolumenie, demona w tle, pulę użytkowników budujących i zmiany w kilku plikach startowych powłoki — rzecz odwracalną, ale nie jednym poleceniem. Od tego momentu administrujesz **drugim menedżerem pakietów**: `nixpkgs` ma własny rytm wydań, własne opóźnienia wersji i własne popsute pakiety, a w dniu, w którym coś się nie zbuduje, jedyną przydatną umiejętnością jest język Nix, nie Rust. Płaci też każdy współpracownik: ktoś, kto chciał poprawić literówkę, najpierw instaluje menedżer pakietów. Dla zespołu to jednorazowy koszt z oczywistym zwrotem, dla projektu otwartego liczącego na przypadkowe poprawki — mur.

Na koniec trzy rzeczy dla kogoś, kto zamierza skopiować cudzą konfigurację, bo to najczęstszy sposób zaczynania z Nixem. Po pierwsze, prawdziwy plik zaczyna się od nagłówka `{ pkgs, lib, config, ... }:` — moduł devenv jest **funkcją** tych argumentów, a bez nagłówka `pkgs.watchexec` jest po prostu niezwiązaną nazwą; to pierwsza rzecz do sprawdzenia, gdy wklejona konfiguracja wywala się na nazwie, która ewidentnie jest poprawna. Po drugie, Nix scala ścieżki atrybutów (`a.b = 1; a.c = 2;` to jeden zbiór), ale zmieszanie ścieżki i całego zbioru pod tą samą nazwą jest błędem — i to sławnie **zależnym od kolejności**: `{ x = { y = 3; }; x.q = 3; }` się liczy, te same dwie linijki odwrotnie już nie. Trzymaj się jednej formy na nazwę. Po trzecie, odtwarzalność zapewnia `devenv.lock`, a nie `devenv.nix` — plik `.nix` tylko nazywa wejścia, a przypina je dopiero blokada, więc commituj ją i traktuj jej aktualizację jak zdarzenie do przeglądu, tak samo jak `Cargo.lock`.

**Szukaj po polsku:** odtwarzalne środowisko programistyczne · menedżer pakietów Nix · `devenv.sh getting started` · `nix attribute already defined` · `direnv devenv shell`
