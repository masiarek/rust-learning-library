# What `&'a T` claims

**Level:** 201 → 301 · working knowledge

**One line:** `r: &'a T` makes three claims at once — `r` holds the address of a `T`, everything `r` ever points at stays valid for the whole of `'a`, and `r` itself is not used beyond `'a` — and it is the third that decides what you are allowed to assign *into* `r`.

```rust
let outer = String::from("outer");
let mut r: &str = &outer;      // claim: whatever `r` points at outlives every use of `r`
println!("{r}");
```

[Lifetime annotations](../lifetime_annotations/README.md) is about writing `<'a>` in a signature. This page is about what the lifetime means once it is sitting in a *type*, because `'a` is not a decoration on `&T` — it is part of the type, and it behaves like one: it constrains what may be assigned in, and the compiler will silently adjust it in one direction and refuse in the other.

---

## Three claims, not one

Read `r: &'a S` as three separate promises:

| The claim | What it constrains |
|---|---|
| `r` holds an address of some `S` | the **type** of the referent |
| any address `r` holds is valid for all of `'a` | the **referent** — it must live at least that long |
| `r` is not used outside `'a` | **`r` itself** — the reference may not outlive its own region |

The second and third pull in opposite directions, and every lifetime error you will meet is one of them failing. `E0597` — *"does not live long enough"* — is the second. `E0106` — *"missing lifetime specifier"* — is the compiler refusing to guess which `'a` the third should be.

Note what is *not* on that list: nothing says `'a` is how long `r` exists, and nothing grants anyone extra life. `'a` is a region of code the compiler works out; the annotation only ever states a requirement it must then verify.

## A longer lifetime goes where a shorter one is wanted

One `'a` covering two arguments does not force them to be equal:

```rust
fn longer_of<'a>(x: &'a str, y: &'a str) -> &'a str { … }

let forever: &'static str = "a literal";
let borrowed = String::from("a local String");
println!("{}", longer_of(forever, &borrowed));   // compiles
```

`'static` and the local's region are not the same region, and the call is still fine. The compiler did not extend the local — it **narrowed the literal**, treating `&'static str` as a `&'a str` for the length of the call. A reference that lives longer is usable anywhere a shorter one is wanted, exactly as a more specific type is usable where a general one is wanted. That is the one direction that is free, and it is why `&'static str` arguments never need special handling.

## What may be assigned into a reference

Because the lifetime is part of the type, assigning to a reference variable is type-checked like any other assignment — against every *later* use of that variable:

```rust
let outer = String::from("outer");
let mut r: &str = &outer;
{
    let inner = String::from("inner");
    r = &inner;             // fine — as long as `r` is not read after this block
    println!("{r}");
}
r = &outer;                 // always fine: `outer` outlives every use of `r`
println!("{r}");
```

Keep the `r = &inner` line and move the final read below the brace, and the same program is refused:

```text
error[E0597]: `inner` does not live long enough
  --> assigned_in_must_outlive.rs:7:13
   |
 6 |         let inner = String::from("inner");
   |             ----- binding `inner` declared here
 7 |         r = &inner;
   |             ^^^^^^ borrowed value does not live long enough
 8 |         println!("{r}");
 9 |     }
   |     - `inner` dropped here while still borrowed
10 |     println!("{r}");
   |                - borrow later used here
```

The rule is one sentence: **whatever you assign in must outlive every later use of the variable.** Not the block, not the declaration — the later *uses*. Which is why the identical `r = &inner;` line is accepted in one program and refused in another that differs only below it.

## The direction reverses behind `&mut`

The free narrowing in section 2 stops at `&mut`. `&'a T` is *covariant* in `'a` — a longer one substitutes for a shorter. `&mut &'a T` is **invariant**: no substitution at all, in either direction.

```rust
fn reassign<'a>(slot: &mut &'a str, value: &'a str) { *slot = value; }
```

The reason is what writing through the pointer would let you do. If `'a` could be narrowed here, you could pass a `&mut &'static str` and a short-lived local, and the function would store the local into a slot whose type promises `'static`. The caller would then be holding a dangling reference with a perfectly good-looking type. So the compiler refuses at the call instead:

```text
error[E0597]: `local` does not live long enough
  --> invariance_behind_mut.rs:9:29
   |
 8 |         let local = String::from("a local String");
   |             ----- binding `local` declared here
 9 |         reassign(&mut slot, &local);
   |                             ^^^^^^ borrowed value does not live long enough
10 |     }
   |     - `local` dropped here while still borrowed
11 |     println!("{slot}");
   |                ---- borrow later used here
```

The same asymmetry is the whole story behind returning references *out of* a `&mut`: handing back a long-lived reference obtained through an exclusive borrow would leave two live paths to one value, one of them mutable, which is the rule [borrowing](../borrowing/README.md) exists to enforce. Returning a reference no longer than the borrow you were given is always fine; returning one that outlives it is refused, and this is why.

## The whole verified run

