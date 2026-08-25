#!/usr/bin/env python3
"""Scaffold a Rust practice tree — the files that have no owner.

    python3 rust_scaffold.py init ~/rust-practice
    python3 rust_scaffold.py new ex01_hello --workspace ~/rust-practice
    python3 rust_scaffold.py adopt ~/RustroverProjects/untitled
    python3 rust_scaffold.py doctor ~/rust-practice

The design rule, which is the whole reason this is short
--------------------------------------------------------
Three things can write files into a Rust project, and only one of them is missing:

    cargo new      the package — Cargo.toml, src/main.rs, .gitignore, a git repo,
                   and (inside a workspace that declares lints) the opt-in line
    the workspace  the compiler, the lint policy and the dependency versions,
                   SHARED from the root with every member, forever
    nobody         the root itself — and everything under .idea/

This script writes the third column and refuses to write the first. `new` shells
out to real `cargo new` instead of templating a package, because a template
*copies* configuration and a workspace *shares* it: forty generated folders are
forty copies of a decision you will change next week, and the ones you wrote in
March become a fossil of what you believed in March.

So there is no template directory here, and no placeholder expansion. What is
left is genuinely unowned: six files at the top of the tree that you write once,
get subtly wrong, and cannot remember six months later — plus `.idea/`, which no
Cargo mechanism can reach at all, which is why this script keeps earning its
place after the tree exists.

Everything it writes is idempotent: an existing file is kept, not clobbered,
unless you pass --force. Re-running it on a live tree is safe and is the
intended way to pick up a new run configuration.

Stdlib only. No dependencies, on purpose — a bootstrap script that needs
bootstrapping is not one.
"""

from __future__ import annotations

import argparse
import re
import shutil
import subprocess
import sys
from dataclasses import dataclass, field
from pathlib import Path

# ---------------------------------------------------------------------------
# Decisions, in one place, so a reader can disagree with them in one place.
# ---------------------------------------------------------------------------

# A DATED nightly, not `nightly`. Bare "nightly" is a manifest with no lockfile:
# it looks like a pin and moves every night, and rustup silently falls back to an
# older one when a component failed to build, so two machines running the same
# command on the same day can get different compilers. Bumping this is meant to
# be a deliberate commit, not a background process.
DEFAULT_CHANNEL = "nightly-2026-08-25"

# rust-analyzer is not optional even though it is an editor concern: with
# profile = "minimal" and no rust-analyzer component, the rustup shim reports
# `rust-analyzer is unavailable for the active toolchain`, falls back to ITSELF,
# and loops — "error: infinite recursion detected", with no useful reason shown
# in the editor. rust-src is what gives you Go-to-definition into std.
COMPONENTS = ["rustfmt", "clippy", "rust-analyzer", "rust-src"]

# Kept deliberately short. Reaching for `itertools` before you have felt why
# `Iterator` was not enough teaches you the crate instead of the language. These
# four are the ones that earn their place early: errors, arguments, data, and the
# crate every tutorial reaches for first.
DEFAULT_DEPS = {
    "anyhow": '"1"',
    "clap": '{ version = "4", features = ["derive"] }',
    "serde": '{ version = "1", features = ["derive"] }',
    "serde_json": '"1"',
    "rand": '"0.10"',
}

# Two lint profiles. `learn` is clippy's own teaching half at warn level; `strict`
# adds the restriction lints that forbid a panic outright. The difference is not
# strictness for its own sake — `deny` on `nursery` is a BUILD FAILURE on code
# with no defect, which is a bad way to meet a suggestion while you are learning.
LINT_PROFILES = {
    "none": "",
    "learn": """[workspace.lints.clippy]
# The two teaching groups, at `warn`: clippy explains the idiom the standard
# library expects without refusing to build. `priority = -1` is load-bearing —
# a group and one of its members conflict by construction, so the negative
# priority lands the groups first and lets a single line below override them.
# Drop it and Cargo rejects the manifest rather than guessing.
pedantic = { level = "warn", priority = -1 }
nursery = { level = "warn", priority = -1 }
""",
    "strict": """[workspace.lints.clippy]
pedantic = { level = "deny", priority = -1 }
nursery = { level = "deny", priority = -1 }
# The panic policy: `restriction` lints, which forbid something legal and
# idiomatic because this codebase has decided against it. Clippy is not calling
# `unwrap` a bug — you are declaring that this program may not abort.
unwrap_used = "deny"
expect_used = "deny"
indexing_slicing = "deny"
panic = "deny"
todo = "deny"
unimplemented = "deny"
unreachable = "deny"
exit = "deny"
# Deliberately NOT here: arithmetic_side_effects. It fires on `n + 1` between
# two integers, so adopting it means checked_add/saturating_add throughout —
# a much larger change to how code reads than the rest of this block together.
""",
}


