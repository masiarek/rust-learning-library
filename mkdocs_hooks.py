"""Build-time fixes that would otherwise cost a pinned plugin dependency.

MkDocs derives a sidebar *section* label from the folder name on disk, so
`01_Foundations/` reads as "01 Foundations" — ordering prefix and all. That
number exists to keep a file listing tidy; in the nav it is noise, and here it
is worse than noise, because reading order is not creation order. The sidebar
ran 00, 01, 15, 16, 17, 18, 14, 19, 13, 12, 02, 03 … — which looks broken and
was in fact correct.

Three jobs:

1. **Clean section labels** — strip the ordering prefix, turn underscores into
   spaces, sentence-case the result (the house style of every page's own `# H1`:
   "What a struct is", not "What a Struct Is"), and apply `LABELS` / `FIXUPS` to
   the names a caser cannot get right (`llvm_and_its_ir` → "LLVM and its IR",
   `if_let` → "`if let`").
2. **Order the sections** — `NAV_ORDER` states the intended reading order per
   folder, keyed by folder path, listing children by their on-disk name. The
   **root is the exception**: it sorts alphabetically, because the question a
   reader asks of a top-level list is "where is the section I can already name",
   and A–Z answers it. Only Home and Start here are pinned. The reading order
   that used to live here, as a numbered `1.`–`21.`, is now a table on the
   homepage under "The course, in order" — a sidebar can be sorted one way, and
   this was the less common question.
3. **Re-thread previous/next** — MkDocs computes the footer's ← → chain *before*
   a hook can reorder anything, so without this the sidebar and the footer
   disagreed on every page: the homepage's "Next" was the Glossary (alphabetically
   first at the root), and Foundations' was Errors rather than the First programs
   it actually hands the reader to. The chain is rebuilt from the sorted tree.

Why order here rather than by renaming folders: a folder name is a permanent URL.
Renumbering `03_` to `04_` to insert a lesson would move every page under it and
break any link anyone ever saved. Ordering is presentation, so it belongs in the
presentation layer — the numbers the reader sees included. They count reading
position; the ones on disk are just what the folder happens to be called.

**The label is computed from the on-disk folder name, not from the title MkDocs
derived from it.** That distinction is the whole reason two of these jobs were
silently doing nothing for as long as the file has existed: by the time a hook
sees a section, `dirname_to_title()` has already turned `llvm_and_its_ir` into
"Llvm and its ir", so a `LABELS` map keyed by folder name — as this one always
was — matched nothing at all, and `PREFIX`, which looks for `01_`, was being
handed `01 `.
"""

from __future__ import annotations

import pathlib
import re

# Words a sentence-caser gets wrong: acronyms, and the identifiers that read
# better as code. Keyed by the lowercased word, applied at any position.
FIXUPS = {
    "api": "API",
    "cli": "CLI",
    "ffi": "FFI",
    "http": "HTTP",
    "io": "I/O",
    "json": "JSON",
    "llvm": "LLVM",
    "mcp": "MCP",
    "opentelemetry": "OpenTelemetry",
    "rust": "Rust",
    "ui": "UI",
    "url": "URL",
    "arc": "`Arc`",
    "cow": "`Cow`",
    "i128": "`i128`",
    "option": "`Option`",
    "rc": "`Rc`",
    "result": "`Result`",
    "str": "`str`",
    "vec": "`Vec`",
}

