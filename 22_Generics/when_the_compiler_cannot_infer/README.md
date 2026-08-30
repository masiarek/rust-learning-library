# When the compiler cannot infer

**Level:** 201 · working knowledge

**One line:** `error[E0282]: type annotations needed` means nothing anywhere in the function pins `T` down — and the fix is to say the type once, in whichever of three places reads best.

```rust
// 1. Annotate the binding.
let annotated: Container<Option<String>> = Container { value: None };

// 2. Turbofish: the same information, written at the call.
let turbofished = Container::<Option<String>>::new(None);

// 3. Name the type on the value itself.
let on_the_value = Container::new(None::<String>);
```

Three spellings of one type. Putting all three in a single `Vec` compiles, which is the proof — a `Vec` holds one type.

## What the error looks like

`None` is a `Option<T>` for every `T` at once, so a container built from one is a `Container<Option<_>>` with a hole in it:

```rust
let ambiguous = Container { value: None };
```

```text
error[E0282]: type annotations needed for `Container<Option<_>>`
 --> e0282.rs:6:9
  |
6 |     let ambiguous = Container { value: None };
  |         ^^^^^^^^^                      ---- type must be known at this point
  |
help: consider giving `ambiguous` an explicit type, where the type for type parameter `T` is specified
  |
6 |     let ambiguous: Container<Option<T>> = Container { value: None };
  |                  ++++++++++++++++++++++
```

The `_` in `Container<Option<_>>` is the compiler showing its work: it got as far as *a container of an option of something* and ran out of evidence. Take the help literally and it will not compile — `T` in the suggested line is a name nothing declares. What it means is *put a real type where I printed `T`*.

## Which of the three to use

| Situation | Reach for |
|---|---|
| The value is being bound to a name | the annotation — `let x: Container<Option<String>> = …` |
| The value is an argument, or part of a longer expression | the [turbofish](../../15_First_Programs/what_an_annotation_does/README.md) — `Container::<Option<String>>::new(None)` |
| Only one sub-expression is ambiguous | name it there — `None::<String>`, `Vec::<u8>::new()` |

They are the same information in three positions, and no rule decides between them beyond what reads well at that line. The annotation is the usual answer; the turbofish exists for when there is no binding to hang it on — `takes_a_container(Container::<Option<String>>::new(None))` has nowhere else to put it.

## Inference is not left-to-right

The compiler reads the whole function body before deciding, so a line *below* can settle a type declared above:

```rust
let mut names = Vec::new();          // T unknown here...
names.push(String::from("Ada"));     // ...and decided here
```

That is why most Rust has no annotations in it at all, and why the ones that remain tend to cluster around `parse`, `collect`, `into` and `Default::default()` — calls whose *return* type is the generic one, with nothing in the arguments to read it from. A use is enough; it does not have to be a value:

```rust
let tally = Container::new(Vec::new());
let total: u32 = tally.value.iter().sum();   // this line is what makes it Vec<u32>
```

The same shape turns up constantly without a struct in sight:

```rust
let n: i32 = "42".parse().unwrap();      // annotation
let m = "42".parse::<i32>().unwrap();    // turbofish
let letters: String = ['R', 'u', 's', 't'].iter().collect();
```

## The variant with nothing to annotate

The purest version of this error is a [phantom type](../../12_Traits/phantom_types/README.md), where the parameter is deliberately absent from the data — nothing in the value can ever narrow it, so the annotation is not a fallback but the only way the type is ever chosen.

One `E0282` is not asking for a type at the call site at all. An associated function inside `impl<T>` that never mentions `T` still requires one to be chosen, and there is nothing in the call to choose from — `Tally::quorum()` is the worked example in [when a struct refuses](../../16_Structs/when_a_struct_refuses/README.md). Naming the type works (`Tally::<i32>::quorum()`), but the better fix is usually to move the function out of the generic block, since a function with no `T` in it never belonged there.

## If you are coming from another language

**Python.** There is no equivalent, because there is nothing to decide: `x = None` is a name bound to `None`, and what it may hold later is not a question the interpreter asks. The nearest experience is mypy reporting `Need type annotation for "x"` on a bare `x = []`, which is the same complaint from the same cause — an empty container tells the checker nothing — with the difference that mypy's version is advisory and this one stops the build. If you write `x: list[int] = []` out of habit, you have already internalised the fix.

**ABAP.** Classic ABAP never infers: `DATA lt_tab TYPE STANDARD TABLE OF ty_row` states the type, always. The inline form added in 7.40 does infer — `DATA(lv_name) = ls_row-name`, `SELECT … INTO TABLE @DATA(lt_result)` — and its rule is much stricter than Rust's: the type must be derivable from *that statement alone*, which is why `DATA(lv_x) = VALUE #( )` is a syntax error and `SELECT * INTO TABLE @DATA(lt_x)` on a dynamic target is not allowed. Rust reads the whole function, so the equivalent of a bare `DATA(lt_x)` is legal as long as some later line pushes into it. What transfers: the moment a value's type stops being written down somewhere, both languages make you put it back — ABAP at the declaration, Rust anywhere in the body.

**C++.** `auto x = …;` has the same failure and the same fix, and CTAD (`std::vector v{1, 2};`) is the direct counterpart of inferring `Container`'s parameter from its argument. `std::optional<std::string> x = std::nullopt;` is exactly the case on this page — `nullopt` is typeless, so the declaration has to carry the type. Rust's turbofish is `Container<Option<std::string>>{}` written after the fact.

**Java.** The diamond (`new ArrayList<>()`) infers from the left-hand side only, so Java's version of this error is *"cannot infer type arguments"* and the fix is always to fill the diamond in. `var` (Java 10) moved a step toward Rust; it still reads one statement, never the whole method.

## The verified output

[`examples/when_the_compiler_cannot_infer.rs`](examples/when_the_compiler_cannot_infer.rs) compiled and run:

<!-- output:when_the_compiler_cannot_infer -->
*Verified output of [`when_the_compiler_cannot_infer.rs`](examples/when_the_compiler_cannot_infer.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
three spellings, one type, 3 values
  None
  None
  None

names ["Ada"]
a Vec<u32> decided one line below where it was built, summing to 0

42 42 Rust
```
<!-- /output -->

## See also

- [What a generic is](../what_a_generic_is/README.md) — the `<T>` this page is failing to fill in
- [What a type annotation does](../../15_First_Programs/what_an_annotation_does/README.md) — the `:` and the turbofish as two positions for one fact
- [When a struct refuses](../../16_Structs/when_a_struct_refuses/README.md) — eight struct errors, `E0282` among them
- [Phantom types](../../12_Traits/phantom_types/README.md) — a parameter no value can ever imply, so the type must always be named
- [`Some` and `None`](../../17_Option_and_Result/some_and_none/README.md) — where a bare `None` gets its `T` from in ordinary code
- [Making a `String`](../../14_Strings/making_a_string/README.md) — `.into()` is the other call that never names its destination
