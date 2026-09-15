#!/usr/bin/env python3
"""Start or update a Cargo project from the training or production template.

    python3 lint_profiles.py new   ~/RustroverProjects/untitled2
    python3 lint_profiles.py new   ~/code/service --profile production
    python3 lint_profiles.py apply ~/RustroverProjects/untitled1
    python3 lint_profiles.py apply ~/code/service --profile production --force
    python3 lint_profiles.py check ~/RustroverProjects/*

The templates are the two project folders beside this script's own --
../training/template and ../production/template -- read at run time. Edit a
template and the next `apply` writes the edit; `check` lists every project that
has not got it yet. No lint is named in this file.

`new` runs `cargo new`, then copies the whole template over it: Cargo.toml with
the name `cargo new` chose, clippy.toml, rust-toolchain.toml, .cargo/config.toml
and the example src/main.rs.

`apply` and `check` touch only what makes a profile a profile:

    Cargo.toml           the [lints.*] and [profile.*] tables
    clippy.toml          all of it
    .cargo/config.toml   the [alias] table -- `cargo strict` or `cargo ci`

[package], [dependencies] and the toolchain stay the project's own.

One more line is in neither template: `--remap-path-prefix` in
.cargo/config.toml, so a diagnostic names the file in full instead of
`src/main.rs`. It spells out the project's directory, because rustflags
interpolate nothing -- so `check` fails once the project has moved, and `apply`
rewrites it. `--no-remap` leaves it out.

What `apply` changes, and what it will not
------------------------------------------
Adding is always done: a missing table is appended with its comments, a missing
key is inserted at the end of its table. A key already present with a DIFFERENT
value is somebody's decision, so it is reported and kept; `--force` replaces it.
The [lints.*] tables belong to the profile, so `--force` also removes a lint the
template does not name. That is what switching training to production needs:
training's `unused_variables = "allow"` would otherwise survive the switch.
Every result is re-parsed with tomllib and compared with the template -- and
with the original, minus what was meant to change -- before anything is written.

Why templates at all, when 05_Tooling/scaffolding argues against them: a
workspace SHARES configuration with its members, and wherever there is a
workspace that is the better mechanism. A standalone project has nothing to
share from, so a copy is the only mechanism there is, and `check` is what keeps
the copies honest.

Stdlib only. Python 3.11+, for tomllib.
"""

from __future__ import annotations

import argparse
import copy
import json
import os
import re
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path

if sys.version_info < (3, 11):
    sys.exit(
        f"lint_profiles.py needs Python 3.11+ for tomllib; this is {sys.version.split()[0]} "
        "(macOS's /usr/bin/python3 is 3.9 -- use a Homebrew or uv python3)"
    )
import tomllib

HERE = Path(__file__).resolve().parent
TEMPLATES = {name: HERE.parent / name / "template" for name in ("training", "production")}

# Any line opening with `[` starts a table, including `[[bin]]`; only a plain
# `[a.b]` header -- quoted parts allowed, as in `[profile.dev.package."*"]` -- has a
# name this script matches against.
BOUNDARY = re.compile(r"^\s*\[")
HEADER = re.compile(r'^\s*\[\s*([A-Za-z0-9_."*-]+)\s*\]\s*(#.*)?$')
KEY = re.compile(r"^\s*([A-Za-z0-9_-]+)\s*=")
REMAP = "--remap-path-prefix=="


class MergeError(Exception):
    pass


def owned(table: str | None) -> bool:
    """A table whose every key the profile decides, so `--force` makes it exact."""
    return table is not None and (table == "lints" or table.startswith("lints."))


def in_profile(table: str | None) -> bool:
    """The Cargo.toml tables a profile consists of."""
    return owned(table) or (table is not None and table.startswith("profile."))