# The half of rustc's `unused` group that is pure noise in throwaway code. A
# scratch file exists to show a form — five ways to make a String, each bound to a
# name nobody reads — and `unused_variables` fires on every one of them, which
# teaches a beginner that warnings are wallpaper. That is a worse habit than any of
# these four lints prevents.
#
# `unused_must_use` is deliberately NOT here. It is in the same group and reads as
# the same kind of pedantry, but it fires on an ignored `Result` — a dropped error,
# not an unread name. Allowing the group wholesale with `unused = "allow"` would
# take it out too, which is why this is four lines instead of one.
SCRATCH_RUST_LINTS = """[workspace.lints.rust]
unused_variables = "allow"
unused_imports = "allow"
unused_mut = "allow"
dead_code = "allow"
"""


@dataclass
class Plan:
    """What a run would write, so --dry-run and the real run share one list."""

    root: Path
    files: list[tuple[Path, str]] = field(default_factory=list)

    def add(self, relative: str, body: str) -> None:
        self.files.append((self.root / relative, body.rstrip("\n") + "\n"))


# ---------------------------------------------------------------------------
# The files
# ---------------------------------------------------------------------------


def toolchain_toml(channel: str) -> str:
    components = ", ".join(f'"{c}"' for c in COMPONENTS)
    return f"""# The compiler every command in this tree gets, including the IDE's.
#
# rustup reads this on any invocation that goes through its shim and installs the
# named toolchain if it is missing, so there is no command for anyone to remember
# — the file is the instruction. That is also how CI installs it: `rustup show`
# as the first step is enough. A rustc that is NOT a rustup shim (a distro
# package, a Nix store path) ignores this completely, which is the usual
# explanation for a pin that appears not to apply.

[toolchain]
channel = "{channel}"
profile = "minimal"
components = [{components}]
"""


def workspace_cargo_toml(lints: str, deps: bool, allow_unused: bool) -> str:
    out = """# The workspace root. Nothing is built from here; everything is inherited.
#
# `members` is a GLOB, so a new directory under exercises/ is a member the moment
# it exists — which is why adding an exercise is `cargo new` and nothing else.

[workspace]
resolver = "3"
members = ["exercises/*"]

[workspace.package]
edition = "2024"
"""
    if lints:
        out += "\n" + lints
    if allow_unused:
        out += "\n" + SCRATCH_RUST_LINTS
    if lints or allow_unused:
        out += """
# Cargo writes `[lints] workspace = true` into a new member's manifest by itself,
# because this table exists. That is the line a template would have had to copy —
# and the line a member created BEFORE the table existed will not have, which is
# what `doctor` checks. Without it a member inherits none of the above, silently.
"""
    if deps:
        body = "\n".join(f"{name} = {spec}" for name, spec in DEFAULT_DEPS.items())
        out += f"""
# A version registry, not an automatic `use`: a member still declares what it
# actually uses, with `anyhow = {{ workspace = true }}`. One place to bump, and no
# two exercises can silently sit on different versions of the same crate.
[workspace.dependencies]
{body}
"""
    return out


