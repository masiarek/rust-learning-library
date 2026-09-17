# Hardware you cannot misuse

**Level:** 301 · deep dive

**One line:** A pin configured as an input and the same pin configured as an output are two different *types*, so `set_high` on an input is a compile error instead of a board that does nothing — typestate, zero-sized tags, and a peripheral singleton that can be taken exactly once.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- Typestate pins: `Pin<Input>` and `Pin<Output>` over one struct with a `PhantomData<Mode>` field, and `into_output(self) -> Pin<Output>` consuming the old value. Calling `set_high` on the input is `` error[E0599]: no method named `set_high` found for struct `Pin<Input>` in the current scope `` — checked on 1.98.0 with a std-only model, which is also how the finished page can compile it.
- Zero cost, measured rather than claimed: is `size_of::<Pin<Output>>()` the size of the pin number alone? [Phantom types](../../12_Traits/phantom_types/README.md) measured the same property for its own tag.
- Singletons: [`cortex_m::Peripherals::take()` ↗](https://docs.rs/cortex-m/latest/cortex_m/peripheral/struct.Peripherals.html) returns `Option<Self>` — "all the core peripherals once" — and `unsafe fn steal()` is the unchecked escape. What does `take` check at run time to know it has already been called?
- The limit [The right to post is a value](../../09_Advanced/one_account_one_review/README.md) ran into: move semantics govern one value and say nothing about how many values a constructor hands out. Here `take` is that constructor, and its run-time flag is what the types rest on.
- [`embedded-hal` ↗](https://docs.rs/embedded-hal/1.0.0/embedded_hal/) 1.0.0: `digital::OutputPin` has `set_low`, `set_high` and a provided `set_state`, each returning `Result<(), Self::Error>`. Why does setting a pin return a `Result` at all — which pins can fail?
- A driver written once against the traits, `fn blink<P: OutputPin>(pin: &mut P)`, runs on any chip whose HAL implements them. What does that cost in monomorphised code on a small flash chip — [static vs dynamic dispatch](../../12_Traits/static_vs_dynamic_dispatch/README.md) with a size budget attached?
- What a type cannot enforce: timing (a pin toggled faster than the part allows), electrical facts (two outputs driving one wire), and any `steal()` anywhere else in the program.

## The trap it exists for

Configuration as run-time state: a `GpioConfig` with `set_direction(bool)` and a check inside every method. It works, it is what the Embedded Rust Book's design-contracts chapter starts from, and every misuse becomes an `Err` at run time on the board — or, where one method forgot its check, a register write made in a state the hardware was never meant to see.

## Where this sits

[Phantom types](../../12_Traits/phantom_types/README.md) teaches `PhantomData` and typestate in general, and [An enum as a state machine](../../13_Enums/an_enum_as_a_state_machine/README.md) the run-time alternative; this page covers only their use to make a peripheral's wrong operation unrepresentable. [Memory-mapped registers](../memory_mapped_registers/README.md) is the raw layer these types wrap.

## See also

- [Phantom types](../../12_Traits/phantom_types/README.md) — the zero-sized tag and its typestate section
- [The right to post is a value](../../09_Advanced/one_account_one_review/README.md) — the same "permission as a value" design, and its hole
- [An enum as a state machine](../../13_Enums/an_enum_as_a_state_machine/README.md) — the state checked at run time instead
- [Marker traits](../../12_Traits/marker_traits/README.md) — traits with no methods, the other zero-cost tag
- [Static vs dynamic dispatch](../../12_Traits/static_vs_dynamic_dispatch/README.md) — what a generic driver compiles into
- [Memory-mapped registers](../memory_mapped_registers/README.md) — the pointers under the types
- [The Embedded Rust Book: Typestate Programming ↗](https://docs.rust-embedded.org/book/static-guarantees/typestate-programming.html) and [Singletons ↗](https://docs.rust-embedded.org/book/peripherals/singletons.html)

## If you are coming from another language

- **C.** A pin's mode lives in a configuration register and in the programmer's head; the function that drives a pin high has the same signature whatever that register says. The same register writes happen in Rust — what changes is that the mode is part of the argument's type, so the wrong call is missing from the API rather than checked inside it.
- **C++.** Templates can encode the same typestate, and a move can hand the pin to its new type — but a moved-from object can still be named and used, and the compiler accepts it. Rust's `into_output(self)` consumes the input pin, so using the stale handle is `E0382`, a compile error.

## Po polsku

Pin skonfigurowany jako wejście i ten sam pin jako wyjście mają w Ruscie **różne typy** (*typestate*), więc wywołanie `set_high` na wejściu kończy się błędem kompilacji `E0599`, a nie płytką, która po cichu nic nie robi. Znacznik trybu ma zerowy rozmiar (`PhantomData`), a dostęp do urządzeń daje singleton: `Peripherals::take()` zwraca `Some` tylko za pierwszym razem. Typy nie wymuszą jednak zależności czasowych ani elektrycznych — i nie obronią się przed `unsafe fn steal()`.

**Szukaj po polsku:** typy stanu w Ruscie · wzorzec singleton na mikrokontrolerze · `rust embedded typestate pins` · `embedded-hal OutputPin` · `cortex_m Peripherals take`