# Reading order per folder path. Children named by on-disk name; anything not
# listed sorts alphabetically after the listed ones.
NAV_ORDER: dict[str, list[str]] = {
    # The root is sorted ALPHABETICALLY, not in reading order (changed
    # 2026-08-29 at Adam's request: "it is easier to find using alphabet
    # sorting"). Only the two doors are pinned -- Home, then Start here, which
    # would otherwise land under S in the middle of the list.
    #
    # Everything else is sorted by `_root_key` below: sections first, then the
    # loose reference pages, each run A-Z. Nothing else belongs in this list;
    # adding a section needs no edit here at all, which is the other reason for
    # the change -- the hand-kept order was a file every new section had to
    # touch.
    #
    # The reading order it replaced is NOT lost: it is a numbered table on the
    # homepage, under "The course, in order". If you reorder the course, that
    # table is the place to do it.
    "": [
        "index.md",
        "00_Start_Here",
    ],
    "00_Start_Here": [
        "README.md",
        "the_book",
        "rust_by_example",
        "rustlings",
    ],
    "13_Enums": [
        "README.md",
        # The declaration, then what a payload costs, then the two pages about
        # what `match` does and does not check for you.
        "what_an_enum_is",
        "variants_that_carry_data",
        "a_typo_becomes_a_binding",
        "an_enum_as_a_state_machine",
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
        # ...and its neighbour in std::marker: the parameter with no data
        # behind it, which only makes sense once markers are in hand.
        "phantom_types",
        "to_owned",
        "clone_into",
        "resources",
    ],
    "01_Foundations": [
        "README.md",
    ],
    "22_Generics": [
        "README.md",
        # The brackets first, then the error everyone meets next, then what a
        # `T` is allowed to do...
        "what_a_generic_is",
        "when_the_compiler_cannot_infer",
        "where_the_bound_goes",
        # ...and then the two shapes that need all three: the same parameter
        # on an enum, and the type that contains itself.
        "generic_enums",
        "a_generic_recursive_type",
    ],
    "20_Compilers": [
        "README.md",
        # The compile-time/run-time line first, since every later page
        # assumes it; then the middle stage, then the machinery it runs
        # on, then the stage that is nobody's compiler.
        "what_a_compiler_does",
        "what_the_optimizer_does",
        "llvm_and_its_ir",
        "the_linker",
        # ...and the same permission aimed backwards.
        "control_flow_flattening",
        # The four stubs, in the order the section README lists them — so the
        # sidebar and that page cannot disagree about what comes next.
        "targets_and_triples",
        "compiled_or_interpreted",
        "reading_a_compilation_failure",
        "build_systems_are_not_compilers",
    ],
    "15_First_Programs": [
        "README.md",
        # How to run any of this at all, before the language itself.
        "rustc_without_cargo",
        # ...and what the text in that file means, before any of it is Rust:
        # two of the four comment forms are parsed, not discarded.
        "comments_that_compile",
        # ...and what the compiler says back about it. Third because everyone
        # meets `unused variable` on day one; its Drop half pays off much later.
        "what_a_warning_is_asking",
        # ...and then the two pieces of punctuation every page below is
        # already using without explaining: the braces that hold a program
        # together, and the braces that print.
        "a_block_is_an_expression",
        "braces_take_a_name",
        # ...and the third piece of punctuation nobody defines: the `:` in
        # `let s: &str`. On a literal it changes nothing, which is why it is
        # easy to read as decoration; on four other shapes it decides the type.
        "what_an_annotation_does",
        # ...and then the other half of printing, once the braces are
        # understood: which of the two traits fills them, and why only one
        # of them can be derived.
        "debug_vs_display",
        # The macro that needs the trait above, and does five things
        # println!("{:?}") does not.
        "what_dbg_does",
        # Last, because it is the first thing that needs anything outside
        # the standard library: the crate almost everyone adds first, and
        # the API the Book still teaches under its old names.
        "randomness",
    ],
    "16_Structs": [
        "README.md",
        # A compound type of your own, before the two the library leans on.
        "what_a_struct_is",
        # ...and where a value of one actually comes from.
        "a_type_is_not_a_constructor",
        "impl_blocks",
        "struct_update",
        "copy_vs_clone",
        # Closes the section with the errors a struct actually produces —
        # eight of them, and the fix each is asking for.
        "when_a_struct_refuses",
        "newtype_score",
        "representing_a_ballot",
    ],
    "17_Option_and_Result": [
        "README.md",
        # The enum itself before anything built on it.
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
        "optional_arguments",
        "option_fields",
        "option_as_collection",
        "nullable_pointers",
        "result_aliases",
        "shadowing_and_unwrap",
        # Closes the section by asking what to do when two variants are not
        # enough: "blank is not zero" stops at two cases, markers need six.
        "six_kinds_of_zero",
    ],
    "18_Ownership": [
        "README.md",
        # Opens the ownership arc — the next thread after the Option run.
        "ownership_and_moves",
        # ...and the trait that is not there, which is why the default needed
        # naming in the first place.
        "no_move_trait",
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
        # Last, as the section README has it: the type that refuses to decide
        # between owning and borrowing until the data makes it.
        "clone_on_write",
        # ...and the other two ways out of a copy, in the order they get
        # reached for: several owners counted, then the same count made
        # atomic so it can cross a thread boundary.
        "reference_counting",
        "sharing_across_threads",
    ],
    "19_Numbers": [
        "README.md",
        # Opens the numbers arc: the unit every other size is counted in.
        "meet_the_byte",
        # ...and how to write one down, which is the last thing before meaning.
        "why_hexadecimal",
        # ...and what people put in one, once they can write it down.
        "bit_flags",
        # ...and the one type that cannot hold the value you typed.
        "what_a_float_stores",
        # Closes the numbers arc on what follows from that: the order of a float
        # sum is part of its answer, so the compiler may not choose it for you —
        # unless you say so. Hands off to the 09_Advanced exactness cluster.
        "letting_the_compiler_reorder",
    ],
    "14_Strings/str_methods": ["README.md"],
    "14_Strings/string_methods": ["README.md"],
    "14_Strings": [
        "README.md",
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
        # The method reference: one page per method, 125 of them. It sits after
        # the lessons and before the stubs because a stub is not a page anyone
        # reads, and burying a reference this size under nine placeholders
        # would hide the most-linked-to part of the section.
        "str_methods",
        "string_methods",
        # The nine that are outlines so far — ordered as they would be read,
        # not as they were written. Each is a real URL from the day it is a stub.
        "parsing_a_string",
        "searching_a_string",
        "the_format_language",
        "raw_strings_and_escapes",
        "comparing_strings",
        "str_is_unsized",
        "boxed_str",
        "string_api_design",
        "when_string_is_too_slow",
        # ...and the outside world: books, essays, the video, and the exercises.
        "resources",
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
        # ...and the protocol that wires an agent into the window just
        # configured, which is also a small Rust program in its own right.
        "what_mcp_is",
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
        # Books argue; these define. Second, because the question "what does
        # Rust actually do here" arrives the moment the first book is closed.
        "official_docs",
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
        # ...and where the bytes those layouts occupy actually came from.
        "the_global_allocator",
    ],
    "21_Observability": [
        "README.md",
        # The case for doing any of it, then the three signals it is made of.
        "why_observability_matters",
        "the_three_pillars",
        # What a Rust program actually emits: a line, an interval, and the
        # async trap that makes the interval harder here than elsewhere.
        "structured_logging",
        "spans_not_lines",
        "instrumenting_async",
        # ...and the same loss of context one boundary further out.
        "context_propagation",
        # The third signal, which is the one with a bill attached.
        "metrics_and_cardinality",
        # Where it all lands: the spec, the collector, and the budget.
        "what_opentelemetry_is",
        "where_the_data_goes",
        "sampling",
        # Last, because it is the only recommendation in the section.
        "what_to_instrument",
    ],
}

