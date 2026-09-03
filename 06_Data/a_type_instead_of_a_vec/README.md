# A type instead of a `Vec`

**Level:** 201 → 301 · deep dive

**One line:** Once `load` and `save` both take a path *and* a `Vec<Memo>`, the two belong together — and the struct that holds them turns four free functions with matching arguments into a thing with methods.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The smell first: every function taking the same two arguments, and every caller responsible for pairing them correctly
- An inherent `impl`, and how a method's `self` differs from the same function taking the struct — including when to take `&self`, `&mut self`, or `self`
- Storing an owned `PathBuf` while accepting `impl AsRef<Path>` at the door — the `From`/`AsRef` hop, and why the owned type is the right one to *keep*
- What the type buys: `save` cannot be called with the wrong file, and the invariant "loaded from here, saved to here" stops being a convention
- Where to stop — a newtype around a `Vec` that only forwards methods is a tax, not an abstraction

## The trap it exists for

The wrapper is easy to build and easy to overbuild. Every method you forward (`len`, `iter`, `push`, `is_empty`) is a line that exists to re-expose something the caller already had. The reason to wrap is the *invariant*; if there is not one, `Vec<Memo>` was already right.

## See also

- [A score is not a number](../../16_Structs/newtype_score/README.md) — the same move on a single value: one private field, one validating door
- [`Path` and `PathBuf`](../../04_Files/path_and_pathbuf/README.md) — the owned/borrowed pair this struct has to choose between
- [What is a record, in memory?](../../16_Structs/representing_a_record/README.md) — choosing the shape of the data, and which bugs each shape makes writeable

## Po polsku

To jest ruch znany z katalogu refaktoryzacji jako „wprowadź obiekt”: skoro `load` i `save` biorą za każdym razem tę samą parę — ścieżkę i `Vec<Memo>` — to ta para jest w rzeczywistości jednym bytem, a cztery luźne funkcje stają się strukturą z metodami. Specyficznie rustowa jest tu kwestia własności: struktura **przechowuje** `PathBuf`, czyli typ, który dane posiada, ale w drzwiach przyjmuje `impl AsRef<Path>`, żeby wołający mógł podać `&str`, `String` albo `&Path` — pożyczać wygodnie jest na wejściu, natomiast trzymać u siebie można tylko to, co się ma na własność. Do tego dochodzi wybór `self` w metodach: `&self` czyta, `&mut self` zmienia zawartość, a samo `self` przejmuje strukturę na własność, czyli po takim wywołaniu wołający już jej nie ma. I to, o co tej stronie chodzi naprawdę: opakowanie, które tylko przekazuje dalej `len`, `iter` i `push`, jest podatkiem, a nie abstrakcją — powodem do zawijania `Vec` jest **niezmiennik** („wczytane stąd, zapisane tam samo”), a jeśli niezmiennika nie ma, to `Vec<Memo>` był już dobrą odpowiedzią.

**Szukaj po polsku:** wzorzec newtype · niezmiennik typu · refaktoryzacja wprowadź obiekt · `rust impl AsRef<Path> PathBuf` · `rust newtype wrapper Vec`