def cargo_config_toml(root: Path) -> str:
    """The `.cargo/config.toml` that makes diagnostics name the file outright.

    rustc prints a span RELATIVE TO ITS OWN WORKING DIRECTORY, and Cargo sets that
    to the workspace root no matter where you invoked it from — so every project
    on the machine reports `--> src/main.rs:5:9` and two IDE windows say the same
    thing about different files.
    """
    return f"""# Absolute paths in compiler, clippy and rustfmt output.
#
# Without this a warning reads `--> src/main.rs:5:9`, because rustc prints a span
# relative to its own working directory and Cargo sets that to the workspace root
# — the same seven characters for every project you have open. With it the span
# names the file outright and stays clickable from any terminal, not just from one
# opened at the right directory.
#
# The DOUBLE `==` is not a typo and is the whole trick. `--remap-path-prefix=OLD=NEW`
# here has an empty OLD, and the match is by PATH COMPONENT rather than by text: an
# empty prefix matches the start of every RELATIVE path and no absolute one at all.
# So your files get spelled out and the registry's and std's — already absolute —
# pass through untouched. A trailing slash is optional for the same reason.
#
# THE PATH BELOW IS THIS DIRECTORY, spelled out because rustflags interpolate
# nothing. Move or rename the tree and every diagnostic will point at where it used
# to be, which is worse than a relative path, not better — so this file is
# gitignored, and `rust_scaffold.py doctor` fails when the two disagree.
#
# Two side effects, both arguably improvements: `file!()` and panic locations become
# absolute too, so a backtrace names a file you can open. If you record a program's
# output as an answer key, record it after adding this.
#
# Note that a `RUSTFLAGS` environment variable REPLACES this wholesale rather than
# adding to it, which is the usual reason it appears not to apply.

[build]
rustflags = ["--remap-path-prefix=={root}/"]
"""


CLIPPY_TOML = """# The carve-out that makes a strict lint policy liveable: prototype with `unwrap`
# inside a unit test and clippy stays quiet, while the same `unwrap` in `main` is
# still an error. Without this, a strict policy makes tests painful to write and
# gets abandoned within a week.
allow-unwrap-in-tests = true
allow-expect-in-tests = true
allow-panic-in-tests = true
allow-indexing-slicing-in-tests = true
"""


def rustfmt_toml(nightly: bool) -> str:
    out = """# Every entry point — `cargo fmt`, the IDE, CI — shells out to the same rustfmt
# binary, so this file is the single answer to the whitespace question.
max_width = 100
"""
    if nightly:
        out += """
# NIGHTLY-ONLY BELOW THIS LINE.
#
# rustfmt applies these on nightly and SKIPS them with a warning on stable. That
# asymmetry is the trap: a nightly laptop formats one way, a stable CI runner
# checks for another, and `cargo fmt --check` fails on a diff the author cannot
# reproduce. They are safe here only because rust-toolchain.toml pins nightly for
# everyone, laptop and CI alike. Change that channel to stable and delete these
# four lines in the same commit — `rust_scaffold.py doctor` fails if you do not.
group_imports = "StdExternalCrate"
imports_granularity = "Crate"
wrap_comments = true
format_code_in_doc_comments = true
"""
    return out


GITIGNORE = """/target
# Machine-specific: it hard-codes this tree's absolute path. See the file.
/.cargo/config.toml
/.idea/workspace.xml
/.idea/shelf/
.DS_Store
"""

BACON_TOML = """# `bacon` re-runs a job on every save in a pane you leave open. The job below is
# the one worth defaulting to: clippy over the tests as well, which is where the
# clippy.toml carve-outs are supposed to be proving themselves.

default_job = "clippy-all"

[jobs.clippy-all]
command = ["cargo", "clippy", "--all-targets", "--all-features", "--color", "always"]
need_stdout = false

[jobs.test]
command = ["cargo", "test", "--workspace", "--color", "always"]
need_stdout = true
"""

EDITORCONFIG = """# The IDE reads this natively; rustfmt does not read it at all. So the one number
# that appears in both files has to be kept equal by hand — that is what the
# doctor subcommand checks. Everything else here is for the files rustfmt never
# sees: the TOML, the YAML, this repo's Markdown.
root = true

[*]
charset = utf-8
end_of_line = lf
insert_final_newline = true
trim_trailing_whitespace = true
indent_style = space

[*.rs]
indent_size = 4
max_line_length = 100

[*.{toml,yml,yaml,json}]
indent_size = 2

[*.md]
trim_trailing_whitespace = false
"""