@dataclass
class Template:
    """A template file, split into what a merge needs.

    `sections[table]` is the table as written -- comment block, header, body --
    for appending whole. `keys[table][key]` is one key's lines: the comment block
    directly above it, then the `key = value` line, for inserting one at a time.
    Table `None` is the top level. The file's opening comment block, up to the
    first blank line, is notes for a human and is never copied.
    """

    name: str
    data: dict
    content: list[str]
    sections: dict[str | None, list[str]]
    keys: dict[str | None, dict[str, list[str]]]


def load_template(profile: str, relative: str, keep=lambda table: True) -> Template:
    path = TEMPLATES[profile] / relative
    text = path.read_text(encoding="utf-8")
    data = tomllib.loads(text)
    lines = text.splitlines()

    start = 0
    while start < len(lines) and lines[start].lstrip().startswith("#"):
        start += 1
    while start < len(lines) and not lines[start].strip():
        start += 1
    content = lines[start:]

    sections: dict[str | None, list[str]] = {}
    keys: dict[str | None, dict[str, list[str]]] = {}
    table: str | None = None
    section: list[str] = []
    pending: list[str] = []
    for line in content:
        if BOUNDARY.match(line):
            m = HEADER.match(line)
            if not m:
                raise SystemExit(f"{path}: only plain [table] headers are supported: {line!r}")
            if table is not None or any(KEY.match(l) for l in section):
                sections[table] = strip_trailing(section[: len(section) - len(pending)])
            table = m.group(1)
            section = pending + [line]
            pending = []
            continue
        section.append(line)
        if line.lstrip().startswith("#"):
            pending.append(line)
        elif not line.strip():
            pending = []
        elif m := KEY.match(line):
            keys.setdefault(table, {})[m.group(1)] = pending + [line]
            pending = []
        else:
            raise SystemExit(f"{path}: one `key = value` per line, no multi-line values: {line!r}")
    sections[table] = strip_trailing(section)
    return Template(
        f"{profile}/template/{relative}",
        data,
        content,
        {t: s for t, s in sections.items() if keep(t)},
        {t: k for t, k in keys.items() if keep(t)},
    )


def strip_trailing(lines: list[str]) -> list[str]:
    while lines and not lines[-1].strip():
        lines = lines[:-1]
    return lines


# ---------------------------------------------------------------------------
# Reading a TOML file as lines
# ---------------------------------------------------------------------------


def parts(table: str) -> list[str]:
    """`profile.dev.package."*"` -> ["profile", "dev", "package", "*"]."""
    return [m.group(1) if m.group(1) is not None else m.group(2) for m in re.finditer(r'"([^"]*)"|([^."]+)', table)]


def dig(data: dict, table: str | None) -> dict | None:
    node = data
    for part in parts(table) if table else []:
        node = node.get(part) if isinstance(node, dict) else None
    return node if isinstance(node, dict) else None


def table_span(lines: list[str], table: str | None) -> tuple[int, int] | None:
    """(header line, first line after the table), or None if there is no such table."""
    bounds = [i for i, line in enumerate(lines) if BOUNDARY.match(line)]
    if table is None:
        return (-1, bounds[0] if bounds else len(lines))
    for n, i in enumerate(bounds):
        m = HEADER.match(lines[i])
        if m and m.group(1) == table:
            return (i, bounds[n + 1] if n + 1 < len(bounds) else len(lines))
    return None


def insertion_point(lines: list[str], span: tuple[int, int]) -> int:
    """After the table's last key -- so a comment block that introduces the NEXT
    table stays attached to it."""
    at = span[0] + 1
    for i in range(span[0] + 1, span[1]):
        s = lines[i].strip()
        if s and not s.startswith("#"):
            at = i + 1
    return at


def keys_in(lines: list[str], span: tuple[int, int]) -> dict[str, int]:
    return {m.group(1): i for i in range(span[0] + 1, span[1]) if (m := KEY.match(lines[i]))}


def toml_value(v: object) -> str:
    if isinstance(v, dict):
        return "{ " + ", ".join(f"{k} = {toml_value(x)}" for k, x in v.items()) + " }"
    if isinstance(v, bool):
        return "true" if v else "false"
    if isinstance(v, str):
        return json.dumps(v, ensure_ascii=False)
    if isinstance(v, list):
        return "[" + ", ".join(toml_value(x) for x in v) + "]"
    return str(v)


