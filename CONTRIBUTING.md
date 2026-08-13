# Adding a lesson

The conventions are few, and most of them exist to keep one promise: **a page never claims something a program has not actually printed.**

## The shape of a lesson

```
NN_Section/
  topic_name/
    README.md                 the lesson
    examples/
      topic_name.rs           a program that demonstrates it
      topic_name.out          its recorded output (generated — do not hand-edit)
```

A folder's overview page is named exactly `README.md`. GitHub only auto-renders a file with that name in a folder's tree view, and MkDocs turns it into that section's landing page — so the descriptive title goes in the page's `# H1`, not the filename.

## The one rule

Mark where output belongs and let the tool fill it:

```markdown
<!-- output:topic_name -->
<!-- /output -->
```

Then run:

```bash
python3 tools/run_examples.py
```

It compiles every `examples/*.rs`, runs it, compares against the sibling `.out`, and rewrites every block. Inside the markers is generated; outside is yours. CI runs `--check`, which writes nothing and fails if the code, the answer key, and the page have drifted apart.

For a brand-new example, record the key first:

```bash
python3 tools/run_examples.py --update
```

Read what it recorded before committing. `--update` accepts whatever the program currently prints, so it will happily enshrine a bug.

**Stems are unique repo-wide.** A Markdown block names a bare stem with no path, so two `examples/basics.rs` in different folders is an error the tool refuses rather than guesses at.

**Examples must be deterministic.** The recorded `.out` is an answer key, so an example that reads the clock, the environment, the filesystem, or a random number prints something different on your machine than in CI, and the check fails for a reason that has nothing to do with the lesson. Simulate those inputs instead — a small hard-coded table reads better on the page anyway, because the reader can see the data the output came from.

## Writing the prose

- **One idea per page.** If a page needs two H1-sized ideas, it is two pages.
- **Lead with the shortest true statement.** Open with a `**One line:**` summary a reader can carry away, then earn it.
- **Learner voice by default** — "you write", not "explain to your audience". A learner page serves a teacher fine; a teacher page fails a learner.
- **`**Level:**`** tags a page's depth: `101` (newcomer), `201` (working knowledge), `301` (deep dive), or `reference`, followed by ` · ` and the audience. Untagged is fine; malformed is not.
- **Say which trap you are describing.** The valuable half of most Rust lessons is the mistake, not the mechanism.
- **Bridge to a language the reader already speaks.** Where a Rust idea has a counterpart elsewhere, name it under an *"If you are coming from another language"* heading — one line per language, saying what transfers *and* what the compiler now enforces that convention used to. `Result` is an exception that travels as a return value (Python) and it is `sy-subrc` you cannot forget to check (ABAP); both land faster than any amount of fresh explanation. Keep it honest: a bridge that glosses over a real difference costs more than it saves, so end it on what actually changed.
- **Don't hard-wrap paragraphs.** One paragraph, one line; Markdown collapses single newlines anyway. Keep real breaks only where they are semantic.

## Linking

- **Link a folder by naming its README**: `[label](some_folder/README.md)`, never `[label](some_folder/)`. The bare form works on GitHub and on the built site but not in a plain Markdown viewer, and MkDocs leaves it unrewritten — it ships to the page and 404s.
- **A repo path in backticks must be a link, not bare code text.** Paths are resolved from the *page's* folder by most readers, so a root-relative path in a code span dead-ends. Put the backticks in the label and a real relative path in the href.
- **Link a jargon term on first meaningful use**, once per page, and never to the page's own subject.

## Reading order in the sidebar

Set it in `NAV_ORDER` in [`mkdocs_hooks.py`](https://github.com/masiarek/rust-learning-library/blob/master/mkdocs_hooks.py) — **never by renaming files to `01_`, `02_`…** on a page. A filename is a permanent URL; inserting one lesson would otherwise move every page after it. Numeric prefixes on *section folders* are fine (the hook strips them from the label) because folders move rarely and deliberately.

## Before you commit

```bash
python3 tools/run_examples.py     # examples verified, pages refilled
uv run --group docs mkdocs build --strict   # the site builds clean
```

`--strict` turns a broken link into a build failure, which is the point — a dead link on the published site is invisible to everyone except the reader who tries to follow it.
