#!/usr/bin/env python3
"""Run every Cargo demo, and hold what it prints to a recorded answer key.

`run_examples.py` compiles one file at a time with bare rustc, and that covers
everything std can show. Some lessons cannot be one file. A procedural macro
has to live in a crate of its own, and the tools everyone writes them with --
`syn`, `quote`, `darling`, `trybuild` -- come from crates.io. Those lessons
keep a small Cargo workspace in a `demo/` folder, and this tool gives their
pages the same promise every other page has: nothing on the page was typed by
hand.

A demo is any folder holding a `cargo_runs.toml`:

    [[run]]
    stem = "three_kinds"                  # unique across the repo
    cmd = ["cargo", "run", "-q", "-p", "app"]

    [[run]]
    stem = "a_derive_cannot_replace"
    cmd = ["cargo", "build", "-q", "-p", "derive_twice"]
    expect = "failure"                    # must exit non-zero; default "success"
    stream = "stderr"                     # "stdout" (default), "stderr", "both"
    env = { RUSTC_BOOTSTRAP = "1" }       # optional
    filters = [["finished in [0-9.]+s", "finished in <elapsed>"]]   # optional

Each run's answer key is `keys/<stem>.out` beside the `cargo_runs.toml`, and a
page shows it with

    <!-- cargo:three_kinds -->
    <!-- /cargo -->

Source files go on the page through `run_examples.py`'s own `<!-- file:path -->`
blocks, so a pasted `lib.rs` cannot drift from the one this tool builds. This
tool refills those too, on the pages of the lessons whose runs it ran, with
`run_examples.py`'s own renderer: a lesson with a demo can be finished without
compiling the whole library's examples, and CI's `run_examples.py --check` still
sees exactly what it would have written.

    python3 tools/run_cargo_demos.py              verify + refill the .md blocks
    python3 tools/run_cargo_demos.py --update     accept current output as the key
    python3 tools/run_cargo_demos.py --check      write nothing; fail on any drift (CI)
    python3 tools/run_cargo_demos.py --only X     a stem, or a folder: every run under it

Four things are pinned so a key recorded on one machine holds on another:

- Every `cargo` command gets `--locked`, so a demo builds from its committed
  `Cargo.lock` and a dependency release cannot change an answer key overnight.
- `CARGO_TARGET_DIR` is shared by every demo (`target/cargo-demos` unless
  `CARGO_DEMOS_TARGET_DIR` says otherwise), so `syn` is compiled once, not once
  per lesson.
- `RUSTFLAGS` and `CARGO_ENCODED_RUSTFLAGS` are set empty. The IDE config that
  `write_cargo_toml.py` puts at the repo root adds `--remap-path-prefix`, and a
  demo inside the repo would otherwise inherit it and print absolute paths.
- `CARGO_TERM_COLOR=never`, so a key never holds an escape code.

Sharing the target directory has one hazard, and the tool refuses it rather
than documenting it: **every package in every demo must have a name no other
demo uses.** Cargo hashes a path package from its name, its path relative to
its workspace and its dependencies, then trusts file times. Two demos that each
hold an `app/` package named `app` with the same dependency names get the same
hash, so the second one's build can be handed the first one's compiled crate
without a word -- which a macro lesson would record as its answer key.

Unlike `run_examples.py` this needs the network the first time, and a minute or
two while the dependencies compile. It runs in CI as its own job.
"""

from __future__ import annotations

import argparse
import os
import re
import subprocess
import sys
import tomllib
from dataclasses import dataclass, field
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from run_examples import BLOCK as EXAMPLE_BLOCK, REPO, fenced_spans, walk  # noqa: E402
from run_examples import rendered_block as rendered_example_block  # noqa: E402

RUNS_FILE = "cargo_runs.toml"

BLOCK = re.compile(
    r"(?P<open><!--\s*cargo:(?P<stem>[A-Za-z0-9_\-]+)\s*-->)"
    r"(?P<body>.*?)"
    r"(?P<close><!--\s*/cargo\s*-->)",
    re.DOTALL,
)


@dataclass
class Run:
    stem: str
    demo: Path
    cmd: list[str]
    expect: str = "success"
    stream: str = "stdout"
    env: dict[str, str] = field(default_factory=dict)
    filters: list[tuple[str, str]] = field(default_factory=list)

    @property
    def key(self) -> Path:
        return self.demo / "keys" / f"{self.stem}.out"

    @property
    def where(self) -> str:
        return f"{(self.demo / RUNS_FILE).relative_to(REPO)} [{self.stem}]"


