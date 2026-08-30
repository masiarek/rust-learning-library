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
5. **A stale by-subject index.** KATAS.md carries a second view of the same
   table, grouped by section, because "which katas are there for strings?" is a
   different question from "what do I do next?" — and a hand-maintained second
   copy of eighty rows would be wrong within a week. It is generated between
   markers from the table above it, so it cannot disagree; ``--fix`` rewrites it.

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


BY_SUBJECT_START = "<!-- by-subject:start -->"
BY_SUBJECT_END = "<!-- by-subject:end -->"

# Section-folder names a mechanical de-prefix reads badly.
SECTION_LABELS = {
    "17_Option_and_Result": "`Option` and `Result`",
    "15_First_Programs": "First programs",
    "19_Numbers": "Numbers and bytes",
    "03_Command_Line": "Command line",
    "25_Control_Flow": "Control flow",
}


def section_label(folder: str) -> str:
    if folder in SECTION_LABELS:
        return SECTION_LABELS[folder]
    return folder.split("_", 1)[-1].replace("_", " ")


def render_by_subject(rows) -> str:
    """One block per section, in the order each section's first kata is reached.

    The reading order and the subject grouping answer different questions, so
    this is a second view of the same rows rather than a second list of them:
    every line below is built from the table, and nothing here is typed twice.
    """
    groups: dict[str, list[tuple[int, str, str]]] = {}
    for i, m in enumerate(rows, 1):
        folder = m.group("lesson_href").split("/", 1)[0]
        groups.setdefault(folder, []).append((i, m.group("kata"), m.group("kata_href")))

    out = [BY_SUBJECT_START, ""]
    out.append("*Generated from the table above by `tools/check_katas.py --fix`. Sections in the order their first kata is reached.*")
    out.append("")
    for folder, items in groups.items():
        readme = f"{folder}/README.md"
        count = len(items)
        out.append(f"**[{section_label(folder)}]({readme})** — {count} kata{'s' if count != 1 else ''}")
        out.append("")
        for num, title, href in items:
            short = title.split(" — ")[0]
            out.append(f"- K{num} · [{short}]({href})")
        out.append("")
    out.append(BY_SUBJECT_END)
    return "\n".join(out)


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

    index_text = INDEX.read_text(encoding="utf-8")
    if BY_SUBJECT_START in index_text and BY_SUBJECT_END in index_text:
        head, rest = index_text.split(BY_SUBJECT_START, 1)
        _, tail = rest.split(BY_SUBJECT_END, 1)
        wanted = render_by_subject(rows)
        if "--fix" in sys.argv:
            INDEX.write_text(head + wanted + tail, encoding="utf-8")
            print(f"{INDEX.name}: by-subject index rewritten from the table")
        elif (BY_SUBJECT_START + rest.split(BY_SUBJECT_END, 1)[0] + BY_SUBJECT_END) != wanted:
            problems.append(
                f"{INDEX.name}: the by-subject index no longer matches the table — "
                "run `python3 tools/check_katas.py --fix`"
            )
    elif "--fix" in sys.argv:
        print(f"{INDEX.name}: no {BY_SUBJECT_START} / {BY_SUBJECT_END} markers to fill")

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
