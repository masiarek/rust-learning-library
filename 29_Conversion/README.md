# Conversion

**One line:** Four ways to turn one type into another, and the only decision that matters is whether the conversion is allowed to fail.

```text
cannot fail      From / Into        u64::from(n)        n.into()
can fail         TryFrom / TryInto  u8::try_from(n)?    n.try_into()?
from text        FromStr            "4".parse::<T>()?
no check at all  as                 n as u8
```

The first three are traits, so they are extensible, greppable, and visible in a signature. The fourth is built into the language, never fails, and loses data in four different ways without saying so.

| Lesson | Level | What it covers |
|---|---|---|
| [`From` and `Into`](from_and_into/README.md) | 201 | Write one, get both — and why `?` converts your errors for free |
| [`TryFrom` and `TryInto`](tryfrom_and_tryinto/README.md) | 201 | The same shape plus a `type Error`, and `FromStr` for the text case |
| [Casting with `as`](casting_with_as/README.md) | 201 | Truncation, sign reinterpretation, saturation and rounding, all silent |

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
