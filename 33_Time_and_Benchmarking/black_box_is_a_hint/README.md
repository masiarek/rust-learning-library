# `black_box` is a hint

**Level:** 201 · working knowledge

**One line:** `std::hint::black_box` is an identity function the optimizer is asked to treat as opaque — it keeps a *value* alive and unknown, but says nothing about how that value was computed, so a loop whose only output goes through it can still be replaced by a formula.

```rust
use std::hint::black_box;

fn main() {
    let n = black_box(1_000u64);        // the optimizer may not assume n is 1000
    let sum: u64 = (0..n).sum();
    println!("{}", black_box(sum));     // 499500
}
```

[What the optimizer does](../../20_Compilers/what_the_optimizer_does/README.md) shows why this function exists: give LLVM a loop over constants and it hands back the answer, so a timer around the loop measures nothing. This page is about what `black_box` promises in return — which is less than its reputation.

## What it promises

It is the identity function: `black_box(x)` returns `x`, and the example below checks that for a number, for a reference (the same address comes back), and inside a `const`, where the docs say it *"is treated as a no-op"*. Everything else is a request to the compiler, and [its docs ↗](https://doc.rust-lang.org/std/hint/fn.black_box.html) — stable since 1.66, `const` since 1.86 — are careful about how little of it is guaranteed:

> Note however, that black_box is only (and can only be) provided on a "best-effort" basis. The extent to which it can block optimisations may vary depending upon the platform and code-gen backend used. Programs cannot rely on black_box for correctness, beyond it behaving as the identity function.

> While not suitable in those mission-critical cases, black_box's functionality can generally be relied upon for benchmarking, and should be used there.

So: yes for a benchmark, and for nothing that has to be correct. The docs close the obvious loophole by name — *"there is no mechanism in the entire Rust language that can provide the guarantees required for constant-time cryptography"* — and add that the same holds for every other LLVM-based compiler.

## It protects a value, not the work

The sentence in those docs that matters most for benchmarking comes further down: *"black_box has no effect on how its input is treated, only its output."* The compiler must produce the value you pass in, and may not assume anything about the value you get back. How it produces the value is still its own business.

Here are two places to put the barrier in the same loop, plus a `main` that times each once:

```rust
use std::hint::black_box;
use std::time::Instant;

#[unsafe(no_mangle)]
pub fn result_only(n: u64) -> u64 {
    let mut sum = 0u64;
    for i in 0..n {
        sum += i;
    }
    black_box(sum)
}

#[unsafe(no_mangle)]
pub fn every_iteration(n: u64) -> u64 {
    let mut sum = 0u64;
    for i in 0..n {
        sum += black_box(i);
    }
    black_box(sum)
}

fn main() {
    let n = black_box(100_000_000);

    let t = Instant::now();
    let empty = t.elapsed();

    let t = Instant::now();
    result_only(n);
    let a = t.elapsed();

    let t = Instant::now();
    every_iteration(n);
    let b = t.elapsed();

    println!("empty {empty:?}   result_only {a:?}   every_iteration {b:?}");
}
```

`#[unsafe(no_mangle)]` only keeps the two names readable in the assembly. `result_only` passes the finished sum through the barrier, and nothing else. LLVM has to produce that sum, and it knows a faster way than adding: 0 + 1 + … + (n − 1) is n(n − 1)/2.

```text title="rustc 1.98.0 -C opt-level=3 --emit asm sums.rs, x86_64-apple-darwin — abridged: one function, .cfi lines removed"
_result_only:
	testq	%rdi, %rdi
	je	LBB6_1
	leaq	-1(%rdi), %rax
	leaq	-2(%rdi), %rcx
	mulq	%rcx
	shldq	$63, %rax, %rdx
	leaq	-1(%rdi,%rdx), %rax
	jmp	LBB6_3
LBB6_1:
	xorl	%eax, %eax
LBB6_3:
	pushq	%rbp
	movq	%rsp, %rbp
	movq	%rax, -8(%rbp)
	leaq	-8(%rbp), %rax
	## InlineAsm Start
	## InlineAsm End
	movq	-8(%rbp), %rax
	popq	%rbp
	retq
```

No loop. A multiply (`mulq`), a shift (`shldq`) and an add compute the sum directly. The empty `InlineAsm` block is `black_box` itself: the sum is stored to memory, its address is handed to an assembly block that does nothing, and the compiler has to assume it did something, so the sum must exist. The loop that was supposed to produce it does not have to.

`every_iteration` passes each `i` through the barrier, so the compiler can no longer know which numbers are being added, and the loop survives — `jne LBB5_2` is the jump back to its top:

```text title="the same command — abridged: one function, .cfi lines removed"
_every_iteration:
	pushq	%rbp
	movq	%rsp, %rbp
	xorl	%eax, %eax
	testq	%rdi, %rdi
	je	LBB5_3
	leaq	-16(%rbp), %rcx
	xorl	%edx, %edx
	.p2align	4
LBB5_2:
	movq	%rdx, -16(%rbp)
	incq	%rdx
	## InlineAsm Start
	## InlineAsm End
	addq	-16(%rbp), %rax
	cmpq	%rdx, %rdi
	jne	LBB5_2
LBB5_3:
	movq	%rax, -8(%rbp)
	leaq	-8(%rbp), %rax
	## InlineAsm Start
	## InlineAsm End
	movq	-8(%rbp), %rax
	popq	%rbp
	retq
```

Timed with `n` at a hundred million, the two differ by nearly six orders of magnitude, and the fast one is not the answer to *"how long do a hundred million additions take?"*:

```text title="Real runs — sums.rs, rustc 1.98.0 -C opt-level=3, Intel Core i5-10500, macOS 26.6.2, 2026-09-10 — five runs"
empty 74ns   result_only 43ns   every_iteration 38.369571ms
empty 83ns   result_only 43ns   every_iteration 35.337017ms
empty 78ns   result_only 48ns   every_iteration 34.989951ms
empty 93ns   result_only 41ns   every_iteration 34.879396ms
empty 85ns   result_only 41ns   every_iteration 35.45852ms
```

`empty` is an `Instant::now()` followed by `elapsed()` with nothing in between: `result_only` costs no more than asking the time.

## The same experiment in C++

LLVM does the same to C++; GCC does not. Measured on this Mac by reading the assembly, with Google Benchmark's barrier (below) copied into the C++ file:

| Compiler | Barrier on the result only | Barrier on every iteration |
|---|---|---|
| rustc 1.98.0 (LLVM), `-C opt-level=3` | formula, no loop | loop |
| Apple clang 21.0.0 (LLVM), `-O2` | formula, no loop | loop |
| GCC 15.2.0, `-O2` | loop | loop |

So *"does the work in my benchmark survive?"* is not a question about the language. It is a question about one compiler at one optimization level, and the same barrier on the result alone gets one answer from the two LLVM compilers and another from GCC. The [C++ twin of this page ↗](https://masiarek.github.io/cpp-learning-library/01_Time_and_Benchmarking/the_optimizer_deletes_your_benchmark/index.html) times all of them.

## The standardisation gap

Rust has this barrier in its standard library. C++ does not, and the talk's slide on the subject lists `std::hint::black_box` for Rust, `mem.doNotOptimizeAway` for Zig, and an ellipsis for C++. Two proposals tried, both in 2016:

- [P0342R0 ↗](https://wg21.link/p0342r0), *Timing barriers* (Mike Spertus), proposed `std::timing_fence()`, *"a hint that code should not be moved from one side of the fence to the other"* — a barrier for **where** work happens relative to the clock reads. The talk lists it as rejected.
- [P0412R0 ↗](https://wg21.link/p0412r0), *Benchmarking Primitives* (Mikhail Maltsev), proposed `keep()` and `touch()`, which treat their argument as if its bytes were written to, or read from, an unspecified device — the two halves of `black_box`. The talk lists it as stalled at its first revision.

So C++ code borrows a library's barrier, such as [Google Benchmark's `DoNotOptimize` ↗](https://github.com/google/benchmark/blob/main/include/benchmark/utils.h). On GCC and Clang it is an empty inline-assembly statement that claims to read the value and to touch all of memory:

```cpp
asm volatile("" : : "r,m"(value) : "memory");
```

That is the same trick as the `InlineAsm` block in the listings above. Inline assembly in that syntax is a GCC and Clang extension, so MSVC gets a different branch: the value's address, cast to `char const volatile*`, passed to a function defined out of line, where the optimizer cannot see what happens to it. Two notes from the current source: that exact line is the `const&` overload, now deprecated because it *"can permit undesired compiler optimizations"*, in favour of a read-write form (`"+r,m"`); and all of it is one library's convention, not a promise of the language.

## The verified output

<!-- output:black_box_is_a_hint -->
*Verified output of [`black_box_is_a_hint.rs`](examples/black_box_is_a_hint.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. To the program, black_box is the identity function
   black_box(55)          = 55
   black_box(&v) is &v    = true
   const ANSWER           = 42

2. Where it goes changes the machine code, never the answer
   result_only(1_000)     = 499500
   every_iteration(1_000) = 499500
   same answer            = true
```
<!-- /output -->

Both functions return 499500 for `n` = 1000, as they must. This example is built without optimization, like every example in this library, so what the barrier does to the machine code cannot show up in its output. That half is the assembly above.

## If you are coming from another language

- **C++.** The two sections above: no standard barrier, and Google Benchmark's `DoNotOptimize` as the working substitute. Everything this page says about `black_box` — it protects a value, not the work — holds for `DoNotOptimize` too; the table is that claim, measured. The C++ side of this page is [The optimizer deletes your benchmark ↗](https://masiarek.github.io/cpp-learning-library/01_Time_and_Benchmarking/the_optimizer_deletes_your_benchmark/index.html).
- **Python.** There is nothing to defend against. CPython runs its bytecode close to as written: it folds constant expressions such as `2 * 3` into `6` when it compiles, but it keeps a loop whose result nobody uses — `dis` shows the `FOR_ITER` and the `JUMP_BACKWARD` still there (Python 3.14.7). The trap runs the other way: a Python timing mostly measures the interpreter, and `timeit` never needs a `black_box`.

## See also

- [What the optimizer does](../../20_Compilers/what_the_optimizer_does/README.md) — constant folding, and the example this page builds on
- [Timing a block](../timing_a_block/README.md) — what you wrap in `black_box` in the first place
- [LLVM and its IR](../../20_Compilers/llvm_and_its_ir/README.md) — the optimizer rustc and clang both hand their code to
- [Scale the denominator away](../../09_Advanced/scaled_integers/README.md) — a benchmark in this library that timed at zero until `black_box` went in

## Po polsku

`std::hint::black_box` to **funkcja tożsamościowa** (*identity function*) — zwraca dokładnie to, co dostała — z jedną podpowiedzią (*hint*) dla optymalizatora: ma traktować tę wartość jak nieprzejrzystą. Dokumentacja zastrzega, że działa to na zasadzie „najlepszych starań” (*best-effort*): na `black_box` nie wolno opierać poprawności programu, można za to — i należy — używać go w benchmarkach. Do kryptografii o stałym czasie działania (*constant-time*) się nie nadaje; takiego mechanizmu nie ma ani w Ruscie, ani w LLVM.

Najważniejsze zdanie tej strony: `black_box` chroni **wartość**, a nie **pracę**, która do niej doprowadziła. Jeśli przez barierę przechodzi tylko gotowa suma, LLVM ma prawo policzyć ją inaczej — i robi to: pętlę 0 + 1 + … + (n − 1) zastępuje **wzorem zamkniętym** n(n − 1)/2, więc benchmark mierzy jedno mnożenie zamiast stu milionów dodawań. Dopiero bariera w każdej iteracji zachowuje pętlę. Clang robi z C++ to samo, bo to ten sam LLVM, a GCC 15.2 pętlę zostawia — to, czy praca przetrwa, zależy więc od kompilatora, a nie od języka.

W C++ nie ma tego w bibliotece standardowej wcale. Dwie propozycje z 2016 roku — `timing_fence()` (P0342R0) oraz `keep()` i `touch()` (P0412R0) — nie weszły do normy, więc używa się bibliotecznego `DoNotOptimize` z Google Benchmark: pustej wstawki asemblerowej (*inline assembly*). MSVC tej składni nie obsługuje, więc `DoNotOptimize` ma dla niego osobną gałąź kodu.

**Szukaj po polsku:** eliminacja martwego kodu · optymalizator a benchmark · `rust black_box` · `google benchmark DoNotOptimize` · `P0412 keep touch`
