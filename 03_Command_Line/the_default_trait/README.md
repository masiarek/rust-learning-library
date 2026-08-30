# The `Default` trait

**Level:** 101 → 201 · for newcomers

**One line:** [`Default` ↗](https://doc.rust-lang.org/std/default/trait.Default.html) is the value a type takes when nobody said — and `..Default::default()` is how a struct of options grows a tenth field without you editing every place that built one.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- `#[derive(Default)]` and what it produces field by field: every field's own default, which for numbers is zero and for `Option` is `None`
- Struct update syntax — `Config { words: true, ..Default::default() }` — and the fact that it is not inheritance, just the remaining fields filled in
- `#[default]` on an enum variant, so a derived default can pick which variant is the quiet one
- Writing the impl by hand when the type's zero is not your domain's zero
- Where it shows up without being asked: `unwrap_or_default`, `Vec::new`, and every `T: Default` bound in the standard library

## The trap it exists for

A derived `Default` is the **type's** zero, not your domain's — a `Timeout(0)` or a `Retries(0)` is a perfectly good `u32` and a terrible policy. [`unwrap_or_default`](../../17_Option_and_Result/unwrap_or_default/README.md) already tells that story from the calling side; this page is the same fact from the *defining* side, where you can still do something about it.

## See also

- [`unwrap_or_default`](../../17_Option_and_Result/unwrap_or_default/README.md) — the fallback the type chose, and the missing impl that is a guard rail
- [Optional function arguments](../../17_Option_and_Result/optional_arguments/README.md) — the options-struct pattern this makes bearable
- [Flags by hand](../flags_by_hand/README.md) — the struct that wanted a default in the first place

## Po polsku

`Default` to cecha (*trait*) mówiąca, jaką wartość przyjmuje typ, gdy nikt nic nie powiedział — po polsku po prostu „wartość domyślna”, ale uwaga na skrót myślowy: w Ruście **nie ma domyślnych argumentów funkcji**, jak `def f(x=3)` w Pythonie czy `DEFAULT` w ABAP-ie, i to jest właśnie powód, dla którego ich rolę przejmuje struktura opcji z `Default`. `#[derive(Default)]` wypełnia ją pole po polu wartością domyślną każdego typu — dla liczb zerem, dla `Option` wartością `None` — a `#[default]` nad wariantem wyliczenia (*enum*) pozwala wskazać, który wariant jest tym cichym. Zapis `Config { words: true, ..Default::default() }` **nie** jest dziedziczeniem ani rozwinięciem obiektu znanym z JavaScriptu; znaczy dokładnie tyle, co „resztę pól dobierz z domyślnych”, dzięki czemu dziesiąte pole dorzuca się bez ruszania wszystkich miejsc, w których tę strukturę się buduje. Pułapkę warto zapamiętać w tej formie: wyprowadzone `Default` daje zero **typu**, a nie zero twojej dziedziny — `Timeout(0)` jest zupełnie poprawnym `u32` i fatalną polityką, więc gdy sensowna wartość domyślna istnieje, napisz `impl Default` ręcznie, a gdy nie istnieje, nie wyprowadzaj `Default` wcale.

**Szukaj po polsku:** wartość domyślna · domyślne argumenty funkcji · struktura opcji · `rust Default trait derive` · `rust struct update syntax`
