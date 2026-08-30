# Measured claims — six numbers, and what each one counted

**Level:** 101 → 201 · for newcomers

**One line:** Six selling points that arrive with evidence attached, and the evidence turns out to be three different kinds — two numbers you can print on your own machine in a second, three that weighed a pile of contributed programs rather than a language, and one that is not a measurement at all, which is the strongest of the six.

[cheats.rs ↗](https://cheats.rs/) opens with a short list headed *Things Rust does measurably really well*: six bullets, each carrying a link to whatever measured it. The word **measurably** is why the list deserves a page. It is a promise that these are not opinions, and mostly the promise is kept — every link resolves, and every number is real.

But a number is only as good as the sentence naming its subject, and the subjects on this list are not the same thing twice. They are: a corpus of contributed programs, one company's bug database, the toolchain sitting on your disk, an opinion poll, and — once — a proof. Sorting them out is the difference between quoting the list and being caught out by it in an argument.

[Benefits of Rust](../benefits_of_rust/README.md) does this job for the twenty claims Google's course opens with, and the two lists barely touch: two bullets in common out of twenty-six. That is not an accident. Google's list asks **what does the compiler refuse to build**, and answers in error codes. This list asks **what has somebody measured**, and answers in citations. The first kind of answer you can reproduce in a terminal; the second is true of whatever was actually put on the scale, which is the thing to go and check.

## The six, and what kind of claim each is

Eight rows for six bullets, because the first bullet bundles three separate measurements behind one link, and they do not all point the same way — which is the first thing worth knowing about it.

| The claim | The number | What was actually counted | Checked |
|---|---|---|---|
| Performance about the same as C/C++ | 2.19s against C's 2.10s | contributed programs, optimised to unknown and differing degrees | [below](#three-numbers-and-one-pile-of-programs) |
| …and excellent memory efficiency | 3,023 KB against C's 2,482 KB | the same programs, on a meter the bullet's own link does not show | [below](#the-memory-half-does-not-follow) |
| …and excellent energy efficiency | 59J against C's 57J | the same programs again, on a wall-power meter, in 2017 | [below](#the-energy-half-is-those-programs-again) |
| Avoids ~70% of the safety issues in C/C++ | 70% of 912 bugs | Chromium's own high and critical security bugs since 2015 | [C and C++](../../31_C_and_Cpp/README.md) |
| The type system prevents data races | *(none)* | nothing — it is a guarantee, not a sample | [Data races](../../31_C_and_Cpp/data_races/README.md) |
| Seamless C interop, dozens of platforms | 330 targets | what the `rustc` on your disk says it can emit for | [below](#dozens-of-platforms-is-330) |
| Most loved or admired, eight years running | over 80% | people who already use Rust and want to carry on | [below](#one-number-that-measured-an-opinion) |
| Modern tooling — `cargo`, `clippy`, `rustup` | 822 lints | what the `clippy` on your disk says it knows how to spot | [below](#700-clippy-lints-is-822-in-ten-groups) |

## Two you can count yourself

These are the two easiest claims on the list to verify and the two nobody verifies, for the same reason: both are hedged. *"Dozens of platforms"* and *"700+ lints"* are shaped to be waved past. Both are undercounts, and each takes one command.

Every number below came out of the compiler this library pins — `rustc 1.98.0`, see [Pinning the toolchain](../../05_Tooling/pinning_the_toolchain/README.md) — and will move when the pin moves. That is the point of printing them rather than quoting them.

### "Dozens of platforms" is 330

```sh
rustc --print target-list | wc -l
```

```text
330
```

What a target *is*, and what tier 1, 2 and 3 actually promise, belongs to [Targets and triples](../../20_Compilers/targets_and_triples/README.md). The only thing the number adds is scale — and one detail worth pulling out of the list:

```sh
rustc --print target-list | grep -c -- -none
```

```text
65
```

Sixty-five of the 330 name **no operating system at all**. That is the half of "dozens of platforms" that a desktop programmer never sees and that explains why the claim is made in the first place: the same compiler, the same language, and no OS underneath it. The rest are led by 92 Linux targets and 20 Windows ones — `grep -c -- -linux` and `-windows` on the same list.

The number is *what this compiler will emit for*, which is not the same as what anyone tests — and that gap is the entire content of the tier system. "Supported" needs its footnote, and the footnote is on the targets page.

### "700+ clippy lints" is 822, in ten groups

```sh
clippy-driver -Whelp | grep -cE 'clippy::[a-z0-9_-]+ +(allow|warn|deny|forbid)'
```

```text
822
```

The pattern is fussy for a reason: `-Whelp` also prints the ten **groups** — `correctness`, `suspicious`, `style`, `complexity`, `perf`, `pedantic`, `restriction`, `nursery`, `cargo`, and `all` — each on a line listing its members, and a plain `grep -c clippy::` counts those too and reports 832. Ten of them are not lints. The library's own arithmetic should survive being checked, so: **822**.

Split by the level each one ships at, the number says something the bullet does not:

| Default | Count | Meaning |
|---|---|---|
| `warn` | 422 | fires on an ordinary `cargo clippy` |
| `deny` | 67 | fails the build |
| `allow` | 333 | silent until you ask for it |

**Two out of every five clippy lints are switched off**, and they are switched off because they are opinions rather than defects — a `restriction` lint like "never index a slice" is right for some codebases and wrong for most. Turning that set on is a decision with a bill attached, and the bill is what [Strict clippy](../../05_Tooling/strict_lints/README.md) is about. So "700+ lints" understates the count and overstates what arrives switched on.

For scale, `rustc` itself carries 245 lints — 137 `warn`, 47 `deny`, 61 `allow`. Why the two tools draw the line where they do, and what a warning is actually asking you, is [What a warning is asking](../../15_First_Programs/what_a_warning_is_asking/README.md).

## Three numbers, and one pile of programs

The first bullet is three claims wearing one coat — speed, memory, energy — behind a single link to [the Benchmarks Game's box-plot summary ↗](https://benchmarksgame-team.pages.debian.net/benchmarksgame/box-plot-summary-charts.html). Open it and every chart on it plots **time**. Not memory, not energy. Those two halves come from somewhere else, and the somewhere else turns out to be the same programs.

### The speed half

Take one problem, `n-body`, from the [Benchmarks Game ↗](https://benchmarksgame-team.pages.debian.net/benchmarksgame/performance/nbody.html) (version 25.03):

| Program | secs | mem (KB) |
|---|---|---|
| C, gcc #9 | 2.10 | 2,482 |
| C++, g++ #0 | 2.15 | 2,417 |
| Rust #9 | 2.19 | 3,023 |

Close, exactly as advertised. Three things on that same page belong with the number:

- **Its author tells you twice, on the page, to read the source code**, and says plainly that nobody knows how much optimisation went into each program. The site's own epigraph is Leonard Courtney's line about lies, damned lies and statistics. This is not a hostile reading of the Benchmarks Game — it is the Benchmarks Game's reading of itself.
- **All three rows above carry a `*`**, which that page's legend defines as *possible hand-written vector instructions or "unsafe"*. So the Rust program demonstrating "as fast as C" may be one that opted out of the guarantee the *next* bullet is about. Nothing is being hidden — the legend is right there — but the two bullets are not necessarily describing the same program. What `unsafe` does and does not switch off: [What `unsafe` turns off](../../09_Advanced/what_unsafe_turns_off/README.md).
- **The summary chart drops three problems** — `binary-trees`, `pidigits` and `regex-redux` — because library and approach differences would vanish into an average. A summary that excludes its awkward cases is still useful; it is just not the whole game.

And the *mechanism* — the reason this result is even available — is on none of those pages. It is that below LLVM's intermediate representation, the two languages are the same program going through the same optimiser: see [LLVM and its IR](../../20_Compilers/llvm_and_its_ir/README.md) and [What the optimizer does](../../20_Compilers/what_the_optimizer_does/README.md). That is a far better answer to *"how can it possibly be as fast as C"* than any chart, because it explains rather than reports.

### The memory half does not follow

The n-body row already says it: Rust is a hair behind C on time and uses about a fifth more memory than either C or C++ on that problem. And the energy study below supplies the sharper case — on its `fasta` benchmark Rust is the **most energy-efficient language measured**, and would fall **nine places** if the same table were sorted by peak memory instead.

So "excellent memory and energy efficiency" staples together two properties that the sources themselves separate. The claim worth making is a different one and is not about size: no garbage collector and no runtime means memory use that is *predictable* — you can point at the line that allocates. Predictable is not the same as small, and [The global allocator](../../09_Advanced/the_global_allocator/README.md) is where that distinction gets its own page.

### The energy half is those programs again

The energy claim traces to Pereira, Couto, Ribeiro, Rua, Cunha, Fernandes and Saraiva, [*Energy Efficiency across Programming Languages* ↗](https://greenlab.di.uminho.pt/wp-content/uploads/2017/10/sleFinal.pdf) (SLE 2017) — twenty-seven languages over ten problems, with energy, time and peak memory measured for each. Its top five by energy and time:

| Language | Energy | Time |
|---|---|---|
| C | 57 J | 2,019 ms |
| **Rust** | **59 J** | **2,103 ms** |
| C++ | 77 J | 3,155 ms |
| Ada | 98 J | 3,740 ms |
| Java | 114 J | 3,821 ms |

Rust second, just behind C. A good result, and a fairly gathered one.

**But the paper's ten problems are the Computer Language Benchmarks Game's problems** — it says so, and cites the same site the bullet links. So the speed claim and the energy claim are not two independent confirmations. They are one corpus of contributed programs, weighed on two different meters, by two different groups, eight years apart. That does not make either wrong. It does mean that the caveat the Benchmarks Game prints about its own data applies to both bullets at once, and that a weakness in the corpus is a weakness in the whole first bullet rather than a third of it.

The paper is also from 2017 — before `async`/`await`, before const generics, on the compilers of the time. Nothing here re-ran it, so treat the *ranking* as the durable part and the *joules* as a 2017 reading rather than a current one.

## One number that counted real bugs

The 70% is the strongest citation on the list, and it is strongest for a reason none of the others can claim: its subject is neither a benchmark nor an opinion. It is Chromium's own record of shipped, high-severity security bugs in a real C++ program — 912 of them, and around half of the memory-unsafety share is use-after-free alone.

The number, its source and the nine programs it is really about live in [C and C++](../../31_C_and_Cpp/README.md), which compiles and runs each bug rather than describing it. What belongs here is only the shape of the claim, and one boundary: **70% is a share of Chromium's bugs, in C++, in a browser.** Read as "70% of all software bugs", it is wrong twice over — wrong about the population, and wrong about the category, since it counts *security* bugs and not defects in general.

## One claim that is not a number

Bullet three — the type system rules out data races — is the only entry on a list called *measurably* with nothing measured, and it is the best one there.

You cannot count the data races Rust prevented, for the same reason you cannot count the burglaries a locked door prevented. What stands in for the number is a program that does not compile, which is a stronger form of evidence than a sample: a measurement tells you what happened to the cases someone looked at, and `Send`/`Sync` tell you what happens to all of them. Watch it from both sides — the C version losing 850,000 increments in [Data races](../../31_C_and_Cpp/data_races/README.md), and the rule that stops it in [`Send` and `Sync`](../../09_Advanced/send_and_sync/README.md) and [Sharing across threads](../../18_Ownership/sharing_across_threads/README.md).

One narrowing travels with it every time, and [Benefits of Rust](../benefits_of_rust/README.md) states it in full: a data race is not a concurrency bug in general. Deadlocks, lost updates across two separate locks and ordering mistakes in your own logic are all still available, and none of them is a build error.

## One number that measured an opinion

*"Most loved or admired language for eight years in a row."* True, sourced, and the most likely of the six to be quoted for something it does not say.

Stack Overflow's [2023 survey ↗](https://survey.stackoverflow.co/2023/#section-admired-and-desired-programming-scripting-and-markup-languages) defines **admired** precisely: of the people who used a technology in the past year, the share who want to keep using it. Over 80% for Rust. That is a **retention rate among people who already chose it** — so it says nothing about anyone who tried Rust and left, nothing about the far larger group who never tried, and nothing about whether the language is any good. It says that people who write Rust want to carry on writing Rust, which is worth knowing and is not a claim about quality.

The strikethrough joke in the original — ~~4~~ ~~5~~ ~~6~~ ~~7~~ 8 — spans a rename. 2023 is the year the survey replaced *"loved"* with *"admired"* and introduced the admired-versus-desired framing, so the streak is counted across a metric that changed names and presentation partway through. Worth knowing before you put the streak in a slide.

## The half of bullet four with no number

Bullet four is two claims sharing a comma, and only one of them was counted. *"Dozens of platforms"* has 330 behind it. *"Seamless C interop"* has nothing — no benchmark, no survey, no bug database — and it is the one claim on the list where that is the right answer rather than a gap, because what is being promised is the **absence** of a layer: no marshalling step, no generated shim, no runtime to start first. You cannot measure something that is not there; you can only go and look at what the call compiles to.

So it gets a page instead of a number: [Calling C](../../09_Advanced/calling_c/README.md), which does the round trip and prices the part the slogan leaves out. The short version, and the correction to carry: **the call is free, the data is not.** A `&str` is a pointer and a length; a C string is a pointer and a terminator; so every string that crosses is allocated and copied, and `CString::new` can fail outright on text that was perfectly legal a line earlier.

## Why this page has no answer key

Every lesson in this library ends with output that CI recompiles, re-runs and diffs against a recorded file. This page cannot work that way, and it is the third place in the repo where that is true — [the books shelf](../../10_Resources/books/README.md) cannot check a verdict, and [C and C++](../../31_C_and_Cpp/README.md) cannot record an answer key for undefined behaviour. Here the subject is other people's measurements, and re-running them is not something a `.rs` file can do.

So, in place of a key:

- **The two toolchain counts are labelled with the compiler that printed them** (`rustc 1.98.0`) and the exact command, so you can disagree with them in one line rather than by taking my word.
- **Every citation was read at the source rather than in summary**, on 2026-08-30, and every link answered. That is not a formality — reading the sources is how the memory-versus-energy split and the shared corpus turned up at all. Neither is visible from the bullets.
- **Nothing else is claimed.** Where a number is from 2017, this page says 2017.

## If you are coming from another language

Every language community keeps a list like this one, and the failure mode is identical everywhere: the number is real and its subject has quietly been swapped for the language.

- **Python** — "Python is slow" rests on the same Benchmarks Game corpus, and the corpus's pure-Python entries are as unrepresentative of production Python as its hand-vectorised entries are of ordinary Rust. Both directions are the same error.
- **Java** — "the JVM is as fast as C++ after warm-up" is a claim about steady-state throughput on long-running programs, which is a genuine result and a different question from start-up, from tail latency, and from memory. Notice the bundling; it is the same move as "memory and energy efficiency" above.
- **C and C++** — "as fast as C" is the one claim on this list you can check by reading rather than benchmarking, because [below the IR the two are the same program](../../20_Compilers/llvm_and_its_ir/README.md). That makes it the rare marketing sentence with a mechanism behind it.
- **ABAP** — there is no benchmark culture to be misled by, and the equivalent trap is a vendor note quoting a customer's throughput figure with the hardware and dataset left out. Same question, either way: *what was on the scale?*

## See also

- [Benefits of Rust](../benefits_of_rust/README.md) — the other list, sorted by what kind of claim each bullet is: a compile error, a defined run-time behaviour, or a convenience
- [C and C++](../../31_C_and_Cpp/README.md) — the nine bugs behind the 70%, each one compiled and run
- [Targets and triples](../../20_Compilers/targets_and_triples/README.md) — what the 330 actually promises, tier by tier
- [Tooling](../../05_Tooling/README.md) — the third bullet in full: `cargo`, `clippy`, `rustup` and the rest
- [Start here](../README.md) — what to read first, once you are convinced

## Po polsku

Ta strona zadaje przy każdej liczbie jedno pytanie: **co właściwie położono na wadze?** Pytanie przenosi się między językami bez strat, ale liczby przenoszą się gorzej, bo w polskich artykułach, prelekcjach i wpisach na LinkedInie krążą już bez podmiotu — „Rust jest tak szybki jak C”, „Rust eliminuje 70% błędów bezpieczeństwa”, „najbardziej lubiany język od ośmiu lat”. Każde z tych zdań ma źródło i każde źródło mówi coś węższego. Warto też wiedzieć, czego szukać: wszystkie sześć odnośników prowadzi do materiałów po angielsku, więc polskie omówienie jest z definicji drugą ręką, a różnica między liczbą a jej podmiotem gubi się właśnie przy przepisywaniu.

Dwie z sześciu liczb sprawdza się samodzielnie i to jest jedyna część tej listy, przy której język w ogóle nie ma znaczenia — polecenie jest poleceniem. `rustc --print target-list | wc -l` daje 330 (z czego 65 nazw nie wymienia żadnego systemu operacyjnego), a zliczenie lintów `clippy` daje 822, w tym 333 domyślnie wyłączone. Obie „zaokrąglone w dół” formuły z oryginału — *dozens of platforms*, *700+ lints* — są więc zaniżone, a druga dodatkowo sugeruje, że wszystkie te linty działają od razu; **dwa na pięć jest wyłączonych**, bo to opinie, a nie usterki. Liczby na tej stronie pochodzą z `rustc 1.98.0` i przy innej wersji będą inne — o to właśnie chodzi, żeby je drukować zamiast cytować.

Najbardziej myląca po polsku jest ta o sondażu. Stack Overflow zmienił w 2023 roku nazwę kategorii z *loved* na *admired*, a polskie relacje niemal zawsze zostają przy „najbardziej lubiany”, więc po polsku ta pozycja brzmi cieplej niż w oryginale — i tak już nie mówi tego, co ludzie z niej wyczytują. **Admired** ma ścisłą definicję: odsetek osób, które używały danej technologii w minionym roku i chcą używać dalej. To wskaźnik utrzymania wśród tych, którzy już wybrali Rusta — nie mówi nic o tych, którzy spróbowali i odeszli, nic o znacznie liczniejszych, którzy nie próbowali, i nic o jakości języka.

Ostatnia rzecz do poprawienia w polskim marketingu: „Rust jest oszczędny pamięciowo”. To akurat z danych nie wynika — na `n-body` Rust bierze około jednej piątej pamięci więcej niż C, a w badaniu energetycznym spadłby o dziewięć miejsc, gdyby posortować tabelę po szczycie zużycia pamięci. Uczciwe zdanie jest inne i mocniejsze: pamięć w Ruscie jest **przewidywalna**, a nie mała — nie ma odśmiecacza ani środowiska uruchomieniowego, więc da się wskazać palcem linijkę, która alokuje. I jeszcze jeden szczegół z tej samej tabeli, bo dobrze pokazuje ducha całej strony: gwiazdka przy wyniku oznacza możliwe ręczne wektoryzowanie albo `unsafe`, więc program dowodzący „szybki jak C” bywa tym, który zrezygnował z gwarancji opisanej w następnym punkcie listy.

**Szukaj po polsku:** benchmarki Rust a C · wydajność energetyczna języków programowania · najbardziej lubiany język ankieta · `rustc --print target-list` · `benchmarks game caveats` · `stack overflow survey admired vs desired`