def label(table: str | None, key: str | None = None) -> str:
    named = [p for p in (table, key) if p]
    return ".".join(named) if named else "(top level)"


# ---------------------------------------------------------------------------
# Merging a template into a file
# ---------------------------------------------------------------------------


def merge(text: str, tpl: Template, *, rename, force: bool) -> tuple[str, list[str]]:
    """Return the merged text and one note per change. Raises MergeError rather
    than return a file that does not say what the template says."""
    lines = text.splitlines()
    before = tomllib.loads(text)
    notes: list[str] = []
    kept: set[tuple[str | None, str]] = set()
    removed: set[tuple[str, str]] = set()

    for table, blocks in tpl.keys.items():
        name = rename(table)
        want = dig(tpl.data, table) or {}
        span = table_span(lines, name)
        if span is None:
            if lines and lines[-1].strip():
                lines.append("")
            lines.extend(f"[{name}]" if HEADER.match(l) else l for l in tpl.sections[table])
            notes.append(f"added   [{name}] with {len(blocks)} key{'' if len(blocks) == 1 else 's'}")
            continue

        have = dig(before, name) or {}
        where = keys_in(lines, span)
        for key in blocks:
            if key in have and have[key] != want[key]:
                if force and key in where:
                    lines[where[key]] = blocks[key][-1]
                    notes.append(f"forced  {label(name, key)} = {toml_value(want[key])}  (was {toml_value(have[key])})")
                else:
                    kept.add((table, key))
                    notes.append(
                        f"kept    {label(name, key)} = {toml_value(have[key])}  "
                        f"(template: {toml_value(want[key])}; --force replaces it)"
                    )

        extras = [k for k in where if k not in blocks] if owned(table) else []
        if extras and force:
            drop = set()
            for key in extras:
                i = where[key]
                drop.add(i)
                while i - 1 > span[0] and lines[i - 1].lstrip().startswith("#"):
                    i -= 1
                    drop.add(i)
                removed.add((name, key))
            lines = [l for n, l in enumerate(lines) if n not in drop]
            notes.append(f"removed {len(extras)} key(s) {tpl.name} does not name from [{name}]: {', '.join(extras)}")
            span = table_span(lines, name)
        elif extras:
            notes.append(f"kept    {len(extras)} key(s) in [{name}] that {tpl.name} does not name: {', '.join(extras)}  (--force removes them)")

        missing = [k for k in blocks if k not in have]
        if missing:
            at = insertion_point(lines, span)
            lines[at:at] = [l for k in missing for l in blocks[k]]
            notes.append(f"added   {len(missing)} key(s) to [{name}]" if name else f"added   {len(missing)} key(s)")

    merged = "\n".join(lines) + "\n"
    try:
        after = tomllib.loads(merged)
    except tomllib.TOMLDecodeError as e:
        raise MergeError(f"the merged file would not parse ({e})") from None
    for table, blocks in tpl.keys.items():
        got = dig(after, rename(table)) or {}
        want = dig(tpl.data, table) or {}
        for key in blocks:
            if (table, key) not in kept and got.get(key) != want[key]:
                raise MergeError(f"{label(rename(table), key)} would not come out as {tpl.name} says")
        if force and owned(table) and set(got) - set(blocks):
            raise MergeError(f"[{rename(table)}] would still hold {sorted(set(got) - set(blocks))}")
    if strip_keys(before, tpl, rename, removed) != strip_keys(after, tpl, rename, removed):
        raise MergeError("the merge would change something the template does not name")
    return merged, notes


def strip_keys(data: dict, tpl: Template, rename, removed: set[tuple[str, str]]) -> dict:
    d = copy.deepcopy(data)
    doomed = {(rename(t), k) for t, blocks in tpl.keys.items() for k in blocks} | removed
    for table, key in doomed:
        node = dig(d, table)
        if node is not None:
            node.pop(key, None)
    return prune(d)


