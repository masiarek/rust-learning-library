"""Build-time fixes that would otherwise cost a pinned plugin dependency.

MkDocs derives a sidebar *section* label from the folder name on disk, so
`01_Foundations/` would read as "01 Foundations". The numeric prefix is there to
set reading order in a file listing; it should not be visible in the nav.

Two jobs:

1. **Clean section labels** — strip the ordering prefix, turn underscores into
   spaces, and fix acronym casing (`option_vs_result` → "Option vs Result").
2. **Order the sections** — `NAV_ORDER` states the intended reading order per
   folder, keyed by folder path, listing children by their on-disk name.

Why order here rather than by renaming files: a filename is a permanent URL.
Renumbering `03_` to `04_` to insert a lesson would move every page after it and
break any link anyone ever saved. Ordering is presentation, so it belongs in the
presentation layer. Unlisted pages keep their alphabetical slot at the bottom, so
adding a page needs no edit here.
"""

from __future__ import annotations

import re

# Words the naive title-caser gets wrong, and terms with a fixed spelling.
FIXUPS = {
    "Vs": "vs",
    "And": "and",
    "Or": "or",
    "The": "the",
    "To": "to",
    "A": "a",
    "In": "in",
    "Of": "of",
    "Api": "API",
    "Cli": "CLI",
    "Io": "I/O",
    "Ffi": "FFI",
    "Rc": "Rc",
    "Arc": "Arc",
    "Cow": "Cow",
    "Option": "`Option`",
    "Result": "`Result`",
}

# Reading order per folder path. Children named by on-disk name; anything not
# listed sorts alphabetically after the listed ones.
NAV_ORDER: dict[str, list[str]] = {
    "": [
        "index.md",
        "ROADMAP.md",
        "KATAS.md",
        "01_Foundations",
        "09_Advanced",
        "GLOSSARY.md",
    ],
    "01_Foundations": [
        "README.md",
        "some_and_none",
        "option_vs_result",
        "if_let",
        "unwrap_or",
        "unwrap_or_else",
        "what_a_panic_costs",
        "partial_functions",
        "none_on_error",
        "wrong_guard",
        "initial_values",
        "optional_arguments",
        "option_fields",
        "option_as_collection",
        "nullable_pointers",
        "result_aliases",
        "shadowing_and_unwrap",
        # Opens the ownership arc — the next thread after the Option run.
        "ownership_and_moves",
        "newtype_score",
        "representing_a_ballot",
    ],
    "09_Advanced": [
        "README.md",
        "mutex_poisoning",
    ],
}

# Folder names whose label the word-by-word caser cannot get right.
LABELS = {
    "if_let": "`if let`",
}

PREFIX = re.compile(r"^\d+[_\-]")


def clean(label: str) -> str:
    """`01_Foundations` -> `Foundations`; `option_vs_result` -> `Option vs Result`."""
    if label in LABELS:
        return LABELS[label]
    label = PREFIX.sub("", label)
    label = label.replace("_", " ").replace("-", " ")
    words = [w for w in label.split() if w]
    out = []
    for w in words:
        titled = w[:1].upper() + w[1:]
        # Keep an explicitly capitalised word (README, STAR) as the author wrote it.
        out.append(FIXUPS.get(titled, w if w.isupper() else titled))
    return " ".join(out)


def _folder_of(item) -> str:
    """Repo-relative folder path a nav item's children live in."""
    src = None
    if item.is_page:
        src = item.file.src_path
    elif item.is_section and item.children:
        first = item.children[0]
        if first.is_page:
            src = first.file.src_path
        elif first.is_section:
            return ""
    if not src:
        return ""
    return src.rsplit("/", 1)[0] if "/" in src else ""


def _order(items, folder: str) -> None:
    """Sort `items` in place to match NAV_ORDER[folder], then walk deeper."""
    order = NAV_ORDER.get(folder)
    if order:
        def key(item):
            name = None
            if item.is_page:
                name = item.file.src_path.rsplit("/", 1)[-1]
            elif item.is_section and item.children and item.children[0].is_page:
                # A section is named by the folder its index page sits in.
                parts = item.children[0].file.src_path.split("/")
                name = parts[-2] if len(parts) > 1 else None
            if name in order:
                return (0, order.index(name))
            return (1, item.title or "")

        items.sort(key=key)

    for item in items:
        if item.is_section:
            _order(item.children, _folder_of(item))


def on_nav(nav, config, files):
    for item in nav:
        if item.is_section and item.title:
            item.title = clean(item.title)
    _order(list(nav.items) if hasattr(nav, "items") else nav, "")

    # Clean nested section labels too.
    def walk(items):
        for it in items:
            if it.is_section:
                if it.title:
                    it.title = clean(it.title)
                walk(it.children)

    walk(nav)
    return nav