CI_WORKFLOW = """name: rust

on:
  push:
  pull_request:

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      # No toolchain action and no version here on purpose: rust-toolchain.toml
      # is the single source of truth, and `rustup show` installs what it names.
      # A version in this file would be a second place to bump, which is how a
      # laptop and a runner drift apart.
      - run: rustup show
      - run: cargo fmt --all -- --check
      - run: cargo clippy --all-targets --all-features -- -D warnings
      - run: cargo test --workspace
"""

IDEA_GITIGNORE = """# Default ignored files, as JetBrains writes them: the per-user half of .idea/
# stays out of git while the shared half — run configurations, code style — goes in.
/shelf/
/workspace.xml
/httpRequests/
/tasks.xml
/usage.statistics.xml
/dictionaries/
"""

IDEA_VCS_XML = """<?xml version="1.0" encoding="UTF-8"?>
<project version="4">
  <component name="VcsDirectoryMappings">
    <mapping directory="$PROJECT_DIR$" vcs="Git" />
  </component>
</project>
"""


def run_config_xml(name: str, command: str, emulate_terminal: bool = True) -> str:
    """One RustRover run configuration.

    The `type` and `factoryName` are not guessed: they were read out of
    RustRover 2026.2.1's own intellij.rustrover.core.jar, where
    CargoCommandConfigurationType passes the id "CargoCommandRunConfiguration"
    to ConfigurationTypeBase and CargoConfigurationFactory.getId() returns
    "Cargo Command". Get either string wrong and the configuration simply does
    not appear in the dropdown — the IDE reports nothing.
    """
    return f"""<?xml version="1.0" encoding="UTF-8"?>
<component name="ProjectRunConfigurationManager">
  <configuration default="false" name="{name}" type="CargoCommandRunConfiguration" factoryName="Cargo Command">
    <option name="command" value="{command}" />
    <option name="workingDirectory" value="file://$PROJECT_DIR$" />
    <option name="emulateTerminal" value="{str(emulate_terminal).lower()}" />
    <method v="2" />
  </configuration>
</component>
"""


ROOT_RUN_CONFIGS = [
    ("Clippy (all targets)", "clippy --all-targets --all-features"),
    ("Test (workspace)", "test --workspace"),
    ("Format check", "fmt --all -- --check"),
]


def readme(root_name: str, channel: str) -> str:
    return f"""# {root_name}

A Cargo workspace for small practice projects. Add one with:

```sh
cargo new exercises/ex02_options
```

That is the whole ritual — the compiler (`{channel}`), the lint policy and the
dependency versions are inherited from this directory, and Cargo writes the
member's `[lints] workspace = true` opt-in itself.

```sh
cargo run -p ex02_options      # -p is the one real tax of a workspace
cargo clippy --all-targets     # the tests too, which is where the carve-outs live
cargo fmt --all
cargo test --workspace
bacon                          # the same clippy job, re-run on every save
```

| File | What it decides |
|---|---|
| `rust-toolchain.toml` | which compiler every command here gets, including the IDE's |
| `Cargo.toml` | the member glob, the lint policy, the dependency version registry |
| `clippy.toml` | the tests carve-out that makes the lint policy liveable |
| `rustfmt.toml` | the whitespace answer, for `cargo fmt`, the IDE and CI alike |
| `.editorconfig` | the same width, for the files rustfmt never sees |
| `.cargo/config.toml` | absolute paths in compiler output, so a warning names the file |
| `bacon.toml` | the job re-run on every save |
| `.idea/` | run configurations, so the dropdown is populated on first open |

Written by `rust_scaffold.py`. Re-running it is safe: existing files are kept.
"""


# ---------------------------------------------------------------------------
# Writing
# ---------------------------------------------------------------------------


