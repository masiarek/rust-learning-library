# *Rust: The Practical Guide*, chapter 5 exercises, run

[*Rust: The Practical Guide*, run](../README.md) › **Chapter 5 exercises** · §5.7 Practice Exercises and §5.8 Solutions

**Level:** 101 → 201 · a companion to *Rust: The Practical Guide* (Nouman Azam, Rheinwerk 2025), chapter 5

**One line:** The seven exercises that close chapter 5 are about enums that carry data, `Option`, `Result` and a first `HashMap`. All seven of the book's solutions compile on rustc 1.98.0 and print what the book expects. Four of the given programs fail, each for one missing thing: a pattern's argument, a `None`, two patterns, a return type. Three of the solutions draw a default clippy warning, one answers a different type than the task asked for, and one prints the variant name where the task asked for a word.

## The verdict

| # | Exercise | What rustc says about the given code | The book's solution | Verdict | Lesson |
|---|---|---|---|---|---|
| 1 | Storing different types in a vector | `E0599` four times: no variant named `Integer` or `Float`, because the enum body is empty | `Integer(i32)`, `Float(f32)` | compiles and prints the book's two lines; the task said `f64`, and `f32` drops digits past the seventh | [Variants that carry data](../../../13_Enums/variants_that_carry_data/README.md) |
| 2 | Library management system | no code given | a `type_` field, `&Item`, four `{:?}` prints | compiles, clippy-clean; prints the title in quotes and `Book` rather than *book* | [What a struct is](../../../16_Structs/what_a_struct_is/README.md) |
| 3 | The first character of a vector | `E0530` match bindings cannot shadow tuple variants, then `E0425` for `character` | `Some(character) =>` | correct, one word; the function draws `ptr_arg` and `len_zero`, and is `chars.first().copied()` | [`Some` and `None`](../../../17_Option_and_Result/some_and_none/README.md) |
| 4 | Checking for a fruit in a basket | `E0308` expected `Option<String>`, found `()`, with a note that `for` loops evaluate to `()` | `None` after the loop | correct, one line; draws `manual_find`; prints *User's name* for a fruit | [`if let`](../../../17_Option_and_Result/if_let/README.md) |
| 5 | Area and perimeter of shapes | parse error: expected pattern, found `=>` | `Ok(res) =>`, `Err(e) =>` | correct; `Result: 20`; three of the four variants warn `dead_code` in the book's `main` | [`Ok` and `Err`](../../../17_Option_and_Result/ok_and_err/README.md) |
| 6 | The square of a number | parse error: expected type, found `{` | `-> Result<i32, String>` | correct; the book's `main` never shows the `Err` it was written for | [`Option` vs `Result`](../../../17_Option_and_Result/option_vs_result/README.md) |
| 7 | Student management system | no code given | `contains_key`, then `insert` | compiles; draws `map_entry`, two lookups where `entry` does one; `unwrap()` in `main` would panic on the duplicate the method refuses | [`HashMap`](../../../26_Collections/the_hashmap/README.md) |

Four of the seven given programs stop for the same reason: an arm or a signature that names the shape but not what is inside it. The two arms that fix exercises 3 and 5, in a program that runs:

```rust
fn main() {
    let first: Option<char> = Some('a');
    match first {
        Some(character) => println!("First character: {character}"), // First character: a
        None => println!("Empty array"),
    }

    let square: Result<i32, String> = Ok(49);
    match square {
        Ok(res) => println!("Result: {res}"), // Result: 49
        Err(e) => println!("Error: {e}"),
    }
}
```

Exercise 4 needs a `None` where a function runs out of loop, and exercise 6 needs `Result<i32, String>` where a signature runs out of text. Every transcript below is rustc 1.98.0 or its clippy, compiled from a scratch file named what the fence title says.

## Practice

### 1. Storing different types in a vector

A `Vec` holds one type. To keep an integer and a float in the same one, wrap both in an enum. Complete `Value` so that the given `main` compiles: a variant `Integer` carrying an `i32` and a variant `Float` carrying an `f64`.

```rust,compile_fail
#[derive(Debug)]
enum Value {
    // Add code here
}

fn main() {
    let some_val = vec![Value::Integer(12), Value::Float(15.5)];
    for i in some_val {
        match i {
            Value::Integer(num) => println!("Integer: {} ", num),
            Value::Float(num) => println!("Float: {}", num),
        }
    }
}
```

