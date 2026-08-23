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
    "I128": "`i128`",
    "Option": "`Option`",
    "Result": "`Result`",
}

# Reading order per folder path. Children named by on-disk name; anything not
# listed sorts alphabetically after the listed ones.
NAV_ORDER: dict[str, list[str]] = {
    "": [
        "index.md",
        "OPTION.md",
        "ROADMAP.md",
        "KATAS.md",
        "01_Foundations",
        "05_Tooling",
        "09_Advanced",
        "GLOSSARY.md",
    ],
    "01_Foundations": [
        "README.md",
        # How to run any of this at all, before the language itself.
        "rustc_without_cargo",
        "some_and_none",
        "option_vs_result",
        "if_let",
        "while_let",
        "unwrap_or",
        "unwrap_or_else",
        "unwrap_or_default",
        "map_or",
        "expect",
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
        # Closes the Result run by asking how any of it reaches a person:
        # the sentence you wrote for a human, and the paths that print the other one.
        "debug_vs_display",
        # Opens the ownership arc — the next thread after the Option run.
        "ownership_and_moves",
        "borrowing",
        # Closes the ownership arc by putting shadowing back through it:
        # what a shadow does to the value, rather than to the name.
        "shadowing_does_not_drop",
        "newtype_score",
        "representing_a_ballot",
        # The sequel: "blank is not zero" stops at two cases; markers need six.
        "six_kinds_of_zero",
        # Opens the numbers arc: the unit every other size is counted in.
        "meet_the_byte",
        # ...and how to write one down, which is the last thing before meaning.
        "why_hexadecimal",
        # Closes the numbers arc, and hands off to the 09_Advanced exactness cluster.
        "what_a_float_stores",
    ],
    "05_Tooling": [
        "README.md",
        "compile_times",
    ],
    "09_Advanced": [
        "README.md",
        "mutex_poisoning",
        "one_person_one_vote",
        "scaled_integers",
        # The technique first, then what the type it lands on actually guarantees.
        "i128_exactness",
        # ...and the method the scaling technique cannot reach at all.
        "compounding_weights",
        # Picks up that page's "if the rule specifies the scale": what to do
        # when the scale was your decision and a margin came in close.
        "interval_arithmetic",
    ],
}

# Folder names whose label the word-by-word caser cannot get right.
LABELS = {
    "if_let": "`if let`",
    "while_let": "`while let`",
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
