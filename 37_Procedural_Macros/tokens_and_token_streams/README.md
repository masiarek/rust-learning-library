# Tokens and token streams

**Level:** 201 · working knowledge

**One line:** A `TokenStream` is a sequence of token trees of four kinds: `Ident`, `Punct`, `Literal`, and `Group`, which is a delimited stream of its own. Each tree carries its text and a span saying where it was written, and nothing about types or meaning. A macro reads and writes only these.

[Three kinds of procedural macro](../three_kinds_of_procedural_macro/README.md) printed what each kind of macro receives, as `to_string()` text. This page goes one level down and takes the same kind of input apart one token tree at a time.

## A macro that lists its input

Four macros, all in the compiler's own `proc_macro` crate and nothing else. `token_trees!` walks its input and prints each tree's kind, its spacing or delimiter, and the line and column its span starts at; the other three return `to_string()` from each kind of macro:

<!-- file:demo/inspect_tokens/src/lib.rs -->
```rust title="demo/inspect_tokens/src/lib.rs"
//! Macros that report what they were handed, one level below `to_string()`:
//! every token tree, its kind, its spacing or delimiter, and where in the
//! source it came from. The text each one reports goes back as a string
//! literal built as a token, so no quote or backslash in it needs escaping.

use std::fmt::Write;

use proc_macro::{Literal, TokenStream, TokenTree};

/// One line per token tree, with a group's contents indented under it.
#[proc_macro]
pub fn token_trees(input: TokenStream) -> TokenStream {
    let mut lines = String::new();
    walk(input, 0, &mut lines);
    string_literal(lines.trim_end())
}

/// The input's `to_string()`.
#[proc_macro]
pub fn text(input: TokenStream) -> TokenStream {
    string_literal(&input.to_string())
}

/// A derive's input's `to_string()`, as a constant beside the item.
#[proc_macro_derive(Text)]
pub fn derive_text(item: TokenStream) -> TokenStream {
    let mut out: TokenStream = "pub const DERIVE_INPUT: &str =".parse().unwrap();
    out.extend([string_literal(&item.to_string()), ";".parse().unwrap()]);
    out
}

/// An attribute's item's `to_string()`, as a constant beside the item.
#[proc_macro_attribute]
pub fn text_attr(_args: TokenStream, item: TokenStream) -> TokenStream {
    let text = item.to_string();
    let mut out = item;
    out.extend(["pub const ATTRIBUTE_INPUT: &str =".parse().unwrap(), string_literal(&text), ";".parse().unwrap()]);
    out
}

fn walk(stream: TokenStream, depth: usize, out: &mut String) {
    for tree in stream {
        let span = tree.span();
        let at = format!("{}:{}", span.line(), span.column());
        let indent = "    ".repeat(depth);
        let _ = match &tree {
            TokenTree::Ident(ident) => writeln!(out, "{at:>5}  {indent}Ident    {ident}"),
            TokenTree::Punct(punct) => {
                let (ch, spacing) = (punct.as_char(), punct.spacing());
                writeln!(out, "{at:>5}  {indent}Punct    {ch}  {spacing:?}")
            }
            TokenTree::Literal(literal) => writeln!(out, "{at:>5}  {indent}Literal  {literal}"),
            TokenTree::Group(group) => writeln!(out, "{at:>5}  {indent}Group    {:?}", group.delimiter()),
        };
        if let TokenTree::Group(group) = tree {
            walk(group.stream(), depth + 1, out);
        }
    }
}

fn string_literal(text: &str) -> TokenStream {
    TokenTree::Literal(Literal::string(text)).into()
}
```
<!-- /file -->

`string_literal` makes the reported text a token, `TokenTree::Literal(Literal::string(…))`, instead of pasting it into Rust source and calling `.parse()`: `Literal::string` does the escaping, so a `"` in the text cannot end the literal early. The fixed parts of the derive's and the attribute's output are still parsed from strings, and `extend` joins the pieces into one stream.

The program feeds it five kinds of input:

