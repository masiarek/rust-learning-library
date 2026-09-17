# Documenting unsafe contracts

**Level:** 301 · deep dive

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** An FFI function's safety depends on promises the compiler cannot check, so each one is written down — a `/// # Safety` section with numbered invariants on the function, a `// SAFETY:` comment naming which invariant each `unsafe` block relies on — and generated into the C header, because the C caller is the one who has to keep them.

## What it has to cover

- **The `# Safety` section:** numbered invariants a caller must uphold (non-null, points to N initialised bytes, not used after `_free`, one thread at a time), written so each can be checked by reading the call site
- **`// SAFETY:` comments** on every `unsafe` block, citing the invariant numbers — and the Clippy lints that enforce both: `missing_safety_doc` (on by default) and `undocumented_unsafe_blocks` (opt-in, restriction group)
- The docs reaching C: doc comments carried into the generated header by `cheadergen`, so the contract sits beside the declaration a C programmer reads
- **Build integration and automated checks:** regenerate the header in CI and fail on drift; `improper_ctypes` as an error; a test that the header compiles with the C compiler the project uses
- **The FFI surface as an API maintained forever:** versioning, adding without breaking, deprecating a function C code still calls, and keeping the exported list small enough to review

## The trap it exists for

A safety contract that exists only in the Rust source. The C programmer reads the header, sees `int foo_len(const foo *f);`, and has no way to know that `f` must not have been freed on another thread a moment ago.

## See also

- [Generating bindings](../generating_bindings/README.md) — the header generator that carries the docs
- [Strict lints](../../../05_Tooling/strict_lints/README.md) — turning Clippy lints on for a whole project
- [What `unsafe` turns off](../../../09_Advanced/what_unsafe_turns_off/README.md) — the obligations the invariants describe
- [Safe wrappers](../safe_wrappers/README.md) — the Rust-side consumers of the same contracts

## Po polsku

Bezpieczeństwo funkcji FFI zależy od obietnic, których kompilator nie sprawdzi, więc każdą trzeba zapisać: sekcja `/// # Safety` z ponumerowanymi **niezmiennikami** przy funkcji i komentarz `// SAFETY:` przy każdym bloku `unsafe`, wskazujący, na którym niezmienniku polega. Ta dokumentacja powinna trafić do wygenerowanego nagłówka C, bo to programista C musi tych obietnic dotrzymać. Powierzchnię FFI traktuje się jak API utrzymywane bezterminowo.

**Szukaj po polsku:** dokumentowanie kodu unsafe · niezmienniki · `rust safety doc comment convention` · `clippy undocumented_unsafe_blocks`
