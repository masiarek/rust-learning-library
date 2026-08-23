# Command line

The front door. Everything in [Foundations](../01_Foundations/README.md) happens inside a program that somebody has already started; this section is about the handful of strings they typed to start it, and about proving the program does the right thing with them.

**These pages are stubs** — outlines waiting for a runnable example. See the [Errors](../02_Errors/README.md) section for what that means and how a page graduates.

| Lesson | Level | What it will teach |
|---|---|---|
| [Command-line arguments](command_line_arguments/README.md) | 101 | `env::args()` — an iterator whose first item is the program's own name, and the filename that is not valid UTF-8 |
| [Flags by hand](flags_by_hand/README.md) | 201 | What a flag actually is, ten lines that parse one, and the eleventh line where you start rewriting `clap` |
| [Deriving a parser with `clap`](clap_derive/README.md) | 201 | A struct becomes the whole interface: parsing, `--help`, `--version`, and the error message for a bad flag |
| [Testing a command](testing_a_command/README.md) | 201 → 301 | Unit tests prove a function; only running the binary proves the program — status, streams, and assertions that are not brittle |
| [The `Default` trait](the_default_trait/README.md) | 101 → 201 | The value a type takes when nobody said — and `..Default::default()`, which is how an options struct grows a field |
| [Arguments and the environment](arguments_and_environment/README.md) | 201 | Which inputs belong on the command line, which belong in the environment, and why the 2024 edition made `set_var` `unsafe` |
