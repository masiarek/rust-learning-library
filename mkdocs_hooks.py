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
        "SHADOWING.md",
        "STRUCTS.md",
        "STRINGS.md",
        "TOOLCHAIN.md",
        "ROADMAP.md",
        "KATAS.md",
        "00_Start_Here",
        "01_Foundations",
        # ...and the feature the rest of the language is built out of, which is
        # why it comes straight after the foundations rather than at the end.
        "12_Traits",
        # The command-line-tool arc: what a program does about failure, what it
        # is handed on the way in, and the three things outside it that can say
        # no — the filesystem, a saved file, a server.
        "02_Errors",
        "03_Command_Line",
        "04_Files",
        "05_Tooling",
        "06_Data",
        "07_Clients",
        "08_Interfaces",
        "09_Advanced",
        "10_Resources",
        # The shell the compiler is run from — not Rust, but two of its three
        # tools are written in it, which is the section's reason for existing.
        "11_Unix",
        "GLOSSARY.md",
    ],
    "00_Start_Here": [
        "README.md",
        "the_book",
        "rust_by_example",
        "rustlings",
    ],
    "12_Traits": [
        "README.md",
        # The declaration, then how a call reaches it, then what happens when a
        # function has to hand one back.
        "what_a_trait_is",
        "trait_in_scope",
        "returning_a_trait",
        # One std trait in depth, and the one whose name causes the most
        # confusion next to `clone`.
        # ...and then the three that need all of the above.
        "static_vs_dynamic_dispatch",
        "supertraits",
        "marker_traits",
        "to_owned",
        "resources",
    ],
    "01_Foundations": [
        "README.md",
        # How to run any of this at all, before the language itself.
        "rustc_without_cargo",
        # ...and what the text in that file means, before any of it is Rust:
        # two of the four comment forms are parsed, not discarded.
        "comments_that_compile",
        # ...and what the compiler says back about it. Third because everyone
        # meets `unused variable` on day one; its Drop half pays off much later.
        "what_a_warning_is_asking",
        # ...and the messages that DO stop the build, read the same way.
        "when_a_struct_refuses",
        # ...and then the two pieces of punctuation every page below is
        # already using without explaining: the braces that hold a program
        # together, and the braces that print.
        "a_block_is_an_expression",
        "braces_take_a_name",
        # A compound type of your own, before the two the library leans on.
        "what_a_struct_is",
        # ...and where a value of one actually comes from.
        "a_type_is_not_a_constructor",
        "impl_blocks",
        "struct_update",
        "copy_vs_clone",
        "some_and_none",
        "some_is_a_constructor",
        "option_vs_result",
        # ...and, much later, the name for the shape all of these share.
        "what_a_monad_is",
        "if_let",
        "while_let",
        # ...and what goes INSIDE a pattern, now that three pages have been
        # writing them: the `|` and the `..=` that let one arm cover many values.
        "one_arm_many_values",
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
        # The first crate almost everyone adds, and the API the Book still
        # teaches under its old names.
        "randomness",
        "optional_arguments",
        "option_fields",
        "option_as_collection",
        "nullable_pointers",
        "result_aliases",
        "shadowing_and_unwrap",
        # Closes the Result run by asking how any of it reaches a person:
        # the sentence you wrote for a human, and the paths that print the other one.
        "debug_vs_display",
        # The macro that needs the trait above, and does five things
        # println!("{:?}") does not.
        "what_dbg_does",
        # Opens the ownership arc — the next thread after the Option run.
        "ownership_and_moves",
        # ...and the thing everyone tries first to SEE a move happen, which
        # shows the header moving and the bytes staying put.
        "what_an_address_shows",
        "borrowing",
        # ...and the scaffold most people are told to use while it lands.
        "how_to_learn_lifetimes",
        # The borrow rule immediately pays for itself: it is what proves a
        # shadow makes a second place, since it accepts `let y = &x; let x = 6`
        # and rejects the `mut` spelling of the same lines.
        "a_name_is_not_a_place",
        # Closes the ownership arc by putting shadowing back through it:
        # what a shadow does to the value, rather than to the name.
        "shadowing_does_not_drop",
        # ...and then the judgement call both of those leave open, which needs
        # the name half AND the value half already in the reader's hands.
        "when_to_shadow",
        # ...and last, the question the judgement call raises: what would
        # have caught me? Needs the three bugs already seen to land.
        "nothing_checks_a_shadow",
        # Closes the arc by separating the three things "out of scope" is
        # asked to mean. Last, because it measures the borrow region, the
        # drop order and the shadow — all three already met by here.
        "scope_is_about_names",
        # Opens the strings arc — the pair of types the ownership pages were
        # secretly about all along: one owns the text, one looks at it.
        "string_vs_str",
        # ...then the view in its own right: what a slice stores, the stale
        # index it replaces, and the byte offsets that make it panic.
        "string_slices",
        # ...then what the owning half is made of: three words on the stack,
        # bytes on the heap, and a capacity that is not the length.
        "anatomy_of_a_string",
        # ...how to get one: five spellings, and the trait behind the useful one.
        "making_a_string",
        # ...then the first thing anyone does with two of them, and the E0369
        # that refuses it: `+` grows the left operand, so the left must own one.
        "concatenating_strings",
        # ...and how to grow one, plus the operator that eats its left operand.
        "building_a_string",
        # ...then what those bytes encode: a char is not a byte, and
        # "how many characters" has three honest answers.
        "meet_the_char",
        # ...and how to walk them: three item types, and the split family.
        "walking_a_string",
        # The lifetime half, once the two types are solid: three spellings of
        # one type, and the claim about String that is not true.
        "static_str",
        # Closes the arc: the other four string types, and the one
        # owned/borrowed pattern all six repeat.
        "six_kinds_of_string",
        "newtype_score",
        "representing_a_ballot",
        # The sequel: "blank is not zero" stops at two cases; markers need six.
        "six_kinds_of_zero",
        # Opens the numbers arc: the unit every other size is counted in.
        "meet_the_byte",
        # ...and how to write one down, which is the last thing before meaning.
        "why_hexadecimal",
        # ...and what people put in one, once they can write it down.
        "bit_flags",
        # Closes the numbers arc, and hands off to the 09_Advanced exactness cluster.
        "what_a_float_stores",
    ],
    # Reading order for the command-line-tool arc. These sections are stubs for
    # now — outlines with no examples behind them yet — so the order is the one a
    # finished section would want, not the order they were written in.
    "02_Errors": [
        "README.md",
        # Failures you did not notice: the Result you threw away, and the loop
        # that never ends because "no more input" is not an error.
        "readers_are_fallible",
        "endless_iteration",
        # ...then failures that reach a person.
        "main_returns_result",
        "stderr_and_exit_status",
        "keep_going_or_stop",
        # ...and last, the type they travel in, which is the deepest question.
        # ...but first: why there is an unwrap there at all, which is almost
        # never a decision and almost always a paste from a crate's own README.
        "unwrap_is_a_todo",
        "not_every_error_is_io_error",
        "anyhow_and_context",
        "thiserror_vs_anyhow",
    ],
    "03_Command_Line": [
        "README.md",
        "command_line_arguments",
        "flags_by_hand",
        "clap_derive",
        # The struct the flags land in, once there is more than one.
        "the_default_trait",
        "arguments_and_environment",
        # Last, because it needs all of the above to have something to assert on.
        "testing_a_command",
    ],
    "04_Files": [
        "README.md",
        "opening_a_file",
        "path_and_pathbuf",
        "reading_lines_efficiently",
        "missing_is_not_empty",
        "temp_dirs_in_tests",
    ],
    "05_Tooling": [
        "README.md",
        # Day zero: the window you will be reading all of this through, and the
        # one decision that is cheapest to make badly and expensive to revisit.
        "editors",
        # The toolchain arc, cheapest answer first — TOOLCHAIN.md is its map.
        # Which compiler actually runs, how to make that a decision rather than
        # luck, and the one channel choice invisible from inside the project.
        "rustup",
        "pinning_the_toolchain",
        "nightly",
        # Then what the build pulls in: the three commands that turn a loose
        # .rs file into a project that can have a dependency at all, what the
        # requirement they wrote actually permits, and how to lay out many
        # small projects so the config above is written once rather than
        # copied per folder.
        "scratch_with_a_crate",
        "cargo_dependencies",
        "practice_workspace",
        # Day one, and cheap: settle the whitespace question before anything else.
        "formatting",
        "strict_lints",
        # The inner loop: feedback without asking for it, and the IDE wiring
        # that connects the window to everything configured above.
        "bacon",
        "nextest",
        "commit_on_green",
        "rustrover_setup",
        "rustrover_code_vision",
        "neovim_setup",
        # Then, having met all of the above: the script that writes them, and
        # the checks that keep the files agreeing with each other afterwards.
        "scaffolding",
        "compile_times",
        # Last, because it subsumes every rung above and costs the most.
        "devenv",
    ],
    "06_Data": [
        "README.md",
        "serde_derive",
        "json_round_trip",
        "a_type_instead_of_a_vec",
    ],
    "07_Clients": [
        "README.md",
        "http_with_reqwest",
        "deserializing_a_response",
        # Testability is one move — turn a decision into a parameter — so the
        # page that makes it comes before the page that spends it.
        "injecting_the_base_url",
        "mocking_a_server",
        "units_are_types",
    ],
    "08_Interfaces": [
        "README.md",
        "rust_ui_options",
    ],
    "10_Resources": [
        "README.md",
        "books",
        "exercises",
        "going_deeper",
        "haskell",
        # The shelf's one topic page; last, because it is the odd one out —
        # the four above sort by the moment you are in, this one by subject.
        "structs",
    ],
    "11_Unix": [
        "README.md",
        # The interactive tool first: it is the one with a visible payoff on
        # the first keypress, and it is what sends you to fd.
        "fuzzy_finding",
        # ...then why the list it is reading was cheap to produce.
        "search_tools_in_rust",
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
        # Layout rather than arithmetic: the other thing `unsafe` buys you, and
        # the enum that already gives it to you safely.
        "what_a_union_is",
    ],
}

