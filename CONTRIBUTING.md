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

For a brand-new example, record the key first — and **scope it to your own stem**:

```bash
python3 tools/run_examples.py --update --only my_new_example
```

Read what it recorded before committing. `--update` accepts whatever the program currently prints, so it will happily enshrine a bug.

**Always pass `--only` with `--update`.** A bare `--update` re-records *every* answer key in the repo, including examples someone else is midway through editing — so a run meant to record one file can quietly rewrite a colleague's in-flight key with output they have not looked at yet. With `--only`, everything outside your selection is left alone completely — not run, not re-recorded, and not even *read*, so no page block naming it can be rewritten either. The flag takes a stem, a path to the `.rs`, or the lesson folder that holds it, and a name matching nothing is an error rather than an empty selection — a typo that records nothing otherwise looks exactly like a clean run. It is not a CI flag: `--check` there has to stay whole-repo, since spotting repo-wide drift is its entire job. Do a full run of your own before committing, which is what the checklist at the bottom of this page is for.

**Stems are unique repo-wide.** A Markdown block names a bare stem with no path, so two `examples/basics.rs` in different folders is an error the tool refuses rather than guesses at.

**Examples must be deterministic.** The recorded `.out` is an answer key, so an example that reads the clock, the environment, the filesystem, or a random number prints something different on your machine than in CI, and the check fails for a reason that has nothing to do with the lesson. Simulate those inputs instead — a small hard-coded table reads better on the page anyway, because the reader can see the data the output came from.

**And no local gate can catch a machine-dependent one.** `std::env::consts::OS`, `cfg!(target_os = …)`, `size_of::<usize>()`, a path separator: each is a *constant* on your machine, so the example is perfectly reproducible there and the key matches every time you re-check it — including in the extracted-tree run at the bottom of this page, which is a different tree on the same machine. Only CI ever asks the question that matters. An attributes page printed `env::consts::OS` to illustrate `#[cfg]` and reddened the examples workflow for four commits while every local gate stayed green; the docs deploy stayed green too, which is the trap the CI section below already names. So when an example's output could conceivably differ on another platform, assume it does and pick something that cannot — `cfg!(test)` is false in every non-`--test` build everywhere, and makes the same point.

## Stubs

A **stub** is a lesson page with no example behind it yet: an H1, a `**Level:**`, a `**One line:**`, and the questions the finished page has to answer. It exists so that an arc has a shape and a permanent URL before the prose does — [`02_Errors/`](02_Errors/README.md), [`03_Command_Line/`](03_Command_Line/README.md), [`04_Files/`](04_Files/README.md), [`06_Data/`](06_Data/README.md) and [`07_Clients/`](07_Clients/README.md) are stubs throughout.

Every stub carries the same notice directly under its `**Level:**` line, so nobody mistakes an outline for a checked page:

```markdown
> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.
```

Two things a stub must not have. An `<!-- output: -->` block — there is no answer key to fill it from, so the block would be a promise the tool cannot keep. And a `## Practice` section — `check_katas.py` will fail it for the missing row, and rightly: an exercise whose solution is not compiled is the one thing a practice page must never ship. A stub graduates by gaining an `examples/` program, losing the notice, and, if it gained a kata on the way, its row in [KATAS.md](KATAS.md).

Prefer a stub to a bullet on a *Planned* list when the page's **boundaries** are the useful part — what it covers, what the neighbouring page covers instead, and the see-also links between them. Those cost nothing to write while the whole arc is in your head, and are retrofitted badly months later. Prefer the Planned bullet when all you have is a title.

## Katas

A kata is an exercise, and it lives **on the page for the topic it teaches** — under a `## Practice` heading near the end, with the solution folded into a `<details markdown="1">` block. There is no folder of katas, and no `K01_` prefix anywhere on disk: folders are permanent URLs, and a numbered sequence is exactly the thing that gets reordered. The sequence lives in [KATAS.md](KATAS.md) instead, which costs nothing to reshuffle. Same reasoning as the sidebar order below.

