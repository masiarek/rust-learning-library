# `fd`, `rg`, `bat`: what the Rust rewrites actually bought

**Level:** 201 · working knowledge

**One line:** On this repository `rg` answers in 0.02 s where `grep -r` needs 1.96 s — and when you decompose that hundredfold gap, the largest single factor is not the Rust engine and not the twelve cores, but a changed default that reads `.gitignore`.

## The three tools

```sh
brew install fd ripgrep bat
```

| Tool | Stands in for | Written in | Version measured here |
|---|---|---|---|
| `fd` | `find` | Rust | 10.3.0 |
| `rg` (ripgrep) | `grep -r` | Rust | 15.1.0 |
| `bat` | `cat` | Rust | 0.26.1 |

Each is a single binary and none of them replaces its ancestor — `find` and `grep` are POSIX, they are on every machine you will ever ssh into, and a script that depends on `rg` is a script that does not run there. These are interactive tools for the machine you sit at.

## The measurement

One query — which files in this repository mention `unwrap` — run five times warm on a 12-core iMac, August 2026.

| Command | Files it searched | Files it reported | Time |
|---|---|---|---|
| `grep -rl unwrap .` | 19,640 | 397 | 1.96 s |
| `rg -uuu -j1 -l unwrap` | 19,640 | 397 | 0.70 s |
| `rg -uuu -l unwrap` | 19,640 | 397 | 0.17 s |
| `rg -l unwrap` | 443 | 146 | 0.02 s |

The four rows are the same question asked with one variable changed each time, so the gaps between them are separable:

| From → to | Factor | What changed |
|---|---|---|
| row 1 → 2 | **2.8×** | the search engine, on one thread, doing identical work |
| row 2 → 3 | **4.1×** | twelve threads instead of one |
| row 3 → 4 | **8.5×** | reading `.gitignore`, so 19,197 files are never opened |
| row 1 → 4 | **98×** | all three, multiplied |

**The biggest single factor has nothing to do with Rust.** Reading `.gitignore` is a product decision; any tool could make it, and `grep` cannot only because thirty years of scripts depend on it not doing so. The engine — SIMD literal search, a regex compiler that refuses to backtrack — is worth 2.8× here, and the parallel directory walk is worth another 4.1×. All three are real, and the one you would have predicted last is the one that dominates.

Three honesty notes, because a benchmark without them is advocacy:

- macOS ships **BSD grep**, which is slow even by the standards of C. GNU grep, which is what you get on Linux, is substantially faster and would narrow that 2.8× — I have not measured by how much, and neither should you believe a number I did not run.
- These are **warm-cache** timings on an external SSD. The first cold run of each was between three and six times slower, and the ratios shift because everything becomes I/O-bound.
- The 397-versus-146 difference is **not `rg` missing things**. The extra 251 files are the built `site/` directory, `.git/` and a Python `.venv/` — all three in `.gitignore`, none of them anything you meant.

## The default is also the trap

`rg` skips what `.gitignore` skips, and it skips hidden files. Most of the time that is exactly right and it is why the tool feels fast. The rest of the time it means **your search silently finds nothing** — in `target/`, in a vendored dependency, in `node_modules`, in a build log, in `.github/`.

There is a ladder of `-u` flags for turning the defaults off, one rung at a time:

| Flag | Adds back |
|---|---|
| `-u` | files ignored by `.gitignore` |
| `-uu` | ...and hidden files and directories |
| `-uuu` | ...and binary files |

`rg -uuu` is `grep -r`. When a search comes back empty and you are certain the string is there, that is the first thing to try — and if `-uuu` finds it, the file was ignored rather than absent, which is usually itself the answer.

`fd` uses the same convention with different spellings: `--no-ignore` for the first rung, `--hidden` for the second, and `-u` as shorthand for both.

## Why this belongs in a Rust library

`rg` is not a program with some Rust in it. It is a set of published crates that happen to ship a binary, and three of them are the answer when your own program needs the same job done:

| Crate | Version today | What it gives you |
|---|---|---|
| [`ignore`](https://docs.rs/ignore) | 0.4.33 | a parallel directory walker that respects `.gitignore`, `.ignore` and global git excludes |
| [`globset`](https://docs.rs/globset) | 0.4.20 | matching many glob patterns at once, compiled into a single automaton |
| [`grep-searcher`](https://docs.rs/grep-searcher) | 0.1.17 | line-oriented searching over a file or a stream |
| [`memchr`](https://docs.rs/memchr) | 2.8.3 | SIMD byte and substring search — the 2.8× in the table above, mostly |

So the row-3-to-row-4 factor is available to you in four lines of `Cargo.toml`, and the row-1-to-row-2 factor is a dependency rather than something to write. That is a more useful thing to know about Rust than any benchmark: the fast parts of the famous programs are **libraries**, versioned and documented, and the binary is a thin shell around them.

The parallel walk is the part worth pausing on. `ignore`'s walker hands paths to a pool of threads while itself descending the tree, which in C is precisely where the bugs live — a shared visited-set, a work queue, a directory handle closed on one thread while another reads it. Rust does not make that code correct, but it makes the wrong version fail to compile, which is why a program of this shape ships as a weekend project rather than a five-year one.

## The odd one out

**fzf is not Rust.** It is Go, and it is the tool on the [previous page](../fuzzy_finding/README.md). This matters only because the four are almost always installed together and described as one thing; they are not, and fzf's design shows it — it is a filter over stdin with no opinion about what produced the lines, which is why pointing it at `fd` is a configuration rather than a feature request.

## See also

- [Fuzzy finding with fzf](../fuzzy_finding/README.md) — the interactive half, and how to make it use `fd` for its file list
- [Adding a dependency](../../05_Tooling/cargo_dependencies/README.md) — `cargo add`, and what a version like `"0.4.33"` actually permits
