# Private registries

**Level:** 301 · deep dive

**One line:** A private registry is crates.io's protocol on a server you control — Cargo already speaks it, so the work is a `[registries]` entry, a `publish = [...]` guard, and deciding where the token lives.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The `[registries]` table in `.cargo/config.toml` and `registry = "name"` on a dependency: how one manifest pulls from two registries
- The two index protocols — git and `sparse+https://` — and what the sparse one saves on a cold build
- `publish = ["company"]` in `Cargo.toml` as the guard that stops an internal crate going to crates.io by accident
- Hosted options (Shipyard, Cloudsmith, JFrog Artifactory, Kellnr as self-hosted) versus vendoring and git dependencies — what each costs a team of five
- Replacing crates.io itself with a mirror: `[source.crates-io] replace-with`, and how that differs from a second registry
- How CI authenticates without a person present — a token in `CARGO_REGISTRIES_<name>_TOKEN`, the variable the Cargo configuration reference documents

## The trap it exists for

Leaving out `publish = ["company"]`. Without it, `cargo publish` in a private crate's folder defaults to crates.io, and a publish there cannot be deleted.

## Where this sits

[Publishing a crate](../publishing_a_crate/README.md) is the command. [Registry authentication](../registry_authentication/README.md) is the credentials problem, including SSH keys for a git index. [Vendoring and `[patch]`](../vendoring_and_patch/README.md) is the alternative that needs no server at all.

## See also

- [Publishing a crate](../../05_Tooling/publishing_a_crate/README.md) — the command a registry receives
- [Registry authentication](../../05_Tooling/registry_authentication/README.md) — tokens, credential providers and SSH keys
- [Vendoring, and the `[patch]` table](../../05_Tooling/vendoring_and_patch/README.md) — the no-server alternative
- [Adding a dependency](../../05_Tooling/cargo_dependencies/README.md) — what `cargo add` writes, with `--registry` added
- [`Cargo.lock`](../../05_Tooling/cargo_lock/README.md) — which registry a locked package came from

## If you are coming from another language

- **Python.** An extra index URL in `pip`/`uv` is the same idea, and dependency confusion (a public package shadowing a private name) is the same attack. Cargo's per-dependency `registry = …` closes it more explicitly than a global extra index does.
- **Java.** A Nexus or Artifactory repository in `settings.xml` is the closest match, down to the mirror setting that replaces the central repository.
- **Go.** `GOPRIVATE` and `GOPROXY` split the same way: which module paths bypass the public proxy, and which server answers for the rest.

## Po polsku

Prywatny rejestr (*registry*) to ten sam protokół co crates.io, tyle że na serwerze firmy — Cargo obsługuje go bez wtyczek. Najważniejsza linijka to `publish = ["nazwa"]` w `Cargo.toml`, bo bez niej `cargo publish` domyślnie wysyła wewnętrzną skrzynię na publiczne crates.io, skąd nie da się jej usunąć.

**Szukaj po polsku:** prywatny rejestr skrzyń · `cargo alternative registry` · `shipyard.rs cargo` · `cargo sparse registry`