def prune(d: dict) -> dict:
    return {k: prune(v) if isinstance(v, dict) else v for k, v in d.items() if not (isinstance(v, dict) and not prune(v))}


def compare(data: dict, tpl: Template, rename) -> tuple[list[str], list[str], list[str]]:
    """(missing, differs, extra) keys, for `check`."""
    missing, differs, extra = [], [], []
    for table, blocks in tpl.keys.items():
        have = dig(data, rename(table)) or {}
        want = dig(tpl.data, table) or {}
        for key in blocks:
            if key not in have:
                missing.append(label(rename(table), key))
            elif have[key] != want[key]:
                differs.append(f"{label(rename(table), key)} = {toml_value(have[key])} (template: {toml_value(want[key])})")
        if owned(table):
            extra += [f"{label(rename(table), k)} = {toml_value(v)}" for k, v in have.items() if k not in blocks]
    return missing, differs, extra


# ---------------------------------------------------------------------------
# The remap line
# ---------------------------------------------------------------------------

REMAP_COMMENT = [
    "# Diagnostics name the file in full, not `src/main.rs`: an empty OLD in",
    "# --remap-path-prefix=OLD=NEW matches every relative path and no absolute one.",
    "# The path is THIS directory, spelled out -- rustflags interpolate nothing --",
    "# so `lint_profiles.py check` fails once the project moves. A RUSTFLAGS",
    "# environment variable replaces this line wholesale.",
]


def remap(text: str, root: Path) -> tuple[str, list[str]]:
    flag = f"{REMAP}{root}/"
    data = tomllib.loads(text)
    flags = (data.get("build") or {}).get("rustflags")
    lines = text.splitlines()

    if flags is None:
        entry = REMAP_COMMENT + [f"rustflags = [{toml_value(flag)}]"]
        span = table_span(lines, "build")
        if span is None:
            if lines and lines[-1].strip():
                lines.append("")
            lines.extend(["[build]"] + entry)
        else:
            at = insertion_point(lines, span)
            lines[at:at] = entry
        return "\n".join(lines) + "\n", ["added   build.rustflags remap — diagnostics name the file in full"]

    if not isinstance(flags, list):
        return text, ["skipped build.rustflags is a string, not an array; add the remap by hand"]
    remaps = [f for f in flags if f.startswith(REMAP)]
    if not remaps:
        return text, [f"skipped build.rustflags has no remap, and this script does not edit an existing array; add {toml_value(flag)} by hand"]
    if remaps == [flag]:
        return text, []
    old = remaps[0]
    for quoted in (toml_value(old), f"'{old}'"):
        if text.count(quoted) == 1:
            new = text.replace(quoted, toml_value(flag))
            if tomllib.loads(new).get("build", {}).get("rustflags") == [flag if f == old else f for f in flags]:
                return new, [f"updated remap — it named {old[len(REMAP):]}, and the project is now here"]
    return text, [f"skipped remap names {old[len(REMAP):]}, not this directory; fix it by hand"]


# ---------------------------------------------------------------------------
# Commands
# ---------------------------------------------------------------------------


def shown(path: Path) -> str:
    return f".cargo/{path.name}" if path.parent.name == ".cargo" else path.name


def write(path: Path, old: str | None, new: str, notes: list[str], *, dry_run: bool) -> None:
    if new == (old or ""):
        print(f"  ok      {shown(path)}")
        return
    for note in notes:
        print(f"  {note}{'' if note.startswith(('kept', 'skipped')) else f'  ({shown(path)})'}")
    if not dry_run:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(new, encoding="utf-8")


def ignore_config(root: Path, *, dry_run: bool) -> None:
    """The remap names this machine's path, so the file it lives in stays out of git."""
    if not (root / ".git").exists():
        return
    if subprocess.run(["git", "-C", str(root), "check-ignore", "-q", ".cargo/config.toml"]).returncode == 0:
        return
    print("  added   /.cargo/config.toml to .gitignore — the remap names this machine's path")
    if not dry_run:
        gitignore = root / ".gitignore"
        body = gitignore.read_text(encoding="utf-8") if gitignore.exists() else ""
        gitignore.write_text(body + ("\n" if body and not body.endswith("\n") else "") + "/.cargo/config.toml\n", encoding="utf-8")


