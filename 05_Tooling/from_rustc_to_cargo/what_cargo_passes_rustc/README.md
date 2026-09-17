# What `cargo run -v` shows: the `rustc` line, flag by flag

**Level:** 201 · for anyone who has run `cargo` and wondered what it adds

**One line:** For a package with no dependencies, a Cargo build is one `rustc` command, and `-v` prints it. Each flag is either a manifest setting, a profile default, or bookkeeping for Cargo's own caches. Paste the line into a shell, after `mkdir -p target/debug/deps`, and `rustc` writes the same bytes Cargo did.

The package is the one from [the previous page](../README.md): `ok.rs` printing `OK`, adopted by `cargo init`. Transcripts come from Cargo 1.98.0 in the `rust:1.98-slim` container, in `/tmp/ok`. macOS runs are labelled.

## The line

```text title="Real output — rm -rf target/ && cargo run -v, Linux"
   Compiling ok v0.1.0 (/tmp/ok)
     Running `/usr/local/rustup/toolchains/1.98.0-x86_64-unknown-linux-gnu/bin/rustc --crate-name ok --edition=2024 ok.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=cfa1e0f6b73ad44b -C extra-filename=-864ffab949c86c66 --out-dir /tmp/ok/target/debug/deps -C incremental=/tmp/ok/target/debug/incremental -L dependency=/tmp/ok/target/debug/deps`
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/ok`
OK
```

On this Mac the same package prints the same line with three differences. The compiler is plain `rustc` rather than a path into the toolchain. There is one extra flag, `-C split-debuginfo=unpacked`, directly after `-C debuginfo=2`. And the two hashes differ.

## The flags

| Flag | Where it comes from | What it does |
|---|---|---|
| `--crate-name ok` | `name` under `[[bin]]` | names the crate; it shows up in symbol names, as in `_RNv…_2ok4main` |
| `--edition=2024` | `edition` in `[package]` | without it `rustc` compiles as [edition 2015](../../../15_First_Programs/rustc_without_cargo/README.md#the-trap-rustcs-default-edition-is-2015) |
| `ok.rs` | `path` under `[[bin]]` | the crate root, the one file `rustc` is given; `mod` declarations lead it to the rest |
| `--crate-type bin` | it is a `[[bin]]` target | an executable rather than a library |
| `--error-format=json` | Cargo | diagnostics come out as JSON rather than text, so Cargo can [read them](#diagnostics-as-json) |
| `--json=diagnostic-rendered-ansi,artifacts,future-incompat` | Cargo | the JSON carries a ready-coloured text rendering, a message as each output file is written, and a [report of warnings due to become errors ↗](https://doc.rust-lang.org/rustc/json.html#future-incompatible-reports) |
| `--emit=dep-info,link` | Cargo | two outputs: the `.d` file listing every source file read, and the executable. The `.d` file is how Cargo later knows a `touch` means [`Dirty`](../../../20_Compilers/makefiles/README.md) |
| `-C embed-bitcode=no` | Cargo, when LTO is off | skips the LLVM bitcode that only link-time optimization reads. With `CARGO_PROFILE_DEV_LTO=true` the flag disappears and `-C lto` takes its place |
| `-C debuginfo=2` | `debug = true` in the `dev` profile | full debug information. The `--release` line has no `debuginfo` flag and adds `-C opt-level=3` and `-C strip=debuginfo` |
| `-C split-debuginfo=unpacked` | Cargo's default on macOS, when debug info is on | [below](#macos-split-debuginfounpacked) |
| `--check-cfg 'cfg(docsrs,test)'` | Cargo | the extra `cfg` names this package expects; any other name [warns](#check-cfg-a-misspelt-cfg-warns-only-under-cargo) |
| `--check-cfg 'cfg(feature, values())'` | `[features]`, which is empty | so every `cfg(feature = "…")` warns until the feature is declared |
| `-C metadata=cfa1e0f6b73ad44b` | Cargo, hashed from the package and its settings | [mixed into symbol names](#-c-metadata-and-extra-filename) |
| `-C extra-filename=-864ffab949c86c66` | Cargo | a suffix on every output file: `deps/ok-864ffab949c86c66` |
| `--out-dir /tmp/ok/target/debug/deps` | Cargo | everything compiles into `deps/`. Cargo then puts the executable at `target/debug/ok` |
| `-C incremental=…/incremental` | `incremental = true` in the `dev` profile | where `rustc` keeps what it can reuse next time; the `--release` line has none |
| `-L dependency=/tmp/ok/target/debug/deps` | Cargo | where to look for dependencies of dependencies. Direct ones arrive by name: with a path dependency `util` added, the line gained `--extern util=/tmp/ok/target/debug/deps/libutil-0312309ececa22b3.rlib` |

The [rustc book's command-line and codegen chapters ↗](https://doc.rust-lang.org/rustc/codegen-options/index.html) define each flag; the table says why Cargo passes it.

## Paste it yourself

The line is a complete build. Straight after `cargo clean` it fails, because `rustc` does not create the output folder, and the failure itself comes out as JSON:

```text title="Real output — the pasted line after cargo clean, abridged"
{"$message_type":"diagnostic","message":"error writing dependencies to `/tmp/ok/target/debug/deps/ok-864ffab949c86c66.d`: No such file or directory (os error 2)","code":null,"level":"error",…}
```

Make the folder first, and it works:

```text title="Real output — Linux"
$ mkdir -p target/debug/deps
$ <the rustc line>
{"$message_type":"artifact","artifact":"/tmp/ok/target/debug/deps/ok-864ffab949c86c66.d","emit":"dep-info"}
{"$message_type":"artifact","artifact":"/tmp/ok/target/debug/deps/ok-864ffab949c86c66","emit":"link"}
```

Those two lines are the `artifacts` messages. `cmp` found the new `deps/ok-864ffab949c86c66` identical to a copy of Cargo's own build. The pasted line leaves out the rest of Cargo's job: making the folders, and putting the executable at `target/debug/ok`. After a real `cargo build` in the container, `target/debug/ok` and `deps/ok-864ffab949c86c66` were one file under two names (one inode, link count 2). On this Mac they were two separate files.

## Diagnostics as JSON

Why Cargo wants JSON: it has to *count* warnings, and to show them again on a build that compiles nothing. Both are visible with an unused variable in `ok.rs`:

```text title="Real output — cargo build twice, the second with nothing changed, filtered"
   Compiling ok v0.1.0 (/tmp/ok)
