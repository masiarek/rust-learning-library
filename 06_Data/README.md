# Data

A program that forgets everything when it exits is a calculator. This section is the step after [Files](../04_Files/README.md): not bytes on a disk, but *your types* going out to a file and coming back as themselves.

**These pages are stubs** — outlines waiting for a runnable example. See the [Errors](../02_Errors/README.md) section for what that means and how a page graduates.

| Lesson | Level | What it will teach |
|---|---|---|
| [Deriving `Serialize` and `Deserialize`](serde_derive/README.md) | 201 | What `serde`'s derive actually writes for you, and why the format is a separate crate |
| [The round trip](json_round_trip/README.md) | 201 | Save then load, asserted equal — the test that designs the API before the API exists |
| [A type instead of a `Vec`](a_type_instead_of_a_vec/README.md) | 201 → 301 | Wrapping the collection in a struct that knows where it lives, and the methods that stop callers passing the wrong list |
