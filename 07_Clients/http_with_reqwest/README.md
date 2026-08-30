# An HTTP request

**Level:** 201 · working knowledge

**One line:** [`reqwest` ↗](https://docs.rs/reqwest) makes a `GET` a two-line affair — and the first real decision is made before either line: blocking, or `async`?

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The smallest complete request, and the three failures folded into it: the connection, the status code, and the body
- **A non-2xx response is not an `Err`.** The request succeeded; the server said no. `error_for_status` is the opt-in, and forgetting it is how a 404's HTML ends up in your parser
- Blocking against `async`: what a runtime is, why `async` needs one, and why a command-line tool making one request usually does not
- TLS and what "needs a feature" means when the failure is a linker error rather than a compile error
- Timeouts, which are not the default, and the request that hangs until somebody notices

## The trap it exists for

`async` arrives as a fashion rather than a decision. A tool that makes a single request pays a runtime, a coloured-function split, and a much harder testing story to save nothing at all — while a program making a thousand concurrent requests genuinely needs it. The page's job is to make that a question with an answer, not a default.

## See also

- [Injecting the base URL](../injecting_the_base_url/README.md) — the very next thing to do to this code, before it is written twice
- [Deserializing a response](../deserializing_a_response/README.md) — what to do with the body
- [`anyhow` and context](../../02_Errors/anyhow_and_context/README.md) — three failure modes that all deserve to say which URL they were fetching

## Po polsku

Po polsku mówi się „błąd 404” — i właśnie dlatego tak łatwo tu wpaść w pułapkę, przed którą ta strona ostrzega: dla `reqwest` odpowiedź `404` **nie jest** żadnym `Err`. Żądanie się powiodło, tylko serwer odpowiedział „nie”, więc dostajesz `Ok`, a dopiero jawne `error_for_status()` zamienia status spoza `2xx` w błąd — bez tego HTML strony „nie znaleziono” wjeżdża prosto do parsera JSON-a i wywala się dużo dalej, z komunikatem nie na temat. Druga decyzja zapada jeszcze przed pierwszą linijką kodu: blocking czy `async` — i tu warto uważać na słowo *runtime*, bo polskie „środowisko uruchomieniowe” podsuwa skojarzenie z JVM albo .NET-em, a chodzi o zwykły crate (`tokio`), który sam dokładasz do `Cargo.toml`. Jedno żądanie z narzędzia w wierszu poleceń nie potrzebuje go wcale; tysiąc równoległych — jak najbardziej.

**Szukaj po polsku:** żądanie HTTP w Ruscie · programowanie asynchroniczne w Ruscie · `rust reqwest error_for_status` · `rust reqwest blocking vs async` · `rust reqwest timeout`
