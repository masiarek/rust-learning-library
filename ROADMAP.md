# The long way round to a column summary

**Level:** reference · the plan

**One line:** Summarising a column of numbers is about thirty lines of code, which makes it a poor destination and an excellent excuse for a walk.

Read a file of rows. Total each column. Report the two largest, and say honestly what happened if they are equal. That is the whole program, and you could write it in an afternoon in a language you already know. Written slowly in Rust it is something better: a chain of small, concrete questions, each of which happens to be the question a Rust chapter is trying to answer.

**The learning is the point.** If we reach a working tool, good — but if we stop at rung 4 and wander off into iterators for a month, that is not a failure of the plan; it *is* the plan. Every rung is a standalone lesson that stands up with no interest in CSV files whatsoever.

---

## Two rules of the road

**1. Order by what Rust wants to teach, not by what the program needs.** The program's natural order would have us fighting the borrow checker on day two. The rungs below are sequenced so each one needs exactly one idea you do not have yet.

**2. Cheat forward, and write the cheat down.** A rung will sometimes need something two rungs ahead. Take the shortcut — `clone()` the thing, `unwrap()` the `Option`, hard-code the input — and record it on the page as an **IOU**. Later, the lesson that pays it off opens with the debt it is settling, which is a far better motivation than *"and now, lifetimes."* The list of outstanding IOUs is a reading order that writes itself.

A third, quieter one: **each rung is a complete program in one file, and it stays frozen once written.** Rung 5 does not import rung 4. That looks wasteful and is not: the *diff* between two rungs is the lesson, and a shared library would mean editing rung 3 to break rung 9's page. Turning the pile of files into a real crate is itself a rung — number 10.

## The ladder

| # | The step | The Rust idea it exists to teach | Status |
|---|---|---|---|
| 1 | A rating is 0–5, not any integer | Newtypes, `struct`/`impl`/`derive`, and why privacy is per *module* | [written](16_Structs/newtype_score/README.md) |
| 2 | What *is* a record, in memory? | The data types: array vs `Vec` vs tuple vs struct vs `HashMap`, and owned vs borrowed | [written](16_Structs/representing_a_record/README.md) |
| 3 | A column is not its heading | Index newtypes, `&str` vs `String`, and lookup by position rather than by string | planned |
| 4 | Total each column | Iterators — `map` / `sum` / `fold` — and the overflow you get for choosing the wrong integer to sum *into* | planned |
| 5 | Take the top two | `Ord` vs `PartialOrd`, `sort_by_key`, why floats cannot be `Ord`, and what to do about a tie | planned |
| 6 | Say what a tie is | Enums that carry data, exhaustive `match`, and a return type that cannot lie about a tie | planned |
| 7 | Read rows from text | `parse`, `Result`, `?`, and `collect::<Result<Vec<_>, _>>()` — an error that names the row it came from | planned |
| 8 | Who owns the rows? | Ownership, borrowing, `&[T]` vs `Vec<T>` — and the first lifetime you actually have to write | planned |
| 9 | More than one summary | Traits: one `Summarize` interface over total, mean and median; `impl Trait` vs `dyn Trait` | planned |
| 10 | Make it a real program | `cargo`, modules across files, `#[test]`, doc tests, and the day the single-file rule retires | planned |
| 11 | Test the *rules*, not the cases | Property testing — e.g. "scaling every row by N leaves the ranking unchanged", which is a claim about the code rather than about one file | planned |
| 12 | Check it against something that already works | Run the same file through `awk` and `sort`, and diff. The [Unix section](11_Unix/README.md) is the other half of this rung | planned |

Rung 12 is the one where the tool stops being a toy, and it is deliberately last: everything before it is Rust, and only there does the answer become one somebody else can check without reading your code.

## Detours are first-class

Some of the best pages here will have nothing to do with the ladder. Closures, `Rc` and `RefCell`, `Cow`, iterator laziness, formatting, `thiserror` versus `anyhow`, why `async` is the way it is — if one of them is the thing you want to understand this week, write that page and let the ladder wait. It is not going anywhere, and a concept learned because you were curious sticks better than one learned because a table said it was next.

The only thing worth protecting is the house rule that makes the library trustworthy: [every claim on a page is printed by a program that CI actually runs](CONTRIBUTING.md).

## Po polsku

To jest plan powolnej, całkowicie opcjonalnej ścieżki przez bibliotekę: podsumowanie kolumny liczb jako pretekst do nauki Rusta. Cały program mieści się w trzech zdaniach — wczytaj wiersze, zsumuj każdą kolumnę, podaj dwie największe i powiedz uczciwie, co zrobić przy remisie — i właśnie dlatego jest świetnym ćwiczeniem, a kiepskim celem: kodu jest ze trzydzieści linii, więc cała wartość leży w drodze, nie w mecie.

Kolejność szczebli wynika z tego, czego chce nauczyć Rust, a nie z tego, czego potrzebuje program: naturalna kolejność algorytmu kazałaby walczyć z **pożyczaniem** (*borrow checker*) już drugiego dnia. Druga zasada jest równie ważna i rzadziej spotykana w kursach: wolno **pożyczyć z przyszłości** — `clone()`, `unwrap()`, wpisane na sztywno dane — pod warunkiem że zapisze się to na stronie jako dług (*IOU*). Lekcja, która ten dług spłaca, zaczyna się wtedy od konkretnego problemu, a nie od zdania „a teraz czasy życia”.

Każdy szczebel tej ścieżki stoi samodzielnie i uczy czegoś, czego i tak trzeba się nauczyć. Plik CSV jest tylko wymówką, żeby pytania przychodziły w sensownej kolejności, zamiast z listy tematów.

**Szukaj po polsku:** przetwarzanie plików CSV · podsumowanie kolumny · testy własnościowe · `rust learning project ideas` · `rust csv parsing`
