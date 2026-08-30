#!/usr/bin/env python3
"""Hold `mkdocs.yml`'s redirect map to the three promises it makes.

A folder name here is a permanent URL, so the rare deliberate move needs a
redirect or a reader's saved link 404s with nothing to debug. That safety net
has three ways of failing silently, and `mkdocs build --strict` catches none of
them — it validates links between pages that exist, and a redirect map is a
statement about pages that no longer do.

    1. A DUPLICATE KEY. YAML has no duplicate-key error: the loader keeps the
       last value and drops the first without a word, so one of the two moves
       the map claims to cover is not covered. (This is what kept the sibling
       star-voting-library's docs deploy red for fourteen commits.)

    2. A MISSING DESTINATION. The plugin writes a stub pointing at a path that
       was itself later moved or deleted, so the redirect resolves to a 404 —
       strictly worse than no redirect, because it looks handled.

    3. A CHAIN. `a -> b` and `b -> c` do not compose: the stub for `a` sends the
       reader to `b`, whose own page no longer exists. A page that moves twice
       needs its FIRST path repointed at the NEWEST one, as a second entry
       rather than an edit — the historical key must never change.

Stdlib only, and it reads mkdocs.yml as text rather than through a YAML parser
precisely so that rule 1 is checkable at all.

    python3 tools/check_redirects.py
"""

from __future__ import annotations

import pathlib
import re
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
MKDOCS = REPO / "mkdocs.yml"

ENTRY = re.compile(r"^\s{8}(\S+\.md):\s*(\S+\.md)\s*$")


def parse() -> list[tuple[int, str, str]]:
    """Every `old.md: new.md` line under `redirect_maps:`, with its line number."""
    out: list[tuple[int, str, str]] = []
    inside = False
    for n, line in enumerate(MKDOCS.read_text().splitlines(), 1):
        if line.strip() == "redirect_maps:":
            inside = True
            continue
        if inside:
            stripped = line.strip()
            if stripped and not line.startswith(" " * 8):
                break  # dedented out of the block
            if stripped.startswith("#") or not stripped:
                continue
            m = ENTRY.match(line)
            if not m:
                out.append((n, line.strip(), ""))  # unparseable — reported below
                continue
            out.append((n, m.group(1), m.group(2)))
    return out


def main() -> int:
    entries = parse()
    if not entries:
        print("no redirects declared; nothing to check.")
        return 0

    problems: list[str] = []

    seen: dict[str, int] = {}
    for n, old, new in entries:
        if not new:
            problems.append(f"mkdocs.yml:{n}: cannot read this redirect line: {old}")
            continue
        if old in seen:
            problems.append(
                f"mkdocs.yml:{n}: duplicate key {old!r} (first seen on line {seen[old]}). "
                "YAML keeps the last value silently, so one of these moves is not covered."
            )
        seen[old] = n

    keys = {old for _, old, new in entries if new}
    for n, old, new in entries:
        if not new:
            continue
        if not (REPO / new).exists():
            problems.append(
                f"mkdocs.yml:{n}: destination {new!r} does not exist on disk — "
                "the redirect resolves to a 404, which is worse than no redirect."
            )
        if new in keys:
            problems.append(
                f"mkdocs.yml:{n}: {new!r} is itself a redirect key, and redirects do "
                f"not chain. Point {old!r} straight at the newest path instead."
            )

    live = [old for _, old, new in entries if new and (REPO / old).exists()]

    if problems:
        print("FAILED:")
        for p in problems:
            print(f"  - {p}")
        return 1

    print(f"{len(entries)} redirect(s): no duplicate keys, every destination exists, none chain.")
    if live:
        print(
            "  note: these keys are still files on disk, so the page is on GitHub but "
            "the site serves a redirect at its URL — deliberate, but check it is:"
        )
        for old in live:
            print(f"    {old}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
