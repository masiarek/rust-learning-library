# Hygiene

**Level:** 301 · deep dive

**One line:** A `let` written inside a macro can neither be seen nor clobbered by the code that calls it — local variables and labels resolve where the macro was defined — while functions, types and `crate::` paths resolve where it was called, which is why an exported macro writes `$crate::`.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The refusal: a macro that expands to `let x = 1;`, then the caller reads `x` — `E0425`, *cannot find value `x` in this scope*, with the 1.98.0 help line *an identifier with the same name is defined here, but is not accessible due to macro hygiene*
- The protection it buys: a rule `{ let tmp = $e; tmp * tmp }` called as `square!(tmp + 1)` with the caller's `tmp = 3` gives 16 and leaves the caller's `tmp` at 3. std's `dbg!` relies on the same thing — it binds its argument to a local named `tmp`
- Handing a name in on purpose: an `$name:ident` fragment carries the caller's identifier, so `make_named!(z)` does create a `z` the caller can use
- Mixed-site hygiene as the Reference defines it: loop labels, block labels and local variables are looked up at the definition site, everything else at the invocation site — so a macro that calls `helper()` calls whichever `helper` is in scope where it is used
- `$crate`: the path to the crate that defined the macro, for helpers the calling crate never imported — and the Reference's warning that `$crate` does not bypass privacy, so the helper still has to be visible from the call site
- `crate::` inside a macro means the *calling* crate, which is what `clippy::crate_in_macro_def` (suspicious, warn by default) catches in an exported macro
- Seeing it rather than inferring it: `-Zunpretty=expanded,hygiene` prints a syntax-context number on every identifier — see [Expanding a macro](../expanding_a_macro/README.md)
- What procedural macros get instead of one fixed rule: a span chosen per token — [absolute paths and hygiene](../../37_Procedural_Macros/absolute_paths_and_hygiene/README.md) in the procedural macros section

## The trap it exists for

Assuming hygiene covers every name. It covers locals and labels only. A macro that calls `validate(&$x)` calls the caller's `validate` if there is one, and an exported macro that writes `crate::DEFAULTS` passes the unit tests of the crate that defines it — where the calling crate and the defining crate are the same — and fails to compile in the first other crate that calls it.

## Where this sits

[Scope is about names, not values](../../18_Ownership/scope_is_about_names/README.md) covers the ordinary scoping rules that hygiene bends. [Exporting a macro](../exporting_a_macro/README.md) covers where the macro's *own* name is visible; this page covers only the names inside its body.

## See also

- [Scope is about names, not values](../../18_Ownership/scope_is_about_names/README.md) — the rules a macro body does not follow
- [Exporting a macro](../exporting_a_macro/README.md) — `#[macro_export]`, and why `$crate` matters once you use it
- [Expanding a macro](../expanding_a_macro/README.md) — the printout that erases hygiene, and the flag that keeps it
- [What `dbg!` does](../../15_First_Programs/what_dbg_does/README.md) — a std macro whose internal `tmp` never meets yours
- [Macros by example: hygiene ↗](https://doc.rust-lang.org/reference/macros-by-example.html#r-macro.decl.hygiene) — the definition-site and invocation-site rule, and `$crate`
- [The Little Book of Rust Macros: hygiene ↗](https://lukaswirth.dev/tlborm/decl-macros/minutiae/hygiene.html) — a longer walk through the same rules

## If you are coming from another language

- **C.** The preprocessor has no hygiene. With `#define SWAP(a, b) { int tmp = a; a = b; b = tmp; }`, the call `SWAP(tmp, x)` declares a second `tmp` that hides the caller's; built with `cc` on macOS and warnings silenced it compiles, runs and prints `1 2` — nothing swapped. The fix in C is a naming convention; in Rust the compiler keeps the two `tmp`s apart.
- **C++.** The same preprocessor, and the same bug. The standard advice is to replace such a macro with an inline function or a template, whose locals are ordinary scoped variables — which is the guarantee Rust gives the macro itself.

## Po polsku

Higiena makr (*macro hygiene*) oznacza, że zmienna lokalna utworzona wewnątrz makra nie widzi zmiennych wywołującego i nie może ich przypadkiem nadpisać — `tmp` w makrze i `tmp` w twoim kodzie to dla kompilatora dwie różne nazwy. Higiena w `macro_rules!` jest jednak **mieszana** (*mixed-site*): dotyczy tylko zmiennych lokalnych i etykiet, a nazwy funkcji, typów i ścieżka `crate::` są rozwiązywane w miejscu wywołania. Dlatego eksportowane makro odwołuje się do własnych pomocników przez `$crate::`.

**Szukaj po polsku:** higiena makr · zmienne lokalne w makrze · `rust macro hygiene` · `rust $crate macro` · `mixed site hygiene`
