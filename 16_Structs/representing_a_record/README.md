# What is a record, in memory?

**Level:** 201 · working knowledge

**One line:** One row of ratings can be stored at least six ways in Rust, and the choice does not merely trade memory for speed — it decides which bugs are *possible to write*.

speed 5, price 3, support 0. That is the entire content of one survey response, and it is the smallest interesting data-modelling question there is: small enough to hold in your head, big enough that every major Rust container has an honest claim on it.

---

## Before the types: what is the answer *mathematically*?

Worth doing first, because it explains why the three ordinary kinds of form question need three different Rust shapes rather than three flavours of the same one.

| Question type | The mathematical object | The natural Rust type |
|---|---|---|
| **Rate each item** | a **function** from question to a value in 0..=5 | `[Score; N]` — one slot per question, always |
| **Tick all that apply** | a **subset** of the options | a bitset (`u64`), or `HashSet<OptionId>` |
| **Drag into order** | an **ordering** of the options — a permutation, and usually a *partial* one | `Vec<OptionId>` — the order *is* the data |

A rating question has a slot per item whether or not the respondent used it. An ordering question has no slots at all: its content is a sequence, and someone who orders two of five options has given you a sequence of length two, not a length-five thing with holes. Allowing ties ("these two are equal") pushes it further, to `Vec<Vec<OptionId>>` — a sequence of tiers. That is why one answer type cannot quietly serve both, and why a form parser needs one reader for `5,3,0` and a different one for `a>c>b`.

Everything below is about the rating row, the simplest of the three.

## Six shapes for the same three numbers

| Shape | Rust | Makes easy | Makes possible to get wrong | Reach for it when |
|---|---|---|---|---|
| Fixed array | `[Score; 3]` | Everything; it *is* the data, no heap, `Copy` | Column meaning lives elsewhere | The column count is known when you compile |
| Growable list | `Vec<Score>` | A count known only at runtime | Same column problem, plus a length mismatch | The real world — you load rows from files |
| Tuple struct | `Row(Score, Score, Score)` | Fixing the *arity* in the type | `.0` / `.1` are still positions | You want an array with a name |
| Named struct | `NamedRow { speed, price, support }` | Naming; misalignment becomes unwriteable | Nothing much — and that is the point | The columns are fixed and few |
| Map | `BTreeMap<QuestionId, Score>` | Lookup by name, sparse rows, no order at all | Iteration order (see below) | The columns vary per row |
| Flat matrix | one `Vec<u8>`, row-major | The whole table in one allocation | Index arithmetic | You are writing the actual data frame |

The sizes are worth seeing (they are printed by the program below): `[u8; 3]`, `[Stars; 3]`, the tuple struct and the named struct are **all 3 bytes** — identical layouts, differing only in what you are allowed to say about them. A `Vec<u8>` is **24 bytes before any data exists**, because it is a handle: pointer, length, capacity. That 24 bytes is precisely the price of not knowing the column count until the program runs, and it is usually worth paying.

## The bug positional storage cannot see

Three rows, `[[5,3,0], [4,5,1], [0,5,4]]`, and a header naming the columns:

```text
header speed,price,support -> speed 9, price 13, support 5   top: price
header price,speed,support -> price 9, speed 13, support 5   top: speed
```

Same numbers, different answer, and **nothing failed** — no parse error, no panic, not even a warning. The meaning of column 0 was never in the data; it was in a header the type system never saw. Every positional format has this hole, which is why CSV imports go wrong everywhere and always have.

The named struct closes it by construction: you write `.speed`, not `[0]`, and swapping the two is not a mistake you can make. The price is severe, though — the columns are baked into the *type*, so a program that loads a file with a header row cannot use it. **That tension is the actual content of this lesson.** Naming is safest and least flexible; positions are flexible and unsafe; a real reader buys back safety by making the position→column mapping a *value it carries* rather than a convention it remembers, which is what the flat matrix below does.

## Blank is not zero

Someone who answers `0` and someone who leaves the box empty both contribute nothing to the total. They did not do the same thing.

```rust
let scored_zero: Vec<Option<Stars>> = vec![Some(Five), Some(Three), Some(Zero)];
let left_blank:  Vec<Option<Stars>> = vec![Some(Five), Some(Three), None];
```

