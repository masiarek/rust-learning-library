# Deriving `Serialize` and `Deserialize`

**Level:** 201 · working knowledge

**One line:** [`serde` ↗](https://serde.rs) splits the job in two — the derive teaches *your type* how to describe itself, and a separate crate like `serde_json` decides what the bytes look like.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The two-crate shape, and why it is not gratuitous: one derive gets you JSON, YAML, TOML, MessagePack and CBOR without touching the struct again
- What `#[derive(Serialize, Deserialize)]` generates, at a level a reader can picture — a visitor walking your fields, not a reflection API, because Rust has none at runtime
- The Cargo detail that stops everyone once: `serde` needs `features = ["derive"]`, and the error when it is missing names the macro rather than the feature
- The attributes worth knowing early — `#[serde(rename = "…")]`, `rename_all`, `default`, `skip_serializing_if` — for the day the JSON was designed by somebody else
- An `enum` over the wire: what the default tagging looks like, and why it is the first thing people override

## The trap it exists for

Deriving both traits makes a struct part of a **file format**. Rename a field and yesterday's saved file no longer loads; add a field and it loads with an error unless something supplied a default. The struct stopped being an implementation detail at the moment of the derive, and nothing in the code says so.

## See also

- [The round trip](../json_round_trip/README.md) — the test that catches exactly the breakage above
- [The `Default` trait](../../03_Command_Line/the_default_trait/README.md) — what `#[serde(default)]` reaches for when a field is missing
- [Debug and Display](../../15_First_Programs/debug_vs_display/README.md) — three ways to turn a value into text, and the audience each one has
