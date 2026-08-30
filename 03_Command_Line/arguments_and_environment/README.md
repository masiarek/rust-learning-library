# Arguments and the environment

**Level:** 201 · working knowledge

**One line:** Command-line arguments are what a user types this once; environment variables are what a machine was configured with — and the test for which is roughly *"would I mind this appearing in `ps` output and in my shell history?"*

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- [`env::var` ↗](https://doc.rust-lang.org/std/env/fn.var.html) returns a `Result`, and its two failure modes are genuinely different: not set, and set to something that is not Unicode
- The precedence ladder every configurable program grows — flag beats environment beats config file beats built-in default — and writing it down before it grows by accident
- `clap`'s `env` attribute, which puts the ladder in the struct
- Secrets: an API key on the command line is visible in the process list to every user on the machine
- Why [`std::env::set_var` ↗](https://doc.rust-lang.org/std/env/fn.set_var.html) is `unsafe` in the 2024 edition, and what that means for a test that sets one — the environment is process-global, and Rust's tests run in threads

## The trap it exists for

The environment is the one input that is not visible in the code, the arguments, or the file. A program that behaves differently on two machines with the same command and the same input is nearly always reading something nobody wrote down — which is an argument for making every environment variable appear in `--help`.

## See also

- [Command-line arguments](../command_line_arguments/README.md) — the other half of the program's input
- [Deriving a parser with `clap`](../clap_derive/README.md) — where the precedence ladder gets declared instead of coded
- [Testing a command](../testing_a_command/README.md) — why a test that sets an environment variable is not automatically safe to run in parallel

## Po polsku

Ta lekcja stawia pytanie, które w polskich materiałach rzadko pada wprost: co powinno trafić do argumentu, a co do zmiennej środowiskowej (*environment variable*). Test jest praktyczny — argument widać w liście procesów (`ps`) i w historii powłoki, więc ścieżka do pliku owszem, ale klucz API już nie; z tego rozdziału sam z siebie wyrasta porządek pierwszeństwa, który lepiej zapisać, niż wyhodować przypadkiem: flaga bije zmienną środowiskową, ta bije plik konfiguracyjny, a ten wartość domyślną. Dla polskiego czytelnika najbardziej znajomym „wejściem, którego nie widać” są `LANG` i `LC_ALL` — to samo polecenie na dwóch maszynach potrafi inaczej posortować listę z ogonkami; w Ruście akurat `std` celowo nie czyta ustawień regionalnych, więc taka niespodzianka przychodzi zwykle z zewnętrznego narzędzia w potoku, a nie z `format!`. Dwie rzeczy warto zapamiętać dosłownie: `env::var` zwraca `Result`, a jego dwa błędy są naprawdę różne (zmiennej nie ustawiono albo ustawiono ją na coś, co nie jest poprawnym Unicode), oraz `std::env::set_var` jest w edycji 2024 oznaczone jako `unsafe` — środowisko jest globalne dla całego procesu, a testy Rusta biegną w wątkach.

**Szukaj po polsku:** zmienne środowiskowe · ustawienia regionalne · kolejność pierwszeństwa konfiguracji · `rust std::env::var` · `rust set_var unsafe edition 2024`
