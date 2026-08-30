# Deserializing a response

**Level:** 201 · working knowledge

**One line:** The JSON you fetch was designed by somebody else, so the choice is between mirroring their whole shape in structs and reaching into it for the two fields you actually wanted.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The struct-per-level approach: faithful, self-documenting, and a lot of types you will never construct
- The pointer approach — `serde_json::Value` plus a [JSON Pointer ↗](https://docs.rs/serde_json/latest/serde_json/value/enum.Value.html#method.pointer) like `/current/temperature` — small, and it moves the failure from compile time to run time
- Which to reach for: how much of the response you need, how stable it is, and who finds out when it changes
- `#[serde(rename = "…")]` for a field name that is not a legal Rust identifier or not the name you want
- Missing versus null versus absent — three different things that all read as "no value", and how `Option` plus `#[serde(default)]` tell them apart

## The trap it exists for

Both approaches fail on a changed API; they differ in *when*. Structs fail loudly at the boundary with the field named; a JSON pointer returns `None` and lets the default flow onward — which is the same silent-wrong-answer shape as [collecting an iterator with `filter_map(Result::ok)`](../../02_Errors/keep_going_or_stop/README.md), one layer out.

## See also

- [Deriving `Serialize` and `Deserialize`](../../06_Data/serde_derive/README.md) — the derive, and the attributes that survive somebody else's naming
- [Mocking a server](../mocking_a_server/README.md) — where a stored real response earns its keep
- [`Option` vs `Result`](../../17_Option_and_Result/option_vs_result/README.md) — absent, or failed? The field that is missing has to pick one

## Po polsku

Deserializacja (*deserialization*) cudzego JSON-a to wybór strategii, a nie kwestia gustu: albo odwzorowujesz cały kształt odpowiedzi strukturami — wiernie, samodokumentująco i z tuzinem typów, których nigdy ręcznie nie stworzysz — albo bierzesz `serde_json::Value` i wskaźnikiem JSON Pointer (`/current/temperature`) sięgasz po te dwa pola, o które naprawdę ci chodziło. Rzecz w tym, że zmiany cudzego API nie przetrwa żadna z tych dróg; różnią się **kiedy** się o tym dowiesz — struktura wywala się głośno, zaraz na granicy programu i z nazwą pola w komunikacie, a pointer po cichu zwraca `None`, po czym dalej płynie wartość domyślna i cichy zły wynik. Osobno warto zapamiętać rozróżnienie, które po polsku zwykle zlewa się w jedno „brak wartości”, zwłaszcza gdy przychodzi się od SQL-a albo ABAP-a: **klucza nie ma w odpowiedzi w ogóle**, **klucz jest, a w nim `null`** i **klucz jest z wartością pustą** to trzy różne zdarzenia, a `Option` razem z `#[serde(default)]` służą właśnie do ich odróżnienia. Do tego `#[serde(rename = "…")]` na wypadek, gdy cudza nazwa pola nie jest legalnym identyfikatorem Rusta — w cudzym API to raczej reguła niż wyjątek.

**Szukaj po polsku:** deserializacja JSON w Ruscie · parsowanie odpowiedzi API · `rust serde_json Value pointer` · `rust serde default vs Option` · `rust serde rename field`
