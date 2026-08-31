# Meet the `bool`

**Level:** 101 → 201 · for newcomers

**One line:** `bool` holds one bit of information in one byte of space, and it is **not a number** — Rust has no truthiness anywhere, so every conversion between a condition and a count is one you write down.

```rust
let voted: bool = true;
let counted = voted as u8;              // 1 — the cast is never implicit
let flags = [true, false, true];
let approvals = flags.iter().filter(|&&b| b).count();
println!("{voted} {counted} {approvals}");   // true 1 2
```

## Two values, one byte

`bool` has exactly two values, and it occupies **one byte** — not one bit — because a byte is the smallest thing a machine can address. Eight of them in an array are eight bytes; packing eight into one byte is [bit flags](../../19_Numbers/bit_flags/README.md), and it is deliberate work you opt into.

The interesting size is the third one:

```text
size_of::<bool>()         = 1
size_of::<Option<bool>>() = 1   <- None hides in one of the 254 unused patterns
size_of::<[bool; 8]>()    = 8
```

A `bool` uses two of its 256 bit patterns, so `Option<bool>` costs nothing extra — the compiler stores `None` *in* the value rather than beside it. That is the [niche optimization](../../17_Option_and_Result/option_as_collection/README.md), and `bool` is its clearest case.

## There is no truthiness — anywhere

`if` demands a `bool` and accepts nothing else. No zero-is-false, no empty-is-false, no null-is-false:

```rust
let n = 1;
// if n { }        // error[E0308]: mismatched types — expected `bool`, found integer
if n != 0 { }      // say what you meant
```

```text title="Abridged — real rustc output for truthy.rs"
error[E0308]: mismatched types
 --> truthy.rs:3:8
  |
3 |     if n {
  |        ^ expected `bool`, found integer
```

This is not `if` being fussy. It is the same rule everywhere a condition appears — `while`, `match` guards, `filter`, `assert!` — because there is nothing in the language that converts a value to a truth. A `Vec` is not false when empty; an `Option` is not false when `None`; `0.0` is not false. Each has its own question (`is_empty()`, `is_none()`, `== 0.0`), and Rust makes you pick the one you meant.

## `bool` is not a number either

```rust
// let x = true + 1;   // error[E0369]: cannot add `{integer}` to `bool`
let x = true as u8 + 1;   // 2 — the cast said out loud
```

Both directions need saying:

| you want | write | why not a cast |
|---|---|---|
| `bool` → number | `b as u8`, or [`u8::from(b)` ↗](https://doc.rust-lang.org/std/primitive.u8.html#impl-From%3Cbool%3E-for-u8) | `true` is 1 and `false` is 0, guaranteed |
| number → `bool` | `n != 0` | `1u8 as bool` does not compile |

The second row is the one worth remembering, because rustc will hand you the answer:

```text title="Abridged — real rustc output for tobool.rs"
error[E0054]: cannot cast `u8` as `bool`
 --> tobool.rs:2:13
  |
2 |     let b = 1u8 as bool;
  |             ^^^^^^^^^^^
help: compare with zero instead
  |
2 -     let b = 1u8 as bool;
2 +     let b = 1u8 != 0;
```

"Compare with zero instead" is the whole design in one line: `1u8 != 0` states which comparison you wanted, and a cast never could.

## The trap: `&` compiles on bools and does not short-circuit

`&&` and `||` short-circuit — the right side is not evaluated when the left already decides the answer. `&` and `|` also work on `bool`, produce the same value, and **always evaluate both sides**:

```rust
let scores = vec![5u8, 3, 0];
let i = 7;
let safe = (i < scores.len()) && (scores[i] > 0);   // false — the index is never read
// let bug = (i < scores.len()) & (scores[i] > 0);  // panics: index out of bounds
println!("{safe}");   // false
```

Nothing warns you. Both forms type-check, both are `bool → bool → bool`, and on a guard whose entire job is to make the right half *safe to ask*, the single `&` reads the element the left half was protecting you from. The same shape bites with a side effect instead of a panic — `cache_ok & fetch_from_network()` does the fetch every time.

Use `&`/`|` on bools only when you want both sides evaluated on purpose, which is rare enough to deserve a comment.

## A `bool` can hand you a value without an `if`

```rust
let quorum_met = true;
assert_eq!(quorum_met.then_some(42), Some(42));
assert_eq!(false.then_some(42), None);
assert_eq!(quorum_met.then(|| "counted"), Some("counted"));
```

[`then_some` ↗](https://doc.rust-lang.org/std/primitive.bool.html#method.then_some) takes a ready value; [`then` ↗](https://doc.rust-lang.org/std/primitive.bool.html#method.then) takes a closure, so the value is only built when the condition holds. Both turn a condition into an [`Option`](../../17_Option_and_Result/some_and_none/README.md), which is what lets a check join a chain of `?` and `unwrap_or` instead of becoming a block.

And `false < true`, so bools sort, `max`, and compare like any other [`Ord`](../../12_Traits/marker_traits/README.md) type — `false` first.

## Parsing one

`"true"` and `"false"` parse; nothing else does, including `"TRUE"` and `"1"`:

```text
 "true".parse::<bool>() = Ok(true)
 "TRUE".parse::<bool>() = Err(provided string was not `true` or `false`)
    "1".parse::<bool>() = Err(provided string was not `true` or `false`)
```

If your input format says `Y`/`N` or `1`/`0`, that is a match arm you write, not something `parse` is hiding from you.

## If you are coming from another language

**Python.** Two differences, and the first is the one that quietly changes what your code means.

Python's `bool` is a **subclass of `int`**. `True` *is* 1 — `True + 1 == 2`, `sum([True, False, True]) == 2`, and `[False, True][flag]` indexes a list with a boolean. None of that has a Rust translation, because `bool` is its own type with no arithmetic at all. Python's `sum(flags)` becomes a question about counting:

```python
sum(flags)                                  # Python: 2
```

```rust
flags.iter().filter(|&&b| b).count()        // Rust: 2 — and it says "how many are true"
```

You can write `flags.iter().map(|&b| b as u32).sum()` and get the same number, but the reader has to work out from a cast that counting was the point. Prefer `filter(..).count()`.

The second is truthiness. Python has it everywhere — `if []`, `if ""`, `if 0`, `if None` are all false, and `__bool__` lets any class join in. Rust has none of it, and the habit that transfers badly is `if my_vec:` → `if !my_vec.is_empty()`. The gain is that `if x` in Rust never depends on which type `x` turned out to be, which is the failure mode of a truthiness check on a value that arrived as `0` when you expected `None`.

One that *does* transfer: `and`/`or` short-circuit in Python exactly as `&&`/`||` do in Rust, and Python's `&`/`|` on bools are likewise non-short-circuiting bitwise operators. The trap in this page's `&`-versus-`&&` section is the same trap in both languages — Rust just gives you more ways to feel it, since the right-hand side may panic rather than merely be slow.

**ABAP.** This is the bridge worth reading carefully, because ABAP's boolean is the one thing on this page that has no real counterpart: **classic ABAP has no boolean data type.** What it has is `ABAP_BOOL`, a *character of length 1*, with the constants `abap_true = 'X'`, `abap_false = ' '` and `abap_undefined = '-'`. So the everyday ABAP idiom is a string comparison wearing a boolean's clothes:

```abap
DATA(lv_voted) = abap_true.
IF lv_voted = abap_true.        " comparing a char to a char
```

Three consequences, all of which Rust removes rather than improves:

- **The comparison is the ceremony.** `IF lv_flag = abap_true` is written in full because `IF lv_flag` would be comparing a character field to nothing — there is no condition type to stand alone. In Rust `if flag` *is* the condition, and it is exactly as short as it looks.
- **A third state exists and is easy to hit.** `abap_undefined` and an uninitialised char field are both "not `X`", so the classic ABAP bug is code that treats every non-`X` value as false and quietly merges *unknown* into *no*. Rust's `bool` has two values, and the moment you have a third state the type has to say so — `Option<bool>`, which costs the same one byte, and which the compiler will not let you read without handling `None`.
- **Character semantics leak.** Because `abap_bool` is text, `'x'` and `'X'` are different values, and a field of the wrong length or an unexpected space are ordinary bugs rather than impossible ones.

Where the two languages *agree* is short-circuiting: ABAP's `AND` and `OR` short-circuit, so the guard idiom `IF lines( lt_tab ) > 0 AND lt_tab[ 1 ]-score > 0` protects the second half in ABAP for the same reason `&&` does in Rust. The Rust-specific hazard is that `&` is available and silently drops the protection; ABAP has no such near-miss operator to reach for by accident. Going the other way, `boolc( )` and `xsdbool( )` — which build an `abap_bool` from a logical expression — are the closest thing to Rust's `b as u8` in spirit: an explicit conversion between "a condition" and "a stored value", because the two are not the same thing in either language.

---

## Practice

**The guard that stops protecting you.** Write a bounds check whose left half exists to make the right half safe: `(i < scores.len()) && (scores[i] > 0)`. Run it for an index that is in range and one that is not, then change the single `&&` to `&` and run both again. Predict what changes before you do it.

Then take a slice of `bool` approvals and write Python's `sum(flags)` three ways in Rust. Say which of the three you would leave in a code review, and why the other two make the reader work harder.

Finish by naming the two errors this page's rules produce — the one from `true + 1` and the one from `1u8 as bool` — and the fix rustc suggests for the second.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:meet_the_bool_kata -->
*[`meet_the_bool_kata.rs`](examples/meet_the_bool_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: the guard that stops working when `&&` becomes `&`, and the
//! three Rust spellings of Python's `sum(flags)`.

fn main() {
    println!("=== part 1: the guard, written both ways ===");
    let scores: Vec<u8> = vec![5, 3, 0];

    // The whole job of the left half is to make the right half safe to ask.
    for i in [1usize, 7usize] {
        let ok = (i < scores.len()) && (scores[i] > 0);
        println!("  index {i}: (i < len) && (scores[i] > 0) = {ok}");
    }

    println!("\n  now with a single &, which evaluates BOTH sides whatever the left says:");
    for i in [1usize, 7usize] {
        let hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let attempt = std::panic::catch_unwind(|| (i < scores.len()) & (scores[i] > 0));
        std::panic::set_hook(hook);
        match attempt {
            Ok(v) => println!("  index {i}: (i < len) &  (scores[i] > 0) = {v}"),
            Err(_) => println!("  index {i}: (i < len) &  (scores[i] > 0) = PANIC -- index out of bounds"),
        }
    }
    println!("  both forms compile, both are `bool -> bool -> bool`, and one of them");
    println!("  reads the element it was supposed to be protecting you from.");

    println!("\n=== part 2: counting the trues -- Python's sum(flags), three ways ===");
    let approvals = [true, false, true, true, false];
    println!("  approvals = {:?}", approvals);

    let a = approvals.iter().filter(|&&b| b).count();
    let b: u32 = approvals.iter().map(|&f| f as u32).sum();
    let c: u32 = approvals.iter().copied().map(u32::from).sum();
    println!("  filter(|&&b| b).count()        = {a}   <- says 'how many are true'");
    println!("  map(|f| f as u32).sum()        = {b}   <- says 'add them up', via a cast");
    println!("  map(u32::from).sum()           = {c}   <- the same cast as a named conversion");
    println!("  the first one is the one to write: the other two make you re-read a cast");
    println!("  to find out that counting was the point.");

    println!("\n=== part 3: what a bool cannot do, and the escape hatch ===");
    println!("  `true + 1`  -> error[E0369]: cannot add `{{integer}}` to `bool`");
    println!("  `if 1 {{ }}`  -> error[E0308]: mismatched types, expected `bool`, found integer");
    println!("  so the cast is never implicit and never accidental:");
    println!("    true as u8  = {}      false as u8 = {}", true as u8, false as u8);
    println!("    u8::from(true) = {}   u8::from(false) = {}", u8::from(true), u8::from(false));
    println!("  and going back the other way needs a comparison, not a cast:");
    println!("    1u8 as bool -> error[E0054]: cannot cast `u8` as `bool`");
    println!("                   help: compare with zero instead");
    println!("    1u8 != 0    = {}   <- which is the question you actually meant", 1u8 != 0);
}
```
<!-- /source -->

<!-- output:meet_the_bool_kata -->
*Verified output of [`meet_the_bool_kata.rs`](examples/meet_the_bool_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
=== part 1: the guard, written both ways ===
  index 1: (i < len) && (scores[i] > 0) = true
  index 7: (i < len) && (scores[i] > 0) = false

  now with a single &, which evaluates BOTH sides whatever the left says:
  index 1: (i < len) &  (scores[i] > 0) = true
  index 7: (i < len) &  (scores[i] > 0) = PANIC -- index out of bounds
  both forms compile, both are `bool -> bool -> bool`, and one of them
  reads the element it was supposed to be protecting you from.

=== part 2: counting the trues -- Python's sum(flags), three ways ===
  approvals = [true, false, true, true, false]
  filter(|&&b| b).count()        = 3   <- says 'how many are true'
  map(|f| f as u32).sum()        = 3   <- says 'add them up', via a cast
  map(u32::from).sum()           = 3   <- the same cast as a named conversion
  the first one is the one to write: the other two make you re-read a cast
  to find out that counting was the point.

=== part 3: what a bool cannot do, and the escape hatch ===
  `true + 1`  -> error[E0369]: cannot add `{integer}` to `bool`
  `if 1 { }`  -> error[E0308]: mismatched types, expected `bool`, found integer
  so the cast is never implicit and never accidental:
    true as u8  = 1      false as u8 = 0
    u8::from(true) = 1   u8::from(false) = 0
  and going back the other way needs a comparison, not a cast:
    1u8 as bool -> error[E0054]: cannot cast `u8` as `bool`
                   help: compare with zero instead
    1u8 != 0    = true   <- which is the question you actually meant
```
<!-- /output -->

</details>

## The verified output

<!-- output:meet_the_bool -->
*Verified output of [`meet_the_bool.rs`](examples/meet_the_bool.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
=== two values, one byte ===
  size_of::<bool>()         = 1   <- one bit of information, one byte of space
  size_of::<Option<bool>>() = 1   <- None hides in one of the 254 unused patterns
  size_of::<[bool; 8]>()    = 8   <- eight bools are eight BYTES, not one
  true as u8  = 1
  false as u8 = 0

=== bool is not a number, so you say the conversion out loud ===
  approvals            = [true, false, true, true, false]
  filter(..).count()   = 3
  map(|b| b as u32).sum() = 3
  map(u32::from).sum()    = 3   <- the same cast, spelled as a conversion

=== false < true, so bools sort and max ===
  sorted               = [false, false, true, true]   <- false first
  false < true         = true
  approvals.iter().any(|&b| b) = true
  approvals.iter().all(|&b| b) = false

=== && short-circuits; & does NOT, and both compile on bool ===
  false && expensive()  -> log []
  false &  expensive()  -> log ["expensive() ran"]   <- both sides always evaluated
  the values agree; only the work differs: false vs false

=== a bool can hand you a value without an if ===
  true.then(|| "counted")   = Some("counted")
  false.then(|| "counted")  = None
  true.then_some(42)        = Some(42)   <- no closure when the value is ready
  false.then_some(42)       = None

=== parsing text into one ===
   "true".parse::<bool>() = Ok(true)
  "false".parse::<bool>() = Ok(false)
   "TRUE".parse::<bool>() = Err(provided string was not `true` or `false`)
      "1".parse::<bool>() = Err(provided string was not `true` or `false`)
    "yes".parse::<bool>() = Err(provided string was not `true` or `false`)
  only the exact lowercase words parse; there is no truthiness here either
```
<!-- /output -->

## See also

- [Values](../values/README.md) — the census `bool` is one row of, and where the other primitive widths live
- [Meet the byte](../../19_Numbers/meet_the_byte/README.md) — why one bit of information costs a whole byte
- [Bit flags](../../19_Numbers/bit_flags/README.md) — packing eight bools into that one byte, on purpose
- [An enum instead of a `bool`](../../13_Enums/an_enum_instead_of_a_bool/README.md) — the next question: when two values are the wrong number
- [`if` expressions](../../25_Control_Flow/if_expressions/README.md) — the place the "must be a `bool`" rule is felt most
- [`Some` and `None`](../../17_Option_and_Result/some_and_none/README.md) — where `then` and `then_some` deliver their answer
- [`std::primitive::bool` ↗](https://doc.rust-lang.org/std/primitive.bool.html) — the full method list, which is short

## Po polsku

`bool` przechowuje jeden bit informacji w jednym bajcie pamięci — nie w jednym bicie — bo bajt jest najmniejszą jednostką, która ma własny adres. Osiem wartości logicznych w tablicy zajmuje więc osiem bajtów, a upakowanie ich w jeden bajt to [flagi bitowe](../../19_Numbers/bit_flags/README.md), czyli świadoma, osobna praca. Ciekawy jest trzeci rozmiar: `Option<bool>` też zajmuje **jeden** bajt, bo `bool` wykorzystuje tylko dwa z 256 możliwych układów bitów, więc `None` mieści się w jednym z pozostałych — to optymalizacja niszy (*niche optimization*).

Najważniejsza różnica wobec Pythona i C nazywa się brakiem „prawdziwościowości" (*truthiness*): w Ruscie **nie ma jej nigdzie**. `if n` dla liczby całkowitej to `error[E0308]: expected bool, found integer`, i ta sama reguła obowiązuje w `while`, w strażnikach `match`, w `filter` i w `assert!`. Pusty wektor nie jest fałszem, `None` nie jest fałszem, `0.0` nie jest fałszem — każdy z nich ma własne pytanie (`is_empty()`, `is_none()`, `== 0.0`) i trzeba wskazać to, o które naprawdę chodziło. `bool` nie jest też liczbą: `true + 1` to `error[E0369]`, a konwersję pisze się jawnie w obie strony — `b as u8` albo `u8::from(b)` w jedną, i `n != 0` w drugą, bo `1u8 as bool` się nie kompiluje (`E0054`, z podpowiedzią *„compare with zero instead"*).

Pułapka warta zapamiętania dotyczy operatorów. `&&` i `||` obliczają prawą stronę tylko wtedy, gdy lewa nie rozstrzyga wyniku, natomiast `&` i `|` **też działają na `bool`**, dają tę samą wartość i **zawsze obliczają obie strony**. Nic nie ostrzega: oba zapisy się kompilują i oba mają typ `bool → bool → bool`. Na strażniku, którego jedynym zadaniem jest sprawić, by prawa strona była bezpieczna do zadania — `(i < scores.len()) & (scores[i] > 0)` — pojedynczy `&` sięga po element, przed którym lewa strona miała chronić, i program panikuje. W ABAP-ie ta pomyłka jest niemożliwa, bo `AND` nie ma takiego „prawie identycznego" sąsiada.

Dla czytelnika znającego ABAP najważniejsze jest to, że **klasyczny ABAP nie ma typu logicznego**. `ABAP_BOOL` to znak długości 1 ze stałymi `abap_true = 'X'`, `abap_false = ' '` i `abap_undefined = '-'`, więc codzienne `IF lv_flag = abap_true` jest porównaniem znaku ze znakiem, a nie odczytem warunku. Stąd trzy rzeczy, które Rust nie tyle poprawia, co usuwa: porównanie przestaje być obowiązkowym rytuałem (`if flag` *jest* warunkiem); trzeci stan przestaje być łatwy do przypadkowego wpuszczenia (w ABAP-ie `abap_undefined` i niezainicjowane pole są oba „różne od `X`", więc *nieznane* po cichu zlewa się z *nie* — w Ruscie trzeci stan wymaga `Option<bool>`, który zajmuje ten sam jeden bajt i którego nie da się odczytać bez obsłużenia `None`); i znikają semantyki znakowe, przez które `'x'` to co innego niż `'X'`. Najbliższym odpowiednikiem `boolc( )` / `xsdbool( )` jest w Ruscie właśnie `b as u8` — jawna konwersja między *warunkiem* a *wartością*, bo w żadnym z tych języków nie są tym samym.

Na koniec dwie wygody: `false < true`, więc wartości logiczne się sortują i porównują jak każdy typ `Ord` (`false` idzie pierwsze), a `then_some` i `then` zamieniają warunek w `Option`, dzięki czemu sprawdzenie może wejść do łańcucha z `?` i `unwrap_or`, zamiast rozrastać się w blok `if`. Parsuje się tylko dokładnie `"true"` i `"false"` — `"TRUE"` i `"1"` zwracają `Err`, bo prawdziwościowości nie ma również tutaj.

**Szukaj po polsku:** typ logiczny · brak prawdziwościowości · `abap_bool` a `bool` · skrócone obliczanie wyrażeń logicznych · `rust if expected bool found integer` · `rust bool as u8`