def build_plan(args: argparse.Namespace) -> Plan:
    root = Path(args.path).expanduser().resolve()
    plan = Plan(root)
    nightly = args.channel.startswith("nightly")

    plan.add("rust-toolchain.toml", toolchain_toml(args.channel))
    plan.add(
        "Cargo.toml",
        workspace_cargo_toml(LINT_PROFILES[args.lints], not args.no_deps, not args.warn_unused),
    )
    plan.add("rustfmt.toml", rustfmt_toml(nightly))
    plan.add(".gitignore", GITIGNORE)
    plan.add(".editorconfig", EDITORCONFIG)
    plan.add("README.md", readme(root.name, args.channel))
    if args.lints != "none":
        plan.add("clippy.toml", CLIPPY_TOML)
    if not args.no_abs_paths:
        plan.add(".cargo/config.toml", cargo_config_toml(root))
    if not args.no_bacon:
        plan.add("bacon.toml", BACON_TOML)
    if not args.no_ci:
        plan.add(".github/workflows/rust.yml", CI_WORKFLOW)
    if not args.no_idea:
        plan.add(".idea/.gitignore", IDEA_GITIGNORE)
        plan.add(".idea/vcs.xml", IDEA_VCS_XML)
        for name, command in ROOT_RUN_CONFIGS:
            plan.add(f".idea/runConfigurations/{slug(name)}.xml", run_config_xml(name, command))
    return plan


def slug(name: str) -> str:
    return re.sub(r"[^A-Za-z0-9]+", "_", name).strip("_")


def apply(plan: Plan, *, force: bool, dry_run: bool) -> int:
    written = kept = 0
    for path, body in plan.files:
        rel = path.relative_to(plan.root)
        if path.exists() and not force:
            print(f"  kept   {rel}")
            kept += 1
            continue
        if not dry_run:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(body, encoding="utf-8")
        verb = "would write" if dry_run else "wrote"
        print(f"  {verb:<6} {rel}")
        written += 1
    print(f"\n{written} written, {kept} kept.")
    return 0


# ---------------------------------------------------------------------------
# Subcommands
# ---------------------------------------------------------------------------


def cmd_init(args: argparse.Namespace) -> int:
    plan = build_plan(args)
    print(f"{'Would scaffold' if args.dry_run else 'Scaffolding'} {plan.root}\n")
    if not args.dry_run:
        (plan.root / "exercises").mkdir(parents=True, exist_ok=True)
    rc = apply(plan, force=args.force, dry_run=args.dry_run)
    if not args.dry_run and not args.no_git and not (plan.root / ".git").exists():
        subprocess.run(["git", "init", "-q", str(plan.root)], check=False)
        print("Initialised a git repository.")
    if not args.dry_run:
        print(f"\nNext:  python3 {Path(__file__).resolve()} new ex01_hello --workspace {plan.root}")
    return rc


def cmd_new(args: argparse.Namespace) -> int:
    root = Path(args.workspace).expanduser().resolve()
    if not (root / "Cargo.toml").exists():
        print(f"error: no Cargo.toml in {root} — run `init` there first", file=sys.stderr)
        return 1
    target = root / "exercises" / args.name
    if target.exists():
        print(f"error: {target} already exists", file=sys.stderr)
        return 1

    # Real cargo new, not a template. It writes the package AND the workspace
    # lint opt-in; anything this script wrote instead would be a copy that rots.
    kind = "--lib" if args.lib else "--bin"
    proc = subprocess.run(["cargo", "new", kind, str(target)], cwd=root)
    if proc.returncode != 0:
        return proc.returncode

    manifest = (target / "Cargo.toml").read_text(encoding="utf-8")
    if "workspace = true" in manifest:
        print("  cargo wrote the [lints] workspace opt-in itself")
    else:
        print("  note: no [lints] opt-in written — the root declares no [workspace.lints]")

    if not args.no_tests and not args.lib:
        seed_tests(target / "src" / "main.rs", args.name)
        print("  seeded src/main.rs with a #[cfg(test)] module")

    if not args.no_idea:
        cfg = root / ".idea" / "runConfigurations" / f"Run_{slug(args.name)}.xml"
        cfg.parent.mkdir(parents=True, exist_ok=True)
        cfg.write_text(run_config_xml(f"Run {args.name}", f"run -p {args.name}"), encoding="utf-8")
        print(f"  wrote {cfg.relative_to(root)}")

    print(f"\ncargo run -p {args.name}")
    return 0


