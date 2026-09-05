"""Make ```rust,<attr> fences render on the site.

`tools/check_fences.py` compiles every hand-authored ```rust fence in its
SCOPE, and CONTRIBUTING documents two escape hatches borrowed from rustdoc:

    ```rust,ignore         not compiled
    ```rust,compile_fail   compiled, and it must FAIL

Both are read off the info string as `lang,attr`. GitHub renders that fine.
`pymdownx.superfences` does not: it accepts a bare language or the brace
form, so a comma info string is not recognised as a fence *at all* and the
block's contents spill into the page as a literal paragraph, backticks
included. `mkdocs build --strict` says nothing, because a fence that fails
to parse is simply prose — so the breakage is invisible everywhere except
on the published page.

This strips the attribute for the site build only. The Markdown on disk
keeps the form GitHub and the checker both want; superfences sees plain
```rust and highlights it.

Fence-aware on purpose: CONTRIBUTING and the pages about this mechanism
display the very info strings it rewrites, inside fences of their own.
Rewriting those would edit the documentation into a lie.
"""

import re

_FENCE = re.compile(r"^(?P<indent>[ \t]*)(?P<marker>`{3,}|~{3,})(?P<info>.*)$")


def _strip_attrs(text: str) -> str:
    out: list[str] = []
    open_marker: str | None = None

    for raw in text.splitlines(keepends=True):
        line = raw.rstrip("\n")
        eol = raw[len(line):]
        m = _FENCE.match(line)
        info = m.group("info").strip() if m else ""

        if m and open_marker is None:
            first = info.split()[0] if info else ""
            lang, *attrs = first.split(",")
            if lang == "rust" and any(attrs):
                trailing = info[len(first):]
                raw = f"{m.group('indent')}{m.group('marker')}rust{trailing}{eol}"
            open_marker = m.group("marker")
        elif (
            m
            and open_marker is not None
            and not info
            and m.group("marker")[0] == open_marker[0]
            and len(m.group("marker")) >= len(open_marker)
        ):
            open_marker = None

        out.append(raw)

    return "".join(out)


def on_page_markdown(markdown, page, config, files):
    return _strip_attrs(markdown)