warning: unused variable: `x`
warning: `ok` (bin "ok") generated 1 warning (run `cargo fix --bin "ok" -p ok` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
-- again, nothing changed
warning: unused variable: `x`
warning: `ok` (bin "ok") generated 1 warning (run `cargo fix --bin "ok" -p ok` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
```

The second build ran no `rustc`, and there is no `Compiling` line, yet the warning is printed again. Cargo stored the JSON the first time. Run `rustc` by hand with the same two flags and each diagnostic arrives as one JSON object. Its `rendered` field holds the coloured text Cargo prints, starting with the ANSI codes for bold yellow `warning`.

## Check-cfg: a misspelt `cfg` warns only under Cargo

```rust title="ok.rs"
fn main() {
    #[cfg(tset)]
    println!("never");
    println!("OK");
}
```

```text title="Real output — cargo build, Linux"
   Compiling ok v0.1.0 (/tmp/ok)
warning: unexpected `cfg` condition name: `tset`
 --> ok.rs:2:11
  |
2 |     #[cfg(tset)]
  |           ^^^^ help: there is a config with a similar name: `test`
  |
  = help: consider using a Cargo feature instead
  = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
           [lints.rust]
           unexpected_cfgs = { level = "warn", check-cfg = ['cfg(tset)'] }
  = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(tset)");` to the top of the `build.rs`
  = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration
  = note: `#[warn(unexpected_cfgs)]` on by default

warning: `ok` (bin "ok") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
```

`rustc --edition 2024 ok.rs -o /tmp/bare` compiled the same file with no output at all and exit status 0. Without a `--check-cfg` flag `rustc` has no list to check against, so `tset` is quietly false and the `println!` quietly disappears. The same holds for a `cfg(feature = "fast")` in a package with no `fast` feature: under Cargo it warned ``unexpected `cfg` condition value: `fast` ``.

## `-C metadata` and `extra-filename`

`-C metadata` goes into symbol names. The same `ok.rs` built twice by hand on this Mac, once with `-C metadata=aaa` and once with `-C metadata=bbb`, gave `main` two different names:

```text title="Real output — nm, macOS"
00000001000008b0 t __RNvCslWx8eiX81oS_2ok4main
00000001000008b0 t __RNvCs6ixn5cevWKm_2ok4main
```

That is what lets two versions of one crate link into one program without their symbols colliding: [Two versions of one crate](../../two_versions_of_one_crate/README.md) is the case where you need it. `-C extra-filename` only renames the output files, so builds with different settings do not overwrite each other in `deps/`. The book's older Cargo passed one hash for both; 1.98 passes two different ones.

## macOS: `split-debuginfo=unpacked`

On macOS the executable does not hold the debug information. With `CARGO_PROFILE_DEV_SPLIT_DEBUGINFO=off` this Mac's `ok` measured 458,928 bytes, and with Cargo's `unpacked` 458,904. What the setting decides is whether a `.dSYM` bundle gets built:

| Build | `.dSYM` produced |
|---|---|
| `rustc … -C debuginfo=2`, by hand, no split flag | `out/ok.dSYM` |
| `cargo build` (Cargo passes `unpacked`) | none |
| `CARGO_PROFILE_DEV_SPLIT_DEBUGINFO=packed cargo build` | `target/debug/ok.dSYM`, and one in `deps/` |

`unpacked` leaves the debug information in the `.o` files under `deps/`. It skips running `dsymutil`, which is what builds a `.dSYM`. The [Cargo book ↗](https://doc.rust-lang.org/cargo/reference/profiles.html#split-debuginfo) gives `unpacked` as the macOS default for profiles with debug info. On Linux the rustc default, `off`, puts the debug information inside the executable, so Cargo passes no flag there.

## Flags Cargo did not choose

The book's line ends in `-C link-arg=-fuse-ld=lld`, which no Cargo default adds. Flags from `RUSTFLAGS` are appended after Cargo's own:

```text title="Real output — cargo build -v with RUSTFLAGS set to -C link-arg=-fuse-ld=lld, the end of the line"
-L dependency=/tmp/ok/target/debug/deps -C link-arg=-fuse-ld=lld
```

So a flag after `-L dependency=…` in someone's transcript came from their environment or configuration, not from Cargo.

## `-vv`: the environment too

`cargo build -vv` puts the environment in front of the same line. On Linux:

```text title="Real output — the variables in front of rustc under cargo build -vv, one per line"
CARGO=/usr/local/rustup/toolchains/1.98.0-x86_64-unknown-linux-gnu/bin/cargo
CARGO_BIN_NAME=ok
CARGO_CRATE_NAME=ok
CARGO_MANIFEST_DIR=/tmp/ok
CARGO_MANIFEST_PATH=/tmp/ok/Cargo.toml
CARGO_PKG_AUTHORS=''
CARGO_PKG_DESCRIPTION=''
CARGO_PKG_HOMEPAGE=''
CARGO_PKG_LICENSE=''
CARGO_PKG_LICENSE_FILE=''
CARGO_PKG_NAME=ok
CARGO_PKG_README=''
CARGO_PKG_REPOSITORY=''
CARGO_PKG_RUST_VERSION=''
CARGO_PKG_VERSION=0.1.0
CARGO_PKG_VERSION_MAJOR=0
CARGO_PKG_VERSION_MINOR=1
CARGO_PKG_VERSION_PATCH=0
CARGO_PKG_VERSION_PRE=''
CARGO_PRIMARY_PACKAGE=1
LD_LIBRARY_PATH='/tmp/ok/target/debug/deps:/usr/local/rustup/toolchains/1.98.0-x86_64-unknown-linux-gnu/lib'
```

`CARGO_PKG_NAME` is the variable [Running a scratch program](../../../15_First_Programs/rustc_without_cargo/README.md) reads with `option_env!` to tell whether Cargo compiled it. On this Mac the last variable is `DYLD_FALLBACK_LIBRARY_PATH` instead of `LD_LIBRARY_PATH`.

## If you are coming from another language

**C and C++.** `make` prints each command before it runs it, and `make -n` prints the commands without running them. `cargo build -v` is the first of those, and Cargo has no second. The flags map closely: `-C debuginfo=2` is `-g`, `-C opt-level=3` is `-O3`, `--emit=dep-info` is `-MMD`, `-L` is `-L`. `--error-format=json` has a counterpart in `gcc -fdiagnostics-format=json`, which in GCC 14 printed `unused variable 'x'` inside a JSON array. The difference is that a Makefile does nothing with that JSON, while Cargo reads every line of it.

**Python.** Plain `python ok.py` runs no compiler, so there is no command line to show. The nearest thing is `python -v`, which prints a line for every module imported: 47 `import` lines for `python3 -v -c pass` on this Mac. That answers the question the `.d` file answers for Cargo: which files this run depended on.

## See also

- [From one `.rs` file to a Cargo project](../README.md) — the package this line builds, and the three traps on the way to it
- [Makefiles](../../../20_Compilers/makefiles/README.md) — the `.d` file from `--emit=dep-info`, read line by line
- [Compile times](../../compile_times/README.md) — the profile settings behind `debuginfo`, `incremental` and `opt-level`, and what changing them buys
- [Two versions of one crate](../../two_versions_of_one_crate/README.md) — where `-C metadata` earns its place
- [RustRover setup](../../rustrover_setup/README.md) — the warnings replayed from cache, seen from the IDE
- [rustc book — codegen options ↗](https://doc.rust-lang.org/rustc/codegen-options/index.html) · [JSON output ↗](https://doc.rust-lang.org/rustc/json.html) · [check-cfg under Cargo ↗](https://doc.rust-lang.org/rustc/check-cfg/cargo-specifics.html)

## Po polsku

`cargo run -v` (*verbose*) pokazuje polecenie `rustc`, które Cargo naprawdę uruchamia. Dla pakietu bez zależności to jedna linia. Każda flaga ma jedno z trzech źródeł. Pierwsze to ustawienie z `Cargo.toml`: `--crate-name`, `--edition`, ścieżka pliku. Drugie to domyślna wartość profilu (*profile*): `-C debuginfo=2`, `-C incremental`, w trybie `--release` także `-C opt-level=3`. Trzecie to księgowość samego Cargo: `-C metadata`, `-C extra-filename`, `--out-dir`, `-L dependency`. Linia pojawia się tylko wtedy, gdy coś się kompiluje. Zaraz po poprzedniej kompilacji Cargo wypisze w jej miejscu `Fresh`, więc najpierw `cargo clean` albo `touch ok.rs`.

Linię da się wkleić do powłoki i zbudować ten sam program, bajt w bajt. Wcześniej trzeba jednak utworzyć katalog `target/debug/deps`, bo katalogi zakłada Cargo, nie `rustc`. Wtedy też widać, po co Cargo prosi o `--error-format=json`: komunikaty przychodzą jako obiekty JSON. Cargo je liczy („generated 1 warning”) i zapamiętuje, żeby pokazać ostrzeżenia ponownie przy kompilacji, która nic nie kompiluje.

Dwie flagi mają praktyczny skutek, którego nie widać na pierwszy rzut oka. `--check-cfg` sprawia, że literówka w `#[cfg(tset)]` daje ostrzeżenie `unexpected_cfgs`. Goły `rustc` przepuszcza ją bez słowa i kod po prostu znika z programu. `-C split-debuginfo=unpacked`, dodawane tylko na macOS, zostawia informacje dla debugera w plikach `.o` i pomija `dsymutil`, więc nie powstaje pakiet `.dSYM`. Flaga na samym końcu linii, za `-L dependency=…`, taka jak `-C link-arg=-fuse-ld=lld` w *Rust in Action*, pochodzi z `RUSTFLAGS` albo konfiguracji autora, a nie z Cargo.

**Szukaj po polsku:** flagi kompilatora `rustc` · `cargo build verbose rustc flags` · `rustc codegen options` · `rustc error-format json` · `unexpected_cfgs check-cfg` · `split-debuginfo unpacked macos`
