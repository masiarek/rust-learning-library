#!/usr/bin/env python3
"""Hold KATAS.md and the Practice sections to each other.

The kata index is the one file in the repo that no single lesson owns, and the
one most likely to be edited by two sessions at once: every kata that lands adds
a row, and every re-sort rewrites all of them. Nothing about a page's own content
reveals that its row is missing, stale, or pointing somewhere else — so this
check is the thing standing in for a reader who noticed.

Four things it will not let past:

1. **An orphan kata.** A lesson with a ``## Practice`` section and no row is a
   published exercise nobody can find from the index.
2. **A row pointing nowhere.** Both links must resolve on disk, and the kata
   link's ``#practice`` must land on a heading that exists — the failure a
   re-sort or a folder rename causes, and the one ``mkdocs --strict`` catches
   only for the site, never for GitHub.
3. **Numbers that have drifted.** The IDs must read K1, K2, K3… in table order.
   Reordering rows is meant to be free; that is only true if renumbering is
   mechanical, and this fails when someone moves a row and leaves the labels.
4. **A number printed on a lesson page.** The ID lives in the table alone. A
   ``K7`` in a page's prose is a second copy that will disagree with the first —
   which it did, within a day, the first time we tried it.

Stdlib only, and no network: same rule as run_examples.py.

    python3 tools/check_katas.py
"""

from __future__ import annotations

import os
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
INDEX = REPO / "KATAS.md"
SKIP_DIRS = {".git", "site", ".venv", "target", "__pycache__", ".github"}

ROW = re.compile(
    r"^\|\s*K(?P<num>\d+)\s*\|\s*\[(?P<kata>[^\]]+)\]\((?P<kata_href>[^)]+)\)\s*"
    r"\|\s*\[(?P<lesson>[^\]]+)\]\((?P<lesson_href>[^)]+)\)\s*\|\s*(?P<level>[^|]+)\|"
)
# "**K7 — …**" or "**K7 …**" at the head of a Practice section.
NUMBER_IN_PROSE = re.compile(r"\*\*K\d+\b")


def pages():
    for dirpath, dirnames, filenames in os.walk(REPO):
        dirnames[:] = [d for d in dirnames if d not in SKIP_DIRS]
        for name in filenames:
            if name.endswith(".md"):
                yield Path(dirpath) / name


def practice_pages() -> set[Path]:
    return {p for p in pages() if re.search(r"^## Practice\s*$", p.read_text(encoding="utf-8"), re.M)}


def main() -> int:
    problems: list[str] = []

    if not INDEX.exists():
        print(f"ERROR: {INDEX.name} is missing")
        return 1

    rows = [m for m in (ROW.match(line) for line in INDEX.read_text(encoding="utf-8").splitlines()) if m]
    if not rows:
        print(f"ERROR: no kata rows parsed out of {INDEX.name} — has the table changed shape?")
        return 1

    listed: dict[Path, int] = {}
    for i, m in enumerate(rows, 1):
        num = int(m.group("num"))
        if num != i:
            problems.append(
                f"{INDEX.name}: row {i} is labelled K{num} — IDs must read K1, K2, K3… "
                "in table order, so that reordering stays free"
            )

        kata_target, _, anchor = m.group("kata_href").partition("#")
        kata_path = (REPO / kata_target).resolve()
        lesson_path = (REPO / m.group("lesson_href")).resolve()

        for label, path in (("kata", kata_path), ("lesson", lesson_path)):
            if not path.exists():
                problems.append(f"{INDEX.name}: K{num}'s {label} link points at {path.relative_to(REPO)}, which does not exist")

        if kata_path.exists():
            listed[kata_path] = num
            if anchor and anchor != "practice":
                problems.append(f"{INDEX.name}: K{num} links to #{anchor}; the exercise anchor is #practice")
            elif anchor and not re.search(r"^## Practice\s*$", kata_path.read_text(encoding="utf-8"), re.M):
                problems.append(
                    f"{INDEX.name}: K{num} links to #practice on "
                    f"{kata_path.relative_to(REPO)}, which has no `## Practice` heading"
                )

    for page in sorted(practice_pages()):
        if page not in listed:
            problems.append(
                f"{page.relative_to(REPO)}: has a `## Practice` section but no row in {INDEX.name} — "
                "an exercise nobody can find from the index"
            )
        text = page.read_text(encoding="utf-8")
        section = text.split("## Practice", 1)[1].split("\n## ", 1)[0]
        if NUMBER_IN_PROSE.search(section):
            problems.append(
                f"{page.relative_to(REPO)}: prints a kata number in its Practice section. "
                f"The number lives only in {INDEX.name}; open with a bold title instead"
            )

    if problems:
        print("FAILED:")
        for p in problems:
            print(f"  - {p}")
        return 1

    print(f"{len(rows)} kata(s) indexed; every row resolves and every Practice section is listed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
