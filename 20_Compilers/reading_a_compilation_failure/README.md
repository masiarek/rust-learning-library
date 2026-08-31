# Reading a compilation failure

**Level:** 101 → 201 · newcomer

**One line:** Before reading the message, work out which stage produced it — a parse error, a type error, a borrow error and a link error come from four different programs with four different vocabularies, and the fix you are looking for is different in each.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The four dialects side by side, each with a real transcript: `expected one of`, `E0308 mismatched types`, `E0502 cannot borrow`, `undefined symbols`
- `rustc --explain E0308` — the fuller writeup with an example, available offline, and the codes that are worth learning by heart
- Reading a diagnostic's structure: the `-->` location, the `|` span, the `help:` versus `note:` distinction, and which suggestions are safe to apply blind
- Why the *first* error is the one to fix, and how a single missing brace generates eleven downstream complaints
- `cargo check` versus `cargo build`: the same front end, stopping before codegen, so an error list arrives sooner
- Errors that are not rustc at all — a linker's, a build script's, a proc macro's panic — and how to tell each from a real compile error at a glance
- The one that fools everyone: a trait bound reported at the call site when the missing `impl` is somewhere else entirely

## The trap it exists for

A borrow error and a type error look alike on the page and are nothing alike in the head. One says *the shape is wrong*; the other says *the shape is fine and the timing is not*, and applying a type-error habit to a lifetime problem produces the `.clone()` that makes the message go away without teaching anything.

## See also

- [What a warning is asking](../../15_First_Programs/what_a_warning_is_asking/README.md) — the diagnostics that let the build succeed, and why they are still worth reading
- [What a compiler does before your program runs](../what_a_compiler_does/README.md) — which stage can raise which error
- [The linker](../the_linker/README.md) — the failure that is not the compiler's, in someone else's words

## Po polsku

Pierwsze pytanie przy czerwonym komunikacie nie brzmi „co on znaczy”, tylko **który etap go wypisał** — `expected one of` pochodzi od parsera, `E0308 mismatched types` od kontroli typów, `E0502 cannot borrow` od borrow checkera, a `undefined symbols` od konsolidatora (*linker*), czyli od programu, który kompilatorem już nie jest. Cztery słowniki, cztery zupełnie różne rodzaje poprawki. Najkosztowniejsza pomyłka to wzięcie błędu pożyczania za błąd typów: błąd typów mówi „kształt się nie zgadza”, a błąd pożyczania — „kształt jest dobry, zły jest moment”, więc odruchowe `.clone()` wycisza komunikat, nie ucząc niczego. Komunikaty rustc są po angielsku i polskiej wersji nie ma, ale zanim cokolwiek wkleisz w wyszukiwarkę, uruchom `rustc --explain E0308` — ten sam błąd z dłuższym opisem i przykładem, lokalnie; i poprawiaj zawsze **pierwszy** błąd z listy, bo jeden brakujący nawias klamrowy potrafi wygenerować kilkanaście kolejnych, które znikną same.

**Szukaj po polsku:** komunikaty błędów kompilatora Rust · błąd pożyczania · `rust --explain E0308` · `rust E0502 cannot borrow` · `rust undefined symbols linker`
