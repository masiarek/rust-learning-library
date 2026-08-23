# The round trip

**Level:** 201 · working knowledge

**One line:** Write it, read it back, assert it is the same value — the round-trip test is the cheapest test in this whole section, and it is also how you find out what your `save` and `load` signatures should have been.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The test first: two functions that do not exist yet, called the way you *wish* they were called, and the compiler errors as a design conversation
- `serde_json::to_writer` / `from_reader` against `to_string` / `from_str`, and when the buffered ones matter
- The derives a test needs and the code does not: `PartialEq` to compare and `Debug` so a failure prints something you can read
- What a round trip does **not** prove — the on-disk shape. Two programs agreeing with themselves can still disagree with each other, which is why a stored fixture of real JSON belongs beside it
- Floating point in a round trip, where "the same value" stops being obvious

## The trap it exists for

A round-trip test passes forever under a *renamed field*, because both halves renamed together. It proves the pair is consistent, not that today's program can read yesterday's file — which is the property anyone with saved data actually cares about.

## See also

- [Deriving `Serialize` and `Deserialize`](../serde_derive/README.md) — the two traits this exercises
- [Temporary directories in tests](../../04_Files/temp_dirs_in_tests/README.md) — where the file in the middle of the round trip should live
- [What a float actually stores](../../01_Foundations/what_a_float_stores/README.md) — why "the same number" needs care on the way back