<!-- output:what_a_reference_claims -->
*Verified output of [`what_a_reference_claims.rs`](examples/what_a_reference_claims.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
──── 1. One name, two different lifetimes, and it still compiles
  longer_of(forever, &borrowed) = a string literal, alive for the whole program
  `'a` was asked to cover both. The compiler did not widen the local —
  it NARROWED the literal, because a longer lifetime is usable wherever
  a shorter one is wanted. That direction is the whole rule.

──── 2. What may be assigned INTO a reference
  r = outer — declared first, so it outlives `inner`
  r = inner — declared inside the block
     ^ legal, because `r` is not read below this block.
  r = outer — declared first, so it outlives `inner`
     ^ always legal: `outer` outlives every region `r` is used in.
  Keep the `r = &inner` line and read `r` after the brace and it is
  E0597 — the value assigned in must outlive every LATER use of `r`.

──── 3. The direction reverses behind `&mut`
  slot = initially a literal
  slot = outer — declared first, so it outlives `inner`
  `&mut &'a str` is INVARIANT in 'a: the compiler will not quietly
  shorten `'a` here the way it did in section 1, because writing
  through the pointer could leave the caller holding a shorter
  reference than its own type promises. This call is fine only
  because `outer` genuinely outlives `slot`'s last use.
```
<!-- /output -->

## If you are coming from another language

- **Python.** Nothing in the type system corresponds to this, but the *failure* does, and it is worth naming because Python hides it well. A reference into a structure that later changes is fine in Python precisely because the garbage collector keeps the referent alive — which is a different guarantee, not a stronger one: your object survives, but the container it belonged to may no longer contain it, so a cached `row = table[0]` can go quietly stale rather than dangling. Rust's `'a` is the machinery that lets it drop values deterministically and still refuse the stale read. If you have written `weakref` code and thought about whether the referent is still there, that is the closest Python gets to holding a lifetime in your head — and Rust's compiler is doing that reasoning for you, always, for every reference.
- **ABAP.** Reference variables (`REF TO`) are counted, so the dangling case does not arise, and the honest bridge is again `FIELD-SYMBOLS`, which is not counted and can dangle. What has no ABAP counterpart at all is the *variance* half of this page: the idea that a value of one type is silently acceptable where another is wanted, in one direction only. The nearest familiar shape is the assignment rules between a subtype and its supertype in a class hierarchy — a `REF TO cl_child` may be assigned to a `REF TO cl_parent` and not the reverse, and the widening cast is the free direction. `&'static str` into `&'a str` is that same one-way substitution, with "lives longer" playing the role of "is more specific".
- **C++.** A reference has no lifetime in the type at all, which is the entire difference: `const std::string&` binding to a temporary, and the dangling reference that follows when the temporary dies, is a bug C++ can only warn about heuristically. The C++ reader's real leverage is elsewhere — **variance is already familiar from templates**. `std::shared_ptr<Derived>` converts to `std::shared_ptr<Base>` but `std::shared_ptr<Derived>*` does not convert to `std::shared_ptr<Base>*`, for exactly the reason section 4 gives: writing through the pointer would break the type. Rust's `&'a T` versus `&mut &'a T` is that same rule, with lifetimes in the place of class hierarchies, and enforced rather than left to convention.

## See also

- [Lifetime annotations](../lifetime_annotations/README.md) — how to write `<'a>`, what `E0106` is asking, and the three elision rules
- [What a lifetime does at the call site](../lifetimes_at_the_call_site/README.md) — what these claims cost the *caller*
- [Borrowed state](../borrowed_state/README.md) — the lock the second claim puts on the referent
- [`&'static str`](../../14_Strings/static_str/README.md) — the longest lifetime, and why `T: 'static` does not mean "lives forever"
- [Generics](../../22_Generics/README.md) — where the type-parameter half of this analogy is made properly

## Po polsku

Zapis `r: &'a T` to **trzy obietnice naraz**: `r` przechowuje adres jakiegoś `T`; wszystko, na co `r` kiedykolwiek wskazuje, pozostaje ważne przez cały `'a`; oraz sam `r` nie jest używany poza `'a`. Druga i trzecia ciągną w przeciwne strony i każdy błąd czasu życia to porażka jednej z nich.

Czego na tej liście **nie ma**: nigdzie nie jest napisane, że `'a` to czas istnienia zmiennej `r`, i nic nikomu nie przedłuża życia. `'a` to obszar kodu, który kompilator sam wyznacza — adnotacja jedynie formułuje wymaganie, które potem trzeba spełnić.

Najważniejsza reguła praktyczna dotyczy **przypisania do referencji**: to, co przypisujesz, musi żyć dłużej niż każde *późniejsze użycie* tej zmiennej. Nie dłużej niż blok, nie dłużej niż deklaracja — dłużej niż użycia. Dlatego identyczna linia `r = &inner;` bywa przyjęta w jednym programie i odrzucona w drugim, który różni się wyłącznie tym, co jest *pod* nią.

Referencja żyjąca dłużej może wystąpić wszędzie tam, gdzie oczekiwana jest krótsza (`&'static str` w miejsce `&'a str`) — to jedyny darmowy kierunek, zwany kowariancją. Za `&mut` ten kierunek znika: `&mut &'a T` jest **inwariantne**, bo zapis przez taki wskaźnik pozwoliłby wstawić krótko żyjącą wartość do miejsca, którego typ obiecuje `'static`. Analogia dla znających C++: `shared_ptr<Derived>` konwertuje się do `shared_ptr<Base>`, ale `shared_ptr<Derived>*` już nie — z dokładnie tego samego powodu.

**Szukaj po polsku:** czasy życia w Ruscie · wariancja · kowariancja i inwariancja · `rust lifetimes` · `rust variance` · `rust E0597`
