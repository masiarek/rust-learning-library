# Vendoring, and the `[patch]` table: copying dependencies in, and the two ways Cargo refuses your edit

**Level:** 201 · working knowledge

**One line:** `cargo vendor` copies every dependency's source into a folder in your repo and prints the four config lines that make Cargo read from there — but the copies are read-only in a precise, two-part sense: an incremental build **ignores** an edit to one without a word, a clean build **refuses** it by checksum, and `[patch.crates-io]` is the door Cargo holds open for changing a dependency on purpose.

## The command, and what it printed

```text title="Real output — cargo 1.98.0, a project whose only dependency is cfg-if"
$ cargo vendor
   Vendoring cfg-if v1.0.4 (/Users/…/.cargo/registry/src/index.crates.io-…/cfg-if-1.0.4) to vendor/cfg-if
To use vendored sources, add this to your .cargo/config.toml for this project:

[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
```

Two things worth reading off that. The source it copied from is `~/.cargo/registry/src/…` — the download cache that every project on the machine already shares, so vendoring is a *second* copy, in your tree, under version control. And it did not configure anything: the four lines are printed for you to put in `.cargo/config.toml` yourself, and until you do, builds carry on reading the registry as before.

What lands in `vendor/cfg-if/` is the crate exactly as published — `Cargo.toml`, `src/`, `tests/`, the licences — plus one file the publisher never wrote:

```text
vendor/cfg-if/.cargo-checksum.json
```

```json title="Its first field, verbatim"
{"$comment":"This file only protects against accidental modifications. It is not a security mechanism and does not protect against malicious changes.","files":{…}}
```

It maps every file in the crate to a SHA-256. The `$comment` is Cargo's own scoping of what that buys, and it is the honest one: this is a tripwire for *you*, not a defence against anyone else.

Why vendor at all: a build that must work with no network (an air-gapped machine, a reproducible-build audit, a CI runner you do not trust to reach crates.io), or a policy that every byte compiled has to be in the repository. If none of those is your situation, the shared registry cache already gives you offline rebuilds of anything you have built once, and vendoring is size for no gain.

## The edit that did nothing

With the config in place, `cargo build --offline` compiles from `vendor/`. Now append a line to a vendored source file and build again:

```text title="Real output — cargo 1.98.0"
$ echo "// edited" >> vendor/cfg-if/src/lib.rs
$ cargo build --offline
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
```

A hundredth of a second and no `Compiling` line. Cargo did not notice the file changed, because it does not *look*: a registry source — and a vendored directory stands in for one — is assumed never to change, so its files are excluded from the freshness check that decides what to rebuild. Your edit is on disk and not in the binary, and nothing says so. This is the incremental half of "read-only", and it is the dangerous half, because it looks like success.

## The clean build that refused

```text title="Real output — cargo 1.98.0 (paths shortened)"
$ cargo clean && cargo build --offline
error: the listed checksum of `…/vend/vendor/cfg-if/src/lib.rs` has changed:
expected: c09723e0890d15810374009e96b20bf0eb2f65f383006516f34db36240835c85
actual:   cdb743bf0348803e8111bbe987c1bf98d3d98dcc2ddc5c5c1aa787f09611ac55

directory sources are not intended to be edited, if modifications are required then it is recommended that `[patch]` is used with a forked copy of the source
```

The loud half. On a full build Cargo reads `.cargo-checksum.json` and compares, and the same edit that was invisible a moment ago is now a hard error — with the fix named in the message. The two halves together are what the Cargo Book means by *"Cargo treats vendored sources as read-only as it does to registry and git sources"*: not that the files are protected, but that Cargo's model of them has no room for a change, so a change is either unseen or rejected depending on which code path happens to run.

## The `[patch]` route

Copy the crate somewhere Cargo considers *yours*, drop the checksum file, edit, and tell the root manifest that this path stands in for the registry's copy:

```sh
cp -r vendor/cfg-if my-cfg-if
rm my-cfg-if/.cargo-checksum.json
echo "// edited" >> my-cfg-if/src/lib.rs
```

