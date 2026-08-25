# Deserializing a response

**Level:** 201 · working knowledge

**One line:** The JSON you fetch was designed by somebody else, so the choice is between mirroring their whole shape in structs and reaching into it for the two fields you actually wanted.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The struct-per-level approach: faithful, self-documenting, and a lot of types you will never construct
- The pointer approach — `serde_json::Value` plus a [JSON Pointer ↗](https://docs.rs/serde_json/latest/serde_json/value/enum.Value.html#method.pointer) like `/current/temperature` — small, and it moves the failure from compile time to run time
- Which to reach for: how much of the response you need, how stable it is, and who finds out when it changes
- `#[serde(rename = "…")]` for a field name that is not a legal Rust identifier or not the name you want
- Missing versus null versus absent — three different things that all read as "no value", and how `Option` plus `#[serde(default)]` tell them apart

## The trap it exists for

Both approaches fail on a changed API; they differ in *when*. Structs fail loudly at the boundary with the field named; a JSON pointer returns `None` and lets the default flow onward — which is the same silent-wrong-answer shape as [collecting an iterator with `filter_map(Result::ok)`](../../02_Errors/keep_going_or_stop/README.md), one layer out.

## See also

- [Deriving `Serialize` and `Deserialize`](../../06_Data/serde_derive/README.md) — the derive, and the attributes that survive somebody else's naming
- [Mocking a server](../mocking_a_server/README.md) — where a stored real response earns its keep
- [`Option` vs `Result`](../../17_Option_and_Result/option_vs_result/README.md) — absent, or failed? The field that is missing has to pick one
