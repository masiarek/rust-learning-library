# A log line a machine can read

**Level:** 201 · working knowledge

**One line:** A line written for a person is a sentence; a line written for a query is a set of named fields — and which one you chose decides whether the question you have six months from now is answerable at all.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The same information, twice. `println!("user {id} failed after {ms}ms")` produces text that a later query has to un-parse with a regex. An event carrying `user_id` and `duration_ms` as *fields* produces a record. The second costs nothing extra to write.
- The [`tracing` ↗](https://docs.rs/tracing) shape: a message plus typed fields — `info!(user_id, duration_ms, "request failed")` — and the subscriber that decides what becomes of it. [`tracing-subscriber` ↗](https://docs.rs/tracing-subscriber) renders for a human during development and JSON for a shipper in production, from the same call site.
- Levels as a cost knob rather than a matter of taste: `EnvFilter`, `RUST_LOG`, per-module targets, and what `debug!` in a hot loop costs when somebody turns it on in production.
- [`log` ↗](https://docs.rs/log) versus `tracing`: the older facade, the bridge between them, and why you end up with both — a dependency chose one and you chose the other.
- Write to stdout and let the platform ship it, versus opening files yourself. Which one is right depends on where the process runs, and that is worth one paragraph rather than a rule.
- Redaction is a design decision, not a review comment: a field is easy to drop centrally, a value interpolated into a sentence is not. Connects to the "never record" list in [what to instrument first](../what_to_instrument/README.md).

## If you are coming from another language

**Python** already has this argument settled by convention: `logging` with `extra={"user_id": id}` and a JSON formatter is the same split between message and fields, and the same trap — the f-string that bakes the value into the text. **ABAP** got there by a different road: the application log (`SLG1`) never held formatted strings in the first place. A message is a class, a number and up to four parameters, resolved to text only when displayed — which is structured logging, arrived at because the text had to be translatable. The habit that transfers is *log the values, format later*; what Rust adds is that the field's type survives to the other end.

## The trap it exists for

The format string that ate the field. Once `id` is inside a sentence, every query about it is a regex over prose — and the regex breaks the day somebody rewords the message, silently, in a commit that reads like a typo fix.

## See also

- [A span is not a log line](../spans_not_lines/README.md) — the same crate, one level up: events inside intervals
- [The three pillars](../the_three_pillars/README.md) — what logs are for, next to the other two
- [`anyhow` and context](../../02_Errors/anyhow_and_context/README.md) — the same instinct applied to an error's own text
