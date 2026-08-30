# Rust by Example

**Level:** 101 · for newcomers

**One line:** [Rust by Example ↗](https://doc.rust-lang.org/rust-by-example/) is the official companion to The Book with the ratio inverted — runnable code with commentary, rather than prose with snippets — which makes it the wrong thing to read cover to cover and the right thing to have open in a tab.

## Why it is foundational

Because the other two cannot answer a question in ten seconds.

The Book teaches a model, and a model is what you need at chapter 4 and not what you need at 4pm when you have forgotten how to write a closure that captures by move. rustlings tells you that you are wrong, which is different from telling you what right looks like. Rust by Example is the one you reach for mid-edit, and the reason it works is that **every example runs in the page**: there is a play button, the code is editable, and you can change a line and see what breaks without leaving the browser or making a project.

That last property is worth more than it sounds for a beginner. The distance between "read that a `&mut` is exclusive" and "changed a `&` to `&mut` and watched the error appear" is the distance between knowing and understanding, and RBE makes it about four seconds.

It also **ships with the toolchain** — `rustup doc --rust-by-example` — so like The Book it is offline and version-matched.

## How to use it

**Not front to back.** It is organised as a reference: Hello World, Primitives, Custom Types, Variable Bindings, Types, Conversion, Expressions, Flow of Control, Functions, Modules, Crates, Cargo, and onward through generics, scoping, traits, macros and error handling. Read in order it is a list of features with no argument connecting them, which is precisely what The Book supplies and RBE deliberately does not.

Three moments it is the right answer:

- **"What is the syntax for…"** — the fastest lookup available, faster than the standard library docs, because the answer is a whole working program rather than a signature.
- **"Show me that idea in code"** — when a Book chapter has explained something and you want to see it small. Its trait, generics and macro sections are especially good this way.
- **"What happens if I change this?"** — the editable examples make it a scratchpad you do not have to set up.

## Where it stops

It shows *what*, rarely *why*. There is no equivalent of The Book's chapter 4 argument; ownership appears as syntax to be demonstrated rather than a design to be justified. Used alone it produces a familiar failure — someone who recognises every construct and cannot decide which to reach for.

So: not a course. A dictionary you happen to be able to run.

## If you are coming from another language

- **Python** — closest to the "Examples" section in a module's docs, or to keeping a REPL open. The editable code blocks are doing the REPL's job, which is why it fills the gap Rust's compile step otherwise opens up.
- **ABAP** — closest to the ABAP demo programs (`DEMO_*`) in SE38: small, complete, runnable, and there to be copied from. Same use, same caveat — a demo shows you a shape, not whether it is the shape you want.

## See also

- [The Book](../the_book/README.md) — the argument RBE deliberately leaves out
- [rustlings](../rustlings/README.md) — the reps
- [Rust by Example, page by page](../../RUST_BY_EXAMPLE.md) — RBE's 24 chapters mapped onto the lessons here, chapter by chapter, with the gaps named
- [cheats.rs ↗](https://cheats.rs/) — even faster lookup once you know what you are looking for, and much denser

## Po polsku

Z całej tej sekcji Rust by Example ma najniższy próg językowy, bo treść niesie tu **uruchamialny program**, a proza jest tylko podpisem pod nim — kod nie wymaga tłumaczenia, a przycisk uruchamiania odpowiada w tym samym języku każdemu. Dla uczącego się po polsku dochodzi do tego drugie zastosowanie, którego angielski czytelnik nie potrzebuje: to najszybszy sposób sprawdzenia, czy dobrze zrozumiałeś polskie wyjaśnienie. Zmień jedną linijkę, uruchom, zobacz, czy kompilator się zgadza — cztery sekundy zamiast szukania drugiego opisu tego samego. Ten sam zbiór masz offline i w wersji zgodnej z twoim kompilatorem: `rustup doc --rust-by-example`.

Cena jest dokładnie tam, gdzie strona ją stawia — RBE pokazuje **co**, rzadko **dlaczego** — i po polsku ta cena jest wyższa, niż się wydaje. Z samego kodu wyniesiesz konstrukcję, ale nie jej **nazwę**, a nazwa jest tym, co się wpisuje w wyszukiwarkę i co pozwala o coś zapytać. Nazwy są angielskie: `match`, `impl`, `trait`, do tego domknięcie (*closure*) i wycinek (*slice*). Kto uczy się wyłącznie z przykładów, kończy jako ktoś, kto rozpoznaje wszystko i nie potrafi nazwać niczego — model i słownictwo bierz z The Book i z lekcji w tej bibliotece, składnię stąd.

**Szukaj po polsku:** przykłady kodu w Ruscie · składnia Rusta · `rust by example` · `rustup doc --rust-by-example` · `cheats.rs`
