# evcxr: no project at all

**Level:** 201 · working knowledge

**One line:** `evcxr` is a Rust REPL — type an expression, see its value, `:dep itertools` to pull a crate into the session — so for "what does this call return" there is no project to create or delete; the price is a four-minute install and a compile behind every line you enter.

## Install and use

```sh
cargo install --locked evcxr_repl      # 3 min 56 s here, evcxr_repl 0.22.0
evcxr
```

```text title="Real output — two expressions piped into evcxr, first run after install"
Welcome to evcxr. For help, type :help
>> let v: Vec<u32> = (1..=5).map(|n| n * n).collect();
>> v.iter().sum::<u32>()
55
                                                          8.850 total
```

```text title="Real output — a crate pulled into the session"
>> :dep itertools
   Compiling either v1.18.0
   Compiling itertools v0.15.0
>> use itertools::Itertools;
>> (1..=5).map(|n| n * n).join("+")
"1+4+9+16+25"
                                                          9.725 total
```

The last line of each transcript is `time` on the whole session: nine seconds for two lines of input. Every entry is compiled as a small crate and linked against the state of the session so far, which is what makes `let v = …` on one line usable on the next — a real Rust REPL, not an interpreter — and is also why each line costs a compile. There is a `:help` with the rest: `:vars` lists the bindings alive in the session, `:t expr` prints a type, `:clear` starts over.

## Where it wins, and where it stops

| Question | evcxr | a project |
|---|---|---|
| what does `"a,b,,c".split(',').count()` return | type it, read `4` | `cargo init`, edit `main.rs`, `cargo run` |
| does this crate's API look the way I think | `:dep`, then try one call | the same, plus a folder to delete |
| I want to write a `fn` and a test | possible, awkward: a REPL has no `#[test]` runner | `cargo test`, already there |
| I want the IDE — types on hover, go-to-definition | no | yes |
| I want to keep what I wrote | copy it out of the terminal | it is a file already |

The line is *expression* versus *program*. A REPL is the right tool for a question whose answer is a value; a project is the right tool the moment the answer is a function. On this machine the crossover is quick, because [`rs`](../shell_function/README.md) gets you a project in two seconds and the REPL takes nine for two lines.

## If you are coming from another language

**Python.** This is the tool that stands where `python3` at the prompt stands, and the surprise is the pause. In Python a REPL line is free; here each is a compile, and the 8.9 s above is what that costs the first time in a session. The trade is the one Rust always makes: what comes back is typed and checked — `:t` on a value prints the real type the compiler inferred, which is a question the Python REPL cannot answer about anything.

**ABAP.** There is no ABAP REPL, and the nearest habit — a throwaway report in SE38 with a `WRITE` — is closer to [`rs`](../shell_function/README.md) than to this. What evcxr adds over that habit is statefulness across lines: the `v` you bound is still there for the next expression, without re-running everything above it.

## See also

- [The hub — a disposable Rust workspace, six ways](../README.md) — the comparison table this page is one row of
- [A shell function](../shell_function/README.md) — the two-second project, for the moment a question becomes a function
- [evcxr on GitHub ↗](https://github.com/evcxr/evcxr) — the REPL, and the Jupyter kernel built on the same engine