<!-- file:demo/app/src/main.rs -->
```rust title="demo/app/src/main.rs"
use inspect_tokens::{Text, text, text_attr, token_trees};

type Meters = u32;

/// A road.
#[derive(Text)]
struct Road {
    length: Meters,
}

/// Greets.
#[text_attr]
fn greet() {}

/// `$e` reaches `token_trees!` wrapped in an invisible group.
macro_rules! twice {
    ($e:expr) => {
        token_trees!($e * 2)
    };
}

/// Hands its tokens to `$inspect!` unchanged, as `println!` and `vec!` do.
macro_rules! forward {
    ($inspect:ident, $($tokens:tt)*) => {
        $inspect!($($tokens)*)
    };
}

/// Every line of `text`, indented under its label.
fn show(label: &str, text: &str) {
    println!("{label}");
    for line in text.lines() {
        println!("    {line}");
    }
}

fn main() {
    show("1. Four kinds of token tree", token_trees!(GET /users/{id} => list_users));
    show("2. Spacing", token_trees!(x += 1; &'a str; 'a'; -1));
    show("3. Delimiters", token_trees!((a) [b] {c}));
    show("   and the invisible one, around a macro_rules! $e", twice!(1 + 1));

    let direct = token_trees! {
        /// hi
    };
    let forwarded = forward! { token_trees,
        /// hi
    };
    show("4. A doc comment, straight to the macro", direct);
    show("   and forwarded by macro_rules!", forwarded);
    let direct = text! {
        /// hi
    };
    let forwarded = forward! { text,
        /// hi
    };
    show("   to_string, straight to a function-like macro", direct);
    show("   to_string, forwarded by macro_rules!", forwarded);
    show("   to_string in a derive", DERIVE_INPUT);
    show("   to_string in an attribute", ATTRIBUTE_INPUT);

    let spacing = [text!(x:i32), text!(x : i32), text!(x
                                                      :     i32)];
    show("5. to_string and whitespace", &spacing.join("\n"));

    greet();
    println!("road.length = {}", Road { length: 5 }.length);
}
```
<!-- /file -->

<!-- cargo:token_trees_one_by_one -->
*Verified output of `cargo run -q -p app` — declared in [`cargo_runs.toml`](demo/cargo_runs.toml) and regenerated by `tools/run_cargo_demos.py`, never hand-typed.*

```text
1. Four kinds of token tree
    38:54  Ident    GET
    38:58  Punct    /  Alone
    38:59  Ident    users
    38:64  Punct    /  Alone
    38:65  Group    Brace
    38:66      Ident    id
    38:70  Punct    =  Joint
    38:71  Punct    >  Alone
    38:73  Ident    list_users
2. Spacing
    39:37  Ident    x
    39:39  Punct    +  Joint
    39:40  Punct    =  Alone
    39:42  Literal  1
    39:43  Punct    ;  Alone
    39:45  Punct    &  Alone
    39:46  Punct    '  Joint
    39:46  Ident    a
    39:49  Ident    str
    39:52  Punct    ;  Alone
    39:54  Literal  'a'
    39:57  Punct    ;  Alone
    39:59  Punct    -  Alone
    39:60  Literal  1
3. Delimiters
    40:40  Group    Parenthesis
    40:41      Ident    a
    40:44  Group    Bracket
    40:45      Ident    b
    40:48  Group    Brace
    40:49      Ident    c
   and the invisible one, around a macro_rules! $e
    18:22  Group    None
    41:71      Literal  1
    41:73      Punct    +  Alone
    41:75      Literal  1
    18:25  Punct    *  Alone
    18:27  Literal  2
4. A doc comment, straight to the macro
     44:9  Punct    #  Alone
     44:9  Group    Bracket
     44:9      Ident    doc
     44:9      Punct    =  Alone
     44:9      Literal  " hi"
   and forwarded by macro_rules!
     47:9  Punct    #  Alone
     47:9  Group    Bracket
     47:9      Ident    doc
     47:9      Punct    =  Alone
     47:9      Literal  r" hi"
   to_string, straight to a function-like macro
    /// hi
   to_string, forwarded by macro_rules!
    #[doc = r" hi"]
   to_string in a derive
    /// A road.
    struct Road { length: Meters, }
   to_string in an attribute
    /// Greets.
    fn greet() {}
5. to_string and whitespace
    x:i32
    x : i32
    x : i32
road.length = 5
```
<!-- /cargo -->