**The solution is a real example, never a pasted snippet.** Put it beside the lesson's other example as `<topic>_kata.rs` and let the tool paste both halves in:

```markdown
<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:some_and_none_kata -->
<!-- /source -->

<!-- output:some_and_none_kata -->
<!-- /output -->

</details>
```

`source:` pastes the file itself, `output:` pastes what it printed, and both are regenerated by `tools/run_examples.py` — so a solution cannot rot into one that no longer compiles, which is the one failure a practice page must never ship. (`md_in_html` is enabled, so the Markdown inside `<details>` renders on the site *and* on GitHub. A `???` Material admonition would not: it prints as literal text on GitHub.)

Then add a row to [KATAS.md](KATAS.md) at the point in the sequence where the kata should be attempted, and renumber — `tools/check_katas.py` fails if the IDs no longer read K1, K2, K3… in table order, if a `## Practice` section has no row, or if a row's links stop resolving. That check exists because the index is the one file no lesson owns: nothing about your page reveals that its row is missing. **Do not print the kata's number on its own page** — open with a short bold title instead. The number lives in that one table, which is what makes reordering the sequence free; a `K7` in a page's prose is a second place to update and the reason a stale one goes unnoticed.

## Writing the prose

- **One idea per page.** If a page needs two H1-sized ideas, it is two pages.
- **Lead with the shortest true statement.** Open with a `**One line:**` summary a reader can carry away, then earn it.
- **Aim at a good GitHub README, not a book.** Code or a compiler error, then a brief plain explanation, then the next one. Word count is not the problem — **preamble, summary and linguistic decoration** are. Open on the thing itself and stop when it is said; spend as many words as the substance needs, none on the joins. Four habits produce nearly all the padding, and all four are deletions:
  - **A sentence explaining why the previous sentence matters** — *"…which is the point"*, *"…and that is the whole behaviour"*, *"worth naming out loud"*. If the point needs restating, fix the first sentence.
  - **Announcing structure** — *"three things are true of this, and the third is surprising"*, *"one more wrinkle"*. Just say the three things.
  - **Prose restating a table** printed directly above it.
  - **Page meta-commentary** — *"twenty pages already use structs and none says what one is; this is that page"*. True, and it teaches nothing.

  Plus **decoration**: intensifiers (*genuinely*, *precisely*, *simply*, *actually*), colour (*at 3am*, *wearing a new name*), and trailing clauses that editorialise rather than inform. Prefer a fragment to a well-formed sentence where the fragment is clearer, and a table to either.

  A rewrite of the five struct pages cut 43% of the words without losing a fact. The **one exception is the language bridges**, which got *longer* — see the bridge rule below.
