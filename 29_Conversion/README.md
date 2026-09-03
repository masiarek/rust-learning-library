# Conversion

**One line:** Four ways to turn one type into another, where the only decision that matters is whether the conversion is allowed to fail — plus a fifth you never write, which the compiler performs for you at a fixed list of places.

```text
cannot fail      From / Into        u64::from(n)        n.into()
can fail         TryFrom / TryInto  u8::try_from(n)?    n.try_into()?
from text        FromStr            "4".parse::<T>()?
no check at all  as                 n as u8
never written    coercion           width(&owned)       &String -> &str
```

The first three are traits, so they are extensible, greppable, and visible in a signature. The fourth is built into the language, never fails, and loses data in four different ways without saying so. The fifth is not something you do at all — it is the compiler adjusting a reference's type where it already knows what it wants, and the list of adjustments it will make is short and closed.

| Lesson | Level | What it covers |
|---|---|---|
| [`From` and `Into`](from_and_into/README.md) | 201 | Write one, get both — and why `?` converts your errors for free |
| [`TryFrom` and `TryInto`](tryfrom_and_tryinto/README.md) | 201 | The same shape plus a `type Error`, and `FromStr` for the text case |
| [Casting with `as`](casting_with_as/README.md) | 201 | Truncation, sign reinterpretation, saturation and rounding, all silent |
| [Coercion: the conversion you never write](coercion/README.md) | 201 | The fifth one, and the only implicit one — where it fires, the closed list of what it will do, and the three places it will not help you |

## The one table

| You are doing | Write |
|---|---|
| widening an integer, or any conversion that cannot fail | `u64::from(n)` |
| narrowing, or anything with an invalid input | `u8::try_from(n)?` |
| turning text into a value | `s.parse::<T>()?` |
| truncating on purpose | `n as u8`, with a comment |

## Where the rest of it is

Text conversion in both directions has its own section: [parsing out of a string](../14_Strings/parsing_a_string/README.md) going in, and [`Display`](../15_First_Programs/debug_vs_display/README.md) coming back out — implement `Display` and `ToString` arrives free, exactly the way `Into` arrives free from `From`. [`ToOwned`](../12_Traits/to_owned/README.md) is the fourth conversion trait, for the borrowed-to-owned case where the owned twin is a different type (`&str` → `String`).

## Where it goes next

Every one of these is a trait with a blanket impl behind it, which is [what a trait is](../12_Traits/README.md) at its most useful. And the reason `try_from` returns something you have to open is [`Option` and `Result`](../17_Option_and_Result/README.md).

## Po polsku

Konwersja typów (*conversion*) rozkłada się w Ruscie na cztery drogi, a wybiera między nimi jedno pytanie: czy konwersja **ma prawo się nie udać**. Nie może — `From`/`Into`, czyli `u64::from(n)`; może — `TryFrom`/`TryInto`, czyli `u8::try_from(n)?`; z tekstu — `FromStr` wywoływany przez `"4".parse::<T>()?`; a `n as u8` to rzutowanie bez żadnego sprawdzenia. Kto przychodzi z C, Javy czy ABAP-a, sięgnie po `as` odruchowo, bo tam rzutowanie *jest* zwykłą drogą konwersji — w Ruscie to wyjątek, który po cichu gubi dane na cztery różne sposoby, podczas gdy pozostałe trzy drogi to cechy (*traits*): rozszerzalne, widoczne w sygnaturze funkcji i możliwe do znalezienia grepem.

Piąta droga to **koercja**, której się nie pisze: kompilator sam dopasowuje typ referencji w kilku ściśle wyliczonych miejscach — dlatego `width(&owned)` działa, mimo że `&owned` to `&String`, a funkcja chce `&str`. Lista tych dopasowań jest zamknięta i nie ma na niej żadnej konwersji liczbowej.

**Szukaj po polsku:** konwersja typów w Ruscie · rzutowanie typów · koercja · `rust From Into trait` · `rust TryFrom` · `rust as cast truncation` · `rust deref coercion`