Both total 8. Only the second remembers that *support* was never answered. `Vec<Option<Stars>>` is the type that can tell you *"2 of 3 questions answered"*, which is a real quantity on any real form — and because `Stars` has six variants and 250 spare bit patterns, `Option<Stars>` is **still one byte**. You are asking for a distinction the machine gives away free, and it is the same [niche optimization](../../17_Option_and_Result/option_as_collection/README.md) as before, now paying for something you actually wanted.

Whether to keep it is a modelling decision, not a performance one. The tabulation does not care. The audit does.

## Maps, and the determinism trap

A map deletes the column problem outright: the key *is* the meaning, rows may be sparse, and no header can drift out of step.

But this program deliberately does **not** print a `HashMap`'s iteration order, and the reason is the point. `HashMap` randomises its hash seed per process, so the order changes run to run — the recorded answer key this library is built on could not exist. Now transplant that into a report: any procedure that breaks a tie by *scanning* a map inherits exactly that irreproducibility, and a total nobody can reproduce is not a total. `BTreeMap` iterates in key order, always, which is why it is the default choice here even though it is asymptotically slower.

Rust makes you notice this. A language whose dictionaries happen to preserve insertion order lets you build the same dependency without ever finding out you have one.

## One allocation for the whole table

```rust
struct Survey {
    questions: Vec<String>,    // position -> which column
    cells: Vec<u8>,            // row-major: row * n + column
}
```

This is the shape a real data frame converges on. All the rows live in one contiguous block, which is fast for the same reason a spreadsheet is fast, and the column names are held **once**, beside the data, instead of being repeated per row or implied by a header somewhere else. The misalignment bug is traded for an index-arithmetic bug — `row * n + col` — and the trade is only worth it because that expression is written once, inside one method, where it can be tested. Written at nine call sites it would be worse than the header.

A weighted group (`42 × 5,4,3`) is the last shape, and it is a **compression** rather than a different table: the count multiplies the row, it does not join it. Storing 42 as a fourth number in the same array would be the same category error as the header — a value whose meaning depends on which column it landed in.

## If you are coming from another language

- **Python** — you have written all six: the list, the dict, the `namedtuple`/dataclass, the list-of-dicts, and the DataFrame, which *is* the flat matrix with the column names carried alongside. What changes in Rust is that the choice is visible in every signature. `def total(rows)` accepts all six shapes and discovers the mismatch at runtime; `fn total(rows: &[Score])` accepts one, and the compiler checks every caller.
- **ABAP** — an internal table of a structure is the named-struct shape, and a `TYPE STANDARD TABLE OF i` is the positional one. The header-drift bug in Step 2 is the classic CSV-into-ITAB defect: field order is a convention held between an upload routine and a `MOVE-CORRESPONDING`, and nothing complains when it slips. Rust's contribution is not the check; it is that the shape is part of the type, so the two ends cannot disagree silently.

## IOUs from this rung

Debts taken deliberately, each one a later lesson:

- The rows are **hard-coded**, not read from a file — no parsing, no `Result`.
- Column names are `&str` and `String`, chosen carelessly and cloned freely.
- `top()` calls `.unwrap()` on `max_by_key`, which panics on an empty table, and says nothing about a tie.
- Three columns everywhere, because `[u8; 3]` is a compile-time size. Const generics (`[Score; N]`) would generalise it.

---

## Practice

**The line you forgot.** Store one record two ways — parallel `Vec`s of column names and ratings, and a `Vec` of one-row-per-column structs — with a lookup for each. Then remove a column from the parallel version and forget the second `remove`.

