# Drawing `sret`: the room the caller lends

**Level:** 201 → 301 · working knowledge · the drawing for [Returned by value](../returned_by_value/README.md)

**One line:** For a return value too big for registers, the caller reserves the room in its own stack frame and passes the room's address as a hidden first argument. The callee writes the value straight into that room and hands the address back. LLVM calls that hidden argument `sret`, and on x86-64 it travels in `rdi`.

```text
let t = three_numbers();            fn three_numbers() -> [u64; 3] { [7, 8, 9] }

1. main, before the call: room for t, 24 bytes, not written yet

 main's frame
┌───────────────────────────┐
│ t[0]  +0    ?? leftover   │◄──── rdi = address of t     lea rdi, [rbp - 24]
│ t[1]  +8    ?? leftover   │
│ t[2]  +16   ?? leftover   │                             call three_numbers
└───────────────────────────┘

2. inside three_numbers: every write goes through rdi, into main's frame

 main's frame                        three_numbers
┌───────────────────────────┐      ┌───────────────────────────────┐
│ t[0]  +0    7  ◄──────────┼──────┤ mov qword ptr [rdi], 7        │
│ t[1]  +8    8  ◄──────────┼──────┤ mov qword ptr [rdi + 8], 8    │
│ t[2]  +16   9  ◄──────────┼──────┤ mov qword ptr [rdi + 16], 9   │
└───────────────────────────┘      │ mov rax, rdi                  │
                                   └───────────────┬───────────────┘
                                                   │ ret
3. back in main: t is already filled               ▼
                                          rax = address of t
 main's frame
┌───────────────────────────┐
│ t[0]  +0    7             │      nothing is copied on the way out:
│ t[1]  +8    8             │      the value was built where it lives
│ t[2]  +16   9             │
└───────────────────────────┘
```