def apply(root: Path, profile: str, *, force: bool, with_remap: bool, dry_run: bool) -> int:
    print(f"{root}  ({profile}{'; dry run, nothing is written' if dry_run else ''})")
    manifest = root / "Cargo.toml"
    if not manifest.is_file():
        print("  error   no Cargo.toml here", file=sys.stderr)
        return 1
    cargo = load_template(profile, "Cargo.toml", keep=in_profile)
    clippy = load_template(profile, "clippy.toml")
    config = load_template(profile, ".cargo/config.toml")
    try:
        text = manifest.read_text(encoding="utf-8")
        data = tomllib.loads(text)
        if (data.get("lints") or {}).get("workspace") is True:
            print("  skipped Cargo.toml inherits [workspace.lints]; apply the profile to the workspace root")
        else:
            # A workspace root's lints table is [workspace.lints.*], and a wrong
            # name is silent: [workspace.lints] in a package manifest is never read.
            ws = "workspace" in data and "package" not in data
            rename = (lambda t: f"workspace.{t}" if owned(t) else t) if ws else (lambda t: t)
            new, notes = merge(text, cargo, rename=rename, force=force)
            write(manifest, text, new, notes, dry_run=dry_run)
            if ws and new != text:
                print("  note    members need `[lints] workspace = true` to inherit it")

        path = root / ".clippy.toml" if (root / ".clippy.toml").exists() else root / "clippy.toml"
        text = path.read_text(encoding="utf-8") if path.exists() else None
        new, notes = merge(text or "", clippy, rename=lambda t: t, force=force)
        write(path, text, new, notes, dry_run=dry_run)

        path = root / ".cargo" / "config.toml"
        text = path.read_text(encoding="utf-8") if path.exists() else None
        new, notes = merge(text or "", config, rename=lambda t: t, force=force)
        if with_remap:
            new, more = remap(new, root)
            notes += more
            tomllib.loads(new)
        write(path, text, new, notes, dry_run=dry_run)
    except (MergeError, tomllib.TOMLDecodeError) as e:
        print(f"  error   {e}; nothing more written here", file=sys.stderr)
        return 1
    if with_remap:
        ignore_config(root, dry_run=dry_run)
    return 0


def check(root: Path, profile: str) -> int:
    print(f"{root}  ({profile})")
    manifest = root / "Cargo.toml"
    if not manifest.is_file():
        print("  FAIL    no Cargo.toml here")
        return 1
    problems = 0

    def report(path: Path, tpl: Template, rename=lambda t: t) -> None:
        nonlocal problems
        if not path.exists():
            print(f"  FAIL    no {shown(path)}")
            problems += 1
            return
        missing, differs, extra = compare(tomllib.loads(path.read_text(encoding="utf-8")), tpl, rename)
        total = sum(len(b) for b in tpl.keys.values())
        good = total - len(missing) - len(differs)
        if not (missing or differs or extra):
            print(f"  ok      {shown(path)}: {total}/{total} keys as {tpl.name} says")
            return
        problems += 1
        print(f"  FAIL    {shown(path)}: {good}/{total} keys as {tpl.name} says")
        for m in missing:
            print(f"            missing  {m}")
        for d in differs:
            print(f"            differs  {d}")
        for x in extra:
            print(f"            extra    {x}")

    data = tomllib.loads(manifest.read_text(encoding="utf-8"))
    if (data.get("lints") or {}).get("workspace") is True:
        print("  ok      Cargo.toml: lints inherited from [workspace.lints] — check the workspace root")
    else:
        ws = "workspace" in data and "package" not in data
        report(manifest, load_template(profile, "Cargo.toml", keep=in_profile), (lambda t: f"workspace.{t}" if owned(t) else t) if ws else (lambda t: t))
    report(root / ".clippy.toml" if (root / ".clippy.toml").exists() else root / "clippy.toml", load_template(profile, "clippy.toml"))
    config = root / ".cargo" / "config.toml"
    report(config, load_template(profile, ".cargo/config.toml"))

    if config.exists():
        flags = (tomllib.loads(config.read_text(encoding="utf-8")).get("build") or {}).get("rustflags") or []
        remaps = [f[len(REMAP):].rstrip("/") for f in flags if isinstance(f, str) and f.startswith(REMAP)] if isinstance(flags, list) else []
        if not remaps:
            print("  note    no remap: diagnostics will say `src/main.rs`")
        elif Path(remaps[0]) == root:
            print("  ok      remap names this directory")
        else:
            print(f"  FAIL    remap names {remaps[0]} — the project has moved; `apply` rewrites it")
            problems += 1
    if os.environ.get("RUSTFLAGS"):
        print("  FAIL    RUSTFLAGS is set, and it REPLACES build.rustflags: the aliases' flags and the remap stop applying")
        problems += 1
    return 1 if problems else 0