The numbers on the left are [`Span::line()` and `Span::column()` ↗](https://doc.rust-lang.org/proc_macro/struct.Span.html#method.line), stable since Rust 1.88: where each token starts in `main.rs`, counting both from 1. The rest of the page reads the output section by section.

## 1. Four kinds

`GET /users/{id} => list_users` is the input the function-like macro on the three-kinds page printed back as text.

- **`Ident`**: `GET`, `users`, `id`, `list_users`. Keywords are idents too: `struct` in a derive's input is an `Ident` whose text is `struct`, which is what `type_name` on [A proc-macro crate](../a_proc_macro_crate/README.md) looks for.
- **`Punct`**: `/`, `=`, `>`. One character each, never more.
- **`Literal`**: none here; section 2 has `1` and `'a'`.
- **`Group`**: `{id}`, a delimiter and a stream inside it. `id` is indented because it is not in the outer stream at all: iterating the outer stream yields the `Group`, and `id` is reached through `group.stream()`. A token stream is a tree, which is why `walk` calls itself.

`=>` is two puncts. `=` is `Joint`, meaning the next token is punctuation with no space between, and `>` is `Alone`. That is all a parser needs to rebuild `=>`, `+=` or `::` from single characters; a lone `=` followed by a space is `Alone`.

## 2. Spacing

- **`+=`** is `+` `Joint`, then `=` `Alone`.
- **`&'a`** is the trap. `&` is `Alone`, although a `'` touches it. The [`Spacing::Joint` docs ↗](https://github.com/rust-lang/rust/blob/1.98.0/library/proc_macro/src/lib.rs#L1066-L1069) say a `Punct` is `Joint` when *"immediately followed by another `Punct` without a whitespace"*, and in the macro's input `'` is a `Punct`. But `'` and `a` both start at column 46: the compiler read `'a` as one lifetime token, and hands it to the macro as a `'` joined to an `a`. Spacing is decided on the compiler's tokens, not on the characters.
- **`'a'`** is `'a` with one more quote, and it is a single `Literal`, a `char`.
- **`-1`** is two tokens, a `-` and the literal `1`. A negative number is not a literal. The [`Display` docs ↗](https://github.com/rust-lang/rust/blob/1.98.0/library/proc_macro/src/lib.rs#L338-L340) name negative numeric literals as one of the things `to_string()` may not round-trip.

## 3. Delimiters

`(a)`, `[b]` and `{c}` are groups with `Parenthesis`, `Bracket` and `Brace`. There is a fourth [`Delimiter` ↗](https://doc.rust-lang.org/proc_macro/enum.Delimiter.html) that you never type.

`twice!(1 + 1)` is a `macro_rules!` macro that calls `token_trees!($e * 2)`. The proc macro does not receive `1 + 1 * 2`. It receives a `Group` with delimiter `None` holding `1 + 1`, then `*` and `2`: an invisible pair of brackets around the captured expression, so `$e * 2` still means `(1 + 1) * 2`. The [`Delimiter::None` docs ↗](https://github.com/rust-lang/rust/blob/1.98.0/library/proc_macro/src/lib.rs#L925-L939) give exactly that reason, and warn that a `None` group a proc macro *creates* may be ignored by rustc (issue #67062), so use a visible delimiter in your own output.

The spans show where each token came from. The `None` group, `*` and `2` start on line 18, the body of `twice!`; `1 + 1` is on line 41, where `twice!` was called. A span records where a token was written, and tokens from a `macro_rules!` body point into the body.

## 4. A doc comment is an attribute

`/// hi` reaches the macro as five token trees: a `#`, then a bracket group holding `doc`, `=` and the string `" hi"`. Every one of them has the doc comment's span, 44:9. The `///` is gone and the space after it is kept, as part of the string. That is the `#[doc = "…"]` attribute [Comments that compile](../../15_First_Programs/comments_that_compile/README.md) describes, as tokens.

What `to_string()` makes of those tokens depends on where they have been:

| The doc comment reached the macro | The string literal | `to_string()` |
|---|---|---|
| straight from the source, in a function-like macro | `" hi"` | `/// hi` |
| on the item of a derive or an attribute | *(not walked here)* | `/// A road.`, `/// Greets.` |
| forwarded by a `macro_rules!` matcher of `tt`s | `r" hi"`, a raw string | `#[doc = r" hi"]` |

So *"a function-like macro prints a doc comment as `#[doc = r" hi"]`"* is a misreading. The function-like macro prints `/// hi`; what produces the attribute form is a `macro_rules!` that captured the doc comment as `tt`s on the way. That includes calls you might not think of as forwarded: `println!` is `($($arg:tt)*) => { … format_args_nl!($($arg)*) … }` in [`std/src/macros.rs` ↗](https://github.com/rust-lang/rust/blob/1.98.0/library/std/src/macros.rs#L145-L147), so a macro called inside `println!`'s arguments receives tokens that went through a `tt` matcher. Walk the tokens and both forms are a `#` and a bracket group; only the text differs.

## 5. `to_string()` and whitespace

`x:i32` comes back as `x:i32`. `x : i32` comes back as `x : i32`, and so does `x`, a newline, and `:     i32`. In these inputs, tokens written touching came back touching, and any run of whitespace between them, newlines included, came back as one space. The [`Display` docs ↗](https://github.com/rust-lang/rust/blob/1.98.0/library/proc_macro/src/lib.rs#L342-L348) warn that *"the exact form of the output is subject to change"*, which is the case for walking token trees instead of matching text.

The derive's input shows one more thing. `struct Road { … }` was three lines in `main.rs` and came back as one, with the doc comment on a line of its own before it.

## Spans

Every token tree has a span, including the ones a macro makes: `Literal::string` gives its literal `Span::call_site()`, the span of the macro call ([`lib.rs` ↗](https://github.com/rust-lang/rust/blob/1.98.0/library/proc_macro/src/lib.rs#L1373-L1379)). The compiler reports an error at the span of the token it is about. A macro that keeps the user's spans on the tokens it returns, or builds its error from one, gets an error underlining the user's code; one that builds everything at `call_site` gets an error on the macro call. [Errors: from `panic!` to `syn::Error`](../errors_from_panic_to_syn_error/README.md) is about choosing the right span.

## A macro sees tokens, never types

The derive received `struct Road { length: Meters, }`, and `Meters` is an `Ident` whose text is `Meters`. `type Meters = u32;` is a few lines above it, but nothing in the `proc_macro` API turns a name into what it names: there is no call that says `Meters` is `u32`, that `Road` implements `Debug`, or that a trait exists. Macros are expanded before type checking, so a derive knows a field's type only as the tokens written after the colon. A derive that needs to treat `String` fields differently can compare the tokens, and it will not recognise a `type Text = String;` alias; the usual way out is to generate code that lets the compiler decide, such as a trait bound.

## If you are coming from another language

- **C and C++.** The preprocessor works on tokens too, and `#` and `##` are its versions of `stringify!` and making an identifier. The difference is shape: C's preprocessing tokens are a flat sequence, and only a macro's argument list pays attention to parentheses. Rust's lexer pairs every `()`, `[]` and `{}` before any macro runs, so a macro receives a tree.
- **Python.** The `tokenize` module is the nearest thing: a flat stream of tokens, each with a type, its text, and a `(row, column)` start and end, like a span. Python's `OP` tokens are whole operators, so `**=` is one token; Rust's `Punct` is always one character, with `Spacing` to glue operators back together. The parsed tree, `ast`, is what `syn` gives a Rust macro.
- **Java.** An annotation processor starts from the other end. It never sees tokens: it gets `Element`s from the compiler's model of the program, after parsing and after names and types are resolved, so it can ask a field for its `TypeMirror` and learn that it is a `String`. A Rust derive sees `length: Meters` as three tokens and has no way to ask. What a Rust attribute macro can do that a processor cannot is hand back different code for the item it was given.

## See also

- [Three kinds of procedural macro](../three_kinds_of_procedural_macro/README.md) — what each kind of macro receives, as text
- [A proc-macro crate](../a_proc_macro_crate/README.md) — the previous page, whose `type_name` walks these trees
- [`proc-macro2` makes it testable](../proc_macro2_makes_it_testable/README.md) — the next page: the same four kinds, outside the compiler
- [Parsing with `syn`](../parsing_with_syn/README.md) — what to use instead of walking tokens by hand
- [Errors: from `panic!` to `syn::Error`](../errors_from_panic_to_syn_error/README.md) — spans in the errors your users see
- [Comments that compile](../../15_First_Programs/comments_that_compile/README.md) — why `///` is an attribute
- [What an attribute is](../../27_Modules/what_an_attribute_is/README.md) — the `#[…]` syntax a doc comment turns into
- [`proc_macro::TokenTree` ↗](https://doc.rust-lang.org/proc_macro/enum.TokenTree.html)

## Po polsku

**Strumień tokenów** (`TokenStream`) to ciąg **drzew tokenów** (`TokenTree`) czterech rodzajów: **identyfikator** (`Ident`, także słowa kluczowe), **znak interpunkcyjny** (`Punct`, zawsze jeden znak), **literał** (`Literal`) i **grupa** (`Group`) — ograniczniki `()`, `[]`, `{}` albo niewidzialne `None` z własnym strumieniem w środku. Wieloznakowy operator jak `=>` to dwa `Punct`: pierwszy ma **odstęp** (*spacing*) `Joint`, drugi `Alone`. Pułapka: w `&'a` znak `&` jest `Alone`, bo kompilator czyta `'a` jako jeden token czasu życia. `-1` to dwa tokeny, a `$e` z `macro_rules!` przychodzi owinięty w grupę `None`, która zachowuje kolejność działań.

Komentarz dokumentacyjny `/// hi` przychodzi jako `#` i `[doc = " hi"]`. `to_string()` wypisuje go jako `/// hi`, chyba że tokeny przeszły wcześniej przez `macro_rules!` (na przykład przez argumenty `println!`) — wtedy jako `#[doc = r" hi"]`. Każdy token ma **zakres** (*span*) — miejsce w źródle, na które wskaże błąd kompilatora. Makro nie widzi typów: dla derive `Meters` to tylko identyfikator, nie `u32`.

**Szukaj po polsku:** tokeny w makrach Rusta · `TokenStream` i `TokenTree` · `rust proc_macro Spacing Joint Alone` · `rust Delimiter None macro_rules` · `rust proc macro doc comment tokens`