A value that fits in registers never gets a room. `fn two_numbers() -> (u64, u64)` is `mov eax, 7` and `mov edx, 8`, and the caller copies `rax` and `rdx` into its variable after the call. `sret` is the other path, taken by every row of [the measured table](../returned_by_value/README.md#what-the-machine-does-with-it) that shows `ptr sret(…)`, from `[u64; 2]` to `[u8; 1000]` (rustc 1.98.0, this target).

## Where each part of the drawing comes from

| In the drawing | Its source |
|---|---|
| the room is in **main's** frame, 24 bytes | `sub rsp, 32` then `lea rdi, [rbp - 24]` in `_caller`, [below](#sret-is-an-out-parameter-the-compiler-writes-for-you) |
| its address goes in `rdi`, as a hidden first argument | the System V psABI, "Returning of Values": for a result of class MEMORY *"the caller provides space for the return value and passes the address of this storage in %rdi as if it were the first argument"* |
| the callee writes through `rdi` | `mov qword ptr [rdi + 16], 9` in `three_numbers` |
| `rax` holds the same address on return | the same psABI rule goes on: *"On return %rax will contain the address that has been passed in by the caller in %rdi"*; `mov rax, rdi` in the assembly |
| the name `sret` | LLVM IR: `define void @three_numbers(ptr sret([24 x i8]) align 8 %_0)`. The [LangRef ↗](https://llvm.org/docs/LangRef.html#parameter-attributes) defines it as a pointer parameter holding *"the address of a structure that is the return value of the function in the source program"*, and a function taking one returns `void` |
| "leftover", not zeroes | the debugger, [below](#the-drawing-checked-in-a-debugger) |

The psABI rule is the C one; Rust's own calling convention has [no stability guarantee ↗](https://doc.rust-lang.org/reference/items/external-blocks.html#abi). Every row above was measured on rustc 1.98.0, x86_64-apple-darwin, so the register names in the drawing are this target's.

## `sret` is an out-parameter the compiler writes for you

Write the hidden pointer out as a real parameter and the machine code barely changes. Two versions of the same function, compiled side by side:

```rust
#[unsafe(no_mangle)]
#[inline(never)]
pub fn three_numbers() -> [u64; 3] {
    [7, 8, 9]
}

#[unsafe(no_mangle)]
#[inline(never)]
pub fn three_numbers_into(out: &mut [u64; 3]) {
    *out = [7, 8, 9];
}

#[unsafe(no_mangle)]
pub fn caller() -> u64 {
    let t = three_numbers();
    t[0] + t[1] + t[2]
}

#[unsafe(no_mangle)]
pub fn caller_by_hand() -> u64 {
    let mut t = [0u64; 3];
    three_numbers_into(&mut t);
    t[0] + t[1] + t[2]
}
```

```text title="rustc 1.98.0, x86_64-apple-darwin, opt-level=0 — the define lines of by_hand.rs"
define i64 @caller() unnamed_addr #0 {
define i64 @caller_by_hand() unnamed_addr #0 {
define void @three_numbers(ptr sret([24 x i8]) align 8 %_0) unnamed_addr #1 {
define void @three_numbers_into(ptr align 8 %out) unnamed_addr #1 {
```

```text title="rustc 1.98.0, x86_64-apple-darwin, opt-level=1 — by_hand.s, .cfi lines removed, tabs set as spaces, lined up side by side by a script"
_caller:                                  _caller_by_hand:
    push    rbp                               push    rbp
    mov     rbp, rsp                          mov     rbp, rsp
    sub     rsp, 32                           sub     rsp, 32
                                              xorps   xmm0, xmm0
                                              movaps  xmmword ptr [rbp - 32], xmm0
                                              mov     qword ptr [rbp - 16], 0
    lea     rdi, [rbp - 24]                   lea     rdi, [rbp - 32]
    call    _three_numbers                    call    _three_numbers_into
    mov     rax, qword ptr [rbp - 16]         mov     rax, qword ptr [rbp - 24]
    add     rax, qword ptr [rbp - 24]         add     rax, qword ptr [rbp - 32]
    add     rax, qword ptr [rbp - 8]          add     rax, qword ptr [rbp - 16]
    add     rsp, 32                           add     rsp, 32
    pop     rbp                               pop     rbp
    ret                                       ret

_three_numbers:                           _three_numbers_into:
    push    rbp                               push    rbp
    mov     rbp, rsp                          mov     rbp, rsp
    mov     rax, rdi
    mov     qword ptr [rdi], 7                mov     qword ptr [rdi], 7
    mov     qword ptr [rdi + 8], 8            mov     qword ptr [rdi + 8], 8
    mov     qword ptr [rdi + 16], 9           mov     qword ptr [rdi + 16], 9
    pop     rbp                               pop     rbp
    ret                                       ret
```

The two callees write the same three values through the same register. The two differences are the two things `sret` adds:

- **The room is not zeroed.** `caller_by_hand` spends three instructions writing zeroes (`xorps`, `movaps`, `mov … 0`), because safe Rust will not hand out a `&mut [u64; 3]` to bytes nobody has written, so the source had to say `[0u64; 3]`. `caller` has nothing to zero: before the call `t` does not exist yet, and filling it is the callee's job. The optimizer kept the zeroes although it marked *both* parameters `writeonly` and `initializes((0, 24))` in the IR at this level.
- **The address comes back in `rax`.** `three_numbers` has `mov rax, rdi`; the hand-written version returns nothing.

The program below writes both versions out, and checks each arrow of the drawing as a comparison. Part 3 uses [`MaybeUninit` ↗](https://doc.rust-lang.org/std/mem/union.MaybeUninit.html) for a room nobody has written to ([Validity invariants](../../09_Advanced/validity_invariants/README.md) is why that needs a type of its own), and its `write` method returns a `&mut` to the value it just stored: the `mov rax, rdi`, in Rust.

<!-- output:drawing_the_return_slot -->
*Verified output of [`drawing_the_return_slot.rs`](examples/drawing_the_return_slot.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The room main reserves for `let t: [u64; 3]`
   24 bytes; t[0] at +0, t[1] at +8, t[2] at +16

2. sret by hand, with a room main has to zero first
   before the call: [0, 0, 0]
   the pointer handed back is main's t: true
   after the call:  [7, 8, 9]

3. sret by hand, with a room nobody has written to
   write() hands back the room it filled: true
   read back through it: [7, 8, 9]
```
<!-- /output -->

## The drawing, checked in a debugger

The program above can only imitate `sret`. A debugger can watch the real one: stop inside `three_numbers`, read `rdi` and the bytes it points at, then stop in `main` after the call and ask where `t` lives.

```rust
#[inline(never)]
fn three_numbers() -> [u64; 3] {
    [7, 8, 9]
}

fn main() {
    let t = three_numbers();
    println!("{}", t[0] + t[1] + t[2]);
}
```

```sh
rustc --edition 2024 -g -C opt-level=0 slot.rs -o slot
lldb --batch -o "breakpoint set --name slot::three_numbers" -o "breakpoint set --file slot.rs --line 8" -o "run" -o "register read rdi" -o "memory read --format u --size 8 --count 3 \$rdi" -o "continue" -o "frame variable -L t" ./slot
```

```text title="Abridged — lldb-2100.0.17.203, rustc 1.98.0, x86_64-apple-darwin; source listings and process lines removed"
(lldb) breakpoint set --name slot::three_numbers
Breakpoint 1: where = slot`slot::three_numbers + 7 at slot.rs:3:5, address = 0x0000000100000927
(lldb) breakpoint set --file slot.rs --line 8
Breakpoint 2: 3 locations.
(lldb) run
* thread #1, name = 'main', queue = 'com.apple.main-thread', stop reason = breakpoint 1.1
    frame #0: 0x0000000100000927 slot`slot::three_numbers at slot.rs:3:5
(lldb) register read rdi
     rdi = 0x00007ff7bfefe648
(lldb) memory read --format u --size 8 --count 3 $rdi
0x7ff7bfefe648: 140703464097186
0x7ff7bfefe650: 0
0x7ff7bfefe658: 4296945504
(lldb) continue
* thread #1, name = 'main', queue = 'com.apple.main-thread', stop reason = breakpoint 2.1
    frame #0: 0x0000000100000951 slot`slot::main at slot.rs:8:20
(lldb) frame variable -L t
0x00007ff7bfefe648: (unsigned long[3]) t = {
0x00007ff7bfefe648:   [0] = 7
0x00007ff7bfefe650:   [1] = 8
0x00007ff7bfefe658:   [2] = 9
}
```

- **Panel 1:** inside `three_numbers`, before its first write, `rdi` is `0x7ff7bfefe648` and the 24 bytes there are leftovers from earlier code. Nobody zeroed the room.
- **Panel 2** falls between the two stops. The assembly above is its record: every write in `three_numbers` is `[rdi + n]`.
- **Panel 3:** back in `main`, `t` lives at `0x7ff7bfefe648`, the address `three_numbers` was handed, and holds `7, 8, 9`.

Your addresses and leftover bytes will differ. lldb launches programs with address randomisation off (`target.disable-aslr` is `true`), so on this machine the same address came back on every run. This is a debug build, which is what lets lldb name `t`; the optimized assembly above passes the same hidden pointer.

## If you are coming from another language

**C.** This is the C rule, not a Rust invention: the psABI text quoted above is written for C, and a C function returning `struct Triple { long v[3]; }` gets the same hidden pointer. What C does not have is the out-parameter's safety: `void fill(long *out)` accepts a null pointer, a pointer into freed memory, or an address the caller is also reading through another name. The psABI requires that the room *"must not overlap any data visible to the callee through other names"*; in C that is a rule for the compiler to rely on, and in Rust the `&mut` in `three_numbers_into` is the same promise, checked. [The C library's page on arrays decaying at a call ↗](https://masiarek.github.io/c-learning-library/03_Strings/a_string_is_bytes_up_to_a_nul/index.html#what-the-four-cases-show) is why C wraps an array in a struct to return it at all.

**C++.** C++17's guaranteed copy elision is this drawing made into a language rule: `return Pinned();` constructs the object straight into the caller's room, so a type with deleted copy and move constructors can still be returned (Apple clang 21: it compiles under `-std=c++17` and is *"call to deleted constructor of 'Pinned'"* under `-std=c++14`). The room also decides more than size there: a one-byte C++ struct with a user-written copy constructor goes through `sret` too ([the reading list](../returned_by_value_resources/README.md) has the clang run). Rust needs no such rule, because moving a value runs no user code, so building in place is only ever a speed question.

**Python and Java.** There is no room to draw for an object. A returned object is a reference to something already on the heap, so what crosses the call is one pointer. The heap allocation the drawing avoids is the one those languages make for every object.

## Practice

**Draw the room for `-> [Point; 2]`, then build it.** `Point` is `#[repr(C)] struct Point { x: i32, y: i32 }`. Before running anything, draw the room the caller of `fn corners() -> [Point; 2]` reserves: its size, and the offset of each of the four fields. Then write `corners` with the hidden pointer spelled out, as `fn corners_into(slot: &mut MaybeUninit<[Point; 2]>) -> &mut [Point; 2]`, and print the size, the four offsets (`offset_of!`), and whether the reference it hands back is the room the caller reserved.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:drawing_the_return_slot_kata -->
*[`drawing_the_return_slot_kata.rs`](examples/drawing_the_return_slot_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: draw the room for `-> [Point; 2]`, then build it by hand.
//!
//! `#[repr(C)]` fixes the field order and offsets, so the drawing below is a
//! promise rather than today's layout. Every line is a size, an offset or a
//! comparison, so the answer key is the same on every run.
//!
//!   rustc --edition 2024 drawing_the_return_slot_kata.rs -o /tmp/dtrsk && /tmp/dtrsk

use std::mem::{MaybeUninit, offset_of, size_of};
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

// The drawing, before running anything:
//
//   room for [Point; 2], 16 bytes
//   +0   corners[0].x
//   +4   corners[0].y
//   +8   corners[1].x
//   +12  corners[1].y

/// `fn corners() -> [Point; 2]`, with the hidden pointer written out.
#[inline(never)]
fn corners_into(slot: &mut MaybeUninit<[Point; 2]>) -> &mut [Point; 2] {
    slot.write([Point { x: 0, y: 0 }, Point { x: 640, y: 480 }])
}

fn main() {
    let point = size_of::<Point>();
    println!("the room: {} bytes", size_of::<[Point; 2]>());
    for i in 0..2 {
        println!("   +{:<2}  corners[{i}].x", i * point + offset_of!(Point, x));
        println!("   +{:<2}  corners[{i}].y", i * point + offset_of!(Point, y));
    }

    let mut slot = MaybeUninit::<[Point; 2]>::uninit();
    let room = slot.as_ptr();
    let corners = corners_into(&mut slot);
    println!("the callee filled main's room and handed it back: {}", ptr::eq(&*corners, room));
    println!("{:?}", *corners);
}
```
<!-- /source -->

<!-- output:drawing_the_return_slot_kata -->
*Verified output of [`drawing_the_return_slot_kata.rs`](examples/drawing_the_return_slot_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
the room: 16 bytes
   +0   corners[0].x
   +4   corners[0].y
   +8   corners[1].x
   +12  corners[1].y
the callee filled main's room and handed it back: true
[Point { x: 0, y: 0 }, Point { x: 640, y: 480 }]
```
<!-- /output -->

`#[repr(C)]` is what makes the drawing a promise. Without it Rust may reorder fields, and the offsets are whatever this compiler chose.

</details>

## See also

- [Returned by value](../returned_by_value/README.md): why the room needs one size, and the table of which types get registers and which get `sret`
- [The call stack](../the_call_stack/README.md): the frame the room sits in, reserved on entry and released on return
- [A stack slot is reused](../a_stack_slot_is_reused/README.md): the leftover bytes in panel 1 are an earlier call's frame, reissued
- [What an address shows](../what_an_address_shows/README.md): why a printed address is a weak answer key, and why the example prints comparisons
- [Drawing the owner and the view](../../14_Strings/drawing_the_owner_and_the_view/README.md): the same kind of page for a `String` and a `&str`
- [LLVM and its IR](../../20_Compilers/llvm_and_its_ir/README.md): reading a `define` line
- [Returned by value: reading](../returned_by_value_resources/README.md): the psABI, the LangRef and Aria Desires' ABI notes, with the one sentence in them to read with care

## Po polsku

**`sret`** (*structure return*) to ukryty pierwszy argument funkcji, która zwraca wartość za dużą na rejestry. Wywołujący rezerwuje na nią miejsce we własnej ramce stosu i przekazuje adres tego miejsca w `rdi`. Funkcja wpisuje wartość bezpośrednio pod ten adres (`mov qword ptr [rdi + 16], 9`), a na koniec oddaje ten sam adres w `rax`. Przy wyjściu nic nie jest kopiowane: wartość od początku powstaje tam, gdzie ma żyć.

To zwykły parametr wyjściowy, dopisany przez kompilator. Wersja ręczna, `fn three_numbers_into(out: &mut [u64; 3])`, kompiluje się do tych samych trzech zapisów. Różnice są dwie: wywołujący musi wcześniej wyzerować miejsce, bo bezpieczny Rust nie da `&mut` do niezainicjowanej pamięci, a `sret` oddaje adres w `rax`. Debugger (lldb) pokazuje to wprost: w `three_numbers` rejestr `rdi` wskazuje na przypadkowe bajty, a po powrocie `t` w `main` leży pod tym samym adresem i zawiera `7, 8, 9`.

**Szukaj po polsku:** zwracanie struktury przez ukryty wskaźnik · parametr wyjściowy · konwencja wywołań x86-64 · `sret llvm` · `System V ABI return value rdi rax` · `rust return value optimization`