# The seed is `const fn` and backticks its own doc comment because the `learn`
# profile above catches both otherwise — `missing_const_for_fn` (nursery) and
# `doc_markdown` (pedantic). A scaffolder whose own output trips its own lint
# policy teaches the reader to ignore warnings on their first run, which is the
# opposite of the point. Verified by running clippy on what this writes.
SEED = '''//! `{name}`

const fn answer() -> u32 {{
    42
}}

fn main() {{
    println!("{{}}", answer());
}}

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn the_answer_is_the_answer() {{
        assert_eq!(answer(), 42);
    }}
}}
'''


def seed_tests(main_rs: Path, name: str) -> None:
    main_rs.write_text(SEED.format(name=name), encoding="utf-8")


def cmd_adopt(args: argparse.Namespace) -> int:
    """Give an EXISTING package the two scratch defaults.

    `init` covers trees this script made. The throwaway projects are the ones it
    did not: RustRover's New Project dialog runs a plain `cargo new`, so every
    `untitled2` starts out printing `--> src/main.rs` and warning about the
    bindings a scratch file exists to demonstrate. This is that fix, applied to a
    directory that already has a Cargo.toml.
    """
    root = Path(args.path).expanduser().resolve()
    manifest = root / "Cargo.toml"
    if not manifest.exists():
        print(f"error: no Cargo.toml in {root}", file=sys.stderr)
        return 1

    body = manifest.read_text(encoding="utf-8")
    workspace = "[workspace]" in body and "[package]" not in body
    changed = 0

    cfg = root / ".cargo" / "config.toml"
    if cfg.exists() and not args.force:
        print("  kept   .cargo/config.toml")
    else:
        cfg.parent.mkdir(parents=True, exist_ok=True)
        cfg.write_text(cargo_config_toml(root), encoding="utf-8")
        print("  wrote  .cargo/config.toml — diagnostics now name the file")
        changed += 1

    # The table name differs by shape, and getting it wrong is silent: a
    # `[workspace.lints]` table in a non-workspace manifest is simply never read.
    table = "[workspace.lints.rust]" if workspace else "[lints.rust]"
    if "lints.rust]" in body:
        print(f"  kept   {table} (already present)")
    else:
        block = SCRATCH_RUST_LINTS.replace("[workspace.lints.rust]", table)
        manifest.write_text(body.rstrip("\n") + "\n\n" + block, encoding="utf-8")
        print(f"  wrote  {table} in Cargo.toml — unused bindings no longer warn")
        changed += 1
        if workspace:
            print("  note: members need `[lints] workspace = true` to inherit it")

    gitignore = root / ".gitignore"
    line = "/.cargo/config.toml"
    existing = gitignore.read_text(encoding="utf-8") if gitignore.exists() else ""
    if line not in existing:
        with gitignore.open("a", encoding="utf-8") as fh:
            fh.write(f"\n# Machine-specific: it hard-codes this tree's absolute path.\n{line}\n")
        print("  wrote  .gitignore entry (the remap names THIS directory)")
        changed += 1

    if changed:
        print(f"\n{changed} change(s). The next build recompiles: rustflags are part of the fingerprint.")
    else:
        print("\nNothing to do — already adopted.")
    return 0


