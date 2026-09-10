# `HashSet`

**Level:** 101 → 201 · for newcomers

**One line:** A `HashSet<T>` is a `HashMap<T, ()>` — membership, uniqueness, and four set operations — and `insert` returns the `bool` that answers the question you were about to ask separately.

```rust
use std::collections::HashSet;

fn main() {
    let mut voted: HashSet<&str> = HashSet::new();
    for name in ["Ada", "Ben", "Ada"] {
        if !voted.insert(name) {
            println!("{name} has already voted");   // Ada has already voted
        }
    }
}
```

`insert` is `true` when the value was **not** already there. Duplicate detection is therefore one line and one lookup, not a `contains` followed by an `insert`.

## The four operations

```rust
use std::collections::HashSet;

fn main() {
    let round1: HashSet<&str> = ["Ada", "Ben", "Cara"].into_iter().collect();
    let round2: HashSet<&str> = ["Ben", "Cara", "Dan"].into_iter().collect();
    let mut both: Vec<&str> = round1.intersection(&round2).copied().collect();
    both.sort();
    println!("{both:?}");   // ["Ben", "Cara"]
}
```

| method | operator | round1 vs round2 |
|---|---|---|
| [`union` ↗](https://doc.rust-lang.org/std/collections/hash_set/struct.HashSet.html#method.union) | `\|` | `["Ada", "Ben", "Cara", "Dan"]` |
| [`intersection` ↗](https://doc.rust-lang.org/std/collections/hash_set/struct.HashSet.html#method.intersection) | `&` | `["Ben", "Cara"]` |
| [`difference` ↗](https://doc.rust-lang.org/std/collections/hash_set/struct.HashSet.html#method.difference) | `-` | `["Ada"]` |
| [`symmetric_difference` ↗](https://doc.rust-lang.org/std/collections/hash_set/struct.HashSet.html#method.symmetric_difference) | `^` | `["Ada", "Dan"]` |

The methods return **iterators** and allocate nothing until you `collect`; the operators build a new `HashSet` directly. And `is_subset`, `is_superset` and `is_disjoint` answer the yes/no versions without building anything at all.

## `difference` is not symmetric

`a.difference(&b)` is *in a, not in b*. Swapping the arguments asks a completely different question, and reading it as "the difference between the sets" is how the two get confused — the practice below turns that into two genuinely different answers about the same election. The symmetric one has its own name for exactly this reason.

## Order is thrown away, and can be kept beside it

A `HashSet` has no order, per-run or otherwise, for the same reason a [`HashMap`](../the_hashmap/README.md) does not. Collecting into one deduplicates and shuffles in a single step. When you want *first-seen* order, keep a `Vec` beside the set and let `insert`'s bool decide whether to push:

```rust
use std::collections::HashSet;

fn main() {
    let mut kept: Vec<&str> = Vec::new();
    let mut seen: HashSet<&str> = HashSet::new();
    for name in ["Cara", "Ada", "Cara", "Ben"] {
        if seen.insert(name) { kept.push(name); }
    }
    println!("{kept:?}");   // ["Cara", "Ada", "Ben"]
}
```

`BTreeSet` is the other answer: sorted by construction, `Ord` instead of `Hash`, O(log *n*) instead of O(1), and a printout that is the same on every run.

## The trap: `len()` difference counts ballots, not people

`received.len() - distinct.len()` tells you how many surplus items arrived. It does **not** tell you how many keys were duplicated — three ballots from one person contribute 2 to that number, and the set alone can never tell you which person. For *who*, you need the `insert` bool as it happens, or a count per key in a `HashMap`.

## If you are coming from another language

- **Python.** `set`, and the operators are the same characters: `|`, `&`, `-`, `^`, plus `<=` for subset. Two differences to carry across. Python's `s.add(x)` returns `None` while Rust's `insert` returns the bool, so the `if x in s: … else: s.add(x)` two-step that Python teaches is a habit to drop here. And Python sets are unordered but iterate the same way within a process for a given insertion history, which tempts people into depending on it; Rust randomises per process, so the dependency breaks immediately rather than in production. `frozenset` has no direct counterpart — an immutable set in Rust is just a `HashSet` you do not have a `&mut` to, which the borrow checker enforces for free.
- **ABAP.** The nearest thing is a `HASHED TABLE … WITH UNIQUE KEY` whose row is only the key, and the closest idiom to `insert`'s bool is `INSERT … INTO TABLE` followed by checking `sy-subrc` — 0 for inserted, 4 for already present. That is the same one-lookup pattern this page is about, so the habit transfers exactly. What ABAP does not have is set algebra as an operation: intersecting two internal tables means a `LOOP` with a `READ TABLE … WITH TABLE KEY` inside it, which is what `intersection` compiles to anyway but with the loop written by hand each time. The `FOR … IN … WHERE` table comprehension in newer releases gets closer, but there is still no `union` you can name.
- **Java / C#.** `HashSet<T>`, with `add` returning the same boolean Rust does. Java's `retainAll`/`removeAll` mutate in place where Rust's operations return a new iterator; `TreeSet` is `BTreeSet`.

---

## The verified output

<!-- output:the_hashset -->
*Verified output of [`the_hashset.rs`](examples/the_hashset.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. A set is a map with nothing on the right-hand side
   HashSet<T> is literally HashMap<T, ()> underneath, so the same
   rules apply: T: Eq + Hash, and no defined iteration order.

2. `insert` answers the question you were about to ask
   insert("Ada") -> true
   insert("Ben") -> true
   insert("Ada") -> false   <- already voted
   insert("Cara") -> true
   insert("Ben") -> false   <- already voted
   set: ["Ada", "Ben", "Cara"]
   The bool is `true` when the value was NOT already there, which
   makes duplicate detection one line with no second lookup.

3. The four operations
   round1 | round2  union                 ["Ada", "Ben", "Cara", "Dan"]
   round1 & round2  intersection          ["Ben", "Cara"]
   round1 - round2  difference            ["Ada"]
   round1 ^ round2  symmetric_difference  ["Ada", "Dan"]
   Each returns an iterator, not a set — nothing is allocated until
   you collect. The operator forms (`&`, `|`, `-`, `^`) exist too and
   build a new HashSet directly.

4. Containment, both directions
   round1.contains("Ada")        = true
   round1.is_subset(&round2)     = false
   round1.is_disjoint(&round2)   = false
   core.is_subset(&round1)       = true

5. Deduplicating, and what it costs
   ["Cara", "Ada", "Cara", "Ben", "Ada", "Cara"]
   -> 3 distinct: ["Ada", "Ben", "Cara"]
   first-seen order: ["Cara", "Ada", "Ben"]
   A HashSet throws the order away. If you need it, keep a Vec beside
   the set and let `insert`'s bool decide whether to push.
```
<!-- /output -->

## Practice

**Who voted twice, who never voted, and one wrong answer.** Given a roll of eligible voters and a list of ballots received — with repeats, and one name that is not on the roll — produce three answers: the people who voted more than once, the eligible people who never voted, and the ballots from people who are not eligible.

Two of those three are the same set operation with its arguments swapped, so say which and check you have them the right way round. Then compute turnout, and say why `received.len() / roll.len()` is the wrong formula for this data.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:the_hashset_kata -->
*[`the_hashset_kata.rs`](examples/the_hashset_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: who voted twice, who never voted, and one wrong answer.
//!
//!   rustc --edition 2024 the_hashset_kata.rs -o /tmp/hsk && /tmp/hsk

use std::collections::{BTreeSet, HashSet};

const ROLL: [&str; 6] = ["Ada", "Ben", "Cara", "Dan", "Eve", "Fay"];
const RECEIVED: [&str; 8] = ["Cara", "Ada", "Ben", "Cara", "Dan", "Ada", "Cara", "Zed"];

fn sorted<'a>(s: &'a HashSet<&'a str>) -> Vec<&'a str> {
    let mut v: Vec<&str> = s.iter().copied().collect();
    v.sort();
    v
}

fn main() {
    println!("1. Three questions, one pass");
    let roll: HashSet<&str> = ROLL.into_iter().collect();
    let mut seen: HashSet<&str> = HashSet::new();
    let mut duplicates: Vec<&str> = Vec::new();
    for name in RECEIVED {
        if !seen.insert(name) {
            duplicates.push(name);
        }
    }
    duplicates.sort();
    let never: Vec<&str> = {
        let mut v: Vec<&str> = roll.difference(&seen).copied().collect();
        v.sort();
        v
    };
    let unknown: Vec<&str> = {
        let mut v: Vec<&str> = seen.difference(&roll).copied().collect();
        v.sort();
        v
    };
    println!("   roll     : {ROLL:?}");
    println!("   received : {RECEIVED:?}");
    println!("   voted twice or more : {duplicates:?}");
    println!("   on the roll, never voted : {never:?}");
    println!("   voted but not on the roll : {unknown:?}");
    println!("   The last two are the SAME operation with the arguments swapped.");
    println!("   `difference` is not symmetric, and reading it as \"the difference");
    println!("   between the sets\" is how the two get confused.");

    println!();
    println!("2. The wrong answer: counting duplicates with the set alone");
    println!("   received.len() = {}, distinct = {}, so {} extra ballots arrived.",
             RECEIVED.len(), seen.len(), RECEIVED.len() - seen.len());
    println!("   That is a count of surplus ballots, not of people: Cara sent three,");
    println!("   which is 2 of those {}. distinct-vs-total tells you HOW MANY too",
             RECEIVED.len() - seen.len());
    println!("   many, never WHO — for that you need the `insert` bool above, or a");
    println!("   count per name in a HashMap.");
    println!("   people who repeated: {} ({:?})",
             duplicates.iter().collect::<HashSet<_>>().len(),
             {
                 let mut d: Vec<&str> = duplicates.iter().copied().collect::<HashSet<_>>()
                     .into_iter().collect();
                 d.sort();
                 d
             });

    println!();
    println!("3. Turnout, as a set operation");
    let turned_out: HashSet<&str> = roll.intersection(&seen).copied().collect();
    println!("   eligible and voted: {:?}", sorted(&turned_out));
    println!("   turnout = {}/{} = {:.0}%", turned_out.len(), roll.len(),
             100.0 * turned_out.len() as f64 / roll.len() as f64);
    println!("   Note it is the intersection, not received.len(): the stray ballot");
    println!("   from Zed would otherwise inflate turnout above the electorate.");

    println!();
    println!("4. When you want the order back");
    let ordered: BTreeSet<&str> = seen.iter().copied().collect();
    println!("   BTreeSet: {:?}", ordered);
    println!("   Sorted by construction, O(log n) instead of O(1) per operation,");
    println!("   and it needs Ord rather than Hash. For six names the difference is");
    println!("   nothing and the printout is stable — which is why this library's");
    println!("   examples reach for it whenever output is the point.");
}
```
<!-- /source -->

<!-- output:the_hashset_kata -->
*Verified output of [`the_hashset_kata.rs`](examples/the_hashset_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Three questions, one pass
   roll     : ["Ada", "Ben", "Cara", "Dan", "Eve", "Fay"]
   received : ["Cara", "Ada", "Ben", "Cara", "Dan", "Ada", "Cara", "Zed"]
   voted twice or more : ["Ada", "Cara", "Cara"]
   on the roll, never voted : ["Eve", "Fay"]
   voted but not on the roll : ["Zed"]
   The last two are the SAME operation with the arguments swapped.
   `difference` is not symmetric, and reading it as "the difference
   between the sets" is how the two get confused.

2. The wrong answer: counting duplicates with the set alone
   received.len() = 8, distinct = 5, so 3 extra ballots arrived.
   That is a count of surplus ballots, not of people: Cara sent three,
   which is 2 of those 3. distinct-vs-total tells you HOW MANY too
   many, never WHO — for that you need the `insert` bool above, or a
   count per name in a HashMap.
   people who repeated: 2 (["Ada", "Cara"])

3. Turnout, as a set operation
   eligible and voted: ["Ada", "Ben", "Cara", "Dan"]
   turnout = 4/6 = 67%
   Note it is the intersection, not received.len(): the stray ballot
   from Zed would otherwise inflate turnout above the electorate.

4. When you want the order back
   BTreeSet: {"Ada", "Ben", "Cara", "Dan", "Zed"}
   Sorted by construction, O(log n) instead of O(1) per operation,
   and it needs Ord rather than Hash. For six names the difference is
   nothing and the printout is stable — which is why this library's
   examples reach for it whenever output is the point.
```
<!-- /output -->

</details>

---

**A word ladder, and the key a lookup hands back.** Write `word_ladder_length(begin, end, list)`: the number of words in the shortest chain from `begin` to `end` where each step changes one letter and every word after the first comes from `list`, or 0 when there is no chain. Search breadth-first with a `VecDeque`, and keep the unvisited words in a `HashSet<&str>` borrowed from the list. Build each candidate in one scratch buffer and look it up with `take` — then find out which `&str` comes back, the probe's or the set's, and why that lets the queue hold borrows instead of `String`s.

```rust
// rustc --edition 2024 --test word_ladder.rs -o t && ./t
fn word_ladder_length(begin_word: &str, end_word: &str, word_list: &[&str]) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_ladder() {
        assert_eq!(
            word_ladder_length("hit", "cog", &["hot", "dot", "dog", "lot", "log", "cog"]),
            5
        );

        assert_eq!(
            word_ladder_length("hit", "cog", &["hot", "dot", "dog", "lot", "log"]),
            0 // no possible transformation
        );

        assert_eq!(word_ladder_length("a", "b", &["b"]), 2);
    }
}
```

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:word_ladder_kata -->
*[`word_ladder_kata.rs`](examples/word_ladder_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: a breadth-first word ladder over a `HashSet<&str>` — the
//! set borrows the caller's words, and a lookup made with a scratch buffer
//! hands back the set's own `&str`.
//!
//!   rustc --edition 2024 word_ladder_kata.rs -o /tmp/wlk && /tmp/wlk
//!   rustc --edition 2024 --test word_ladder_kata.rs -o /tmp/wlkt && /tmp/wlkt

use std::collections::{HashSet, VecDeque};

/// Breadth first: every word one change away is tried before any word two
/// changes away, so the first time `end_word` is reached the count is the
/// shortest. Assumes lowercase ASCII words of equal length — the candidates
/// are made by overwriting one byte at a time.
fn word_ladder_length(begin_word: &str, end_word: &str, word_list: &[&str]) -> i32 {
    let mut unseen: HashSet<&str> = word_list.iter().copied().collect();
    if !unseen.contains(end_word) {
        return 0;
    }
    unseen.remove(begin_word);
    let mut frontier = VecDeque::from([(begin_word, 1)]);
    while let Some((word, steps)) = frontier.pop_front() {
        let mut buf = word.as_bytes().to_vec();
        for i in 0..buf.len() {
            let original = buf[i];
            for b in b'a'..=b'z' {
                buf[i] = b;
                let candidate = std::str::from_utf8(&buf).expect("ASCII in, ASCII out");
                // `take` looks up with the temporary and returns the stored
                // `&str`: a borrow of the caller's list, not of `buf`.
                if let Some(next) = unseen.take(candidate) {
                    if next == end_word {
                        return steps + 1;
                    }
                    frontier.push_back((next, steps + 1));
                }
            }
            buf[i] = original;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_ladder() {
        assert_eq!(
            word_ladder_length("hit", "cog", &["hot", "dot", "dog", "lot", "log", "cog"]),
            5
        );

        assert_eq!(
            word_ladder_length("hit", "cog", &["hot", "dot", "dog", "lot", "log"]),
            0 // no possible transformation
        );

        assert_eq!(word_ladder_length("a", "b", &["b"]), 2);
    }
}

fn main() {
    println!("1. Adam's three cases");
    let list = ["hot", "dot", "dog", "lot", "log", "cog"];
    let with_cog = word_ladder_length("hit", "cog", &list);
    let without = word_ladder_length("hit", "cog", &list[..5]);
    let one_letter = word_ladder_length("a", "b", &["b"]);
    assert_eq!((with_cog, without, one_letter), (5, 0, 2));
    println!("   hit -> cog, cog in the list       {with_cog}");
    println!("   hit -> cog, cog missing           {without}   (0 means no ladder)");
    println!("   a -> b                            {one_letter}");
    println!("   The count includes both ends: hit hot dot dog cog is five words.");

    println!();
    println!("2. Which &str a lookup hands back");
    let owned = vec![String::from("hot"), String::from("dot")];
    let set: HashSet<&str> = owned.iter().map(String::as_str).collect();
    let probe = String::from("hot");
    let found = set.get(probe.as_str()).expect("hot is in the set");
    println!("   the answer points into the list   {}", found.as_ptr() == owned[0].as_ptr());
    println!("   the answer points into the probe  {}", found.as_ptr() == probe.as_ptr());
    println!("   A HashSet<&str> can be searched with any &str, here one built in a");
    println!("   scratch buffer, and it answers with the key it stored. So the frontier");
    println!("   holds borrows of the caller's words, and no String is made per visit.");

    println!();
    println!("3. The assumption the one-byte overwrite makes");
    let mut bytes = "żab".as_bytes().to_vec();
    bytes[0] = b'a';
    println!("   \"żab\" is {} bytes for 3 letters; overwrite byte 0 and from_utf8 is_err = {}",
        "żab".len(),
        std::str::from_utf8(&bytes).is_err()
    );
    println!("   One byte per letter holds for Adam's words and fails for Polish ones;");
    println!("   a ladder over real words would walk a Vec<char> instead.");
}
```
<!-- /source -->

<!-- output:word_ladder_kata -->
*Verified output of [`word_ladder_kata.rs`](examples/word_ladder_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. Adam's three cases
   hit -> cog, cog in the list       5
   hit -> cog, cog missing           0   (0 means no ladder)
   a -> b                            2
   The count includes both ends: hit hot dot dog cog is five words.

2. Which &str a lookup hands back
   the answer points into the list   true
   the answer points into the probe  false
   A HashSet<&str> can be searched with any &str, here one built in a
   scratch buffer, and it answers with the key it stored. So the frontier
   holds borrows of the caller's words, and no String is made per visit.

3. The assumption the one-byte overwrite makes
   "żab" is 4 bytes for 3 letters; overwrite byte 0 and from_utf8 is_err = true
   One byte per letter holds for Adam's words and fails for Polish ones;
   a ladder over real words would walk a Vec<char> instead.
```
<!-- /output -->

</details>

## See also

- [`HashMap`](../the_hashmap/README.md) — the same table with a value on the right, and where `Eq + Hash` is explained
- [`Vec`](../the_vec/README.md) — where the order goes when you need it back
- [Iterators are lazy](../../24_Iterators/iterators_are_lazy/README.md) — why `union` costs nothing until you `collect`
- [Marker traits](../../12_Traits/marker_traits/README.md) — `Eq` and the contract it signs with `Hash`

## Sources

[Std library types: HashSet ↗](https://doc.rust-lang.org/rust-by-example/std/hash/hashset.html) in Rust by Example, and [`std::collections::HashSet` ↗](https://doc.rust-lang.org/std/collections/struct.HashSet.html), which documents the `HashMap<T, ()>` implementation this page opens with.

## Po polsku

`HashSet<T>` to dosłownie `HashMap<T, ()>` — ta sama tablica mieszająca, tylko z pustą prawą stroną — więc po polsku wygodnie mówić o nim po prostu „zbiór”, tak jak na lekcjach matematyki. Obowiązują te same warunki (`T: Eq + Hash`) i ten sam brak ustalonej kolejności. Rzecz, którą trzeba zapamiętać na pierwszym miejscu, jest jednak inna: `insert` zwraca `bool`, i to `true` znaczy „wstawiono, bo tego jeszcze nie było”. Wykrycie duplikatu to więc jedna linia i **jedno** przeszukanie — nawyk „najpierw `contains`, potem `insert`” trzeba tu porzucić.

Cztery operacje na zbiorach mają w polszczyźnie gotowe, szkolne nazwy i warto się nimi posługiwać, bo lepiej trzymają znaczenie niż angielskie: `union` to suma (`A ∪ B`), `intersection` to część wspólna (`A ∩ B`), `difference` to różnica (`A \ B`), a `symmetric_difference` to różnica symetryczna. Ta polska nomenklatura od razu rozbraja pułapkę tej strony: **`difference` nie jest symetryczna**, co po polsku widać w samym zapisie `A \ B`, podczas gdy angielskie *„the difference between the sets”* podpowiada błędnie, że kolejność argumentów nic nie zmienia. W ćwiczeniu z tej strony są to dwa zupełnie różne pytania o te same dane: `roll.difference(&seen)` to uprawnieni, którzy nie zagłosowali (`["Eve", "Fay"]`), a `seen.difference(&roll)` to głosy spoza listy (`["Zed"]`). Warto też wiedzieć, że wersje metodowe zwracają **iteratory** i nie alokują niczego aż do `collect`, wersje operatorowe (`|`, `&`, `-`, `^`) budują nowy zbiór od razu, a `is_subset`, `is_superset` i `is_disjoint` odpowiadają „tak/nie” bez budowania czegokolwiek.

Zostają dwie konsekwencje, o które łatwo się potknąć. Pierwsza: zbiór **wyrzuca kolejność**, więc `collect` do `HashSet` jednocześnie usuwa duplikaty i miesza wynik. Jeśli zależy ci na kolejności pierwszego wystąpienia, trzymaj obok zbioru zwykły `Vec` i pozwól, żeby to `bool` z `insert` decydował o `push`; jeśli chcesz porządku alfabetycznego i powtarzalnego wydruku — weź `BTreeSet`, który wymaga `Ord` zamiast `Hash` i sortuje się z definicji. Druga: różnica liczebności mówi *ile*, nigdy *kto*. `received.len() - distinct.len()`, czyli 8 − 5 = 3, to liczba nadmiarowych kart, a nie liczba osób — Cara oddała trzy karty i wnosi do tej trójki 2, a powtarzających się wyborców było dwoje. Na pytanie „kto” odpowiada tylko `bool` przechwycony w chwili wstawiania albo licznik na klucz w `HashMap`.

**Szukaj po polsku:** operacje na zbiorach · różnica symetryczna · `rust HashSet insert returns bool` · `rust HashSet vs BTreeSet` · `rust set union intersection difference`
