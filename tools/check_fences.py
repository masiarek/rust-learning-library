#!/usr/bin/env python3
"""Compile the hand-authored ```rust fences that no answer key stands behind.

`run_examples.py` reaches `examples/*.rs` and nothing else. Every other Rust
block on every page — the snippet in the prose, the signature, the four-line
illustration — is a claim with no gate behind it, and the reader's response to a
code block is to paste it. That gap is where three bare statements sat on
finished pages of `26_Collections/vec_methods/`: syntax errors for anyone who
pasted them, invisible to CI, and invisible to a proofreader too, because a
fragment reads perfectly well.

**Read this before adding a path to `SCOPE`.** The check is deliberately NOT
repo-wide, and the reason is not that the rest of the library is unfinished. Run
over every page it fails 142 of the 178 that carry a hand-authored Rust fence —
four in five — and almost none of those are defects:

    15_First_Programs/variables/README.md   opens with `let x = 5;`

which is the correct way to open a page about `let`. Wrapping it in an
`fn main()` to satisfy a compiler would make it worse to read, and the library
does this deliberately across `14_Strings/`, `18_Ownership/` and `31_C_and_Cpp/`.

So the thing being checked is not syntax, it is a PROMISE, and pages make
different ones:

    a reference page   — this fence is a complete demonstration you can paste
    a teaching page    — this fence is an illustration of one line

No compiler can see which a page promised. `SCOPE` is where that judgment is
written down, one path at a time, and a tree belongs in it only if every page
under it makes the first promise. Use `--survey <path>` to measure a candidate
before adding it; adding a tree that fails turns a green gate red for its author.

Two escape hatches, borrowed from rustdoc so they need no explaining:

    ```rust,ignore         not compiled — for a fence that cannot stand alone
    ```rust,compile_fail   compiled, and it must FAIL — for a page about a trap

`compile_fail` is the one worth reaching for. A reference page that shows the
error a method gives is asserting something, and this turns that assertion into
a check: the day the compiler starts accepting it, the page is wrong and says so.

Stdlib only, and no network: same rule as run_examples.py.

    python3 tools/check_fences.py                       # gate the scoped trees
    python3 tools/check_fences.py --survey 14_Strings   # measure a candidate
"""

from __future__ import annotations

import argparse
import os
import pathlib
import re
import subprocess
import sys
import tempfile

REPO = pathlib.Path(__file__).resolve().parent.parent
EDITION = "2024"  # the edition run_examples.py builds with; keep them together

# Trees whose pages promise a pasteable fence. One path per line, with the page
# that earned it. Read the docstring above before extending this.
SCOPE = (
    "26_Collections/vec_methods",  # a per-method reference; every fence is a whole demonstration
    "26_Collections/slice_methods",  # its slice-side companion, built to the same promise
    "14_Strings/str_methods",      # the tree that one was modelled on; same promise, brought up to it
)

SKIP_DIRS = {".git", "site", ".venv", "target", "__pycache__", ".github"}

# The generated blocks `run_examples.py` fills. A ```rust fence inside one is the
# example's own source, already compiled and answer-keyed by that tool — checking
# it here would double the work and blame the wrong file for a failure.
BLOCK = re.compile(
    r"<!--\s*(?P<kind>output|source):[A-Za-z0-9_\-]+\s*-->.*?<!--\s*/(?P=kind)\s*-->",
    re.DOTALL,
)

FENCE = re.compile(r"^[ \t]*(?P<f>`{3,}|~{3,})(?P<info>.*)$")


def fenced_spans(text: str) -> list[tuple[int, int]]:
    """Character ranges covered by fenced code blocks.

    Needed because the pages that DOCUMENT the generated-block mechanism show its
    markers inside a fence, and those are prose about a marker, not a marker.
    """
    spans: list[tuple[int, int]] = []
    open_at: int | None = None
    open_fence = ""
    for m in re.finditer(r"^[ \t]*(?P<f>`{3,}|~{3,})", text, re.MULTILINE):
        fence = m.group("f")
        if open_at is None:
            open_at, open_fence = m.start(), fence
        elif fence[0] == open_fence[0] and len(fence) >= len(open_fence):
            spans.append((open_at, m.end()))
            open_at = None
    if open_at is not None:
        spans.append((open_at, len(text)))
    return spans


def generated_lines(text: str) -> set[int]:
    """Line numbers covered by a real generated block (not one shown as an example)."""
    spans = fenced_spans(text)
    covered: set[int] = set()
    for m in BLOCK.finditer(text):
        if any(a <= m.start() < b for a, b in spans):
            continue  # the marker is inside a fence: documentation, not a block
        first = text.count("\n", 0, m.start()) + 1
        last = text.count("\n", 0, m.end()) + 1
        covered.update(range(first, last + 1))
    return covered


def rust_fences(text: str) -> list[tuple[int, set[str], str]]:
    """(line, attrs, code) for every top-level ```rust fence outside a generated block.

    Nesting follows CommonMark: a fence closes on the first line of at least as
    many of the same character with no info string, which is why a ```rust shown
    inside a ````markdown block is body text rather than a fence of its own.
    """
    skip = generated_lines(text)
    out: list[tuple[int, set[str], str]] = []
    open_marker: str | None = None
    start = 0
    attrs: set[str] = set()
    lang = ""
    buf: list[str] = []

    for n, line in enumerate(text.splitlines(keepends=True), 1):
        m = FENCE.match(line)
        info = m.group("info").strip() if m else ""
        if m and open_marker is None:
            open_marker, start, buf = m.group("f"), n, []
            chunk = info.split()[0] if info else ""
            lang, *rest = chunk.split(",")
            attrs = {a for a in rest if a}
            continue
        if (
            m
            and open_marker is not None
            and not info
            and m.group("f")[0] == open_marker[0]
            and len(m.group("f")) >= len(open_marker)
        ):
            if lang == "rust" and start not in skip:
                out.append((start, attrs, "".join(buf)))
            open_marker = None
            continue
        if open_marker is not None:
            buf.append(line)
    return out