def find_runs() -> dict[str, Run]:
    runs: dict[str, Run] = {}
    for path in sorted(walk(REPO)):
        if path.name != RUNS_FILE:
            continue
        data = tomllib.loads(path.read_text(encoding="utf-8"))
        for entry in data.get("run", []):
            unknown = set(entry) - {"stem", "cmd", "expect", "stream", "env", "filters"}
            if unknown:
                sys.exit(f"ERROR: {path.relative_to(REPO)}: unknown keys {sorted(unknown)}")
            run = Run(
                stem=entry["stem"],
                demo=path.parent,
                cmd=list(entry["cmd"]),
                expect=entry.get("expect", "success"),
                stream=entry.get("stream", "stdout"),
                env=dict(entry.get("env", {})),
                filters=[(a, b) for a, b in entry.get("filters", [])],
            )
            if run.expect not in ("success", "failure"):
                sys.exit(f"ERROR: {run.where}: expect must be success or failure")
            if run.stream not in ("stdout", "stderr", "both"):
                sys.exit(f"ERROR: {run.where}: stream must be stdout, stderr or both")
            if run.stem in runs:
                sys.exit(
                    f"ERROR: duplicate cargo run stem {run.stem!r}\n"
                    f"  {runs[run.stem].where}\n  {run.where}"
                )
            runs[run.stem] = run
    check_package_names({run.demo for run in runs.values()})
    return runs


def check_package_names(demos: set[Path]) -> None:
    """Exit if two demos contain a package with the same name. See the docstring."""
    seen: dict[str, Path] = {}
    clashes: list[str] = []
    for demo in sorted(demos):
        for manifest in sorted(demo.rglob("Cargo.toml")):
            if "target" in manifest.relative_to(demo).parts:
                continue
            name = tomllib.loads(manifest.read_text(encoding="utf-8")).get("package", {}).get("name")
            if name is None:
                continue
            if name in seen and demo not in seen[name].parents:
                clashes.append(
                    f"{name!r}: {seen[name].relative_to(REPO)} and {manifest.relative_to(REPO)}"
                )
            seen.setdefault(name, manifest)
    if clashes:
        sys.exit(
            "ERROR: demo package names must be unique across the repo, because every "
            "demo shares one target directory:\n  " + "\n  ".join(clashes)
        )


def command(run: Run) -> list[str]:
    cmd = list(run.cmd)
    if cmd and cmd[0] == "cargo" and len(cmd) > 1 and "--locked" not in cmd:
        cmd.insert(2, "--locked")
    return cmd


def execute(run: Run) -> tuple[str | None, str]:
    """Run one command. Returns (output, error); output is None on a wrong exit."""
    env = dict(os.environ)
    env.update(
        CARGO_TARGET_DIR=os.environ.get(
            "CARGO_DEMOS_TARGET_DIR", str(REPO / "target" / "cargo-demos")
        ),
        CARGO_TERM_COLOR="never",
        RUSTFLAGS="",
        CARGO_ENCODED_RUSTFLAGS="",
    )
    env.pop("RUST_BACKTRACE", None)
    env.update(run.env)
    proc = subprocess.run(
        command(run), cwd=run.demo, env=env, capture_output=True, text=True, timeout=900
    )
    failed = proc.returncode != 0
    if failed != (run.expect == "failure"):
        wanted = "to fail" if run.expect == "failure" else "to succeed"
        return None, (
            f"{run.where}: expected {wanted}, exited {proc.returncode}\n"
            f"--- stdout\n{proc.stdout}--- stderr\n{proc.stderr}"
        )
    text = {
        "stdout": proc.stdout,
        "stderr": proc.stderr,
        "both": proc.stdout + proc.stderr,
    }[run.stream]
    for pattern, replacement in run.filters:
        text = re.sub(pattern, replacement, text)
    return text, ""


def rendered_block(run: Run, output: str, page: Path) -> str:
    href = os.path.relpath(run.demo / RUNS_FILE, page.parent)
    shown = " ".join(run.cmd)
    exit_note = ", which fails on purpose" if run.expect == "failure" else ""
    body = output.strip("\n")
    return (
        f"\n*Verified output of `{shown}`{exit_note} — declared in "
        f"[`{RUNS_FILE}`]({href}) and regenerated by `tools/run_cargo_demos.py`, "
        f"never hand-typed.*\n\n"
        f"```text\n{body}\n```\n"
    )


