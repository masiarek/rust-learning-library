# Arguments and the environment

**Level:** 201 · working knowledge

**One line:** Command-line arguments are what a user types this once; environment variables are what a machine was configured with — and the test for which is roughly *"would I mind this appearing in `ps` output and in my shell history?"*

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- [`env::var` ↗](https://doc.rust-lang.org/std/env/fn.var.html) returns a `Result`, and its two failure modes are genuinely different: not set, and set to something that is not Unicode
- The precedence ladder every configurable program grows — flag beats environment beats config file beats built-in default — and writing it down before it grows by accident
- `clap`'s `env` attribute, which puts the ladder in the struct
- Secrets: an API key on the command line is visible in the process list to every user on the machine
- Why [`std::env::set_var` ↗](https://doc.rust-lang.org/std/env/fn.set_var.html) is `unsafe` in the 2024 edition, and what that means for a test that sets one — the environment is process-global, and Rust's tests run in threads

## The trap it exists for

The environment is the one input that is not visible in the code, the arguments, or the file. A program that behaves differently on two machines with the same command and the same input is nearly always reading something nobody wrote down — which is an argument for making every environment variable appear in `--help`.

## See also

- [Command-line arguments](../command_line_arguments/README.md) — the other half of the program's input
- [Deriving a parser with `clap`](../clap_derive/README.md) — where the precedence ladder gets declared instead of coded
- [Testing a command](../testing_a_command/README.md) — why a test that sets an environment variable is not automatically safe to run in parallel