def cmd_doctor(args: argparse.Namespace) -> int:
    root = Path(args.path).expanduser().resolve()
    problems: list[str] = []

    def check(ok: bool, good: str, bad: str) -> None:
        print(f"  {'✓' if ok else '✗'} {good if ok else bad}")
        if not ok:
            problems.append(bad)

    print(f"Checking {root}\n")

    check(shutil.which("rustup") is not None, "rustup on PATH", "rustup not on PATH")
    check(shutil.which("cargo") is not None, "cargo on PATH", "cargo not on PATH")

    toolchain_file = root / "rust-toolchain.toml"
    channel = ""
    if toolchain_file.exists():
        text = toolchain_file.read_text(encoding="utf-8")
        m = re.search(r'^\s*channel\s*=\s*"([^"]+)"', text, re.M)
        channel = m.group(1) if m else ""
        check(bool(channel), f"rust-toolchain.toml pins {channel}", "rust-toolchain.toml has no channel")
        check(
            channel not in ("stable", "beta", "nightly"),
            "the pin names a version, not a moving channel",
            f'channel = "{channel}" moves under you — pin a version or a dated nightly',
        )
    else:
        check(False, "", "no rust-toolchain.toml — every machine gets its own compiler")

    active = run_capture(["rustup", "show", "active-toolchain"], cwd=root)
    if active:
        print(f"  · active toolchain here: {active.splitlines()[0]}")
        if channel:
            check(
                active.startswith(channel),
                "the active toolchain is the pinned one",
                f"active toolchain is not {channel} — is this rustc a rustup shim?",
            )

    installed = run_capture(["rustup", "component", "list", "--installed"] + (["--toolchain", channel] if channel else []), cwd=root)
    for component in COMPONENTS:
        check(
            component in installed,
            f"component {component}",
            f"component {component} missing — rustup component add {component}",
        )

    # The check that pays for the whole subcommand: unstable rustfmt options are
    # APPLIED on nightly and SKIPPED with a warning on stable, so this pair drifts
    # silently and only shows up as a CI diff nobody can reproduce locally.
    fmt_file = root / "rustfmt.toml"
    if fmt_file.exists():
        body = fmt_file.read_text(encoding="utf-8")
        unstable = sorted(o for o in NIGHTLY_ONLY_FMT_OPTIONS if re.search(rf"^\s*{o}\s*=", body, re.M))
        if unstable:
            check(
                channel.startswith("nightly"),
                f"rustfmt.toml's nightly-only options are backed by a nightly pin ({', '.join(unstable)})",
                f"rustfmt.toml sets nightly-only options ({', '.join(unstable)}) but the channel is "
                f'"{channel}" — they are silently ignored, so CI and your editor will disagree',
            )
        width = re.search(r"^\s*max_width\s*=\s*(\d+)", body, re.M)
        ec = root / ".editorconfig"
        if width and ec.exists():
            ec_width = re.search(r"^\s*max_line_length\s*=\s*(\d+)", ec.read_text(encoding="utf-8"), re.M)
            if ec_width:
                check(
                    width.group(1) == ec_width.group(1),
                    f"rustfmt max_width and .editorconfig max_line_length agree ({width.group(1)})",
                    f"rustfmt says {width.group(1)}, .editorconfig says {ec_width.group(1)} — "
                    "the IDE's ruler will not match the formatter",
                )

    for tool, why in (("bacon", "the save-triggered loop"), ("cargo-nextest", "the faster test runner")):
        present = shutil.which(tool) is not None
        print(f"  {'✓' if present else '·'} {tool}{'' if present else f' not installed — optional, {why}'}")

    # A member that predates the [workspace.lints] table inherits NOTHING from it
    # and says nothing about that — the manifest is valid, the build is clean, and
    # the lint policy simply does not apply to that one package.
    root_manifest = root / "Cargo.toml"
    if root_manifest.exists() and "[workspace.lints" in root_manifest.read_text(encoding="utf-8"):
        members = sorted((root / "exercises").glob("*/Cargo.toml"))
        missing = [
            m.parent.name
            for m in members
            if "workspace = true" not in m.read_text(encoding="utf-8")
        ]
        if members:
            check(
                not missing,
                f"all {len(members)} member(s) opt in to the workspace lint policy",
                f"{', '.join(missing)} lack `[lints] workspace = true` — the root's lint "
                "policy does not apply to them, silently; add the two lines to each manifest",
            )

    # The remap hard-codes an absolute path, so it is the one file here that a
    # `git clone` or a `mv` silently invalidates — and it fails in the worst
    # direction: diagnostics keep printing confident absolute paths, at the old
    # location. Relative would have been better than wrong.
    cargo_cfg = root / ".cargo" / "config.toml"
    if cargo_cfg.exists():
        m = re.search(r"--remap-path-prefix==([^\"]*)", cargo_cfg.read_text(encoding="utf-8"))
        if m:
            baked = Path(m.group(1).rstrip("/"))
            check(
                baked == root,
                "compiler output names files absolutely (the remap points at this tree)",
                f"the path remap points at {baked}, but this tree is {root} — every "
                "diagnostic names a file that is not here; re-run `init` with --force",
            )
        else:
            print("  · .cargo/config.toml present but sets no path remap")
    else:
        print("  · no .cargo/config.toml — diagnostics stay relative (`--> src/main.rs:5:9`)")

    idea = root / ".idea"
    if idea.exists():
        configs = sorted((idea / "runConfigurations").glob("*.xml")) if (idea / "runConfigurations").exists() else []
        print(f"  · .idea/ present, {len(configs)} run configuration(s)")
        gi = idea / ".gitignore"
        check(
            gi.exists() and "workspace.xml" in gi.read_text(encoding="utf-8"),
            ".idea/workspace.xml is ignored (it is per-user churn)",
            ".idea/workspace.xml is not ignored — it will fight every commit",
        )

    # The one thing no file can carry, and the reason the page beside this script
    # exists: RustRover's external linter is an IDE-global setting, not a project
    # file, so no scaffolder can set it for you.
    print("\n  ! Set by hand, once, in the IDE — no project file can carry it:")
    print("      Settings → Rust → External Linters → Clippy (the default is Cargo Check)")

    print(f"\n{len(problems)} problem(s).")
    return 1 if problems else 0


