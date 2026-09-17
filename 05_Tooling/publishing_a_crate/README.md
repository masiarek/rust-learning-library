# Publishing a crate

**Level:** 201 · working knowledge

**One line:** `cargo publish` uploads a snapshot you can never change or delete — only yank — so everything worth checking happens before it: the metadata, the files in the package, and the version number you are about to spend.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- What `cargo package --list` shows, and why a file in your repo is not necessarily in the upload (`include`, `exclude`, `.gitignore`)
- The metadata crates.io refuses to publish without — `license` or `license-file`, `description` — and the fields that only matter to readers (`repository`, `documentation`, `readme`, `keywords`, `categories`)
- `cargo publish --dry-run`: the verify build that compiles your package from the tarball, which is how a missing file is caught
- `cargo yank --version` — what it stops (new lockfiles resolving to it) and what it does not (existing lockfiles, downloads)
- Choosing the version: SemVer as Cargo applies it, `0.x` where the *minor* number is the breaking one — see [two versions of one crate](../two_versions_of_one_crate/README.md)
- The login token: `cargo login`, where the token is stored, and the credential providers that keep it out of a plain file (questions for the finished page; see [registry authentication](../registry_authentication/README.md))

## The trap it exists for

Publishing to find out whether it works. A crate version is permanent, so a broken `0.3.1` stays on crates.io forever and the fix has to be `0.3.2`. `cargo publish --dry-run` builds the package the same way the upload would.

## Where this sits

This page is the public registry. [Private registries](../private_registries/README.md) is the same command pointed somewhere else, [MSRV](../msrv/README.md) is the `rust-version` field you set before publishing, and [Adding a dependency](../cargo_dependencies/README.md) is the other side of the transaction.

## See also

- [Adding a dependency](../../05_Tooling/cargo_dependencies/README.md) — the caret requirement your version number is read through
- [Two versions of one crate](../../05_Tooling/two_versions_of_one_crate/README.md) — what "SemVer compatible" means to Cargo, which decides your next number
- [`Cargo.lock`](../../05_Tooling/cargo_lock/README.md) — why yanking does not break anyone who already locked
- [Private registries](../../05_Tooling/private_registries/README.md) — publishing somewhere other than crates.io
- [Cargo features](../../05_Tooling/cargo_features/README.md) — the part of your public interface that lives in `Cargo.toml`
- [Packages and crates](../../27_Modules/packages_and_crates/README.md) — what exactly the tarball contains

## If you are coming from another language

- **Python.** `cargo publish` is `uv publish` / `twine upload`, and crates.io's no-delete rule matches PyPI's no-reuse rule for a filename. The Python library's [`pyproject.toml` ↗](https://masiarek.github.io/python-learning-library/02_Projects_and_Environments/pyproject_toml/) page is the counterpart to the metadata table.
- **Java.** Maven Central is the closer match: immutable releases, required metadata, and a staging step that plays the role of `--dry-run`.
- **ABAP.** No public registry, but a released transport is the same promise — you do not edit it, you send another one on top.

## Po polsku

Opublikowanej wersji skrzyni (*crate*) nie da się zmienić ani usunąć, można ją tylko wycofać (*yank*), a wycofanie nie psuje nikomu istniejącego `Cargo.lock`. Dlatego wszystko, co warto sprawdzić, robi się przed wysłaniem: `cargo package --list` i `cargo publish --dry-run`.

**Szukaj po polsku:** publikowanie skrzyni · wersjonowanie semantyczne · `cargo publish dry run` · `cargo yank what does it do`