- **Learner voice by default** — "you write", not "explain to your audience". A learner page serves a teacher fine; a teacher page fails a learner.
- **`**Level:**`** tags a page's depth: `101` (newcomer), `201` (working knowledge), `301` (deep dive), or `reference`, followed by ` · ` and the audience. Untagged is fine; malformed is not.
- **Say which trap you are describing.** The valuable half of most Rust lessons is the mistake, not the mechanism.
- **Bridge to a language the reader already speaks.** Where a Rust idea has a counterpart elsewhere, name it under an *"If you are coming from another language"* heading, saying what transfers *and* what the compiler now enforces that convention used to. **Take the words this needs** — a bridge to a language the reader already thinks in is the fastest teaching in the library, and it is the one place where more detail reliably pays. (This used to say "one line per language"; Adam lifted that cap in 2026-08 — the cap was compressing the most useful sections on the page.) `Result` is an exception that travels as a return value (Python) and it is `sy-subrc` you cannot forget to check (ABAP); both land faster than any amount of fresh explanation. Keep it honest: a bridge that glosses over a real difference costs more than it saves, so end it on what actually changed.
- **Expected output goes in a trailing comment**, on the line that prints it — `println!("{x}");  // 10`, never as a bare line under the code block. A snippet here is *input to a compiler*, because the reader pastes it: an output line sitting under the last statement gets pasted too, and the reader's first experience of the lesson is a syntax error in code that was actually correct. A transcript too long for a comment goes in a separate fenced block clearly below the code, never as loose lines abutting it. For a compiled `examples/*.rs` the sibling `.out` stays the answer key — the comment is a courtesy to whoever reads the source, not a second source of truth.
- **Never open a page with code that does not compile.** The first block on a page is the one that gets pasted, and a lesson that greets its reader with an error has taught nothing yet. Lead with the working answer; put the refusals in their own section further down. Where a broken line has to be shown at all, write it as a `//` comment inside an otherwise-valid snippet — inert if pasted, still legible if only read — and keep the compiler's own transcript in a separate `text` fence, which nobody can paste into a `.rs` by accident. This does not weaken *say which trap you are describing*: the trap is still the valuable half of the page, it just is not the opening. Same reason as the trailing-comment rule above — a snippet here is input to a compiler.
- **A claim about code that does not compile has no answer key behind it.** `run_examples.py` verifies every number a program printed and reaches nothing on a page whose subject is a *refusal* — there is no `examples/*.rs` behind a trap, so a wrong error code, a wrong message or a wrong count can sit there for months. Verify those by hand, and verify the **fence on the page** rather than the file you were experimenting in: on 2026-08-29 a bounds page printed one snippet and a count taken from another, and the number was wrong for both. Two traps in counting compiler errors, both from that day. `grep -c "^error"` also matches rustc's own summary line — `error: aborting due to 6 previous errors` — so it reads one too high on every file that fails to compile; `grep -cE "^error\["` is the one you want. And the count is not a property of the trap: the same struct-bound mistake is six errors with one `impl` block and eight with a second method, so write *"and it grows with every further block that names the type"* rather than a digit that an incidental edit invalidates. Some of these cannot be made verbatim at all: a trap's transcript names a file, and that file cannot live in `examples/` precisely because it does not compile. Two ways to close the gap honestly, and they are not equivalent. **Dropping** information — the `--> file:line` header, a repeated `note:` block — leaves a visible seam and asserts nothing, and is what [`option_fields`](17_Option_and_Result/option_fields/README.md) already does — its `E0308` fence goes straight from the error line to `7 | let p = …`, under a `title="Abridged — real rustc output for …"`. Label it that way and it is honest. **Substituting** information is the one to avoid: a header reading `--> ballot.rs:1:15` on output produced by `e0392.rs` asserts something specific and false, and nothing on the page shows the seam. The cheap fix is one `mv` — name the scratch file what the transcript says, recompile, and the fence is byte-checkable from then on. Underneath both is one habit worth carrying past this file — **report what you measured, not what you concluded**, whenever those are two different sentences. A gate failing on somebody else's file is a measurement with a timestamp: date it or re-check before you send it, or twenty minutes later it reads as a claim about how they work.
- **Don't hard-wrap paragraphs.** One paragraph, one line; Markdown collapses single newlines anyway. Keep real breaks only where they are semantic.

## Linking