def new(path_arg: str, profile: str, *, with_remap: bool) -> int:
    target = Path(path_arg).expanduser()
    made = subprocess.run(["cargo", "new", "--quiet", str(target)])
    if made.returncode:
        return made.returncode
    root = target.resolve()
    name = tomllib.loads((root / "Cargo.toml").read_text(encoding="utf-8"))["package"]["name"]
    source = TEMPLATES[profile]
    print(f"{root}  ({profile})")
    for path in sorted(p for p in source.rglob("*") if p.is_file()):
        rel = path.relative_to(source)
        if rel.parts[0] == "target" or rel.name == "Cargo.lock":
            continue
        body = path.read_text(encoding="utf-8")
        if rel == Path("Cargo.toml"):
            body = re.sub(r'(?m)^name = "[^"]*"', f"name = {toml_value(name)}", body, count=1)
        dest = root / rel
        dest.parent.mkdir(parents=True, exist_ok=True)
        dest.write_text(body, encoding="utf-8")
        print(f"  wrote   {rel}")
    if with_remap:
        config = root / ".cargo" / "config.toml"
        text = config.read_text(encoding="utf-8") if config.exists() else ""
        body, notes = remap(text, root)
        tomllib.loads(body)
        write(config, text, body, notes, dry_run=False)
        ignore_config(root, dry_run=False)
    return 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    sub = parser.add_subparsers(dest="command", required=True)

    def profile_flag(p: argparse.ArgumentParser) -> None:
        p.add_argument("--profile", choices=sorted(TEMPLATES), default="training", help="which template (default: training)")

    p = sub.add_parser("new", help="`cargo new`, then the whole template over it")
    p.add_argument("path")
    profile_flag(p)
    p.add_argument("--no-remap", action="store_true", help="leave build.rustflags out")

    p = sub.add_parser("apply", help="add a profile's lints, profiles, clippy.toml and aliases to existing projects")
    p.add_argument("paths", nargs="+")
    profile_flag(p)
    p.add_argument("--force", action="store_true", help="replace differing values, and make [lints.*] exactly the template's")
    p.add_argument("--no-remap", action="store_true", help="leave build.rustflags alone")
    p.add_argument("--dry-run", action="store_true", help="say what would change; write nothing")

    p = sub.add_parser("check", help="compare projects with a template; exit 1 on any difference")
    p.add_argument("paths", nargs="+")
    profile_flag(p)

    args = parser.parse_args(argv)
    if args.command == "new":
        return new(args.path, args.profile, with_remap=not args.no_remap)

    status = 0
    for i, raw in enumerate(args.paths):
        if i:
            print()
        root = Path(raw).expanduser().resolve()
        if args.command == "apply":
            status |= apply(root, args.profile, force=args.force, with_remap=not args.no_remap, dry_run=args.dry_run)
        else:
            status |= check(root, args.profile)
    return status


if __name__ == "__main__":
    sys.exit(main())