def fill_pages(
    outputs: dict[str, str],
    runs: dict[str, Run],
    write: bool,
    problems: list[str],
    only: set[str] | None,
) -> list[str]:
    drift: list[str] = []
    lessons = {r.demo.parent.resolve() for s, r in runs.items() if s in outputs}
    for page in sorted(walk(REPO)):
        if page.suffix != ".md":
            continue
        text = page.read_text(encoding="utf-8")
        own_files = page.parent.resolve() in lessons and "<!-- file:" in text
        if "<!-- cargo:" not in text and not own_files:
            continue
        skip = fenced_spans(text)

        def replace_file(m: re.Match) -> str:
            if m.group("kind") != "file" or any(lo <= m.start() < hi for lo, hi in skip):
                return m.group(0)
            target = (page.parent / m.group("stem")).resolve()
            if REPO not in target.parents or not target.is_file():
                problems.append(
                    f"{page.relative_to(REPO)}: asks for file block {m.group('stem')!r}, "
                    "but no file inside the repo has that path from this page"
                )
                return m.group(0)
            return m.group("open") + rendered_example_block("file", target, "", page) + m.group("close")

        def replace(m: re.Match) -> str:
            if any(lo <= m.start() < hi for lo, hi in skip):
                return m.group(0)
            stem = m.group("stem")
            if only is not None and stem not in only:
                return m.group(0)
            if stem not in runs:
                problems.append(
                    f"{page.relative_to(REPO)}: asks for cargo block {stem!r}, "
                    f"but no {RUNS_FILE} declares that stem"
                )
                return m.group(0)
            if stem not in outputs:
                return m.group(0)
            return m.group("open") + rendered_block(runs[stem], outputs[stem], page) + m.group("close")

        new = text
        if own_files:
            new = EXAMPLE_BLOCK.sub(replace_file, new)
            # Both closures read `skip` when they run; the fences moved.
            skip = fenced_spans(new)
        new = BLOCK.sub(replace, new)
        if new != text:
            drift.append(str(page.relative_to(REPO)))
            if write:
                page.write_text(new, encoding="utf-8")
    return drift


def resolve_selection(raw: list[str], runs: dict[str, Run]) -> set[str]:
    wanted: set[str] = set()
    unknown: list[str] = []
    for token in (t.strip() for value in raw for t in value.split(",")):
        if not token:
            continue
        if token in runs:
            wanted.add(token)
            continue
        held: set[str] = set()
        for base in (Path(token), REPO / token):
            if base.is_dir():
                folder = base.resolve()
                held = {
                    s for s, r in runs.items()
                    if r.demo.resolve() == folder or folder in r.demo.resolve().parents
                }
                if held:
                    break
        if held:
            wanted |= held
        else:
            unknown.append(token)
    if unknown:
        sys.exit(
            f"ERROR: --only names no such cargo run: {', '.join(unknown)}\n"
            f"Known stems: {', '.join(sorted(runs))}"
        )
    return wanted


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--update", action="store_true", help="record current output as the answer key")
    ap.add_argument("--check", action="store_true", help="write nothing; fail on drift (CI)")
    ap.add_argument(
        "--only",
        action="append",
        metavar="STEM[,STEM…]",
        help="restrict to these run stems, or every run in a folder; repeat or "
        "comma-separate. Everything else is neither run, re-recorded, nor refilled.",
    )
    args = ap.parse_args()

    runs = find_runs()
    if not runs:
        print(f"No Cargo demos found (looked for {RUNS_FILE}).")
        return 0
    selected = resolve_selection(args.only, runs) if args.only else None

    outputs: dict[str, str] = {}
    failures: list[str] = []
    for stem, run in runs.items():
        if selected is not None and stem not in selected:
            continue
        output, error = execute(run)
        if output is None:
            failures.append(error)
            continue
        outputs[stem] = output
        if args.update:
            run.key.parent.mkdir(exist_ok=True)
            run.key.write_text(output, encoding="utf-8")
            print(f"  recorded  {run.key.relative_to(REPO)}")
        elif not run.key.exists():
            failures.append(f"{run.where}: no answer key — run with --update")
        elif run.key.read_text(encoding="utf-8") != output:
            failures.append(f"{run.where}: output differs from {run.key.relative_to(REPO)}")
        else:
            print(f"  ok        {run.where}")

    drift = fill_pages(outputs, runs, write=not args.check, problems=failures, only=selected)
    if args.check and drift:
        failures.append(
            "Markdown cargo blocks are stale: " + ", ".join(drift)
            + " — run tools/run_cargo_demos.py"
        )
    elif drift:
        for page in drift:
            print(f"  filled    {page}")

    if failures:
        print("\nFAILED:", file=sys.stderr)
        for failure in failures:
            print(f"  {failure}", file=sys.stderr)
        return 1
    print(f"\nAll {len(outputs)} cargo runs match their answer keys.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
