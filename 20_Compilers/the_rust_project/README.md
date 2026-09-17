# The Rust Project

**Level:** 201 · working knowledge

**One line:** Rust is made by the Rust Project — teams that decide substantial changes through RFCs and ship a stable compiler every six weeks, with editions as the opt-in place for changes that would otherwise break code — and the Rust Foundation is a separate organisation beside it.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The top-level teams listed on rust-lang.org/governance on 2026-09-16: the Leadership Council, then compiler, dev tools, infrastructure, language, launching pad, library and moderation — what each one owns, and where teams that are not on that list, such as release, sit
- The RFC process: which changes need one — the RFC book names any semantic or syntactic change to the language that is not a bugfix, removing a language feature, and large additions to std, among others — and the path from an RFC pull request to a decision
- The release train from The Book's Appendix G, "How Rust is Made and Nightly Rust": nightly, beta and stable, a stable release every six weeks, and what "stabilised in 1.xx" means for a feature you read about
- Unstable features: `#![feature(…)]` on nightly, a tracking issue per feature, and why `RUSTC_BOOTSTRAP=1` is a hole in the policy rather than a way around it ([Printing the HIR](../printing_the_hir/README.md))
- Editions — 2015, 2018, 2021, 2024: chosen per crate, and bound by the edition guide's rule that crates in one edition must interoperate with crates compiled in another. What changed in each, and why `rustc` alone still defaults to 2015
- The Rust Foundation: what it funds and runs, how it relates to the Project's teams, and which decisions it does not make — settled from its own about page and the Project's governance pages rather than from secondhand descriptions
- Where the work is visible: issues and pull requests on rust-lang/rust, the RFC repository, and the `#t-compiler` channel on Zulip that the rustc dev guide points newcomers to
- A worked case: [`str::as_str`](../../14_Strings/str_as_str/README.md), which reached stabilisation and is still `E0658` on stable — what the stretch between a stabilisation PR and a stable release is for

## The trap it exists for

Reading an accepted RFC as a shipped feature. Acceptance means the design was agreed; implementation, a tracking issue, a stabilisation decision and a release come after, sometimes years later and sometimes never. The release notes and the stability badge in std's docs say what is on stable — the RFC does not.

## Where this sits

[rustup](../../05_Tooling/rustup/README.md), [`rustup default nightly`](../../05_Tooling/nightly/README.md) and [Pinning the toolchain](../../05_Tooling/pinning_the_toolchain/README.md) are the release train from a user's side. The three RFC pages in [Strings](../../14_Strings/README.md) each read one decision closely. This page is the organisation that produces both; [A first contribution](../a_first_contribution/README.md) is how you join it.

## See also

- [What a compiler does before your program runs](../what_a_compiler_does/README.md) — the program this organisation maintains
- [Printing the HIR](../printing_the_hir/README.md) — `RUSTC_BOOTSTRAP=1`, and what it opts out of
- [rustup: the `rustc` you run is not the compiler](../../05_Tooling/rustup/README.md) — stable, beta and nightly as channels you install
- [`rustup default nightly`](../../05_Tooling/nightly/README.md) — what choosing nightly actually decides
- [Pinning the toolchain](../../05_Tooling/pinning_the_toolchain/README.md) — picking one train car on purpose
- [RFC 1054 — the method that renamed itself to promise less](../../14_Strings/rfc_1054_str_words/README.md) — one RFC, start to finish
- [When the type checker is wrong](../when_the_type_checker_is_wrong/README.md) — soundness bugs as issues the Project tracks in public
- [Rust governance ↗](https://www.rust-lang.org/governance) — the teams and the RFC process, from the source
- [The RFC book ↗](https://rust-lang.github.io/rfcs/) — when an RFC is needed and its life-cycle
- [The edition guide ↗](https://doc.rust-lang.org/edition-guide/editions/index.html) — what an edition is and what each changed

## If you are coming from another language

- **Python.** PEPs are the counterpart of RFCs, the Steering Council is the nearest thing to the Leadership Council, and the Python Software Foundation stands beside the core developers much as the Rust Foundation stands beside the Project. Python ships a feature release a year (PEP 602) rather than every six weeks.
- **Java.** JEPs propose features for OpenJDK, which ships a feature release every six months; preview features that must be switched on are the counterpart of nightly-only `#![feature]`.
- **C++.** The ISO committee WG21 votes papers into a standard every three years, and compilers implement it afterwards, each on its own schedule. Rust has one main implementation and ships continuously, with editions also arriving on a three-year rhythm so far.
- **Go.** Proposals go through the Go team's review process and Go ships two releases a year under the Go 1 compatibility promise; the `go` line in `go.mod` selects language behaviour per module, the nearest thing Go has to an edition.

## Po polsku

Rusta tworzy Rust Project — zespoły (kompilator, język, biblioteka i inne) pod Leadership Council, które o istotnych zmianach decydują w procesie RFC i co sześć tygodni wydają stabilny kompilator; zmiany, które zepsułyby istniejący kod, z zasady trafiają do nowej edycji (*edition*), wybieranej osobno dla każdego crate'a. Rust Foundation to odrębna organizacja obok Projektu. Pułapka: zaakceptowany RFC to jeszcze nie funkcja w stabilnym Ruście — to, co naprawdę jest dostępne, mówią informacje o wydaniu i znaczniki stabilności w dokumentacji.

**Szukaj po polsku:** jak powstaje Rust · proces RFC · `rust release train` · `rust governance teams` · `rust foundation vs rust project`
