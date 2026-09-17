# Safe wrappers

**Level:** 301 · deep dive

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** Generated bindings are all `unsafe`, and a safe wrapper is the crate that makes them impossible to misuse — a handle type that frees itself in `Drop`, lifetimes that tie borrowed C data to its owner, and a deliberate decision about `Send` and `Sync` — so that every `unsafe` block lives in one small module whose invariants can be read in one sitting.

## What it has to cover

- The two-crate split: `foo-sys` holds the raw declarations, `foo` holds the safe API and depends on it
- **RAII handles:** a struct that owns a C pointer, calls the library's `_free` in `Drop`, and cannot be copied — so double-free and use-after-free need `unsafe` to write
- **Lifetimes for borrowed data:** a `&'a str` or slice returned from a C getter, tied to the handle it came from so it cannot outlive it
- **`Send` and `Sync`:** a raw pointer makes a wrapper neither by default; implementing them is a claim about the C library's thread safety, which its documentation has to support
- Soundness: a safe function is unsound if *any* safe caller can trigger undefined behaviour through it — and the checklist that asks that of each method
- Where the unsafe code goes: one private module, each block with a `// SAFETY:` comment

## The trap it exists for

`unsafe impl Send` added to silence a compiler error. It compiles, the wrapper crosses threads, and the C library underneath — never written to be called from two threads — corrupts its own state.

## See also

- [Generating bindings](../generating_bindings/README.md) — the `-sys` side
- [Drop and RAII](../../../12_Traits/drop_and_raii/README.md) — the mechanism a handle relies on
- [`Send` and `Sync`](../../../09_Advanced/send_and_sync/README.md) — what the two impls claim
- [What an invariant is](../../../09_Advanced/what_an_invariant_is/README.md) — what the wrapper must keep true
- [Double-free](../../double_free/README.md) and [Use-after-free](../../use_after_free/README.md) — the C bugs the handle type rules out

## Po polsku

Wygenerowane wiązania są w całości `unsafe`, a **bezpieczne opakowanie** (*safe wrapper*) to crate, który uniemożliwia ich złe użycie: typ uchwytu zwalniający zasób w `Drop`, czasy życia wiążące pożyczone dane C z ich właścicielem i świadoma decyzja o `Send` i `Sync`. Cały kod `unsafe` trafia do jednego małego modułu, którego niezmienniki da się przeczytać za jednym razem.

**Szukaj po polsku:** bezpieczne opakowanie biblioteki C · RAII w Ruscie · `rust safe wrapper around c library` · `rust unsafe impl send ffi`