# Folder names whose label the word-by-word caser cannot get right.
LABELS = {
    # A product name that is lowercase on purpose; the title-caser would say "Devenv".
    "devenv": "devenv",
    "neovim_setup": "Neovim with LazyVim",
    "practice_workspace": "A tree of practice projects",
    "rustrover_setup": "RustRover setup",
    "nextest": "cargo-nextest",
    "bacon": "bacon",
    # Likewise: the tool is spelled lowercase everywhere it appears.
    "rustup": "rustup",
    "if_let": "`if let`",
    "while_let": "`while let`",
    "main_returns_result": "`main` can return a `Result`",
    "stderr_and_exit_status": "Standard error, and exit status",
    "not_every_error_is_io_error": "Not every error is an `io::Error`",
    "anyhow_and_context": "`anyhow` and context",
    "thiserror_vs_anyhow": "`thiserror` vs `anyhow`",
    "flags_by_hand": "Flags by hand",
    "clap_derive": "Deriving a parser with `clap`",
    "the_default_trait": "The `Default` trait",
    "path_and_pathbuf": "`Path` and `PathBuf`",
    "string_vs_str": "`String` vs `&str`",
    "anatomy_of_a_string": "The anatomy of a `String`",
    "meet_the_char": "Meet the `char`",
    "string_slices": "String slices",
    "making_a_string": "Making a `String`",
    "concatenating_strings": "Concatenating strings",
    "building_a_string": "Building a `String`",
    "walking_a_string": "Walking a `String`",
    "static_str": "`&'static str`",
    "temp_dirs_in_tests": "Temporary directories in tests",
    "serde_derive": "Deriving `Serialize` and `Deserialize`",
    "json_round_trip": "The round trip",
    "a_type_instead_of_a_vec": "A type instead of a `Vec`",
    "http_with_reqwest": "An HTTP request",
    "injecting_the_base_url": "Injecting the base URL",
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