# Labels the sentence-caser cannot reach on its own: proper names, code
# identifiers, and the folders whose section README already gives them a shorter
# name than their own H1. Keyed by ON-DISK FOLDER NAME — which only works
# because `clean()` is handed that name rather than MkDocs's rendering of it.
LABELS = {
    # The sections. Folder names are Title_Cased so a file listing reads well;
    # the sidebar follows each section page's own `# H1` instead.
    "00_Start_Here": "Start here",
    "03_Command_Line": "Command line",
    "15_First_Programs": "First programs",
    "19_Numbers": "Numbers and bytes",
    # 00_Start_Here — three product names, one of them lowercase on purpose.
    "the_book": "The Book",
    "rust_by_example": "Rust by Example",
    "rustlings": "rustlings",
    # 02_Errors
    "unwrap_is_a_todo": "`unwrap` is a TODO",
    "keep_going_or_stop": "Keep going, or stop",
    "main_returns_result": "`main` can return a `Result`",
    "stderr_and_exit_status": "Standard error, and exit status",
    "not_every_error_is_io_error": "Not every error is an `io::Error`",
    "anyhow_and_context": "`anyhow` and context",
    "thiserror_vs_anyhow": "`thiserror` vs `anyhow`",
    # 03_Command_Line
    "command_line_arguments": "Command-line arguments",
    "clap_derive": "Deriving a parser with `clap`",
    "the_default_trait": "The `Default` trait",
    # 04_Files
    "path_and_pathbuf": "`Path` and `PathBuf`",
    "temp_dirs_in_tests": "Temporary directories in tests",
    # 05_Tooling — three tools are spelled lowercase wherever they appear, and
    # several pages have a long subtitled H1 the sidebar has no room for.
    "bacon": "bacon",
    "rustup": "rustup",
    "devenv": "devenv",
    "nextest": "cargo-nextest",
    "editors": "Choosing an editor",
    "nightly": "Nightly by default",
    "scratch_with_a_crate": "A throwaway that needs a crate",
    "cargo_dependencies": "Adding a dependency",
    "practice_workspace": "A tree of practice projects",
    "strict_lints": "Strict clippy lints",
    "scaffolding": "Scaffolding a practice tree",
    "rustrover_setup": "RustRover setup",
    "rustrover_code_vision": "RustRover Code Vision",
    "neovim_setup": "Neovim with LazyVim",
    # 06_Data · 07_Clients
    "serde_derive": "Deriving `Serialize` and `Deserialize`",
    "json_round_trip": "The round trip",
    "http_with_reqwest": "An HTTP request",
    # 09_Advanced
    "one_person_one_vote": "One person, one vote",
    # 10_Resources — a topic page on the shelf, named for the shelf so it is not
    # mistaken for the Structs section.
    "structs": "Structs: the shelf",
    # 12_Traits
    "trait_in_scope": "A trait must be in scope",
    "to_owned": "`ToOwned`",
    # 14_Strings
    "string_vs_str": "`String` vs `&str`",
    "anatomy_of_a_string": "The anatomy of a `String`",
    "making_a_string": "Making a `String`",
    "building_a_string": "Building a `String`",
    "walking_a_string": "Walking a `String`",
    "meet_the_char": "Meet the `char`",
    "static_str": "`&'static str`",
    "parsing_a_string": "Parsing out of a string",
    "the_format_language": "The format mini-language",
    "when_string_is_too_slow": "When `String` is too slow",
    # The two method-reference sections. Set explicitly so the pair is
    # symmetric: without this, FIXUPS turns `str_methods` into "`str` methods"
    # while `string_methods` gets a plain "String methods".
    "str_methods": "`str` methods",
    "string_methods": "`String` methods",
    # 15_First_Programs
    "rustc_without_cargo": "Running a scratch program",
    "what_an_annotation_does": "What a type annotation does",
    "debug_vs_display": "`Debug` and `Display`",
    "what_dbg_does": "What `dbg!` does",
    # 16_Structs
    "impl_blocks": "`impl` blocks",
    "struct_update": "Struct update syntax",
    "copy_vs_clone": "`Copy` vs `Clone`",
    "newtype_score": "A score is not a number",
    "representing_a_ballot": "What is a ballot, in memory?",
    # 17_Option_and_Result — these are method names, so they are code.
    "some_and_none": "`Some` and `None`",
    "some_is_a_constructor": "`Some` is a constructor",
    "if_let": "`if let`",
    "while_let": "`while let`",
    "unwrap_or": "`unwrap_or`",
    "unwrap_or_else": "`unwrap_or_else`",
    "unwrap_or_default": "`unwrap_or_default`",
    "map_or": "`map_or`",
    "expect": "`expect`",
    "none_on_error": "Returning `None` on error",
    "option_as_collection": "`Option` as a collection",
    "shadowing_and_unwrap": "Shadowing and `unwrap`",
    "wrong_guard": "Zero wins is not zero games",
    # 18_Ownership
    "no_move_trait": "There is no `Move` trait",
    "shadowing_does_not_drop": "A shadow does not drop",
    "clone_on_write": "`Cow`, clone on write",
    # 20_Compilers
    "llvm_and_its_ir": "LLVM and its IR",
    "control_flow_flattening": "Control-flow flattening",
    "build_systems_are_not_compilers": "A build system is not a compiler",
    # 21_Observability
    "spans_not_lines": "A span is not a log line",
    "what_to_instrument": "What to instrument first",
}

