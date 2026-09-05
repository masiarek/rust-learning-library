# Collections

**One line:** The half-dozen types every Rust program is actually made of — a pair, a fixed block, a growable list, a lookup table, a set, and one value moved to the heap.

Two of them are built into the language and have no `use` line: the tuple and the array. The rest live in `std::collections` or the prelude, and all of them share one design decision worth noticing early — **the operation that can fail returns an `Option` and the operation that asserts panics**, consistently, from `slice.get(i)` to `map.get(k)`. Learn that pair once and the whole module reads the same way.

| Lesson | Level | What it covers |
|---|---|---|
| [Tuples](tuples/README.md) | 101 | A struct with numbered fields — free to return, and readable for about two of them |
| [Arrays and slices](arrays_and_slices/README.md) | 101 → 201 | `[T; N]` is a type per length; `&[T]` is the one that belongs in a signature |
| [`Vec`](the_vec/README.md) | 101 → 201 | Three numbers on the stack, doubling growth you can watch, and two removals with opposite guarantees |
| [Array or `Vec`?](array_or_vec/README.md) | 101 → 201 | The default is `Vec` because most lengths come from input — plus the four things an array buys when the length is a fact about the problem: a compile error for the wrong one, `Copy`, a `const`, and no allocator |
| [What a `Vec` guarantees](vec_guarantees/README.md) | 301 | The promises std makes about the representation itself — never null, never inline, capacity exact to the element — and the three it pointedly does not make |
| [Grids and nested `Vec`s](vec_of_vecs/README.md) | 201 | One allocation per row, rows that are clones rather than aliases, and the `*` that `iter_mut` needs |
| [`HashMap`](the_hashmap/README.md) | 101 → 201 | `entry` is the method the counting loop wants, and the iteration order is different every run |
| [`HashSet`](the_hashset/README.md) | 101 → 201 | Membership, four set operations, and the `bool` that `insert` hands back |
| [`BTreeMap` and `BTreeSet`](sorted_collections/README.md) | 101 → 201 | The sorted pair: collecting into one *is* the sort, `range` asks what a hash map cannot, and it orders by the key when what you wanted sorted was the value |
| [`Box`](the_box/README.md) | 201 | One value on the heap: a type that contains itself, and a size known only at run time |
| [`Vec` methods](vec_methods/README.md) | reference | One page per method — all 46 on stable, plus the three `IntoIterator` impls |
| [`slice` methods](slice_methods/README.md) | reference | One page per slice method the `Vec` reference points at — the 22 reached through `Deref`, from `sort` to `join` |

## Which one

| You have | You want |
|---|---|
| two or three values of *different* types, used right here | a **tuple** |
| a fixed number of values, known when you compile | an **array**, `[T; N]` |
| a list that grows | a **`Vec`** |
| part of any of those, passed to a function | a **slice**, `&[T]` |
| a grid, rows all the same length | a **flat `Vec`** and a width — see [grids](vec_of_vecs/README.md) |
| a grid whose rows differ in length | a **`Vec<Vec<T>>`** |
| a key that finds a value | a **`HashMap`** — or a **[`BTreeMap`](sorted_collections/README.md)** if you need it sorted |
| "have I seen this before?" | a **`HashSet`** — or a **[`BTreeSet`](sorted_collections/README.md)** to get the answers back in order |
| one value too big for the stack, or a type that contains itself | a **`Box`** |

`String` and `&str` are the seventh and eighth entries in that table, and they have [a section of their own](../14_Strings/README.md).

Once you have picked one, the **[`Vec` method reference](vec_methods/README.md)** is the per-method companion to the `Vec` lesson: a page each for `drain`, `retain`, `swap_remove`, `splice` and the other 42, every one with a program CI compiles and runs. It also answers the question the lesson raises and does not settle — why `sort` and `iter` are not on it. (They are slice methods, reached through `Deref` — and the **[`slice` method reference](slice_methods/README.md)** gives each of those 22 a page of the same shape.)

## What is deliberately not here

`Rc` and `Arc` are boxes with a reference count, and they belong to the ownership story rather than this one: [`Rc`](../18_Ownership/reference_counting/README.md), [`Arc`](../18_Ownership/sharing_across_threads/README.md). `VecDeque`, `BinaryHeap` and `LinkedList` are real and rarely the answer — the [`std::collections` ↗](https://doc.rust-lang.org/std/collections/index.html) module page opens with a decision table that covers them in about a screen, and it is better than anything this library would add.

## Where it goes next

Every one of these is iterable, and the chain you build over them is [iterators](../24_Iterators/README.md). Every one of them is generic, which is [what `<T>` means](../22_Generics/README.md). And the reason `map.get` hands you an `Option` rather than a value is [the whole of `Option` and `Result`](../17_Option_and_Result/README.md).

## Po polsku

Pół tuzina typów, z których faktycznie zbudowany jest każdy program: para (`(A, B)`), stały blok (`[T; N]`), rosnąca lista (`Vec<T>`), tablica skojarzeniowa (`HashMap<K, V>`), zbiór (`HashSet<T>`) i jedna wartość przeniesiona na stertę (`Box<T>`).

Warto od razu ustawić dwa polskie słowa, bo bywają mylone. **Tablica** to w Ruscie `[T; N]` — o stałym, znanym w czasie kompilacji rozmiarze, leżąca na stosie. To, co w Pythonie nazywa się listą, a w Javie `ArrayList`, to tutaj `Vec<T>`, po polsku zwykle „wektor". Kto przenosi nawyk z C, gdzie „tablica" bywa dynamiczna przez `malloc`, będzie sięgał po złe słowo i po zły typ. Podobnie `HashMap` to **tablica skojarzeniowa** albo po prostu „mapa" — nie „słownik", choć znaczy to samo co pythonowy `dict`.

Rzecz, której w tym dziale nie ma i to celowo: nie ma tu listy wiązanej jako typu, po który się sięga. `LinkedList` istnieje w bibliotece standardowej i prawie nigdy nie jest właściwą odpowiedzią — polskie kursy algorytmiki poświęcają jej dużo miejsca, co zostawia wrażenie, że jest podstawowym narzędziem. W praktyce `Vec` wygrywa niemal zawsze, również tam, gdzie teoria obiecuje inaczej, bo pamięć podręczna procesora lubi ciągły blok.

**Szukaj po polsku:** kolekcje w Ruscie · tablica a wektor · tablica skojarzeniowa · `rust Vec vs array` · `rust LinkedList why not`
