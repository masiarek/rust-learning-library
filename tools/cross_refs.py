#!/usr/bin/env python3
"""Cross-references, derived rather than authored.

The question this answers: *should pages carry keyword/tag front matter so
cross-reference pages can be generated from it?* This tool is the argument that
they should not need to. The vocabulary already exists in two curated places —
the term of every GLOSSARY.md entry, and the code spans in each lesson's `# H1`
— so a term index can be **derived** from what is already written, with nothing
new to author on 139 pages and nothing new to keep in sync.

That matters here more than it would elsewhere. This repo is read on three
surfaces (GitHub, the MkDocs site, a plain local Markdown viewer), and YAML
front matter is the one thing that renders differently on all three: a table on
GitHub, chips on the site, and a mangled setext heading in a plain viewer. It is
also a second index, and the hand-kept indexes here demonstrably rot — which is
what `--maps` measures.

Two modes, both REPORT ONLY. Neither is a CI gate and neither writes a file:
over-linking is the failure mode for cross-references, so these produce a list
for a human to judge, not a build failure.

    python3 tools/cross_refs.py --maps     # which lessons a topic map has missed
    python3 tools/cross_refs.py --index    # a derived term -> pages index, as Markdown

`--maps` is the one with a proven catch: it finds the lessons whose H1 or
`**One line:**` names a map page's topic and which that map does not link.
OPTION.md had gone stale exactly that way, missing `Some` is a constructor —
the one page whose entire subject is `Some(T)`.
"""

from __future__ import annotations

import argparse
import os
import re
import sys
from collections import defaultdict
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
SKIP_DIRS = {"site", ".venv", ".git", "__pycache__", "target"}

# A topic map and the code terms that mark a lesson as belonging to it. This is
# the one table to extend when a map page is added — cheap to leave stale,
# because nothing fails when it is: the tool reports, it does not gate.
MAPS = {
    "OPTION.md": ("Option", "Some", "None"),
    "STRUCTS.md": ("struct", "Struct"),
    "STRINGS.md": ("String", "str", "&str"),
    "SHADOWING.md": ("shadow", "Shadowing"),
}

FENCE = re.compile(r"^\s*(```|~~~)")
LINK = re.compile(r"\[[^\]\[]*\]\(([^)\s]+)")
CODE_SPAN = re.compile(r"`([^`\n]+)`")


def pages() -> list[Path]:
    return [
        p
        for p in sorted(REPO.rglob("*.md"))
        if not any(part in SKIP_DIRS for part in p.relative_to(REPO).parts)
    ]


def lessons() -> list[Path]:
    """NN_Section/topic/README.md — the pages that teach one idea."""
    return [
        p
        for p in pages()
        if p.name == "README.md"
        and len(p.relative_to(REPO).parts) == 3
        and p.relative_to(REPO).parts[0][:1].isdigit()
    ]


def prose(text: str) -> str:
    """The page with fenced code removed.

    Every match here has to ignore code blocks. An example's source mentions its
    own subject constantly, so counting those would rank each page top for its
    own term and drown the cross-references that matter.
    """
    out, fenced = [], False
    for line in text.splitlines():
        if FENCE.match(line):
            fenced = not fenced
            continue
        if not fenced:
            out.append(line)
    return "\n".join(out)


def header(p: Path) -> str:
    """H1 + `**One line:**` — what the page says it is about."""
    lines = p.read_text().splitlines()
    h1 = next((l for l in lines if l.startswith("# ")), "")
    one = next((l for l in lines if l.startswith("**One line:**")), "")
    return f"{h1} {one}"


def targets(p: Path) -> set[str]:
    """Every internal path this page links, repo-relative."""
    out = set()
    for href in LINK.findall(p.read_text()):
        href = href.split("#")[0]
        if not href or href.startswith(("http://", "https://", "mailto:")):
            continue
        try:
            out.add(str((p.parent / href).resolve().relative_to(REPO)))
        except ValueError:
            continue
    return out


def rel(p: Path) -> str:
    return str(p.relative_to(REPO))


def report_maps() -> int:
    print("Topic maps — lessons that name the topic but are not linked from the map.\n")
    total = 0
    for name, terms in MAPS.items():
        mp = REPO / name
        if not mp.exists():
            print(f"{name}: missing from the repo — drop it from MAPS\n")
            continue
        linked = targets(mp)
        word = re.compile("|".join(rf"\b{re.escape(t)}\b" for t in terms))
        missing = [
            L for L in lessons() if word.search(header(L)) and rel(L) not in linked
        ]
        kept = sum(1 for t in linked if t.endswith("README.md"))
        print(f"{name}  links {kept} pages · {len(missing)} not linked")
        for L in missing:
            print(f"      {rel(L)}")
        total += len(missing)
        print()
    print(f"{total} lesson(s) a topic map does not reach.")
    print("Not all of them belong on the map — a page can mention a topic without")
    print("being about it. This is a list to read, not a defect count.")
    return 0


def vocabulary() -> dict[str, Path]:
    """term -> the lesson that teaches it, from lesson H1 code spans.

    The H1 is the strongest available claim that a page is *about* a term, and
    it costs nothing to write because every page already has one.
    """
    out: dict[str, Path] = {}
    for L in lessons():
        h1 = L.read_text().splitlines()[0]
        for t in CODE_SPAN.findall(h1):
            out.setdefault(t, L)
    return out


def glossary_terms() -> set[str]:
    gl = REPO / "GLOSSARY.md"
    if not gl.exists():
        return set()
    text = gl.read_text()
    return set(re.findall(r"^\*\*`([^`]+)`\*\*", text, re.M))


def report_index() -> int:
    teaches = vocabulary()
    known = glossary_terms()
    mentions: dict[str, list[Path]] = defaultdict(list)
    for p in pages():
        body = prose(p.read_text())
        spans = set(CODE_SPAN.findall(body))
        for t in teaches:
            if t in spans:
                mentions[t].append(p)

    print("# Terms\n")
    print(
        "Derived by `tools/cross_refs.py --index` from lesson `# H1`s and "
        "[GLOSSARY.md](GLOSSARY.md) — no page authors a keyword.\n"
    )
    print("| Term | Taught by | Also mentioned on | Glossary |")
    print("|---|---|---|---|")
    for t in sorted(teaches, key=str.lower):
        home = teaches[t]
        others = [p for p in mentions.get(t, []) if p != home]
        n = len(others)
        gl = "yes" if t in known else "—"
        print(f"| `{t}` | [{home.parent.name}]({rel(home)}) | {n} | {gl} |")
    print(f"\n{len(teaches)} terms · {len(known)} glossary entries carry a code-span term.")
    print(
        "\nThe vocabulary is only as good as the H1s it is read from, and a few "
        "rows are noise — an H1 that code-spans something that is not a term "
        "(`1 usage`, `add`, `info`) earns a row here. Curate before publishing "
        "this as a page; the Glossary column is the best signal of which rows "
        "are real terms."
    )
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--maps", action="store_true", help="topic-map coverage report")
    ap.add_argument("--index", action="store_true", help="derived term index, as Markdown")
    args = ap.parse_args()
    if args.index:
        return report_index()
    return report_maps()


if __name__ == "__main__":
    sys.exit(main())
