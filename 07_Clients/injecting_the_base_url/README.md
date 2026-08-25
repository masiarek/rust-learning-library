# Injecting the base URL

**Level:** 201 → 301 · deep dive

**One line:** A `const BASE_URL` is one line and it makes the function untestable forever; the same URL as a field is one line and it does not.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The rule under the technique: a function that *decides* where to send a request cannot be pointed anywhere else, and testability is downstream of that, not the point of it
- The refactor, in order — free function, then function with a URL argument, then a struct holding client and URL with the request as a method
- Where the real URL then comes from: a default on the struct, an environment variable, a flag — the [precedence ladder](../../03_Command_Line/arguments_and_environment/README.md) again
- `#[must_use]` on a builder-ish method, so a returned client that nobody stores becomes a warning
- The generalisation worth stating once: every "designing for testability" trick in this section is the same move — turn a decision into a parameter

## The trap it exists for

Injection reads as test scaffolding, so it gets skipped in "real" code and retrofitted under pressure later, usually by adding a `#[cfg(test)]` back door. The back door is the tell: it means the production path and the tested path are now two different paths, which is the one thing a test was supposed to rule out.

## See also

- [Mocking a server](../mocking_a_server/README.md) — the thing this makes possible
- [Testing a command](../../03_Command_Line/testing_a_command/README.md) — the same argument one level up: parameters, not constants
- [Optional function arguments](../../17_Option_and_Result/optional_arguments/README.md) — the five shapes a "usually this, sometimes that" argument can take