```text title="Abridged — real rustc output for value_skeleton.rs"
error[E0599]: no variant, associated function, or constant named `Integer` found for enum `Value` in the current scope
 --> value_skeleton.rs:7:32
  |
2 | enum Value {
  | ---------- variant, associated function, or constant `Integer` not found for this enum
...
7 |     let some_val = vec![Value::Integer(12), Value::Float(15.5)];
  |                                ^^^^^^^ variant, associated function, or constant not found in `Value`

error: aborting due to 4 previous errors
```

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:pg5_01_value_enum -->
*[`pg5_01_value_enum.rs`](examples/pg5_01_value_enum.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Exercise 1 (§5.7): one `Vec` that holds two kinds of number.
//!
//! The enum is the answer. The `main` is the book's, with the trailing space
//! in "Integer: {} " dropped. The second half is why the payload is `f64`
//! and not the `f32` the book's solution writes.
//!
//!   rustc --edition 2024 pg5_01_value_enum.rs -o /tmp/pg501 && /tmp/pg501

#[derive(Debug)]
enum Value {
    Integer(i32),
    Float(f64),
}

fn main() {
    let some_val = vec![Value::Integer(12), Value::Float(15.5)];

    // The derive the exercise asks for, put to use: variant name plus payload.
    println!("{some_val:?}");

    for i in some_val {
        match i {
            Value::Integer(num) => println!("Integer: {num}"),
            Value::Float(num) => println!("Float: {num}"),
        }
    }

    // Why f64: an f32 keeps 24 significant bits, so 16 777 217 is the first
    // whole number it cannot hold, and 0.1 + 0.2 looks exact only because
    // seven digits are all an f32 can show.
    let big = Value::Float(16_777_217.0);
    let big_f32 = 16_777_217.0_f32;
    println!("{big:?} as f32 -> {big_f32}");
    let sum = Value::Float(0.1 + 0.2);
    let sum_f32 = 0.1_f32 + 0.2_f32;
    println!("{sum:?} as f32 -> {sum_f32}");
}
```
<!-- /source -->

<!-- output:pg5_01_value_enum -->
*Verified output of [`pg5_01_value_enum.rs`](examples/pg5_01_value_enum.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
[Integer(12), Float(15.5)]
Integer: 12
Float: 15.5
Float(16777217.0) as f32 -> 16777216
Float(0.30000000000000004) as f32 -> 0.3
```
<!-- /output -->

</details>

**The book's solution writes `Float(f32)`.** It compiles, and for `15.5` the two payloads print the same, because 15.5 is exact in either width. The task said `f64`, and the run above shows what the narrower type costs: an `f32` keeps 24 significant bits, so `16777217.0` comes out as `16777216`, and `0.1 + 0.2` prints as `0.3` only because seven digits are all an `f32` shows. [What a float actually stores](../../../19_Numbers/what_a_float_stores/README.md) has the arithmetic behind both lines. Nothing in the book's chapter needs the extra range; the point is that a solution should answer the type the task named.

Two smaller things. The `#[derive(Debug)]` the exercise puts on the enum is never used by the book's `main`, which prints the payloads with `{}`; the solution above prints the whole `Vec` with `{:?}` once, which is what the derive is for. And the given `"Integer: {} "` has a trailing space, so the book's first output line ends in one. The solution drops it and writes `{num}`. Clippy's `uninlined_format_args` would suggest that spelling, but it is allow-by-default on 1.98.0, so the book's `{}` with `num` after the comma passes clippy in silence.

### 2. Library management system

Write a struct `Item` with an `id`, a `title`, a `year` and an `item_type`, where `ItemType` is an enum with the variants `Book` and `Magazine`. Write `display_item_info`, which takes an item and prints its ID, title and year, and whether it is a book or a magazine. No code is given.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:pg5_02_library_item -->
*[`pg5_02_library_item.rs`](examples/pg5_02_library_item.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Exercise 2 (§5.7): a library item, and a function that describes it.
//!
//! The field is `item_type`, as the task names it, because `type` is a
//! keyword. The function borrows the item, so the caller keeps it.
//!
//!   rustc --edition 2024 pg5_02_library_item.rs -o /tmp/pg502 && /tmp/pg502

#[derive(Debug)]
enum ItemType {
    Book,
    Magazine,
}

#[derive(Debug)]
struct Item {
    id: u32,
    title: String,
    year: u16,
    item_type: ItemType,
}

/// Prints the four fields, and says in words which kind of item this is.
fn display_item_info(item: &Item) {
    let kind = match item.item_type {
        ItemType::Book => "book",
        ItemType::Magazine => "magazine",
    };
    println!("ID: {}", item.id);
    println!("Title: {}", item.title);
    println!("Year: {}", item.year);
    println!("This item is a {kind}");
}

fn main() {
    let rust_book = Item {
        id: 1,
        title: String::from("The Rust Programming Language Book"),
        year: 2021,
        item_type: ItemType::Book,
    };
    let rust_magazine = Item {
        id: 2,
        title: String::from("Rust Magazine"),
        year: 2022,
        item_type: ItemType::Magazine,
    };
    display_item_info(&rust_book);
    display_item_info(&rust_magazine);

    // `&Item` left both items with their owners, so they can still be used.
    println!();
    println!("{{}} on the title:   {}", rust_book.title);
    println!("{{:?}} on the title: {:?}", rust_book.title);
    println!("{{:?}} on the type:  {:?}", rust_book.item_type);
    println!("the whole record:  {rust_magazine:?}");
}
```
<!-- /source -->

<!-- output:pg5_02_library_item -->
*Verified output of [`pg5_02_library_item.rs`](examples/pg5_02_library_item.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
ID: 1
Title: The Rust Programming Language Book
Year: 2021
This item is a book
ID: 2
Title: Rust Magazine
Year: 2022
This item is a magazine

{} on the title:   The Rust Programming Language Book
{:?} on the title: "The Rust Programming Language Book"
{:?} on the type:  Book
the whole record:  Item { id: 2, title: "Rust Magazine", year: 2022, item_type: Magazine }
```
<!-- /output -->

</details>

**The book names the field `type_`, because `type` is a keyword.** Written plainly, the struct does not parse, and rustc suggests the escape:

```text title="Abridged — real rustc output for item_type_keyword.rs"
error: expected identifier, found keyword `type`
  --> item_type_keyword.rs:10:5
   |
 6 | struct Item {
   |        ---- while parsing this struct
...
10 |     type: ItemType,
   |     ^^^^ expected identifier, found keyword
   |
help: escape `type` to use it as an identifier
   |
10 |     r#type: ItemType,
   |     ++
```

Three ways out, all of which compile: the book's `type_`, the raw identifier `r#type` that the compiler offers ([Raw identifiers `r#`](../../../15_First_Programs/raw_identifiers/README.md)), and `item_type`, the name the task used. The solution takes the third: a reader of `item.item_type` never has to know about the keyword.

**The task says the function takes an `Item`; the book's takes `&Item`, and that is the better signature.** By value, the first call moves the item into the function and the second call has nothing to pass:

```text title="Abridged — real rustc output for display_by_value.rs"
error[E0382]: use of moved value: `rust_book`
  --> display_by_value.rs:29:23
   |
22 |     let rust_book = Item {
   |         --------- move occurs because `rust_book` has type `Item`, which does not implement the `Copy` trait
...
28 |     display_item_info(rust_book);
   |                       --------- value moved here
29 |     display_item_info(rust_book);
   |                       ^^^^^^^^^ value used here after move
   |
note: consider changing this parameter type in function `display_item_info` to borrow instead if owning the value isn't necessary
  --> display_by_value.rs:13:28
   |
13 | fn display_item_info(item: Item) {
   |    -----------------       ^^^^ this parameter takes ownership of the value
```

A function that only reads its argument borrows it, and rustc's own note says so. The solution's `main` uses both items again after describing them.

**The book prints every field with `{:?}`, so the title comes out in quotes.** Its output reads `Title: "The Rust Programming Language Book"`, and the run above shows the same title both ways: `{}` is the `Display` form for a reader, `{:?}` the `Debug` form for a programmer ([Debug and Display](../../../15_First_Programs/debug_vs_display/README.md)). For the enum field the book has no choice, since a plain enum has no `Display` and `{}` on it is `E0277`; `{:?}` prints the variant's name, `Book`. But the task asked for *whether the item is a book or a magazine*, and the way to say that in words is a `match` on the field, which is what the solution's `kind` does. Two type choices, since the task only said *integer*: an ID and a year are never negative, so the solution uses `u32` and `u16` where the book uses `i32` for both.

### 3. The first character of a vector

`first_character` returns the first element of a `Vec<char>` as an `Option<char>`, so that an empty vector gives `None` instead of a panic. The program does not compile; fix it.

```rust,compile_fail
fn first_character(chars: &Vec<char>) -> Option<char> {
    if chars.len() > 0 {
        Some(chars[0])
    } else {
        None
    }
}

fn main() {
    let my_chars = vec!['a', 'b', 'c', 'd'];
    match first_character(&my_chars) {
        Some => println!("First character: {character}"),
        None => println!("Empty array"),
    }
}
```

```text title="Abridged — real rustc output for first_character.rs"
error[E0530]: match bindings cannot shadow tuple variants
   --> first_character.rs:12:9
    |
 12 |         Some => println!("First character: {character}"),
    |         ^^^^
    |         |
    |         cannot be named the same as a tuple variant
    |         help: try specify the pattern arguments: `Some(..)`

error[E0425]: cannot find value `character` in this scope
  --> first_character.rs:12:45
   |
12 |         Some => println!("First character: {character}"),
   |                                             ^^^^^^^^^ not found in this scope

error: aborting due to 2 previous errors
```

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:pg5_03_first_character -->
*[`pg5_03_first_character.rs`](examples/pg5_03_first_character.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Exercise 3 (§5.7): the first character of a `Vec`, or nothing.
//!
//! The one-word fix is `Some(character)`, a pattern that names the payload.
//! The function itself is the book's with its two clippy warnings fixed, and
//! below it the same function the way std would write it.
//!
//!   rustc --edition 2024 pg5_03_first_character.rs -o /tmp/pg503 && /tmp/pg503

/// A slice parameter instead of `&Vec<char>`, and `is_empty` instead of `len() > 0`.
fn first_character(chars: &[char]) -> Option<char> {
    if chars.is_empty() {
        None
    } else {
        Some(chars[0])
    }
}

/// The same thing in one call: `first` gives `Option<&char>`, `copied` makes it `Option<char>`.
fn first_character_std(chars: &[char]) -> Option<char> {
    chars.first().copied()
}

fn report(chars: &[char]) {
    match first_character(chars) {
        Some(character) => println!("First character: {character}"),
        None => println!("Empty array"),
    }
}

fn main() {
    let my_chars = vec!['a', 'b', 'c', 'd'];
    let no_chars: Vec<char> = Vec::new();

    report(&my_chars);
    report(&no_chars);

    println!("first().copied(): {:?} and {:?}", first_character_std(&my_chars), first_character_std(&no_chars));
}
```
<!-- /source -->

<!-- output:pg5_03_first_character -->
*Verified output of [`pg5_03_first_character.rs`](examples/pg5_03_first_character.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
First character: a
Empty array
first().copied(): Some('a') and None
```
<!-- /output -->

</details>

**The book's fix is the right one word: `Some(character)`.** Read the first error closely, because it is not the one a typo gives. A bare name in a pattern is a *binding*, a fresh variable that matches anything, and `Some` cannot be one because it is already the name of a tuple variant ([`Some` is a constructor, not a flag](../../../17_Option_and_Result/some_is_a_constructor/README.md)). Rustc's help, `Some(..)`, would make the arm parse, but `..` binds nothing, so the second error would stay. The pattern has to be the variant's shape with a name in the hole ([Destructuring enums](../../../30_Pattern_Matching/destructuring_enums/README.md)). The mirror image of this error, a misspelt variant that *does* become a binding and silently matches everything, is [A typo becomes a binding](../../../13_Enums/a_typo_becomes_a_binding/README.md).

**The function the book leaves untouched draws two default clippy warnings**, as written:

```text title="Abridged — real clippy-driver output for first_character_book.rs"
warning: writing `&Vec` instead of `&[_]` involves a new object where a slice will do
 --> first_character_book.rs:1:27
  |
1 | fn first_character(chars: &Vec<char>) -> Option<char> {
  |                           ^^^^^^^^^^
  |
  = note: `#[warn(clippy::ptr_arg)]` on by default

warning: length comparison to zero
 --> first_character_book.rs:2:8
  |
2 |     if chars.len() > 0 {
  |        ^^^^^^^^^^^^^^^ help: using `!is_empty` is clearer and more explicit: `!chars.is_empty()`
  |
  = note: `#[warn(clippy::len_zero)]` on by default
```

A `&Vec<char>` parameter accepts only a `Vec`; a `&[char]` accepts a `Vec`, an array and any slice, and costs the caller nothing, since `&my_chars` coerces. The solution's first function is the book's with both fixed. Its second function is what std already provides: [`slice::first`](../../../26_Collections/slice_methods/slice_first/README.md) returns `Option<&char>`, `copied` turns that into `Option<char>`, and the empty case is handled without a branch.

### 4. Checking for a fruit in a basket

`check_fruit` looks for the given fruit in a fixed basket of three and returns it as `Some(fruit)`, or `None` if it is not there. The program does not compile; fix it.

```rust,compile_fail
fn check_fruit(input_fruit: String) -> Option<String> {
    let fruit_basket = vec![
        String::from("mango"),
        String::from("apple"),
        String::from("banana"),
    ];
    for fruit in fruit_basket {
        if input_fruit == fruit {
            return Some(fruit);
        }
    }
}

fn main() {
    let user_fruit = String::from("apple");
    if let Some(fruit) = check_fruit(user_fruit) {
        println!("User's name: {fruit}");
    }
}
```

```text title="Abridged — real rustc output for check_fruit.rs"
error[E0308]: mismatched types
  --> check_fruit.rs:7:5
   |
 1 |   fn check_fruit(input_fruit: String) -> Option<String> {
   |                                          -------------- expected `Option<String>` because of return type
...
 7 | /     for fruit in fruit_basket {
 8 | |         if input_fruit == fruit {
 9 | |             return Some(fruit);
10 | |         }
11 | |     }
   | |_____^ expected `Option<String>`, found `()`
   |
   = note:   expected enum `Option<String>`
           found unit type `()`
   = note: `for` loops evaluate to unit type `()`
help: try adding an expression at the end of the block
   |
11 ~     }
12 +     None
   |
```

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:pg5_04_check_fruit -->
*[`pg5_04_check_fruit.rs`](examples/pg5_04_check_fruit.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Exercise 4 (§5.7): is the fruit in the basket?
//!
//! The fix is one line, `None` after the loop. Then the same search as
//! clippy's `manual_find` suggests it, with the parameter as `&str`.
//!
//!   rustc --edition 2024 pg5_04_check_fruit.rs -o /tmp/pg504 && /tmp/pg504

/// The book's function with the missing `None`. Kept as a loop on purpose:
/// clippy's `manual_find` would rewrite it into `find_fruit` below.
#[allow(clippy::manual_find)]
fn check_fruit(input_fruit: String) -> Option<String> {
    let fruit_basket = vec![
        String::from("mango"),
        String::from("apple"),
        String::from("banana"),
    ];
    for fruit in fruit_basket {
        if input_fruit == fruit {
            return Some(fruit);
        }
    }
    None
}

/// The same search as an iterator: `find` returns the first match or `None`,
/// so the fall-through case cannot be left out. `&str` accepts a literal too.
fn find_fruit(input_fruit: &str) -> Option<String> {
    let fruit_basket = vec![
        String::from("mango"),
        String::from("apple"),
        String::from("banana"),
    ];
    fruit_basket.into_iter().find(|fruit| fruit == input_fruit)
}

fn main() {
    let user_fruit = String::from("apple");
    if let Some(fruit) = check_fruit(user_fruit) {
        println!("Fruit found: {fruit}");
    }

    for wanted in ["banana", "kiwi"] {
        match find_fruit(wanted) {
            Some(fruit) => println!("Fruit found: {fruit}"),
            None => println!("No {wanted} in the basket"),
        }
    }
}
```
<!-- /source -->

<!-- output:pg5_04_check_fruit -->
*Verified output of [`pg5_04_check_fruit.rs`](examples/pg5_04_check_fruit.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Fruit found: apple
Fruit found: banana
No kiwi in the basket
```
<!-- /output -->

</details>

**The book adds `None` after the loop, and its comment has the reason right.** A function body is an expression, its value is its last expression, and a `for` loop's value is `()`. When no fruit matches, the function falls out of the loop with nothing to return, and `()` is not an `Option<String>`. Rustc names the loop, names its type, and offers the fix. In Python the same function would return `None` silently and the caller would find out later ([`Some` and `None`](../../../17_Option_and_Result/some_and_none/README.md) and [`if let`](../../../17_Option_and_Result/if_let/README.md) are the two halves of the fixed program).

**With the `None` in place, clippy has one thing to say about the loop:**

```text title="Abridged — real clippy-driver output for check_fruit_book.rs"
warning: manual implementation of `Iterator::find`
  --> check_fruit_book.rs:7:5
   |
 7 | /     for fruit in fruit_basket {
 8 | |         if input_fruit == fruit {
 9 | |             return Some(fruit);
...  |
12 | |     None
   | |________^ help: replace with an iterator: `fruit_basket.into_iter().find(|fruit| input_fruit == fruit)`
   |
   = note: you may need to dereference some variables
   = note: `#[warn(clippy::manual_find)]` on by default
```

The note about dereferencing is not decoration: pasted as printed, the suggestion fails, because `find` hands its closure a `&String` and `String == &String` has no implementation:

```text title="Abridged — real rustc output for find_variants.rs"
error[E0277]: can't compare `String` with `&String`
 --> find_variants.rs:3:55
  |
3 |     fruit_basket.into_iter().find(|fruit| input_fruit == fruit)
  |                                                       ^^ no implementation for `String == &String`
  |
help: consider dereferencing here
  |
3 |     fruit_basket.into_iter().find(|fruit| input_fruit == *fruit)
  |                                                          +
```

The solution's `find_fruit` takes the search as a `&str` instead of a `String`, so a literal can be passed and the caller keeps its own string; the closure then compares a `&String` with a `&str`, which std allows. Taking `input_fruit: String` by value, as the book does, moves `user_fruit` into the call for no reason; clippy's `needless_pass_by_value` would say so, but it sits in the pedantic group and is off by default. [`iter`, `iter_mut`, `into_iter`](../../../24_Iterators/iter_iter_mut_into_iter/README.md) is why the loop and `find` both consume the basket. And the book's `println!("User's name: {fruit}")` is a label carried over from some other exercise; the solution prints `Fruit found:`.

### 5. Area and perimeter of shapes

`Measurement` is an enum of four shapes, and `calculate` returns the area or the perimeter as a `Result<f64, String>`, refusing a negative length and a polygon with fewer than three sides. The enum and its `impl` are complete; `main` does not compile. Fix it.

```rust,compile_fail
enum Measurement {
    CircleArea(f64),
    RectangleArea(f64, f64),
    TriangleArea(f64, f64),
    Perimeter(Vec<f64>),
}

impl Measurement {
    fn calculate(self) -> Result<f64, String> {
        match self {
            Self::CircleArea(radius) => {
                if radius < 0.0 {
                    Err(String::from("Radius cannot be negative"))
                } else {
                    Ok(std::f64::consts::PI * radius * radius)
                }
            }
            Self::RectangleArea(length, width) => {
                if length < 0.0 || width < 0.0 {
                    Err(String::from("Length and width cannot be negative"))
                } else {
                    Ok(length * width)
                }
            }
            Self::TriangleArea(base, height) => {
                if base < 0.0 || height < 0.0 {
                    Err(String::from("Base and height cannot be negative"))
                } else {
                    Ok(0.5 * base * height)
                }
            }
            Self::Perimeter(sides) => {
                if sides.len() < 3 {
                    Err(String::from("A polygon must have at least 3 sides"))
                } else {
                    Ok(sides.iter().sum())
                }
            }
        }
    }
}

fn main() {
    let user_input = Measurement::TriangleArea(5.0, 8.0);
    match user_input.calculate() {
        => println!("Result: {res}"),
        => println!("Error: {e}"),
    }
}
```

```text title="Abridged — real rustc output for measurement.rs"
error: expected pattern, found `=>`
  --> measurement.rs:46:9
   |
46 |         => println!("Result: {res}"),
   |         ^^ expected pattern

error: aborting due to 1 previous error
```

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:pg5_05_measurement -->
*[`pg5_05_measurement.rs`](examples/pg5_05_measurement.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Exercise 5 (§5.7): area and perimeter, as a `Result`.
//!
//! The enum and its `impl` are the book's, untouched. The fix is in `main`:
//! each arm needs a pattern, `Ok(res)` and `Err(e)`, before its `=>`.
//!
//!   rustc --edition 2024 pg5_05_measurement.rs -o /tmp/pg505 && /tmp/pg505

enum Measurement {
    CircleArea(f64),
    RectangleArea(f64, f64),
    TriangleArea(f64, f64),
    Perimeter(Vec<f64>),
}

impl Measurement {
    fn calculate(self) -> Result<f64, String> {
        match self {
            Self::CircleArea(radius) => {
                if radius < 0.0 {
                    Err(String::from("Radius cannot be negative"))
                } else {
                    Ok(std::f64::consts::PI * radius * radius)
                }
            }
            Self::RectangleArea(length, width) => {
                if length < 0.0 || width < 0.0 {
                    Err(String::from("Length and width cannot be negative"))
                } else {
                    Ok(length * width)
                }
            }
            Self::TriangleArea(base, height) => {
                if base < 0.0 || height < 0.0 {
                    Err(String::from("Base and height cannot be negative"))
                } else {
                    Ok(0.5 * base * height)
                }
            }
            Self::Perimeter(sides) => {
                if sides.len() < 3 {
                    Err(String::from("A polygon must have at least 3 sides"))
                } else {
                    Ok(sides.iter().sum())
                }
            }
        }
    }
}

/// `calculate(self)` consumes the value, so each `Measurement` is built,
/// handed over, and gone: a `match` on the `Result` is all that is left.
fn report(label: &str, measurement: Measurement) {
    match measurement.calculate() {
        Ok(res) => println!("{label:<28} Result: {res}"),
        Err(e) => println!("{label:<28} Error: {e}"),
    }
}

fn main() {
    let user_input = Measurement::TriangleArea(5.0, 8.0);
    match user_input.calculate() {
        Ok(res) => println!("Result: {res}"),
        Err(e) => println!("Error: {e}"),
    }

    println!();
    report("CircleArea(1.0)", Measurement::CircleArea(1.0));
    report("CircleArea(-1.0)", Measurement::CircleArea(-1.0));
    report("RectangleArea(2.5, 4.0)", Measurement::RectangleArea(2.5, 4.0));
    report("Perimeter([3.0, 4.0])", Measurement::Perimeter(vec![3.0, 4.0]));
    report("Perimeter([3.0, 4.0, 5.0])", Measurement::Perimeter(vec![3.0, 4.0, 5.0]));
}
```
<!-- /source -->

<!-- output:pg5_05_measurement -->
*Verified output of [`pg5_05_measurement.rs`](examples/pg5_05_measurement.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
Result: 20

CircleArea(1.0)              Result: 3.141592653589793
CircleArea(-1.0)             Error: Radius cannot be negative
RectangleArea(2.5, 4.0)      Result: 10
Perimeter([3.0, 4.0])        Error: A polygon must have at least 3 sides
Perimeter([3.0, 4.0, 5.0])   Result: 12
```
<!-- /output -->

</details>

**The book writes `Ok(res) =>` and `Err(e) =>`, and that is the whole fix.** The parser stops at the first arm, so only one error is reported for two missing patterns. The arms print `res` and `e`, which tells you what the patterns have to bind and in which order [`Ok` and `Err`](../../../17_Option_and_Result/ok_and_err/README.md) come in a `Result`. The triangle gives `Result: 20`, half of 5 × 8. The book's `main` builds only that one shape, so its solution compiles with a warning from rustc itself: *variants `CircleArea`, `RectangleArea`, and `Perimeter` are never constructed*, the `dead_code` lint. The solution's `report` runs every variant once, both the `Ok` and the `Err` of each.

**`calculate(self)` consumes the shape.** The method takes `self` by value, so each `Measurement` is built, handed over and gone; a second call on the same variable is a use after move:

```text title="Abridged — real rustc output for calculate_twice.rs"
error[E0382]: use of moved value: `user_input`
  --> calculate_twice.rs:46:18
   |
44 |     let user_input = Measurement::TriangleArea(5.0, 8.0);
   |         ---------- move occurs because `user_input` has type `Measurement`, which does not implement the `Copy` trait
45 |     let first = user_input.calculate();
   |                            ----------- `user_input` moved due to this method call
46 |     let second = user_input.calculate();
   |                  ^^^^^^^^^^ value used here after move
   |
note: `Measurement::calculate` takes ownership of the receiver `self`, which moves `user_input`
  --> calculate_twice.rs:9:18
   |
 9 |     fn calculate(self) -> Result<f64, String> {
   |                  ^^^^
```

For a value that is computed once, that is fine, and it is the shape the book gives. Changing the receiver to `&self` is not the one-character edit it looks like: through a reference, `match self` binds `radius` as a `&f64`, and every `radius < 0.0` becomes `E0308`, *expected `&f64`, found floating-point number*, five times over, each with the help *consider dereferencing the borrow*. [`impl` blocks](../../../16_Structs/impl_blocks/README.md) has the three receivers and what each one lets the caller keep. One more inference worth noticing: `Ok(sides.iter().sum())` never says what it sums to; `sum` takes its output type from the `Result<f64, String>` the function promises.

### 6. The square of a number

`calculate_square` squares a non-negative number, printing the result, and refuses a negative one with a message. The return type is missing from the signature; write it.

```rust,compile_fail
fn calculate_square(num: i32) ->  {
    if num >= 0 {
        let result = num * num;
        println!("The square of {} is: {}", num, result);
        Ok(result)
    } else {
        Err("Negative number provided".to_string())
    }
}

fn main() {
    let number = 7;
    if let Err(e) = calculate_square(number) {
        println!("Error: {e}");
    }
}
```

```text title="Abridged — real rustc output for calculate_square.rs"
error: expected type, found `{`
 --> calculate_square.rs:1:35
  |
1 | fn calculate_square(num: i32) ->  {
  |                                   ^ expected type

error: aborting due to 1 previous error
```

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:pg5_06_calculate_square -->
*[`pg5_06_calculate_square.rs`](examples/pg5_06_calculate_square.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Exercise 6 (§5.7): the square of a number, or an error.
//!
//! The function is the book's with the missing return type written in:
//! `Result<i32, String>`, because the body returns `Ok(result)` where
//! `result: i32` and `Err(...)` with a `String`. Then a version that also
//! refuses a square that does not fit.
//!
//!   rustc --edition 2024 pg5_06_calculate_square.rs -o /tmp/pg506 && /tmp/pg506

fn calculate_square(num: i32) -> Result<i32, String> {
    if num >= 0 {
        let result = num * num;
        println!("The square of {} is: {}", num, result);
        Ok(result)
    } else {
        Err("Negative number provided".to_string())
    }
}

/// `checked_mul` returns `None` instead of overflowing; `ok_or_else` turns
/// that into the `Err` this function promises.
fn checked_square(num: i32) -> Result<i32, String> {
    if num < 0 {
        return Err("Negative number provided".to_string());
    }
    num.checked_mul(num)
        .ok_or_else(|| format!("{num} squared does not fit in an i32"))
}

fn main() {
    let number = 7;
    if let Err(e) = calculate_square(number) {
        println!("Error: {e}");
    }

    // The `if let Err` only speaks up on the error path.
    if let Err(e) = calculate_square(-3) {
        println!("Error: {e}");
    }

    // A `match` sees both paths, and the checked version has one more way to fail.
    for n in [46_340, 46_341] {
        match checked_square(n) {
            Ok(square) => println!("{n} squared is {square}"),
            Err(e) => println!("Error: {e}"),
        }
    }
}
```
<!-- /source -->

<!-- output:pg5_06_calculate_square -->
*Verified output of [`pg5_06_calculate_square.rs`](examples/pg5_06_calculate_square.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
The square of 7 is: 49
Error: Negative number provided
46340 squared is 2147395600
Error: 46341 squared does not fit in an i32
```
<!-- /output -->

</details>

**`Result<i32, String>`, as the book says.** The body decides it: `Ok(result)` carries an `i32`, because `num * num` is one, and `Err("…".to_string())` carries a `String`. An `Option<i32>` would have fitted the happy path and thrown the message away, which is the choice [`Option` vs `Result`](../../../17_Option_and_Result/option_vs_result/README.md) is about. The book's `main` calls the function with `7`, so it prints `The square of 7 is: 49` from inside the function and nothing else: `if let Err(e)` only speaks on the error path, and the `Ok(49)` is dropped unread. The solution calls it with `-3` as well, so the `Err` the function was written for is printed once.

**What the book's function does not check.** `num * num` overflows an `i32` from 46341 upwards, and what happens then depends on how the program was built:

```text title="Abridged — real output of square_overflow.rs, a debug build on rustc 1.98.0"
thread 'main' (41453160) panicked at square_overflow.rs:3:22:
attempt to multiply with overflow
```

The same program built with `-O` prints `The square of 46341 is: -2147479015` and returns `Ok`. The solution's `checked_square` uses `checked_mul`, which returns `None` instead of overflowing, and `ok_or_else` turns that into the `Err` the signature promises; the run shows 46340 succeeding and 46341 refused. [The integer types](../../../19_Numbers/the_integer_types/README.md) has the widths and the debug-versus-release rule.

### 7. Student management system

A `Student` has an ID, a name and a grade. `StudentManager` keeps students in a `HashMap` from ID to `Student`, with `new() -> Self`, `add_student(&mut self, student: Student) -> Result<(), String>`, which refuses an ID that is already on file, and `get_student(&self, id) -> Option<&Student>`. Add a few students and look them up. No code is given.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:pg5_07_student_manager -->
*[`pg5_07_student_manager.rs`](examples/pg5_07_student_manager.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Exercise 7 (§5.7): a student register over a `HashMap`.
//!
//! `add_student` refuses a duplicate ID with one lookup: `entry` finds the
//! slot, and the slot says whether it is taken. Printing goes through fixed
//! IDs, never through the map's own order, which is not defined.
//!
//!   rustc --edition 2024 pg5_07_student_manager.rs -o /tmp/pg507 && /tmp/pg507

use std::collections::HashMap;
use std::collections::hash_map::Entry;

struct Student {
    id: u32,
    name: String,
    grade: String,
}

struct StudentManager {
    students: HashMap<u32, Student>,
}

impl StudentManager {
    fn new() -> Self {
        StudentManager {
            students: HashMap::new(),
        }
    }

    /// `student.id` is copied out before `student` moves into the slot.
    fn add_student(&mut self, student: Student) -> Result<(), String> {
        match self.students.entry(student.id) {
            Entry::Vacant(slot) => {
                slot.insert(student);
                Ok(())
            }
            Entry::Occupied(_) => Err(format!("Student with ID {} already exists", student.id)),
        }
    }

    fn get_student(&self, id: u32) -> Option<&Student> {
        self.students.get(&id)
    }

    fn count(&self) -> usize {
        self.students.len()
    }
}

fn main() {
    let mut manager = StudentManager::new();

    let roster = [(1, "Alice", "A"), (2, "Bob", "B"), (1, "Alicia", "C")];
    for (id, name, grade) in roster {
        let student = Student {
            id,
            name: name.to_string(),
            grade: grade.to_string(),
        };
        match manager.add_student(student) {
            Ok(()) => println!("added {name} as #{id}"),
            Err(e) => println!("refused: {e}"),
        }
    }
    println!("{} students on file", manager.count());

    for id in [1, 2, 3] {
        match manager.get_student(id) {
            Some(student) => println!("#{id}: {} (grade {})", student.name, student.grade),
            None => println!("#{id}: no such student"),
        }
    }
}
```
<!-- /source -->

<!-- output:pg5_07_student_manager -->
*Verified output of [`pg5_07_student_manager.rs`](examples/pg5_07_student_manager.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
added Alice as #1
added Bob as #2
refused: Student with ID 1 already exists
2 students on file
#1: Alice (grade A)
#2: Bob (grade B)
#3: no such student
```
<!-- /output -->

</details>

**The book checks `contains_key` and then calls `insert`, which is two lookups.** Clippy's `map_entry` lint, on by default, names the pattern and rewrites it:

```text title="Abridged — real clippy-driver output for student_manager_book.rs"
warning: usage of `contains_key` followed by `insert` on a `HashMap`
  --> student_manager_book.rs:21:9
   |
21 | /         if self.students.contains_key(&student.id) {
22 | |             Err(format!("Student with ID {} already exists", student.id))
23 | |         } else {
24 | |             self.students.insert(student.id, student);
25 | |             Ok(())
26 | |         }
   | |_________^
   |
   = note: `#[warn(clippy::map_entry)]` on by default
help: try
   |
21 ~         if let std::collections::hash_map::Entry::Vacant(e) = self.students.entry(student.id) {
22 +             e.insert(student);
23 +             Ok(())
24 +         } else {
25 +             Err(format!("Student with ID {} already exists", student.id))
26 +         }
```

`entry(id)` does the hashing once and hands back a slot that is either `Vacant` or `Occupied`; the solution matches on both. The order inside the call matters: `student.id` is copied out as the argument before `student` itself moves into `slot.insert(student)`, so the `Occupied` arm can still read `student.id` for its message. [`HashMap`](../../../26_Collections/the_hashmap/README.md) is the page for `entry`, and [A first `HashMap`](../../../26_Collections/a_first_hashmap/README.md) the on-ramp if this is your first one. There is a method that does the refusal in one call, `try_insert`, but on 1.98.0 it is nightly-only:

```text title="Abridged — real rustc output for try_insert.rs"
error[E0658]: use of unstable library feature `map_try_insert`
 --> try_insert.rs:5:20
  |
5 |     match students.try_insert(1, String::from("Alice")) {
  |                    ^^^^^^^^^^
  |
  = note: see issue #82766 <https://github.com/rust-lang/rust/issues/82766> for more information
```

**The book's `main` calls `manager.add_student(student1).unwrap()`.** With two distinct IDs that is harmless, but the method was written to return an `Err` for a duplicate, and `unwrap` turns that `Err` into a panic ([What `unwrap` does](../../../17_Option_and_Result/what_unwrap_does/README.md)). The solution's `main` adds a third student with ID 1 on purpose and prints the refusal instead. `get_student` returns `Option<&Student>`, a borrow into the map, and the solution looks up IDs 1, 2 and 3 in that order, so the last line shows the `None`. Nothing on the page iterates the map itself: its [order is not defined, and differs between runs](../../../26_Collections/the_hashmap/README.md#iteration-order-is-not-defined-and-not-stable-between-runs), so a program that printed the students by walking it would print a different transcript each time.

## If you are coming from another language

- **Python.** Where Rust writes one enum whose variants carry different data, Python writes a class per kind, or a `dataclass` per kind with a common base, and a `match`/`case` (3.10) that binds like `Some(character)`. The differences are all on the compiler's side. `Optional[str]` is `str | None`, and `None` is an ordinary value: a function that falls off the end of its loop returns `None` silently, which is the bug exercise 4's `E0308` refuses at compile time. There is no `Result`: `calculate_square` would raise `ValueError`, and the caller could forget to catch it. A `dict` is the `HashMap` of exercise 7, `d.get(k)` is `get`, `setdefault` is the `entry` API's vacant branch, and the dict remembers insertion order where the `HashMap` deliberately does not ([the Python paragraph on the `HashMap` page](../../../26_Collections/the_hashmap/README.md#if-you-are-coming-from-another-language)). The quotes in exercise 2 are `repr` versus `str`: `{:?}` is `repr(title)`, `{}` is `str(title)`, and the Python library's [`repr` is not `str` ↗](https://masiarek.github.io/python-learning-library/01_Text_and_Bytes/repr_is_not_str/index.html) is the same distinction from the other side.
- **Java.** An `enum` can carry fields through a constructor, but every constant carries the *same* fields, so `CircleArea(f64)` beside `RectangleArea(f64, f64)` does not fit one. The counterpart is a sealed interface with a record per shape and a `switch` with record patterns (Java 21), which is exhaustive the way `match` is. `Optional<T>` maps onto `Option`, though it is meant for return values rather than fields; `Result` has no counterpart, since a failing `calculateSquare` throws. `HashMap.containsKey` followed by `put` is exercise 7's two-lookup habit, and `putIfAbsent` or `computeIfAbsent` is its `entry`.
- **C.** An `enum` is an integer with names, and a variant that carries data is a `struct` holding a tag and a `union`, kept in step by hand; [What a union is](../../../09_Advanced/what_a_union_is/README.md) shows the tagged version as Rust's `enum`. `None` is a `NULL` pointer or a sentinel value, `Result` is a return code plus `errno`, and the standard library has no hash map at all. The compile errors on this page are the ones C leaves for run time.
- **Ruby.** `nil` is a value, not a variant, and only `nil` and `false` are falsy ([Only `nil` and `false` are falsy ↗](https://masiarek.github.io/ruby-learning-library/01_Objects_and_Values/nil_false_and_truthiness/index.html)); a missing hash key is `nil` where `get` is `None` ([A missing key is `nil` ↗](https://masiarek.github.io/ruby-learning-library/04_Collections/hashes_and_default_values/index.html)). `case … in` binds names out of a shape the way `Some(character)` does ([`case`/`in` matches shape ↗](https://masiarek.github.io/ruby-learning-library/09_Control_Flow_and_Pattern_Matching/case_in_pattern_matching/index.html)), and `Struct`/`Data` are exercise 2's `Item` ([Struct is mutable, Data is not ↗](https://masiarek.github.io/ruby-learning-library/04_Collections/struct_and_data/index.html)).
- **TypeScript.** A discriminated union, one object type per variant with a shared `kind` field and a `switch` on it, is the closest thing to an enum with data, and `never` is how the exhaustiveness check is bolted on. The JavaScript/TypeScript library's [Discriminated unions ↗](https://masiarek.github.io/javascript-typescript-learning-library/24_Narrowing/discriminated_unions/index.html) is an outline so far.

## See also

- [*Rust: The Practical Guide*, run](../README.md) — the shelf page, chapter by chapter
- [What an enum is](../../../13_Enums/what_an_enum_is/README.md) and [Variants that carry data](../../../13_Enums/variants_that_carry_data/README.md) — exercises 1, 2 and 5, and what a payload costs
- [`Vec`](../../../26_Collections/the_vec/README.md) — the collection exercises 1, 3 and 4 hold their values in
- [What a struct is](../../../16_Structs/what_a_struct_is/README.md), [What is a record, in memory?](../../../16_Structs/representing_a_record/README.md) and [`impl` blocks](../../../16_Structs/impl_blocks/README.md) — exercises 2, 5 and 7
- [Raw identifiers `r#`](../../../15_First_Programs/raw_identifiers/README.md) and [Debug and Display](../../../15_First_Programs/debug_vs_display/README.md) — the keyword and the quotes in exercise 2
- [`Some` and `None`](../../../17_Option_and_Result/some_and_none/README.md), [`if let`](../../../17_Option_and_Result/if_let/README.md), [`Ok` and `Err`](../../../17_Option_and_Result/ok_and_err/README.md) and [`Option` vs `Result`](../../../17_Option_and_Result/option_vs_result/README.md) — exercises 3 to 6, one page each
- [Destructuring enums](../../../30_Pattern_Matching/destructuring_enums/README.md) and [`match` expressions](../../../25_Control_Flow/match_expressions/README.md) — the arms, and why every one has to be there
- [`slice::first`](../../../26_Collections/slice_methods/slice_first/README.md) and [`iter`, `iter_mut`, `into_iter`](../../../24_Iterators/iter_iter_mut_into_iter/README.md) — the std methods behind exercises 3 and 4
- [A first `HashMap`](../../../26_Collections/a_first_hashmap/README.md) and [`HashMap`](../../../26_Collections/the_hashmap/README.md) — exercise 7, from the first insert to `entry`
- [KATAS.md](../../../KATAS.md) — every kata in the library, in order
- [`Iterator::find` ↗](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find) · [`HashMap::entry` ↗](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry) · [`HashMap::try_insert` ↗](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.try_insert) · [`i32::checked_mul` ↗](https://doc.rust-lang.org/std/primitive.i32.html#method.checked_mul)

## Po polsku

Siedem ćwiczeń zamykających rozdział 5 książki *Rust: The Practical Guide* (Nouman Azam, Rheinwerk 2025) dotyczy wyliczeń z danymi (*enums with data*), typów `Option` i `Result` oraz pierwszej mapy `HashMap`. Wszystkie rozwiązania z §5.8 kompilują się na rustc 1.98.0 i drukują to, czego książka oczekuje. Cztery podane programy nie kompilują się, a warto znać dokładne brzmienie każdego komunikatu: `Some =>` bez argumentu to `E0530` (*match bindings cannot shadow tuple variants*), a nie literówka, bo goła nazwa we wzorcu (*pattern*) byłaby nowym wiązaniem (*binding*), a `Some` już jest wariantem krotkowym (*tuple variant*); pętla `for` bez `None` po niej to `E0308`, bo wartością pętli jest `()`; ramię `=> println!` bez wzorca i sygnatura `->  {` bez typu zatrzymują już parser. Trzy rozwiązania książki są poprawne, ale clippy ma uwagi domyślne: `&Vec<char>` zamiast wycinka (*slice*, lint `ptr_arg`), `len() > 0` zamiast `is_empty()` (`len_zero`), ręcznie napisane `find` (`manual_find`) oraz `contains_key` plus `insert`, czyli dwa wyszukiwania tam, gdzie `entry` robi jedno (`map_entry`). Dwie niezgodności z treścią zadania: ćwiczenie 1 prosi o `f64`, a rozwiązanie daje `f32`, który gubi cyfry powyżej siódmej (16 777 217 wychodzi jako 16 777 216); ćwiczenie 2 prosi, by wypisać, czy pozycja jest książką, czy czasopismem, a rozwiązanie drukuje nazwę wariantu przez `{:?}`, przez co tytuł ląduje w cudzysłowie. Słowo kluczowe `type` nie może być nazwą pola: wyjściem jest `item_type`, `type_` albo surowy identyfikator (*raw identifier*) `r#type`. Metoda `calculate(self)` konsumuje wartość, więc drugie wywołanie to `E0382`, a zamiana na `&self` nie jest darmowa, bo `radius < 0.0` staje się porównaniem `&f64` z `f64`. `try_insert` na `HashMap` istnieje tylko w nightly (`map_try_insert`), a `num * num` w ćwiczeniu 6 przepełnia `i32` od 46341 wzwyż: w kompilacji debug panika, po `-O` ujemny wynik; `checked_mul` zwraca wtedy `None`.

**Szukaj po polsku:** `rust E0530 match bindings cannot shadow tuple variants` · `rust E0308 for loop evaluates to unit type` · `rust HashMap entry zamiast contains_key insert` · `rust f32 czy f64 precyzja` · wyliczenie z danymi · dopasowanie wzorców · typ opcjonalny · `Result` zamiast wyjątku · przepełnienie arytmetyczne `checked_mul`

---

[*Rust: The Practical Guide*, run](../README.md) › **Chapter 5 exercises** · back to [What an enum is](../../../13_Enums/what_an_enum_is/README.md) · [`Some` and `None`](../../../17_Option_and_Result/some_and_none/README.md) · [`HashMap`](../../../26_Collections/the_hashmap/README.md)
