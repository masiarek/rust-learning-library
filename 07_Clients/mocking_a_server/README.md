# Mocking a server

**Level:** 301 · deep dive

**One line:** A fake HTTP server makes a client's tests fast, offline and deterministic — and every one of them now proves something about *your fake* rather than about the API.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- Why not call the real service: it is slow, it rate-limits, it needs a key nobody should commit, and it fails for reasons your code did not cause
- [`httpmock` ↗](https://docs.rs/httpmock) or an equivalent: start a server on a free port, declare the route and the response, point the client at it — which only works because the URL [is a parameter](../injecting_the_base_url/README.md)
- Asserting the mock was actually **hit**, so a client that silently sent nothing fails instead of passing
- The honest limit: a mock is your belief about the API, frozen. It keeps passing after the real service changes
- What earns trust back — a stored real response as a fixture, and one integration test, run rarely and on purpose, against the live thing

## The trap it exists for

A green suite against a mock is evidence about the mock. That is worth having and it is not the same claim as "the client works", and a page that does not say so out loud teaches a false sense of coverage — the same shape as [a test that cannot fail](../../03_Command_Line/testing_a_command/README.md).

## See also

- [Injecting the base URL](../injecting_the_base_url/README.md) — the precondition for any of this
- [Testing a command](../../03_Command_Line/testing_a_command/README.md) — the other end of the same program, tested the same way
- [Deserializing a response](../deserializing_a_response/README.md) — what the fixture is actually exercising

## Po polsku

Po polsku nazywa się to różnie — atrapa, makieta, zaślepka, namiastka — ale w praktyce, i przede wszystkim w wyszukiwarce, liczy się jedno słowo: `mock`, odmieniane po polsku (*zamockować serwer*, *strzał w mocka*). Warto o tym wiedzieć, zanim się zacznie szukać, bo „atrapa serwera HTTP” nie znajdzie prawie niczego, a `rust httpmock` znajdzie wszystko. Sedno tej lekcji jest jednak niewygodne i lepiej nazwać je wprost: **zielony pasek testów na atrapie jest dowodem na atrapę, a nie na API** — mock zamraża twoje przekonanie o tym, co serwis odpowiada, a zamrożone przekonanie przechodzi testy również wtedy, gdy prawdziwy serwis dawno się zmienił. Część tej straty odrabiają dwie rzeczy: asercja, że atrapa **została trafiona** (klient, który po cichu nie wysłał nic, przechodzi test równie ładnie jak działający), oraz zapisana prawdziwa odpowiedź jako *fixture* i jeden test integracyjny, puszczany rzadko i świadomie przeciwko żywemu serwisowi.

**Szukaj po polsku:** mockowanie w testach · testy integracyjne a jednostkowe · `rust httpmock` · `rust wiremock crate` · `rust mock http server test`
