# scratch.py: the same project, deleted when the window closes

**Level:** 201 · working knowledge

**One line:** The Python version of [`scratch.sh`](../scratch_sh/README.md) makes the same project — mktemp, `cargo init`, the two learner defaults, five crates, a proving `cargo run` — and adds the one thing a shell script does badly: with `--code` the project is removed the moment you close the VS Code window, and with `--rustrover`, which cannot be waited on, it is removed by the next run.

## Run it

[`scratch.py` ↗](https://github.com/masiarek/rust-learning-library/blob/master/05_Tooling/create_rust_proj/scratch_py/scratch.py) — stdlib only, Python 3.11+.

```sh
scratch.py                          # anyhow rand itertools serde serde_json; print the path
scratch.py --code                   # open in VS Code; deleted when that window closes
scratch.py --rustrover              # open in RustRover; deleted by the NEXT run
scratch.py --crates regex clap      # a different crate list
scratch.py --no-crates              # std only, no network
scratch.py --keep                   # never delete this one
```

```text title="Real output — scratch.py, default crates"
$ time python3 scratch.py
  cargo init  /var/folders/bc/gzwjtz8d6mx3l2f9yyf_p8yw0000gn/T/rust-scratch.o6u5i_mt
  wrote       [lints.rust] + .cargo/config.toml
  cargo add   anyhow rand itertools serde serde_json
[/var/folders/bc/gzwjtz8d6mx3l2f9yyf_p8yw0000gn/T/rust-scratch.o6u5i_mt/src/main.rs:14:5] n = 91
2, 4, 6, 8, 10
{"x":3,"y":4}
  cargo run   ok

cd /var/folders/bc/gzwjtz8d6mx3l2f9yyf_p8yw0000gn/T/rust-scratch.o6u5i_mt
python3 scratch.py  10.828 total
```

Same five steps as the shell script, same eleven seconds, same folder shape; [that page](../scratch_sh/README.md) explains each step and does not need repeating. What follows is the part that is different.

## Two lifetimes, because two editors

**VS Code can wait.** `code --wait --new-window <dir>` returns when that window is closed, so `--code` is a straight line:

```python
subprocess.run(["code", "--wait", "--new-window", str(d)])
shutil.rmtree(d, ignore_errors=True)
```

Close the window, the project is gone. `--keep` skips the second line.

**RustRover cannot.** `open -W -a RustRover <dir>` blocks *"until the used applications are closed (even if they were already running)"* — measured on this machine: with the IDE open on the project for a minute, the call had not returned, and it returns only when RustRover quits. Tying deletion to that would mean *quitting the IDE deletes your scratch*, which is a surprise nobody wants twice. So `--rustrover` records the path instead:

```python
subprocess.run(["open", "-a", "RustRover", str(d)], check=True)
with REGISTRY.open("a") as f:      # ~/.cache/rust-scratch/registry
    f.write(f"{d}\n")
```

and every run begins by deleting whatever the registry lists. The invariant is the same in both cases — *what you did not `--keep` is gone within one session* — reached by the tightest hook each editor offers.

## Why Python here and shell there

The shell script is the thing you can read top to bottom in one screen, and for a five-step recipe that is the right tool. The Python version exists for the branch in the paragraph above, plus the two things `argparse` does for free that the shell version does by hand: `--crates regex clap` as a real list rather than a quoted string, and a mutually exclusive `--code` / `--rustrover`. Nothing in it needs a library, and it runs on the `python3` macOS ships.

If you are coming from Python, the `subprocess.run([...], check=True)` calls are the whole of the "shelling out" story: `cargo` is a program, the script runs it, `check=True` turns a non-zero exit into an exception — the same `set -e` contract the shell version has.

## See also

- [The hub — a disposable Rust workspace, six ways](../README.md) — the comparison table this page is one row of
- [`scratch.sh`](../scratch_sh/README.md) — the five steps, explained, and what the crates cost
- [Opening it in the IDE](../open_in_the_ide/README.md) — `open -a`, `open -W`, `code --wait`, and the launcher on `PATH` that points at a disk that is not there
- [Scaffolding a practice tree](../../scaffolding/README.md) — `rust_scaffold.py`, the Python script for the *permanent* tree, which this one borrows its defaults from