NIGHTLY_ONLY_FMT_OPTIONS = (
    "group_imports",
    "imports_granularity",
    "wrap_comments",
    "format_code_in_doc_comments",
    "normalize_comments",
    "condense_wildcard_suffixes",
)


def run_capture(cmd: list[str], cwd: Path) -> str:
    try:
        proc = subprocess.run(cmd, cwd=cwd, capture_output=True, text=True, check=False)
    except OSError:
        return ""
    return proc.stdout


# ---------------------------------------------------------------------------


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(
        prog="rust_scaffold.py",
        description="Scaffold a Rust practice workspace — the files cargo new and the workspace do not write.",
    )
    sub = parser.add_subparsers(dest="cmd", required=True)

    p_init = sub.add_parser("init", help="write the workspace root")
    p_init.add_argument("path")
    p_init.add_argument("--channel", default=DEFAULT_CHANNEL, help=f"toolchain channel (default: {DEFAULT_CHANNEL})")
    p_init.add_argument("--lints", choices=sorted(LINT_PROFILES), default="learn")
    p_init.add_argument("--no-deps", action="store_true", help="omit [workspace.dependencies]")
    p_init.add_argument(
        "--warn-unused",
        action="store_true",
        help="keep rustc's unused_variables/imports/mut and dead_code at warn",
    )
    p_init.add_argument("--no-idea", action="store_true", help="omit the .idea/ files")
    p_init.add_argument(
        "--no-abs-paths",
        action="store_true",
        help="omit .cargo/config.toml, leaving diagnostics relative to the tree root",
    )
    p_init.add_argument("--no-ci", action="store_true", help="omit the GitHub Actions workflow")
    p_init.add_argument("--no-bacon", action="store_true", help="omit bacon.toml")
    p_init.add_argument("--no-git", action="store_true", help="do not run git init")
    p_init.add_argument("--force", action="store_true", help="overwrite existing files")
    p_init.add_argument("--dry-run", action="store_true", help="print what would be written")
    p_init.set_defaults(func=cmd_init)

    p_new = sub.add_parser("new", help="add an exercise (shells out to cargo new)")
    p_new.add_argument("name")
    p_new.add_argument("--workspace", default=".")
    p_new.add_argument("--lib", action="store_true")
    p_new.add_argument("--no-tests", action="store_true", help="leave cargo's hello-world main.rs alone")
    p_new.add_argument("--no-idea", action="store_true")
    p_new.set_defaults(func=cmd_new)

    p_adopt = sub.add_parser(
        "adopt", help="give an existing package the scratch defaults (paths + quiet unused lints)"
    )
    p_adopt.add_argument("path", nargs="?", default=".")
    p_adopt.add_argument("--force", action="store_true", help="overwrite .cargo/config.toml")
    p_adopt.set_defaults(func=cmd_adopt)

    p_doc = sub.add_parser("doctor", help="check the tree against what is installed")
    p_doc.add_argument("path", nargs="?", default=".")
    p_doc.set_defaults(func=cmd_doctor)

    args = parser.parse_args(argv)
    return args.func(args)


if __name__ == "__main__":
    sys.exit(main())