# ---------------------------------------------------------------------------
# The str/String method reference: one folder per method, 125 of them, under
# 14_Strings/str_methods and 14_Strings/string_methods. Every label is
# mechanical -- `str_trim` reads `str::trim` -- so the rule is written once
# instead of as 125 near-identical LABELS lines.
#
# The names are LISTED rather than derived from the `str_`/`string_` prefix,
# because `string_vs_str`, `string_slices` and `str_is_unsized` are lessons in
# the same section and must keep the prose labels set above. clean() is keyed
# on the bare folder name with no path, so a prefix rule could not tell them
# apart.
# ---------------------------------------------------------------------------
STR_METHODS = """
    as_bytes as_bytes_mut as_mut_ptr as_ptr bytes ceil_char_boundary
    char_indices chars contains encode_utf16 ends_with eq_ignore_ascii_case
    escape_debug escape_default escape_unicode find floor_char_boundary
    from_utf8 from_utf8_mut from_utf8_unchecked from_utf8_unchecked_mut get
    get_mut get_unchecked get_unchecked_mut into_boxed_bytes into_string
    is_ascii is_char_boundary is_empty len lines lines_any
    make_ascii_lowercase make_ascii_uppercase match_indices matches parse
    repeat replace replacen rfind rmatch_indices rmatches rsplit
    rsplit_once rsplit_terminator rsplitn slice_mut_unchecked
    slice_unchecked split split_ascii_whitespace split_at split_at_checked
    split_at_mut split_at_mut_checked split_inclusive split_once
    split_terminator split_whitespace splitn starts_with strip_circumfix
    strip_prefix strip_suffix substr_range to_ascii_lowercase
    to_ascii_uppercase to_lowercase to_uppercase trim trim_ascii
    trim_ascii_end trim_ascii_start trim_end trim_end_matches trim_left
    trim_left_matches trim_matches trim_right trim_right_matches trim_start
    trim_start_matches
""".split()

