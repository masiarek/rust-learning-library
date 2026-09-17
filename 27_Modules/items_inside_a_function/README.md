# Items inside a function

**Level:** 201 · working knowledge

**One line:** A `fn`, `struct`, `use` or `const` written inside a function body is an ordinary item that only that block can name — not a closure — so an inner `fn` cannot see the outer function's variables, and asking it to is `E0434`.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The four items people nest most: a helper `fn`, a `struct` used only here, a `use` that shortens names for one function, and a `const` (covered on [`const` and `static`](../const_and_static/README.md))
- `E0434` "can't capture dynamic environment in a fn item" — the full message on 1.98.0 — and the closure that fixes it
- Order does not matter: an item declared at the bottom of the block can be called at the top, unlike a `let`
- `impl` blocks for a nested struct, and why the struct's name cannot escape the function even though its values can
- When a nested helper reads better than a private module-level one, and what it does to testability (a nested fn cannot be unit-tested on its own)
- Nested functions versus closures, side by side: capture, type (a fn item vs an anonymous closure type), and coercion to a [function pointer](../../23_Closures/function_pointers/README.md)

## The trap it exists for

Moving a closure body into a nested `fn` "to give it a name" and meeting `E0434`, because the closure was capturing a local. A nested fn has access only to its parameters and to other items.

## Where this sits

[`const` and `static`](../const_and_static/README.md) already covers a `const` inside a function and what it cannot see. [What a closure is](../../23_Closures/what_a_closure_is/README.md) is the construct that *can* capture. This page covers the other item kinds.

## See also

- [`const` and `static`](../../27_Modules/const_and_static/README.md) — a `const` inside a function
- [What a closure is](../../23_Closures/what_a_closure_is/README.md) — the nested function that can capture
- [Function pointers](../../23_Closures/function_pointers/README.md) — what both coerce to when they capture nothing
- [Functions](../../25_Control_Flow/functions/README.md) — the top-level `fn` this nests
- [Scope is about names, not values](../../18_Ownership/scope_is_about_names/README.md) — a name visible only inside a block
- [Modules and visibility](../../27_Modules/modules_and_visibility/README.md) — the module-level alternative

## If you are coming from another language

- **Python.** A nested `def` *is* a closure in Python — it reads the enclosing function's variables. Rust splits that into two constructs, so a nested `fn` that tries to read a local is an error rather than a capture.
- **JavaScript.** Nested function declarations capture and are hoisted; Rust keeps the hoisting (order does not matter) and drops the capture.
- **C.** Standard C has no nested functions (GCC's extension does, with trampolines); a `static` helper at file scope is the usual substitute.
- **Java.** A local class can read effectively-final locals; a Rust local struct cannot read any.

## Po polsku

`fn`, `struct`, `use` czy `const` napisane wewnątrz ciała funkcji to zwykłe elementy (*items*) widoczne tylko w tym bloku — nie domknięcia. Wewnętrzna funkcja nie widzi zmiennych funkcji zewnętrznej (`E0434`), a kolejność deklaracji nie ma znaczenia. Kto przychodzi z Pythona, gdzie zagnieżdżony `def` jest domknięciem, trafia na ten błąd najczęściej.

**Szukaj po polsku:** funkcja zagnieżdżona · element wewnątrz funkcji · `rust nested function E0434` · `rust fn inside fn vs closure`
