# Anki decks

**Level:** reference · for working programmers

**One line:** Four spaced-repetition decks — 126 cards over `String`, `Vec`, iterators and ownership — whose every code block was compiled and run by `verify.py` before the deck was written, so a card cannot claim output the compiler did not produce.

## Import

Anki → **File → Import** → pick the file → Import. Nothing to configure: the header lines set the deck, the note type and the tags column.

| file | deck | cards |
|---|---|---|
| `Rust_Strings.txt` | `Rust::Strings` | 31 |
| `Rust_Vec.txt` | `Rust::Vec` | 32 |
| `Rust_Iterators.txt` | `Rust::Iterators` | 32 |
| `Rust_Ownership.txt` | `Rust::Ownership` | 31 |

Re-importing is safe and idempotent: Anki matches on the first field, so an edited card updates in place and your review history survives.

## Why these cards look the way they do

Reading an explanation and understanding it is not the same act as retrieving it, and only the second one is what a card can train. So no card here asks you to recognise a definition. Every one asks you to **produce** something:

- **Predict the output** — a program on the front, its verified stdout on the back. The strongest card type in the deck, because you cannot fake having answered one.
- **Does this compile?** — eleven cards over seven distinct errors (`E0106`, `E0277`, `E0382`, `E0499`, `E0502`, `E0507`, `E0515`). These are the errors you will actually hit, and knowing the number means knowing the shape of the mistake.
- **Write the line** — "reverse a string", "join with a comma", "return an iterator from a function".
- **Choose between two** — `remove` vs `swap_remove`, `fold` vs `reduce`, `filter` vs `take_while`, `&str` vs `&String`.

Each back ends with a **Python / ABAP bridge** where one is honest, and a link to the lesson on the site. Some bridges are warnings (`len()` is bytes in Rust, code points in Python; `join` puts the separator on the other side); the best of them is `dedup` ↔ `DELETE ADJACENT DUPLICATES`, which is the *same* trap with the *same* fix.

The decks lean on this library's own framings rather than inventing new ones — a move is [a transfer of responsibility](../../18_Ownership/ownership_and_moves/README.md), a lifetime [names a relationship rather than extending one](../../18_Ownership/lifetime_annotations/README.md), `collect` [asks the target type to build itself](../../24_Iterators/collect_and_fromiterator/README.md), and adapters [compute nothing until a consumer runs them](../../24_Iterators/iterators_are_lazy/README.md). A card that contradicted its own lesson would be worse than no card.

## Regenerating

```bash
python3 verify.py cards_strings cards_vec cards_iterators cards_ownership
python3 build.py
```

`verify.py` uses the same contract as [`tools/run_examples.py`](../../tools/run_examples.py): bare `rustc --edition 2024`, no crates, exact output match. A card with `fails="E0502"` must **not** compile, and must fail with that code — so the deck's compile-error cards are as checked as its output cards.

Edit a card in `cards_*.py`, run both commands, re-import. Never hand-edit the `.txt` files; they are generated.

That gate is not ceremony. Building these four decks it caught a wrong claim about Unicode normalisation, a `peek` example that was itself a borrow error, and three snippets whose escapes had been eaten — all of which read as correct.

## What is not here

Cards are deliberately not written for every method — the [`String`](../../14_Strings/string_methods/README.md) and [`Vec`](../../26_Collections/the_vec/README.md) references already do that job, and a deck that mirrors a reference is a deck you stop reviewing. These 126 are the facts that a working programmer forgets and is then bitten by.

## Po polsku

Anki to darmowy program do powtórek rozłożonych w czasie (*spaced repetition*), po polsku najczęściej nazywanych po prostu „systemem powtórek" albo SRS. Te talie mają 126 kart o `String`, `Vec`, iteratorach i własności, a ich wyróżnikiem jest to, że **każdy blok kodu został skompilowany i uruchomiony**, zanim karta powstała — więc karta nie może twierdzić czegoś, czego kompilator nie wypisał.

Karty są po angielsku i tak zostaje, z tego samego powodu, dla którego cała ta biblioteka trzyma angielskie terminy w widoku: powtarzasz nazwy metod, komunikaty błędów i słowa kluczowe, czyli dokładnie te ciągi znaków, które zobaczysz w terminalu. Karta ucząca `pożyczanie` zamiast `borrow of moved value` nie przygotowałaby do niczego. Jeśli robisz sobie własne karty po polsku, warto trzymać tę samą zasadę: polskie zdanie, angielski termin w środku.

**Szukaj po polsku:** system powtórek Anki · powtórki rozłożone w czasie · fiszki do nauki programowania · `anki rust deck` · `spaced repetition programming`