STRING_METHODS = """
    as_bytes as_mut_str as_mut_vec as_str capacity clear drain
    extend_from_within from_raw_parts from_utf16 from_utf16_lossy
    from_utf16be from_utf16be_lossy from_utf16le from_utf16le_lossy
    from_utf8 from_utf8_lossy from_utf8_unchecked insert insert_str
    into_boxed_str into_bytes into_raw_parts is_empty leak len new pop push
    push_str remove replace_range reserve reserve_exact retain shrink_to
    shrink_to_fit split_off truncate try_reserve try_reserve_exact
    with_capacity
""".split()

LABELS.update({f"str_{m}": f"str::{m}" for m in STR_METHODS})
LABELS.update({f"string_{m}": f"String::{m}" for m in STRING_METHODS})

PREFIX = re.compile(r"^\d+[_\- ]")


def clean(name: str) -> str:
    """`01_Foundations` -> "Foundations"; `option_vs_result` -> "`Option` vs `Result`".

    Takes the **on-disk folder name**, not the title MkDocs derived from it. See
    the module docstring: that distinction is the difference between this map
    working and doing nothing at all.
    """
    if name in LABELS:
        return LABELS[name]
    words = PREFIX.sub("", name).replace("_", " ").replace("-", " ").split()
    out = []
    for i, word in enumerate(words):
        fixed = FIXUPS.get(word.lower())
        if fixed is not None:
            out.append(fixed)
        elif word.isupper():  # README, STAR — spelled that way on purpose
            out.append(word)
        elif i == 0:
            out.append(word[:1].upper() + word[1:])
        else:
            # Sentence case, which is the house style of every page's own H1.
            out.append(word)
    return " ".join(out)


def _src(item) -> str | None:
    """A nav item's source path, slash-separated."""
    if item.is_page and item.file is not None:
        return item.file.src_path.replace("\\", "/")
    return None


def _dirname(item) -> str | None:
    """The on-disk folder a section is built from: `15_First_Programs`."""
    if item.is_section and item.children:
        src = _src(item.children[0])
        if src and "/" in src:
            return src.rsplit("/", 2)[-2]
    return None


def _name_of(item) -> str | None:
    """An item's NAV_ORDER key: a file name, or a section's folder name."""
    src = _src(item)
    if src is not None:
        return src.rsplit("/", 1)[-1]
    return _dirname(item)


def _folder_of(item) -> str:
    """Repo-relative folder path a nav item's children live in."""
    src = _src(item)
    if src is None and item.is_section and item.children:
        src = _src(item.children[0])
    if not src:
        return ""
    return src.rsplit("/", 1)[0] if "/" in src else ""