```toml title="Cargo.toml — appended"
[patch.crates-io]
cfg-if = { path = "my-cfg-if" }
```

```text title="Real output — cargo 1.98.0 (paths shortened)"
$ cargo build --offline
     Locking 1 package to latest Rust 1.98.0 compatible version
      Adding cfg-if v1.0.4 (…/vend/my-cfg-if)
   Compiling cfg-if v1.0.4 (…/vend/my-cfg-if)
   Compiling vend v0.1.0 (…/vend)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s
```

Now the edit compiles, and — the point of the route — *further* edits are seen, because a path dependency is one Cargo watches. In `Cargo.lock` the entry for `cfg-if` loses its `source` and `checksum` lines: there is no registry to name and no published archive to hash.

Four rules from [the Cargo Book's page on overriding ↗](https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html), each of which is a silent failure if you do not know it:

| Rule | What happens if you forget |
|---|---|
| The patched copy's `version` must satisfy the original requirement | the patch is ignored and the registry copy is used — no warning at all; a patch of a `2.x` fork against a manifest saying `"1"` does nothing |
| `[patch]` is read from the **workspace root** manifest only | a patch written in a member crate is ignored |
| `[patch]` overrides for *everyone* in the graph who depends on that crate | which is usually what you wanted, and occasionally a surprise three dependencies deep |
| `[replace]` is the deprecated predecessor | it still parses; write `[patch]` |

And one from crates.io rather than Cargo: a package with a `path` dependency cannot be published there, except under `[dev-dependencies]`. A patch is a local fix while the upstream one is in flight, not a way to ship your fork.

## Alternate registries

The other reason dependencies come from somewhere other than crates.io is a private registry — a company's internal crates, or a mirror. The talk that prompted this page showed one, [kellnr ↗](https://kellnr.io/), configured in `~/.cargo/config.toml` as a named registry with a `sparse+http://…` index and made the default under `[registry]`; the mechanism is [the Cargo Book's *Registries* chapter ↗](https://doc.rust-lang.org/cargo/reference/registries.html). Not tried here, so this page says nothing about how well any of them work — the speaker's own line was that he endorsed none of them and that you should find the one that fits.

## If you are coming from another language

**Python.** Vendoring is `pip download` into a folder plus `pip install --no-index --find-links`, and the read-only rule is the one everyone has broken: editing a file inside `site-packages` to fix a bug, having it work, and losing the fix on the next install. Python does not check; Cargo either ignores the edit or refuses it. `[patch]` is `pip install -e ./my-fork` — a path install that replaces the registry copy and is watched for changes — with one difference in your favour: Cargo's patch is a line in the manifest that travels with the repository, where an editable install is a property of one virtualenv on one machine.

**ABAP.** This is the closest bridge in the whole tooling section, because SAP has the same two-tier rule and enforces it harder. Standard SAP objects are read-only in the sense Cargo means: you can *see* the source of a standard function module, and a change to it is a **modification** — which the system refuses outright until an SSCR object key is registered, and which the Modification Assistant then tracks so that the next upgrade can ask you about each one. The sanctioned route is the one `[patch]` is: copy the object into your own `Z` namespace, change the copy, and point your code at it. The checksum error above is the Modification Assistant's refusal; `my-cfg-if` is the `Z` copy; the rule that a patched `version` must still satisfy the requirement is the same discipline as keeping a `Z` copy's interface compatible with the standard one it replaced. What Cargo lacks is the upgrade dialogue — nothing will ask you, when `cfg-if 1.0.5` is released, whether your patch still applies. It will silently keep using your copy for as long as `1.0.4` satisfies the requirement.

## See also

- [`Cargo.lock`](../cargo_lock/README.md) — the checksums this page's tripwire is made of, and the `source =` line a patched entry loses
- [Adding a dependency](../cargo_dependencies/README.md) — the `path =` form of a dependency, which is what `[patch]` substitutes in
- [A tree of practice projects](../practice_workspace/README.md) — the workspace root, which is the only manifest whose `[patch]` counts
- [The Cargo Book — `cargo vendor` ↗](https://doc.rust-lang.org/cargo/commands/cargo-vendor.html) — including `--versioned-dirs`, for tracking vendored history over time
- [The Cargo Book — source replacement ↗](https://doc.rust-lang.org/cargo/reference/source-replacement.html) — what those four `[source.*]` lines actually configure

---

*No generated output block on this page: every transcript is a property of Cargo's handling of a directory on disk, which the single-file answer-key runner has no way to exercise.*

## Po polsku

*Vendoring* (słowo zostaje po angielsku — „dostarczanie” nikomu nic nie powie) to skopiowanie źródeł wszystkich zależności do katalogu w twoim repozytorium. `cargo vendor` robi kopię z `~/.cargo/registry`, czyli z pamięci podręcznej, którą i tak dzielą wszystkie projekty na maszynie, i **wypisuje** cztery linie konfiguracji, których sam nie wpisuje — dopóki nie wkleisz ich do `.cargo/config.toml`, budowanie dalej czyta rejestr. Kiedy to ma sens: maszyna bez sieci, audyt odtwarzalności, polityka „każdy kompilowany bajt jest w repozytorium”. Jeśli to nie twoja sytuacja, wspólna pamięć podręczna już daje ci budowanie offline wszystkiego, co raz zbudowałeś.

Sedno strony to znaczenie zdania „Cargo traktuje zależności z katalogu `vendor/` jako tylko do odczytu”, bo znaczy ono dwie różne rzeczy naraz. **Przy budowaniu przyrostowym twoja edycja jest niewidoczna**: dopisanie linii do `vendor/cfg-if/src/lib.rs` i `cargo build` kończy się w setną sekundy, bez `Compiling`, bo Cargo zakłada, że źródła rejestrowe się nie zmieniają, i w ogóle ich nie sprawdza — zmiana jest na dysku, a nie w binarce, i nic o tym nie mówi. **Przy czystym budowaniu ta sama edycja jest błędem**: Cargo porównuje sumy kontrolne z pliku `.cargo-checksum.json` i odmawia, podając w komunikacie właściwą drogę. Sam plik sum kontrolnych w pierwszym polu uczciwie ogranicza swoje zadanie: chroni przed *przypadkową* modyfikacją i nie jest mechanizmem bezpieczeństwa.

Właściwą drogą jest tabela `[patch.crates-io]` w manifeście **korzenia** workspace'u: kopiujesz crate'a do własnego katalogu, usuwasz plik sum kontrolnych, edytujesz i wskazujesz ścieżkę. Od tej chwili Cargo pilnuje zmian w tej kopii jak we własnym kodzie, a wpis w `Cargo.lock` traci linie `source` i `checksum`. Cztery zasady, z których każda zawodzi po cichu: wersja załatanej kopii musi mieścić się w pierwotnym wymaganiu (inaczej łatka jest **ignorowana bez ostrzeżenia**); `[patch]` czyta się tylko z manifestu korzenia; łatka działa dla wszystkich w grafie, którzy zależą od tego crate'a; `[replace]` to przestarzały poprzednik. Do tego zasada crates.io: pakietu z zależnością `path` nie da się opublikować, poza `[dev-dependencies]` — łatka jest na czas, gdy poprawka idzie do upstreamu, a nie sposobem na wysyłanie własnego forka.

Dla kogoś z ABAP-u to najbliższy most w całym dziale: standardowe obiekty SAP są „tylko do odczytu” dokładnie w tym sensie, zmiana w nich to **modyfikacja**, której system odmawia bez zarejestrowanego klucza SSCR, a sankcjonowana droga to kopia w przestrzeni `Z` — czyli `[patch]`. Czego Cargo nie ma, to dialogu przy upgrade'ie: gdy wyjdzie `cfg-if 1.0.5`, nic nie zapyta, czy twoja łatka nadal pasuje.

**Szukaj po polsku:** kopiowanie zależności do repozytorium · `cargo vendor` · `[patch.crates-io]` · `directory sources are not intended to be edited` · nadpisywanie zależności w Cargo
