# Two versions of one crate in one binary: what "SemVer compatible" means to Cargo

**Level:** 201 · working knowledge

**One line:** When two of your dependencies each ask for `rand`, Cargo gives them **one** copy if the two requirements overlap under its compatibility rule and **two** copies if they do not — both outcomes are silent, `cargo tree -d` is how you find out which you got, and the day it matters is the day a `StdRng` is not a `StdRng`.

## The rule

Cargo's notion of compatible is one sentence, and it is *not* SemVer's:

> Versions are considered compatible if their left-most non-zero major/minor/patch component is the same. This is different from SemVer which considers all pre-1.0.0 packages to be incompatible.
>
> — [The Cargo Book, *Specifying Dependencies* ↗](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#default-requirements)

| Requirement | Range it means | The component that may not change |
|---|---|---|
| `1.2.3` | `>=1.2.3, <2.0.0` | the `1` |
| `0.2.3` | `>=0.2.3, <0.3.0` | the `2` |
| `0.0.3` | `>=0.0.3, <0.0.4` | the `3` |

So `rand = "0.8"` and `rand = "0.9"` are as far apart as `1` and `2` — the left-most non-zero component differs — and no amount of `cargo update` will ever move one to the other. That is the whole reason a manifest can say `"1"` and let Cargo pick, while a `0.x` crate has to be named to the minor.

## One copy: the requirements overlap

A workspace with an application and two library crates it depends on, each of which wants `rand`:

```toml
# alpha/Cargo.toml
rand = "0.8"
# beta/Cargo.toml
rand = "0.8.5"
```

`>=0.8.0, <0.9.0` and `>=0.8.5, <0.9.0` overlap, so Cargo picks one version that satisfies both and links it once:

```text title="Real output — cargo 1.98.0 (paths shortened)"
$ cargo tree -d
warning: nothing to print.

$ cargo tree -i rand
rand v0.8.8
├── alpha v0.1.0 (…/dupdemo/alpha)
│   └── app v0.1.0 (…/dupdemo/app)
└── beta v0.1.0 (…/dupdemo/beta)
    └── app v0.1.0 (…/dupdemo/app)
```

`-d` lists only crates present in more than one version, and *nothing to print* is the answer you want. `-i` inverts the tree — instead of *what does `rand` need*, *who needs `rand`* — which is the question to ask before changing a requirement.

## Two copies: they do not

Change one line — `beta` now says `rand = "0.9"` — and rerun:

```text title="Real output — cargo 1.98.0 (paths shortened)"
$ cargo tree -d
getrandom v0.2.17
└── rand_core v0.6.4
    ├── rand v0.8.8
    │   └── alpha v0.1.0 (…/dupdemo/alpha)
    │       └── app v0.1.0 (…/dupdemo/app)
    └── rand_chacha v0.3.1
        └── rand v0.8.8 (*)

getrandom v0.3.4
└── rand_core v0.9.5
    ├── rand v0.9.5
    │   └── beta v0.1.0 (…/dupdemo/beta)
    │       └── app v0.1.0 (…/dupdemo/app)
    └── rand_chacha v0.9.0
        └── rand v0.9.5 (*)
```

Nothing failed. `cargo build` compiles both, `Cargo.lock` carries two `[[package]]` blocks named `rand`, and the application links **eight** crates where it needed four — `rand` brought `rand_core`, `rand_chacha` and `getrandom` with it, and each of those is now doubled too, because the `0.8` line of `rand` depends on the `0.6` line of `rand_core`, and so on down.

The Cargo Book is explicit that this is by design, not an accident:

> The following two packages will not have their dependencies on `rand` unified because only incompatible versions are available for each. Instead, two different versions […] will be resolved and built.
>
> — [The Cargo Book, *Dependency Resolution* ↗](https://doc.rust-lang.org/cargo/reference/resolver.html#semver-compatibility)

The alternative — refusing to build — would make every `0.x` bump anywhere in the ecosystem a hard error for everyone downstream until the whole tree had moved in lockstep. Two copies is the price of not having that.

## What two copies cost

**Compile time and binary size, roughly doubled for the affected subtree.** Visible in the tree above: four extra crates to download, build and link.

**And one thing the compiler will tell you about, at the worst moment.** A type from `rand 0.8` and the same-named type from `rand 0.9` are two types. Let `alpha` hand out a random-number generator and `beta` accept one:

```rust
// alpha/src/lib.rs — built against rand 0.8
pub fn make() -> rand::rngs::StdRng { rand::SeedableRng::seed_from_u64(7) }
// beta/src/lib.rs — built against rand 0.9
pub fn take(_r: rand::rngs::StdRng) {}
// app/src/main.rs
fn main() { beta::take(alpha::make()); }
```

```text title="Real output — rustc 1.98.0, via cargo build -p app (paths shortened)"
error[E0308]: mismatched types
  --> app/src/main.rs:2:16
   |
 2 |     beta::take(alpha::make());
   |     ---------- ^^^^^^^^^^^^^ expected `rand::rngs::std::StdRng`, found a different `rand::rngs::std::StdRng`
   |     |
   |     arguments to this function are incorrect
   |
note: there are multiple different versions of crate `rand` in the dependency graph
  --> …/rand-0.9.5/src/rngs/std.rs:70:1
   |
70 | pub struct StdRng(Rng);
   | ^^^^^^^^^^^^^^^^^ this is the expected type
   |
  ::: …/rand-0.8.8/src/rngs/std.rs:34:1
   |
34 | pub struct StdRng(Rng);
   | ----------------- this is the found type
   = help: you can use `cargo tree` to explore your dependency tree
```

*Expected `StdRng`, found a different `StdRng`* is the sentence this page exists to make legible in advance. Older compilers printed the two identical paths with no explanation, and the message read like a compiler bug; the current one names the cause and points at the tool. Any crate that puts a dependency's type in its public API — a `serde` trait, a `tokio` runtime handle, an `http` request type — exposes its users to this the moment two copies exist.

**The other failure is loud and immediate.** Two requirements that *are* compatible but share no version — `=1.0.228` in one crate and `=1.0.229` in another — cannot be unified and cannot be duplicated either, since Cargo will only carry one copy per compatible line. That one is a resolver error at `cargo build` time rather than a type error later, and the Book's sentence for it is *"If two compatible versions cannot be unified because of conflicting version requirements, Cargo will error."*

## Getting back to one copy

`cargo tree -d` tells you the crate; `cargo tree -i rand@0.8.8` tells you who is holding the old line. Then the choices are the obvious three, in order of preference: wait for or contribute the dependent's upgrade; pin the *other* side down to match if it has not yet leaned on the new API; or, if the dependent is yours, move it. `cargo update` cannot help — moving across the incompatible boundary is precisely what a lockfile update is forbidden to do, and the `--breaking` flag that would rewrite the manifest is [nightly only](../cargo_lock/README.md).

## If you are coming from another language

**Python.** There is no equivalent, and the absence is the lesson. A Python environment holds **one** version of each package — `site-packages` has one `requests/` directory — so two libraries that need incompatible versions of the same package cannot be installed together, and `pip` reports the conflict at install time. Rust removes the conflict by allowing both, and moves the cost to the place shown above: a type from one copy is a stranger to the other. The Python reflex "there is only one `requests`" is the thing to unlearn, and `cargo tree -d` is how you check whether it holds today.

**ABAP.** Closer than it looks. A system has one active version of each function module — the ABAP equivalent of Python's one-copy rule — and the nearest thing to two copies is two *releases* of a function group living in different systems, which nobody would call one program. What Rust does is put those two releases in one executable and keep them apart by type. There is no ABAP mechanism to bridge to, only the observation that "same name, different version, different type" is a category the language never had to name.

## See also

- [Adding a dependency](../cargo_dependencies/README.md) — the requirement operators (`^`, `~`, `=`, `*`) and what each permits `cargo update` to do
- [`Cargo.lock`](../cargo_lock/README.md) — where the two `[[package]]` blocks end up, and why `cargo update` will not merge them
- [Randomness, and the `rand` API the Rust Book still teaches](../../15_First_Programs/randomness/README.md) — `rand` is used here because it is the crate most likely to appear twice in a beginner's tree, having changed its API at `0.9` and again at `0.10`
- [Compile times](../compile_times/README.md) — why a duplicated subtree is not free
- [The Cargo Book — version-incompatibility hazards ↗](https://doc.rust-lang.org/cargo/reference/resolver.html#version-incompatibility-hazards) — the same failure, and the advice to crate authors on avoiding it

---

*No generated output block on this page: the example is a three-crate workspace with a dependency graph, and the answer-key runner compiles one dependency-free file at a time.*

## Po polsku

Kiedy dwie twoje zależności proszą o ten sam crate, Cargo daje im **jedną** kopię, jeśli ich wymagania na siebie zachodzą, i **dwie**, jeśli nie — a o tym, które z tych dwóch zaszło, nic nie mówi. Sprawdza się to poleceniem `cargo tree -d` (*duplicates*), którego najlepszą odpowiedzią jest `warning: nothing to print.`, a `cargo tree -i rand` odwraca drzewo i pokazuje, **kto** wymaga danego crate'a.

Reguła zgodności jest jedno zdanie i **nie jest** to reguła SemVer: zgodne są wersje o tym samym skrajnie lewym niezerowym składniku. `1.2.3` przyjmuje wszystko poniżej `2.0.0`, ale `0.2.3` już tylko poniżej `0.3.0`, a `0.0.3` wyłącznie `0.0.3`. Stąd `rand = "0.8"` i `rand = "0.9"` są od siebie tak daleko jak `1` i `2` i żadne `cargo update` nigdy nie przesunie jednej do drugiej — dlatego manifest może powiedzieć `"1"` i zdać się na Cargo, a crate w wersji `0.x` trzeba nazwać do składnika *minor*.

Dwie kopie to świadoma decyzja projektowa, nie wypadek: alternatywą byłoby odmawianie budowania za każdym razem, gdy ktokolwiek w ekosystemie podniesie `0.x`, aż całe drzewo ruszy się równym krokiem. Koszt jest dwojaki. Pierwszy to czas kompilacji i rozmiar binarki — w przykładzie `rand` pociągnął za sobą `rand_core`, `rand_chacha` i `getrandom`, więc zdublowały się cztery crate'y, nie jeden. Drugi ujawnia się w najgorszym momencie: **typ z `rand 0.8` i typ o tej samej nazwie z `rand 0.9` to dwa różne typy**. Komunikat kompilatora brzmi dosłownie *expected `rand::rngs::std::StdRng`, found a different `rand::rngs::std::StdRng`* — i o ile starsze kompilatory drukowały dwie identyczne ścieżki bez słowa wyjaśnienia (co wyglądało jak błąd kompilatora), dzisiejszy dopisuje *there are multiple different versions of crate `rand` in the dependency graph* i odsyła do `cargo tree`. Każdy crate, który wystawia typ swojej zależności w publicznym API, naraża użytkowników na to dokładnie w chwili, gdy pojawi się druga kopia.

Dla kogoś z Pythona brak odpowiednika jest właśnie lekcją: środowisko Pythona trzyma **jedną** wersję każdego pakietu i konflikt zgłasza `pip` przy instalacji. Rust usuwa konflikt, pozwalając na obie kopie, i przenosi koszt tam, gdzie pokazano wyżej. Powrót do jednej kopii to zawsze ruch w manifeście któregoś z zależnych crate'ów — `cargo update` nie przekroczy niezgodnej granicy, bo dokładnie tego plik blokady robić nie może.

**Szukaj po polsku:** zgodność wersji w Cargo · `cargo tree --duplicates` · dwie wersje tego samego crate'a · `expected StdRng found a different StdRng` · lewy niezerowy składnik wersji
