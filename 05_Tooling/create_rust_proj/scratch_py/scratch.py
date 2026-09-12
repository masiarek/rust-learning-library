#!/usr/bin/env python3
"""scratch.py — a disposable Cargo project that deletes itself when the editor closes.

    scratch.py                          # anyhow rand itertools serde serde_json; print the path
    scratch.py --code                   # open in VS Code; deleted when that window closes
    scratch.py --rustrover              # open in RustRover; deleted by the NEXT run (see below)
    scratch.py --crates regex clap      # a different crate list
    scratch.py --no-crates              # std only, no network
    scratch.py --keep                   # never delete this one; print the path and stop

Same project as scratch.sh — mktemp under $TMPDIR, `cargo init`, the two learner
defaults, `cargo add`, a main.rs that uses every crate, one proving `cargo run` — with
the one thing a shell script does badly: a lifetime tied to the editor window.

VS Code can wait (`code --wait`), so with --code the project is removed the moment you
close the window. RustRover cannot be waited on (`open -W` waits for the whole IDE to
quit, not for one project window), so with --rustrover the project is registered in
~/.cache/rust-scratch/registry and deleted the next time this script runs. Either way
nothing accumulates: what you did not --keep is gone within one session.

Stdlib only. Python 3.11+.
"""

from __future__ import annotations

import argparse
import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

DEFAULT_CRATES = ["anyhow", "rand", "itertools", "serde", "serde_json"]
REGISTRY = Path.home() / ".cache" / "rust-scratch" / "registry"

LINTS = """
[lints.rust]
unused_variables = "allow"
unused_imports = "allow"
unused_mut = "allow"
dead_code = "allow"
"""

MAIN_DEFAULT = '''//! Scratch — deleted when you close the editor, or on the next run.

use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() -> anyhow::Result<()> {
    let n: u8 = rand::random();
    dbg!(n);

    let evens = (1..=10).filter(|n| n % 2 == 0).join(", ");
    println!("{evens}"); // 2, 4, 6, 8, 10

    let p = Point { x: 3, y: 4 };
    let json = serde_json::to_string(&p)?;
    println!("{json}"); // {"x":3,"y":4}
    let back: Point = serde_json::from_str(&json)?;
    assert_eq!(p, back);
    Ok(())
}
'''

MAIN_PLAIN = '''//! Scratch — deleted when you close the editor, or on the next run.

fn main() {
    println!("{}", 6 * 7); // 42
}
'''


def run(*cmd: str, cwd: Path | None = None) -> None:
    subprocess.run(cmd, cwd=cwd, check=True)


def prune_registered() -> None:
    """Delete every project an earlier --rustrover run left for us."""
    if not REGISTRY.exists():
        return
    keep: list[str] = []
    for line in REGISTRY.read_text().splitlines():
        p = Path(line)
        if p.is_dir():
            shutil.rmtree(p, ignore_errors=True)
            print(f"  pruned      {p}")
    REGISTRY.write_text("\n".join(keep))


def make_project(crates: list[str]) -> Path:
    tmp = Path(os.environ.get("TMPDIR", "/tmp"))
    d = Path(tempfile.mkdtemp(prefix="rust-scratch.", dir=tmp))
    run("cargo", "init", "--quiet", "--name", "scratch", "--vcs", "none", "--edition", "2024", cwd=d)
    print(f"  cargo init  {d}")

    manifest = d / "Cargo.toml"
    manifest.write_text(manifest.read_text() + LINTS)
    (d / ".cargo").mkdir()
    (d / ".cargo" / "config.toml").write_text(f'[build]\nrustflags = ["--remap-path-prefix=={d}/"]\n')
    print("  wrote       [lints.rust] + .cargo/config.toml")

    if crates:
        extra = ["--features", "serde/derive"] if "serde" in crates else []
        run("cargo", "add", "--quiet", *crates, *extra, cwd=d)
        print(f"  cargo add   {' '.join(crates)}")

    (d / "src" / "main.rs").write_text(MAIN_DEFAULT if crates == DEFAULT_CRATES else MAIN_PLAIN)
    run("cargo", "run", "--quiet", cwd=d)
    print("  cargo run   ok")
    return d


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("--crates", nargs="*", default=DEFAULT_CRATES, metavar="CRATE")
    ap.add_argument("--no-crates", action="store_true")
    g = ap.add_mutually_exclusive_group()
    g.add_argument("--code", action="store_true", help="open in VS Code; delete when the window closes")
    g.add_argument("--rustrover", action="store_true", help="open in RustRover; delete on the next run")
    ap.add_argument("--keep", action="store_true", help="never delete this project")
    a = ap.parse_args()

    prune_registered()
    d = make_project([] if a.no_crates else a.crates)

    if a.code:
        print(f"\n  code --wait {d}   (close the window to delete it)")
        subprocess.run(["code", "--wait", "--new-window", str(d)])
        if a.keep:
            print(f"\ncd {d}")
        else:
            shutil.rmtree(d, ignore_errors=True)
            print("  deleted")
        return 0

    if a.rustrover:
        subprocess.run(["open", "-a", "RustRover", str(d)], check=True)
        if not a.keep:
            REGISTRY.parent.mkdir(parents=True, exist_ok=True)
            with REGISTRY.open("a") as f:
                f.write(f"{d}\n")
            print("  registered  deleted on the next scratch.py run")

    print(f"\ncd {d}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