def compiles(code: str, workdir: str) -> tuple[bool, str]:
    """Build one fence as a LIBRARY crate.

    A library accepts an item-level fragment — a lone `fn`, `impl` or `struct` —
    which is most of what a page legitimately shows, and rejects a bare statement,
    which is what a reader cannot paste anywhere. Building as a binary would
    reject both, behind one useless `main function not found`.
    """
    src = os.path.join(workdir, "fence.rs")
    with open(src, "w") as fh:
        fh.write(code)
    proc = subprocess.run(
        ["rustc", "--edition", EDITION, "--crate-type", "lib",
         "--emit=metadata", "-o", os.path.join(workdir, "fence.rmeta"), src],
        capture_output=True,
        text=True,
    )
    first = next((l for l in proc.stderr.splitlines() if l.startswith("error")), "")
    return proc.returncode == 0, first


def pages_under(root: pathlib.Path):
    for dirpath, dirnames, filenames in os.walk(root):
        dirnames[:] = [d for d in dirnames if d not in SKIP_DIRS]
        for name in sorted(filenames):
            if name.endswith(".md"):
                yield pathlib.Path(dirpath) / name


def check(roots, workdir: str):
    """-> (problems, fences, ignored, proven_failures, pages_touched)"""
    problems: list[str] = []
    fences = ignored = proven = 0
    pages: set[pathlib.Path] = set()

    for root in roots:
        for page in pages_under(root):
            try:
                rel = page.relative_to(REPO)
            except ValueError:  # a path outside the repo: only reachable from a test
                rel = page
            for line, attrs, code in rust_fences(page.read_text(errors="replace")):
                fences += 1
                pages.add(rel)
                if "ignore" in attrs:
                    ignored += 1
                    continue
                ok, err = compiles(code, workdir)
                if "compile_fail" in attrs:
                    if ok:
                        problems.append(
                            f"{rel}:{line}: marked `compile_fail`, but it compiles. "
                            "The page claims an error the compiler no longer gives."
                        )
                    else:
                        proven += 1
                    continue
                if not ok:
                    problems.append(f"{rel}:{line}: {err or 'does not compile'}")
    return problems, fences, ignored, proven, pages


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument(
        "--survey",
        metavar="PATH",
        help="measure a candidate tree and report without failing; nothing is gated",
    )
    args = ap.parse_args()

    with tempfile.TemporaryDirectory() as workdir:
        if args.survey:
            root = REPO / args.survey
            if not root.exists():
                print(f"no such path: {args.survey}")
                return 1
            problems, fences, ignored, proven, pages = check([root], workdir)
            bad = {p.split(":")[0] for p in problems}
            print(f"survey of {args.survey}: {fences} hand-authored rust fence(s) "
                  f"across {len(pages)} page(s)")
            print(f"  fences that do not compile as a lib crate : {len(problems)}")
            print(f"  pages with at least one                   : {len(bad)} of {len(pages)}")
            if ignored or proven:
                print(f"  skipped as `ignore` {ignored}, proven by `compile_fail` {proven}")
            print()
            if not fences:
                print("No hand-authored fences here at all — every Rust block is inside a "
                      "generated block, which run_examples.py already owns. Adding this tree "
                      "to SCOPE checks nothing today, but gates the first snippet anyone "
                      "writes into the prose.")
            elif not problems:
                print("Every fence compiles. This tree is a candidate for SCOPE — add it "
                      "only if its pages really do promise a pasteable fence.")
            else:
                print("Not a candidate as it stands. Each line below is either a defect or "
                      "a fence that is an illustration rather than a demonstration — which "
                      "is a judgment about what the page promised, not about the code:")
                for p in problems[:40]:
                    print(f"  - {p}")
                if len(problems) > 40:
                    print(f"  … and {len(problems) - 40} more")
            return 0

        roots = [REPO / s for s in SCOPE]
        missing = [s for s, r in zip(SCOPE, roots) if not r.exists()]
        if missing:
            print("FAILED:")
            for s in missing:
                print(f"  - SCOPE names {s!r}, which is not on disk. Remove it or fix the path.")
            return 1

        problems, fences, ignored, proven, pages = check(roots, workdir)
        if problems:
            print("FAILED:")
            for p in problems:
                print(f"  - {p}")
            print()
            print("A fence on a reference page is a promise the reader can paste it. Fix the "
                  "code, or mark the fence ```rust,ignore if it genuinely cannot stand alone.")
            return 1

        scope = ", ".join(SCOPE)
        note = ""
        if ignored or proven:
            note = f" ({ignored} skipped as `ignore`, {proven} proven to fail by `compile_fail`)"
        print(f"{fences} hand-authored rust fence(s) across {len(pages)} page(s) in "
              f"{scope}: every one compiles{note}.")
        return 0


if __name__ == "__main__":
    sys.exit(main())