Run the lookup afterwards. It does not panic and it does not return `None`: it returns another column's rating, as a plausible number. Then make the same mistake in the row shape and find that there was no second line to forget. Finish by writing down what the row shape costs, because it does cost something.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:representing_a_record_kata -->
*[`representing_a_record_kata.rs`](examples/representing_a_record_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: two ways to store one record, and the bug each one allows.
//!
//!   rustc --edition 2024 representing_a_record_kata.rs -o /tmp/rrk && /tmp/rrk

/// Shape 1: two parallel Vecs. Nothing ties a column name to a value.
struct Parallel {
    questions: Vec<&'static str>,
    ratings: Vec<u8>,
}

impl Parallel {
    fn rating_for(&self, which: &str) -> Option<u8> {
        let i = self.questions.iter().position(|q| *q == which)?;
        self.ratings.get(i).copied()
    }
}

/// Shape 2: one row per question. The pairing is the type.
#[derive(Debug)]
struct Entry {
    question: &'static str,
    rating: u8,
}

fn rating_for(record: &[Entry], which: &str) -> Option<u8> {
    record.iter().find(|e| e.question == which).map(|e| e.rating)
}

fn main() {
    let mut p = Parallel {
        questions: vec!["speed", "price", "support"],
        ratings: vec![5, 3, 0],
    };
    let rows = vec![
        Entry { question: "speed", rating: 5 },
        Entry { question: "price", rating: 3 },
        Entry { question: "support", rating: 0 },
    ];

    println!("Both hold the same record:");
    println!("  parallel: price -> {:?}", p.rating_for("price"));
    println!("  rows:     price -> {:?}", rating_for(&rows, "price"));

    println!("\nNow a question is dropped from the form, and one line gets forgotten:");
    p.questions.remove(1); // and the matching ratings.remove(1) never happens
    println!("  parallel: questions {:?} ratings {:?}", p.questions, p.ratings);
    println!("  parallel: support -> {:?}   <- WRONG, that is price's 3", p.rating_for("support"));
    println!("      The two Vecs disagree and nothing noticed. This code compiles,");
    println!("      runs, and reports a plausible number.");

    println!("\nThe same mistake in the row shape:");
    let mut rows = rows;
    rows.remove(1);
    println!("  rows: {:?}", rows.iter().map(|e| (e.question, e.rating)).collect::<Vec<_>>());
    println!("  rows: support -> {:?}   <- still right", rating_for(&rows, "support"));
    println!("      There was no second line to forget. The desync is not a bug");
    println!("      you avoided by being careful; it is a bug you cannot write.");

    println!("\nWhat the row shape costs, honestly:");
    println!("  lookup is a scan, not an index — fine for 5 questions, wrong for");
    println!("  50,000 rows in a hot loop, which is where the flat matrix and");
    println!("  the index-newtype shapes start to earn their extra ceremony.");
}
```
<!-- /source -->

<!-- output:representing_a_record_kata -->
*Verified output of [`representing_a_record_kata.rs`](examples/representing_a_record_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Both hold the same record:
  parallel: price -> Some(3)
  rows:     price -> Some(3)

Now a question is dropped from the form, and one line gets forgotten:
  parallel: questions ["speed", "support"] ratings [5, 3, 0]
  parallel: support -> Some(3)   <- WRONG, that is price's 3
      The two Vecs disagree and nothing noticed. This code compiles,
      runs, and reports a plausible number.

The same mistake in the row shape:
  rows: [("speed", 5), ("support", 0)]
  rows: support -> Some(0)   <- still right
      There was no second line to forget. The desync is not a bug
      you avoided by being careful; it is a bug you cannot write.

What the row shape costs, honestly:
  lookup is a scan, not an index — fine for 5 questions, wrong for
  50,000 rows in a hot loop, which is where the flat matrix and
  the index-newtype shapes start to earn their extra ceremony.
```
<!-- /output -->

</details>

---

## The verified output

<!-- output:representing_a_record -->
*Verified output of [`representing_a_record.rs`](examples/representing_a_record.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── Step 1: One record, six shapes
  [u8; 3]                 [5, 3, 0]
  Vec<u8>                 [5, 3, 0]
  Row(Stars, Stars, ..)   Row(Five, Three, Zero)
  NamedRow { .. }         speed=Five price=Three
  BTreeMap<&str, Stars>   {"price": Three, "speed": Five, "support": Zero}
  Vec<(&str, Stars)>      [("speed", Five), ("price", Three)]  <- sparse: support simply absent
    (Row is positional too: .0 = 5, .1 = 3, .2 = 0)
    (every shape above still goes through one door: Stars::new(4) -> Some(Four), Stars::new(6) -> None)

  What each one costs (size_of, the handle only):
    [u8; 3]                      3 bytes  (all of it, no heap)
    [Stars; 3]                   3 bytes
    Row                          3 bytes
    NamedRow                     3 bytes
    Vec<u8>                     24 bytes  + heap for the data
    BTreeMap<&str, Stars>       24 bytes  + heap, + the keys
      A fixed array IS the data. A Vec is a 24-byte handle (pointer,
      length, capacity) pointing at data somewhere else — which is
      the price of not knowing the column count until runtime.

──── Step 2: The bug positional storage cannot see
  same numbers, header speed,price,support -> [("speed", 9), ("price", 13), ("support", 5)]
                        top             -> price (13 points)
  same numbers, header price,speed,support -> [("price", 9), ("speed", 13), ("support", 5)]
                        top             -> speed (13 points)
      Nothing failed. No parse error, no panic, no warning — just a
      different answer, because a column's meaning lives outside the
      data, in a header the type system never saw.
  NamedRow: speed=5 price=3 support=0
      Here the same mistake is unwriteable: you ask for `.speed`, not
      for column 0. The cost is that the questions are baked into
      the type at compile time — fine for a lesson, useless for a
      form you load from a file. That tension is the real lesson.

──── Step 3: Blank is not zero
  scored 0   [Some(Five), Some(Three), Some(Zero)] -> total 8, 3 of 3 questions answered
  left blank [Some(Five), Some(Three), None] -> total 8, 2 of 3 questions answered
      Both total 8: a blank sums as zero. But they are different
      answers, and only the Option remembers which one happened.
  size_of::<Stars>()         = 1
  size_of::<Option<Stars>>() = 1  <- remembering costs nothing

──── Step 4: Order-free lookup, and the determinism trap
  inserted support, speed, price — iterating a BTreeMap gives:
    price    3
    speed    5
    support  0
  record.get("price")  -> Some(3)
  record.get("colour") -> None
      A map drops the column problem entirely: the key IS the meaning.
      This program deliberately does NOT print a HashMap's iteration
      order, because it is randomised per run — the answer key could
      not be recorded. A report that breaks ties by scanning a map
      would inherit exactly that irreproducibility.

──── Step 5: One allocation for the whole survey
  cells (row-major, 3 responses x 3 questions): [5, 3, 0, 4, 5, 1, 0, 5, 4]
  rating(response 2, question 1) = 5
    speed    9
    price    13
    support  5
      One Vec for the whole survey instead of one per response: the
      shape a real data frame uses. It trades the misalignment bug for
      an index-arithmetic bug — `row * n + col`, written once, in one
      method, which is the only reason the trade is worth making.

──── Step 6: Identical responses, compressed
  75 responses stored as 3 rows
    speed    231
    price    298
    support  265
      A weighted row is a COMPRESSION, not a different survey: the
      count multiplies, it does not join the ratings. Storing 42 as a
      fourth number in the same array would be the same category
      error as the header in Step 2 — a value whose meaning depends
      on which column it landed in.
```
<!-- /output -->

Run it yourself:

```bash
rustc --edition 2024 16_Structs/representing_a_record/examples/representing_a_record.rs -o /tmp/rr && /tmp/rr
```

## See also

- [A score is not a number](../newtype_score/README.md) — the `Stars` type this page stores six ways
- [`Option` fields](../../17_Option_and_Result/option_fields/README.md) — the "required by default" instinct, and when `Option` in a struct is right
- [The long way round](../../ROADMAP.md) — the ladder these rungs climb

## Po polsku

Zanim padnie choćby jeden typ, warto zadać pytanie matematyczne — ono tłumaczy, dlaczego trzy zwykłe rodzaje pytania w formularzu potrzebują trzech różnych kształtów w Ruscie, a nie trzech odmian jednego:

- **oceń każdą pozycję** to **funkcja** z pytania w wartość 0..=5, więc każde pytanie ma swoją przegródkę, nawet jeśli odpowiadający jej nie wypełnił — stąd `[Score; N]`;
- **zaznacz wszystkie pasujące** to **podzbiór** opcji — maska bitowa (*bitset*) albo `HashSet<OptionId>`;
- **ustaw w kolejności** to **porządek**, zwykle częściowy — `Vec<OptionId>`, w którym danymi jest sama kolejność, a przy dopuszczonych remisach `Vec<Vec<OptionId>>`, czyli ciąg poziomów.

Ktoś, kto uszeregował dwie z pięciu opcji, oddał ciąg o długości dwa, a nie pięcioelementową strukturę z dziurami — i właśnie dlatego jeden typ odpowiedzi nie obsłuży po cichu dwóch rodzajów pytania naraz.

Sześć kształtów tego samego wiersza różni się nie tyle szybkością, ile tym, **jakie błędy da się w nich w ogóle napisać**. Tablica `[u8; 3]`, `[Stars; 3]`, struktura krotkowa `Row` i struktura nazwana `NamedRow` zajmują **po 3 bajty** — mają identyczny układ w pamięci i różnią się wyłącznie tym, co wolno o nich powiedzieć. `Vec<u8>` to **24 bajty, zanim pojawi się jakakolwiek dana**, bo na stosie leży sam uchwyt (wskaźnik, długość, pojemność), a dane mieszkają na stercie; te 24 bajty to dokładna cena za to, że liczby kolumn nie znasz w czasie kompilacji. W zapisie pozycyjnym czai się przy tym błąd, którego żaden typ nie zobaczy: te same trzy wiersze z nagłówkiem `speed,price,support` dają zwycięzcę `price` (13 punktów), a z nagłówkiem `price,speed,support` — `speed` (też 13). **Nic się nie zepsuło**: ani błąd parsowania, ani panika, ani nawet ostrzeżenie. Znaczenie kolumny 0 nigdy nie było w danych, tylko w nagłówku, którego system typów nie widział. Struktura nazwana zamyka tę dziurę z definicji (piszesz `.speed`, a nie `[0]`), ale wpisuje kolumny do **typu**, więc pliku z własnym wierszem nagłówka już nią nie opiszesz — i ta sprzeczność jest właściwą treścią lekcji.

„Puste” to nie „zero”: pominięta kratka i wpisana ocena 0 wnoszą do sumy tyle samo, ale to nie jest ta sama odpowiedź. `Vec<Option<Stars>>` pamięta różnicę i pozwala powiedzieć „odpowiedziano na 2 z 3 pytań” — wielkość, która w prawdziwym raporcie naprawdę występuje. Najlepsze, że nic nie kosztuje: `Stars` ma sześć wariantów i 250 wolnych układów bitów, więc `Option<Stars>` nadal zajmuje **jeden bajt** (ta sama nisza, *niche optimization*, co przy poprzedniej lekcji). Czy tę informację zachować, jest decyzją modelową, nie wydajnościową — sumowaniu jest wszystko jedno, analizie nie.

Mapa znosi problem kolumn, bo znaczeniem jest sam klucz, ale wprowadza inny, przy raportach szczególnie kłopotliwy: `HashMap` losuje ziarno funkcji skrótu **przy każdym uruchomieniu procesu**, więc kolejność iteracji zmienia się z przebiegu na przebieg. Procedura, która rozstrzyga remis, przeglądając mapę, dziedziczy tę nieodtwarzalność — a wynik, którego nikt nie potrafi powtórzyć, nie jest wynikiem. Dlatego domyślnym wyborem jest tutaj `BTreeMap`, iterujący zawsze w porządku kluczy, mimo gorszej złożoności; w językach, w których słownik przypadkiem zachowuje kolejność wstawiania, tę samą zależność buduje się, nigdy się o niej nie dowiadując. Docelowy kształt to natomiast jedna płaska tablica: `questions: Vec<String>` (pozycja → która kolumna) plus `cells: Vec<u8>` w porządku wierszowym (*row-major*), czyli `row * n + col`. Błąd rozjechanego nagłówka zostaje wymieniony na błąd arytmetyki indeksu, a opłaca się to tylko dlatego, że to wyrażenie pisze się **raz**, w jednej metodzie, którą da się przetestować. Waga grupy (`42 × 5,4,3`) jest zaś kompresją, a nie inną tabelą: liczba mnoży wiersz, a nie dołącza do niego — wstawienie 42 jako czwartej liczby do tej samej tablicy byłoby dokładnie tym samym błędem kategorii co nagłówek.

**Szukaj po polsku:** modelowanie danych w Ruscie · kolejność iteracji HashMap · stos i sterta · `rust BTreeMap vs HashMap deterministic iteration` · `rust size_of Vec pointer len capacity`