def _label_sections(items) -> None:
    """Retitle every section from its folder name, at any depth."""
    for item in items:
        if item.is_section:
            name = _dirname(item)
            if name:
                item.title = clean(name)
            _label_sections(item.children)


def _order(items, folder: str) -> None:
    """Sort `items` in place to match NAV_ORDER[folder], then walk deeper."""
    order = NAV_ORDER.get(folder)
    if order:

        def key(item):
            name = _name_of(item)
            if name in order:
                return (0, order.index(name))
            return (1, item.title or "")

        items.sort(key=key)

    for item in items:
        if item.is_section:
            _order(item.children, _folder_of(item))


ARTICLE = re.compile(r"^(?:the|a|an)\s+")
H1 = re.compile(r"^#\s+(.+?)\s*$", re.MULTILINE)


def _sort_text(title: str) -> str:
    """Sort key for a sidebar label, as a reader would alphabetise it.

    Three normalisations, each earning its place:

    * **Strip Markdown.** Labels carry code spans, and a raw sort puts
      "`Option` and `Result`" first because a backtick sorts before every
      letter.
    * **Strip the ordering prefix**, so `01_Foundations` files under F.
    * **Strip a leading article**, the way a library catalogue does. Six of the
      eight reference pages are named "The ... map", so sorting on the literal
      string files six of them under T and hides the one word that tells them
      apart.
    """
    t = PREFIX.sub("", title.replace("`", "").replace("&", "")).strip().lower()
    return ARTICLE.sub("", t)


def _page_title(item) -> str:
    """A nav page's visible label, which MkDocs has not computed yet.

    `page.title` is filled in when the page is *rendered*, which happens long
    after `on_nav`, so at sort time it is None for every page whose title comes
    from its own `# H1`. Sorting on that put "The Option map" above "The long
    way round…" and looked correct only because the filenames beneath them
    happened to be alphabetical. Read the H1 instead.
    """
    if item.title:
        return item.title
    src = getattr(item.file, "abs_src_path", None) if item.file else None
    if src:
        try:
            m = H1.search(pathlib.Path(src).read_text(encoding="utf-8"))
        except OSError:
            return ""
        if m:
            return m.group(1)
    return ""


def _root_key(item):
    """Alphabetical, with the two doors pinned and sections above loose pages.

    Keeping sections ahead of the topic maps preserves the sidebar's shape --
    folders, then the .md pages that only sequence other folders -- so A-Z
    changes where things are without changing what kind of thing they are.
    """
    name = _name_of(item)
    pinned = NAV_ORDER[""]
    if name in pinned:
        return (0, pinned.index(name), "")
    text = item.title if item.is_section else _page_title(item)
    return (1 if item.is_section else 2, 0, _sort_text(text or ""))


def _relink(nav) -> None:
    """Rebuild nav.pages and the footer's ← → chain from the sorted tree.

    MkDocs threads previous/next inside `get_navigation()`, which runs *before*
    any `on_nav` hook, so reordering the tree here leaves every footer arrow
    pointing along the old alphabetical order — the homepage's "Next" was the
    Glossary, and Foundations' was Errors. Rebuilt from the tree the reader sees.
    """
    pages = []

    def walk(items):
        for item in items:
            if item.is_page:
                pages.append(item)
            elif item.is_section:
                walk(item.children)

    walk(nav.items if hasattr(nav, "items") else nav)
    for page in pages:
        page.previous_page = None
        page.next_page = None
    for prev, nxt in zip(pages, pages[1:]):
        prev.next_page = nxt
        nxt.previous_page = prev
    if hasattr(nav, "pages"):
        nav.pages = pages


def on_nav(nav, config, files):
    # `nav.items` is the real list, so sort it in place. Wrapping it in
    # `list(...)` sorted a throwaway copy, which is why the root order
    # silently never applied while every nested folder's did.
    items = nav.items if hasattr(nav, "items") else nav
    _label_sections(items)
    # The root sorts alphabetically; every folder below it keeps the reading
    # order NAV_ORDER states, because a lesson sequence IS a sequence.
    items.sort(key=_root_key)
    for item in items:
        if item.is_section:
            _order(item.children, _folder_of(item))
    _relink(nav)
    return nav
