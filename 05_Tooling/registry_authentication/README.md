# Registry authentication: tokens and SSH keys

**Level:** 301 · deep dive

**One line:** Cargo authenticates in two unrelated places — to the registry's API with a token, and to a git index or git dependency with whatever git uses — so an SSH key that works for `git clone` can still fail inside `cargo build`.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- Where `cargo login` stores a token, and the credential providers (`cargo:token`, the OS keychain providers) that store it somewhere safer — check the provider names on the pinned Cargo
- Why Cargo's built-in git client may not use your SSH agent or `~/.ssh/config`, and `net.git-fetch-with-cli = true` as the switch that hands fetching to the `git` binary
- An SSH URL for a git dependency (`ssh://git@host/org/repo.git`) versus the `git@host:org/repo` form, and which one Cargo accepts
- Authentication for a `sparse+https` index: the `auth-required` flag in the registry's `config.json`
- CI: a deploy key or token in an environment variable, and why the same key should not be a person's
- Reading the failure: which error text means the index, which means a git dependency, and which means publishing

## The trap it exists for

Testing the key with `ssh -T git@host`, seeing success, and concluding Cargo will work. Cargo's own git implementation is a different client, so the fix is usually one config line rather than another key.

## Where this sits

[Private registries](../private_registries/README.md) sets up the registry itself. This page is only about getting through its door. [Publishing a crate](../publishing_a_crate/README.md) is where the token is spent.

## See also

- [Private registries](../../05_Tooling/private_registries/README.md) — the registry this key opens
- [Publishing a crate](../../05_Tooling/publishing_a_crate/README.md) — the command that needs a token
- [Adding a dependency](../../05_Tooling/cargo_dependencies/README.md) — git dependencies are dependencies too
- [rustup](../../05_Tooling/rustup/README.md) — the other tool in the chain that downloads things

## If you are coming from another language

- **Python.** `pip`/`uv` read credentials from `.netrc` or a keyring; the same split applies to a `git+ssh://` requirement, which goes through `git` rather than the index client.
- **Go.** `go get` from a private repository has the identical problem and the identical fix: tell git (not Go) how to authenticate, via `~/.gitconfig` `insteadOf` or `GOPRIVATE`.

## Po polsku

Cargo uwierzytelnia się w dwóch niezależnych miejscach: w API rejestru tokenem, a przy indeksie git lub zależności z gita — tym, czego używa git. Stąd klasyczna pułapka: `ssh -T git@host` działa, a `cargo build` nie, bo wbudowany klient gita w Cargo nie korzysta z agenta SSH; zwykle wystarcza `net.git-fetch-with-cli = true`.

**Szukaj po polsku:** klucz SSH w Cargo · uwierzytelnianie rejestru · `cargo git-fetch-with-cli` · `cargo ssh key private git dependency`