- **A link that leaves the library ends its label with ` ↗`** — ``[`Sized` ↗](https://doc.rust-lang.org/std/marker/trait.Sized.html)``. An internal link never carries one, so *no arrow* means the page is still in here. Without it a column of blue text says nothing about where any row goes: four trait names side by side, three of them std docs and one a lesson two folders up, and the only way to tell was to hover. The marker sits in the Markdown rather than the site's CSS because the library is read on three surfaces — GitHub, the built site, and a plain local Markdown viewer — and a stylesheet reaches one of them. `python3 tools/check_link_style.py --fix` adds and removes them; CI runs it without `--fix`.
- **Link a folder by naming its README**: `[label](some_folder/README.md)`, never `[label](some_folder)`. The bare form works on GitHub and on the built site but not in a plain Markdown viewer, and MkDocs leaves it unrewritten — it ships to the page and 404s.
- **A repo path in backticks must be a link, not bare code text.** Paths are resolved from the *page's* folder by most readers, so a root-relative path in a code span dead-ends. Put the backticks in the label and a real relative path in the href.
- **Link a jargon term on first meaningful use**, once per page, and never to the page's own subject.

## Moving a page

Folder names are permanent URLs, which is why [reading order lives in a hook](#reading-order-in-the-sidebar) rather than in filenames. Sometimes a page moves anyway — a section outgrows its name, an arc gets split — and then the old URL has to keep working, because the reader who saved it has no way to guess where it went and no error message that helps.

Add an entry to `redirect_maps` in `mkdocs.yml`, one per moved page, both sides written as **source paths**:

```yaml
23_Functional/iterators_are_lazy/README.md: 24_Iterators/iterators_are_lazy/README.md
```

The plugin writes a redirect stub at the old page's built location — for a folder README that is its `index.html`, so both `…/old_page/` and `…/old_page/index.html` keep resolving, and any `#anchor` is carried across.

Three things about an entry, each of which is a way this fails quietly:

- **It is permanent.** Deleting one un-breaks nothing and re-breaks every link anyone saved. Entries accumulate; they are not tidied up later.
- **The key is a historical path and never changes.** If a page moves a second time, add a second entry from the *first* old path to the newest one. Redirects do not chain: `a → b` and `b → c` leaves anyone holding `a` at a page that no longer exists.
- **The destination must exist.** `mkdocs build --strict` will not tell you it does not — it checks links between pages that *are* there, and a redirect map is a statement about pages that are not.

`python3 tools/check_redirects.py` enforces the last two, plus the one YAML will not: a **duplicate key**, which the loader resolves by silently keeping the last value, so one of the two moves it claims to cover is not covered.

Do the whole move in one commit — `git mv`, the link rewrites, the redirect entries — and grep for the old path afterwards, including in `mkdocs_hooks.py`, `GLOSSARY.md`, `KATAS.md` and the course table in the root README. Those four are the ones nothing else will catch.

## Reading order in the sidebar

Set it in `NAV_ORDER` in [`mkdocs_hooks.py` ↗](https://github.com/masiarek/rust-learning-library/blob/master/mkdocs_hooks.py) — **never by renaming files to `01_`, `02_`…** on a page. A filename is a permanent URL; inserting one lesson would otherwise move every page after it. Numeric prefixes on *section folders* are fine, because folders move rarely and deliberately — and the hook strips them from the label, so a reader never sees them.

Four things that follow from that, worth knowing before you add a page:

- **A lesson missing from `NAV_ORDER` still ships** — it lands alphabetically at the bottom of its section. Nothing breaks, nothing warns, and that is exactly how four Compilers pages ended up sitting underneath the pages they come before. Add the row.
- **The root sorts alphabetically; every folder below it does not.** Top-level sections are ordered A–Z by their label, with only Home and Start here pinned, so a new section needs no edit to `NAV_ORDER` at all. Inside a section a lesson sequence *is* a sequence, so those keep the order the list states. The course's reading order lives on the homepage as a numbered table ("The course, in order") — if you reorder the course, that is the table to edit, and nothing in the sidebar will move.
- **A label is built from the folder name, not from the page's `# H1`.** `clean()` strips the prefix, swaps underscores for spaces, and sentence-cases the rest; `FIXUPS` fixes acronyms and code words at any position; `LABELS` overrides the whole label, keyed by folder name. `LABELS` is where a page whose H1 carries a subtitle gets the short name the sidebar has room for.
- **The footer's ← → arrows are re-threaded from the sorted tree**, so they always agree with the sidebar. MkDocs threads them *before* any hook can reorder anything, so without that step the homepage's "Next" was the glossary.

## Before you commit

```bash
python3 tools/run_examples.py     # examples verified, pages refilled
python3 tools/check_katas.py      # every Practice section is indexed, every row resolves
python3 tools/check_link_style.py # every external link carries its ↗, no internal link does
python3 tools/check_redirects.py  # every retired URL still lands somewhere that exists
uv run --group docs mkdocs build --strict   # the site builds clean
```

`--strict` turns a broken link into a build failure, which is the point — a dead link on the published site is invisible to everyone except the reader who tries to follow it.

**Then run them again on what you are actually publishing.** Those three commands read the *working tree*, and in a shared checkout the working tree is nobody's build: it is the commit you are pushing, plus every other session's uncommitted work, minus anything they have staged but not written out. A gate that passes there has told you about a state that will never exist on the server. The failure is not hypothetical — a green `check_katas.py` is exactly what a broken deploy looks like from the author's chair, because the README the row points at is on disk and not in the commit.

So extract the pushed tree and re-run against that:

```bash
T=$(mktemp -d); git archive origin/master | tar -x -C "$T"
(cd "$T" && python3 tools/check_katas.py && python3 tools/check_link_style.py && python3 tools/check_redirects.py && python3 tools/run_examples.py --check)
```

`git archive` writes only tracked, committed content, so an in-flight folder belonging to someone else cannot make this pass and a file you forgot to `git add` cannot hide in it. Do it after you push, against `origin/master` — that is the tree CI builds and the one readers get. `--check` rather than a plain run, since the extract is a throwaway and nothing there is worth rewriting.

A corollary worth keeping: **a gate that fails on a stem you have never heard of is somebody's in-flight folder, not your bug.** Check `git status` for a `??` beside it, and leave it alone — never `--update` a key you did not write.

**And a cancelled CI run is not a failed one.** `.github/workflows/docs.yml` sets `concurrency: {group: pages, cancel-in-progress: true}`, so when somebody pushes seconds after you, *your* docs run is cancelled and only the newest one deploys. `gh run watch --exit-status` returns non-zero for that, which reads exactly like a build failure, and the published page 404s for a minute while the newer run finishes. Before debugging anything, look at what actually happened:

```bash
gh run list --workflow=docs --json headSha,conclusion,status
```

`cancelled` on your SHA with a newer run in flight means nothing is wrong — the behaviour is correct, you *want* the newest site, and the newer run carries your commit anyway.

The asymmetry is what makes this safe to wave off, and it is worth knowing rather than re-checking: **`examples.yml` has no `concurrency` block at all**, and it is the workflow that runs both `run_examples.py --check` and `check_katas.py`. So a cancellation is always the docs deploy and never the gates — a `cancelled` conclusion cannot be hiding a red gate. The converse is the trap: a green docs deploy is *not* evidence `check_katas.py` passed, because that gate runs in the other workflow.

## When several people share the checkout

More than one person often works in this repo at once, through **one working tree, one index, and one HEAD**. Three habits follow, and each is here because the failure has already happened.

**Commit with a pathspec** — `git commit -F msg -- <paths>` — since a plain `git commit` sweeps up whatever else is staged — and in a shared checkout *whatever else* includes a **colleague's** staged work, because the index is shared too. The asymmetry is worth holding onto: `git commit -- <paths>` builds a *temporary* index and takes only what you name, while bare `git commit`, `-a` and `--amend` all use the main one. So somebody else's staged folder rename lands not because anyone named those paths, but because you made one unscoped commit for unrelated work in another folder — and from your side those staged renames are invisible unless you run `git status`. Verified: with a file staged, `git commit -- other.txt` commits only `other.txt` and leaves the staged one staged. Know what the pathspec does and does not buy you: it commits the **working-tree** version of the paths you name, so if a colleague has uncommitted edits in one of *those* files, your commit publishes them. That is usually harmless — their work lands a few minutes early — but the diff you get is not always the diff you wrote, so read `git show --stat HEAD` afterwards.

**Two ways a commit silently takes less than you meant.** Both report success, and both produce a `--stat` that looks plausible until you read the number.

- `git commit -- <paths>` **does not add untracked files.** It commits the working-tree version of tracked paths, so a brand-new folder or section README named in the pathspec is simply skipped. That is how a section can land with all ten of its lessons and no index page: `mkdocs build --strict` then aborts on the dangling link, on the deploy, for everyone. `git add` the new files first — naming them in the pathspec is not enough.
- `git commit` with **neither** a pathspec nor `-a` commits **only the index.** After a `git mv`, the renames are staged and every *edit* you made afterwards is not, so the commit moves the files and leaves the link migration behind. The tell is a `--stat` whose file count equals the number of renames exactly.

Both were done in one session, in consecutive commits, and each reddened the docs deploy. The check that caught them is the extracted-tree run at the bottom of this page — the working tree passed all four gates at the moment each was pushed.

**Land the page before the row that points at it.** An entry in [`01_Foundations/README.md`](01_Foundations/README.md), [`GLOSSARY.md`](GLOSSARY.md), [`KATAS.md`](KATAS.md), [`TOOLCHAIN.md`](TOOLCHAIN.md) or `NAV_ORDER` whose target is not committed yet makes `mkdocs build --strict` fail — and it fails on the *deploy*, for everyone, not for the author, whose working tree contains the file and resolves the link perfectly. Commit the lesson folder first and its index entries second, even if that means two commits a minute apart. This one has bitten twice.

**Then push once, at the end — never the lesson commit on its own.** The ordering above is right and it has a cost the rule does not mention: `check_katas.py` fails any page carrying a `## Practice` section with no row in `KATAS.md`, and most lessons have a kata, so the *first* of those two commits is red by itself. It runs in `examples.yml`, which is a separate workflow from the docs deploy — a green site build is not evidence that this passed. CI only ever judges the pushed tip, so commit, commit, then push, and the red state never exists anywhere anyone can see it.

That red HEAD is not private, either, which is the part worth caring about in a shared checkout. While it stands there is no clean prefix for anyone else to push from: a peer either publishes a failing tree under their own name or waits for you. So land the pair back to back — and **if you have to stop halfway, leave the page uncommitted rather than committed-and-unregistered.** A dirty working tree inconveniences nobody; a committed-and-unregistered HEAD blocks every session sharing the checkout.

**When two people are writing the same lesson, say which page survives — and do not delete yours until they answer.** Two sessions writing the same topic is the normal failure here, and the fix is a *carve*: message the other one with what overlaps and a one-sentence split of who owns which half. That much already works. What does not work is deleting your own page in the same breath, because the other person is writing you the mirror-image message at that moment — on 2026-08-29 a bounds lesson was drafted twice, each session proposed keeping the *other* one, and within a minute the repo had no bounds page at all. Both were untracked, so both were unrecoverable from git; the surviving copy came out of a transcript.

So the carve message states a decision rather than an offer — *I am keeping mine* or *I am deleting mine, tell me if you disagree* — and the delete waits for the reply. If you must act before they answer, `git stash`-free and safe: move the folder aside rather than removing it, and keep it until the other page is committed. The trade is worth naming: two pages on one topic is a bad day for a reader, and zero pages is a bad day nobody notices for a month.

**Never `git checkout HEAD -- <a shared file>`.** It deletes a colleague's uncommitted work with no reflog entry to recover it from — strictly worse than the sweep above, which at least publishes their work rather than destroying it. If you genuinely need HEAD's version of a shared file, to commit your own lines without someone's half-finished ones, copy it aside and restore it **in the same command** — never as a later link in an `&&` chain, because an interleaved commit can turn an earlier link into a no-op, the chain stops there, and the restore never runs. That is exactly how the borrowing lesson came to be committed with none of its index entries.
