# Makefiles: a build graph you write by hand

**Level:** 201 · working knowledge

**One line:** A Makefile is a list of rules — *this file is made from those files, by this command* — and `make` reruns a command only when one of its inputs is newer than its output; that timestamp comparison is the whole engine, which is also why a dependency you forgot to write down leaves you a stale binary and no error.

## A whole Makefile

A C program in three files: `hello.c` calls `greet("Ada")`, `greet.c` prints the greeting, and both include this header:

```c
#define GREETING "Hello"

void greet(const char *name);
```

The Makefile that builds it:

```make
CC     = cc
CFLAGS = -Wall -O2

hello: hello.o greet.o
	$(CC) -o hello hello.o greet.o

hello.o: hello.c greet.h
	$(CC) $(CFLAGS) -c hello.c

greet.o: greet.c greet.h
	$(CC) $(CFLAGS) -c greet.c

.PHONY: clean
clean:
	rm -f hello hello.o greet.o
```

All four files are beside this page — [`Makefile`](demo/Makefile), [`hello.c`](demo/hello.c), [`greet.c`](demo/greet.c), [`greet.h`](demo/greet.h) — and `make` in that folder reproduces the next section's runs. Take the Makefile from there rather than copying it off this page; [the TAB](#the-tab) is why.

| In the file | Called | What `make` does with it |
|---|---|---|
| `greet.o: greet.c greet.h` | a **rule** | target, colon, prerequisites — and the recipe on the lines under it |
| `greet.o` | the **target** | a file name; its modification time is what gets compared |
| `greet.c greet.h` | the **prerequisites** | brought up to date first, then compared against the target |
| the indented line | the **recipe** | shell commands, run only if the target is missing or older than a prerequisite |
| `CC = cc`, then `$(CC)` | a **variable** | text substitution, done before the command runs |
| `.PHONY: clean` | a **phony target** | declares that `clean` is not a file, so `make clean` always runs |
| `hello` coming first | the **default goal** | what a bare `make` builds |

## What `make` does with it

Each edit comes a second or more after the build before it — [the end of the page](#two-makes-on-two-machines) says why that matters on a Mac:

```text title="Real output — GNU Make 3.81 and Apple clang 21, macOS"
$ make
cc -Wall -O2 -c hello.c
cc -Wall -O2 -c greet.c
cc -o hello hello.o greet.o
$ ./hello
Hello, Ada
$ make
make: `hello' is up to date.
$ touch greet.c
$ make
cc -Wall -O2 -c greet.c
cc -o hello hello.o greet.o
$ touch greet.h
$ make -n
cc -Wall -O2 -c hello.c
cc -Wall -O2 -c greet.c
cc -o hello hello.o greet.o
$ make
cc -Wall -O2 -c hello.c
cc -Wall -O2 -c greet.c
cc -o hello hello.o greet.o
```

The first run built in dependency order: both objects, then the link. The second found nothing newer than `hello` and ran nothing. `touch greet.c` changed the file's timestamp and not one byte of it, and that was enough to recompile `greet.c` and relink — but not to recompile `hello.c`, whose inputs had not moved. `touch greet.h` reached both objects, because both rules list it.

`make -n` prints the commands it would run and runs none of them — the first thing to type in a Makefile you did not write.

## The whole rule

For each target, depth first: bring every prerequisite up to date, then run the recipe if the target does not exist or any prerequisite is newer than it. `make` never opens `greet.c`; it reads names from the Makefile and modification times from the filesystem. So:

- **A `touch` is an edit** — shown above.
- **An older file is not an edit.** Put back a file with its old timestamp — `cp -p`, `tar -x`, a restore from backup — and it is older than the object built from the file it replaced:

```text title="Real output — GNU Make 3.81, macOS; 4.4.1 on Linux agrees"
$ sed -i.bak 's/Hello/Goodbye/' greet.h
$ touch -t 202001010000 greet.h
$ make
make: `hello' is up to date.
$ ./hello
Hello, Ada
```

- **A target is only a name.** The `clean` recipe never creates a file called `clean`, so `make clean` always finds its target missing and runs — until a file of that name turns up. Then `clean` is a target that exists and has no prerequisites to be older than, which is the definition of up to date. The Makefile above without its `.PHONY` line, and a stray file called `clean`:

```text title="Real output — GNU Make 3.81, macOS"
$ make clean
make: `clean' is up to date.
$ ls
Makefile
clean
greet.c
greet.h
greet.o
hello
hello.c
hello.o
```

`.PHONY: clean` is the fix: a phony target is never looked up on disk, so its recipe always runs.

## The TAB

A recipe line must start with a TAB character — the single byte `0x09` — not with spaces. Indent one with four spaces and `make` stops before building anything:

```text title="Real output — GNU Make 3.81, macOS; 4.4.1 prints the same line"
$ make
Makefile:5: *** missing separator.  Stop.
```

Both versions add a hint when the indentation is eight spaces — `missing separator (did you mean TAB instead of 8 spaces?)` — and say nothing more for the four an editor usually inserts. A TAB and the spaces it looks like are indistinguishable on screen; `cat -et` shows the TAB as `^I`, with `$` marking each line end:

```text title="Real output — BSD cat, macOS; GNU cat prints the same"
$ cat -et Makefile
CC     = cc$
CFLAGS = -Wall -O2$
$
hello: hello.o greet.o$
^I$(CC) -o hello hello.o greet.o$
```

**Copying a Makefile off a web page is an easy way to lose the TAB.** This site is built by Python-Markdown 3.10.3, which expands every TAB in a page to spaces before it renders a code block — so every Makefile on this page arrives on the site with four spaces where its TAB was, and a copy of it fails with exactly the message above. GitHub's renderer keeps the TAB. The files in [`demo/`](demo/Makefile) are the real bytes.

Since GNU Make 3.82 (2010) a Makefile can choose another prefix with `.RECIPEPREFIX = >`, and 4.4.1 honours it; 3.81 ignores the assignment and stops at the first `>` with `missing separator`.

## The trap: a header nobody listed

Most hand-written Makefiles list the `.c` file and stop. Here are the two object rules written that way:

```make
hello.o: hello.c
	$(CC) $(CFLAGS) -c hello.c

greet.o: greet.c
	$(CC) $(CFLAGS) -c greet.c
```

Build, change the greeting in `greet.h`, build again:

```text title="Real output — GNU Make 3.81, macOS; 4.4.1 differs only in its quote marks"
$ make
cc -Wall -O2 -c hello.c
cc -Wall -O2 -c greet.c
cc -o hello hello.o greet.o
$ ./hello
Hello, Ada
$ sed -i.bak 's/Hello/Goodbye/' greet.h
$ make
make: `hello' is up to date.
$ ./hello
Hello, Ada
$ make -B
cc -Wall -O2 -c hello.c
cc -Wall -O2 -c greet.c
cc -o hello hello.o greet.o
$ ./hello
Goodbye, Ada
```

No rule mentions `greet.h`, so as far as `make` can see, nothing is newer than anything — no error, no warning, and a binary built from the old header. Here the stale part is a greeting. When the header defines a struct and only one of the two `.c` files is edited, only that object is rebuilt: two object files compiled against two layouts of one struct, linked into one program without complaint, because [the linker](../the_linker/README.md) matches names, not layouts. `make -B` rebuilds everything regardless of timestamps, which fixes it once; the lasting fix is to stop writing the header lists by hand.

## Let the compiler write the graph

The compiler knows exactly which headers a file includes — it opened them. `-MMD` makes it write that list down as a side effect of compiling, in Makefile syntax, one `.d` file per object; `-MP` adds an empty rule for each header. The Makefile gains two flags and one line, and loses its header lists:

```make
CC     = cc
CFLAGS = -Wall -O2 -MMD -MP

hello: hello.o greet.o
	$(CC) -o hello hello.o greet.o

hello.o: hello.c
	$(CC) $(CFLAGS) -c hello.c

greet.o: greet.c
	$(CC) $(CFLAGS) -c greet.c

-include hello.d greet.d
```

```text title="Real output — GNU Make 3.81 and Apple clang 21, macOS; GCC 14 writes the same .d files"
$ make
cc -Wall -O2 -MMD -MP -c hello.c
cc -Wall -O2 -MMD -MP -c greet.c
cc -o hello hello.o greet.o
$ cat greet.d
greet.o: greet.c greet.h
greet.h:
$ cat hello.d
hello.o: hello.c greet.h
greet.h:
$ sed -i.bak 's/Hello/Goodbye/' greet.h
$ make
cc -Wall -O2 -MMD -MP -c hello.c
cc -Wall -O2 -MMD -MP -c greet.c
cc -o hello hello.o greet.o
$ ./hello
Goodbye, Ada
```

`-include` reads the `.d` files as more rules, and its dash means *carry on if they are missing* — which on the first build they are, harmlessly, since everything is out of date then anyway. The `greet.h:` lines are `-MP`'s. Without them, deleting a header breaks the next build, because the old `.d` file still names it and nothing can make it. The same project compiled with `-MMD` alone, after `greet.h` is deleted and its two `#include` lines removed:

```text title="Real output — GNU Make 3.81, macOS"
$ make
make: *** No rule to make target `greet.h', needed by `hello.o'.  Stop.
```

With `-MP`, the empty rule is one `make` can always satisfy — a missing file with no recipe counts as freshly made — so the same deletion just rebuilds both objects. [GCC's preprocessor options ↗](https://gcc.gnu.org/onlinedocs/gcc/Preprocessor-Options.html) document the whole `-M` family; Apple's clang wrote the `.d` files above from the same flags.

## The Makefile you will actually meet

Most Makefiles in the wild are shorter than the one at the top, because `make` already knows how to turn a `.c` file into a `.o`, and `.o` files into a program named after one of them. This is a complete Makefile for the same project:

```make
CFLAGS = -Wall -O2 -MMD -MP
OBJS   = hello.o greet.o

hello: $(OBJS)

-include $(OBJS:.o=.d)
```

```text title="Real output — GNU Make 3.81, macOS; 4.4.1 differs only in its quote marks"
$ make
cc -Wall -O2 -MMD -MP   -c -o hello.o hello.c
cc -Wall -O2 -MMD -MP   -c -o greet.o greet.c
cc   hello.o greet.o   -o hello
$ ./hello
Hello, Ada
$ make
make: `hello' is up to date.
```

Nobody wrote those three commands. They come from `make`'s built-in rules — `make -p -f /dev/null` prints the whole database. The first and last of these did the work here; the middle one is the C++ version:

| Built-in rule | Its recipe |
|---|---|
| `%.o: %.c` | `$(COMPILE.c) $(OUTPUT_OPTION) $<` |
| `%.o: %.cpp` | `$(COMPILE.cpp) $(OUTPUT_OPTION) $<` |
| `%: %.o` | `$(LINK.o) $^ $(LOADLIBES) $(LDLIBS) -o $@` |

| Variable | Expands to |
|---|---|
| `COMPILE.c` | `$(CC) $(CFLAGS) $(CPPFLAGS) $(TARGET_ARCH) -c` |
| `COMPILE.cpp` | `$(COMPILE.cc)`, which is `$(CXX) $(CXXFLAGS) $(CPPFLAGS) $(TARGET_ARCH) -c` |
| `LINK.o` | `$(CC) $(LDFLAGS) $(TARGET_ARCH)` |
| `OUTPUT_OPTION` | `-o $@` |

The runs of spaces in `-MMD -MP   -c` are the evidence: two variables nobody set, `CPPFLAGS` and `TARGET_ARCH`, expanded to nothing in place.

The link rule needs an object named after the program — `hello` from `hello.o` — which is why the main file here is `hello.c`. Name it `main.c`, and a Makefile that says `hello: main.o greet.o` compiles both objects, links nothing, and exits 0:

```text title="Real output — GNU Make 3.81, macOS; 4.4.1 differs only in its quote marks"
$ cat Makefile
CFLAGS = -Wall -O2

hello: main.o greet.o
$ make
cc -Wall -O2   -c -o main.o main.c
cc -Wall -O2   -c -o greet.o greet.c
$ ls
Makefile
greet.c
greet.h
greet.o
main.c
main.o
$ make
make: Nothing to be done for `hello'.
```

With no recipe and no built-in rule that fits, `hello` is a target `make` has nothing to do for, so it says so — successfully — and there is still no program. Name the program after an object, or write the link recipe yourself.

Three notations carry most Makefiles:

| Notation | Means |
|---|---|
| `%.o: %.c` | a **pattern rule** — `%` matches the same stem on both sides |
| `$@` · `$<` · `$^` | **automatic variables** — the target, the first prerequisite, all the prerequisites |
| `$(OBJS:.o=.d)` | a **substitution reference** — `hello.o greet.o` with each `.o` swapped for `.d` |

The conventional variables are how you configure the built-in rules without rewriting them, and they are what the first lines of most C and C++ Makefiles set:

| Set | For | Reaches |
|---|---|---|
| `CC`, `CXX` | the compilers | the compile rules; `CC` also links |
| `CFLAGS`, `CXXFLAGS` | warnings, optimization, language standard | C and C++ compiles |
| `CPPFLAGS` | `-I` include paths, `-D` macros | both compiles |
| `LDFLAGS` | `-L` library paths | the link |
| `LDLIBS` | `-l` libraries | the end of the link line, after the objects |

*`CC` also links* is the trap for C++. `%: %.o` uses `LINK.o`, and `LINK.o` is `$(CC)`, so a C++ program built entirely from built-in rules is compiled with `c++` and linked with `cc`, which does not bring in the C++ standard library:

```text title="Abridged — real output, GNU Make 4.4.1 and GCC 14 on Debian, three of the four undefined references cut"
$ make
g++    -c -o hello.o hello.cpp
cc   hello.o   -o hello
/usr/bin/ld: hello.o: in function `main':
hello.cpp:(.text+0xa): undefined reference to `std::cout'
collect2: error: ld returned 1 exit status
make: *** [<builtin>: hello] Error 1
```

macOS fails the same way, as `Undefined symbols for architecture x86_64`. One line above the rule fixes it — `LINK.o = $(CXX) $(LDFLAGS) $(TARGET_ARCH)`:

```text title="Real output — GNU Make 3.81 and Apple clang 21, macOS"
$ make
c++    -c -o hello.o hello.cpp
c++   hello.o   -o hello
$ ./hello
Hello, Ada
```

## Why a Rust project has no Makefile

Not because it has no graph. Cargo keeps one, applies the same test to your own crate's files, and gets the file list the way `-MMD` does — from the compiler. A crate whose `src/main.rs` declares `mod greet;` and whose `src/greet.rs` holds `pub const GREETING: &str = include_str!("greeting.txt");`:

```text title="Real output — cargo 1.98.0, macOS (paths shortened)"
$ cargo build -q && ./target/debug/hello
Hello, Ada
$ cat target/debug/deps/hello-*.d
…/hello/target/debug/deps/hello-4efa6c30a3525e6f.d: src/main.rs src/greet.rs src/greeting.txt

…/hello/target/debug/deps/hello-4efa6c30a3525e6f: src/main.rs src/greet.rs src/greeting.txt

src/main.rs:
src/greet.rs:
src/greeting.txt:
$ printf Goodbye > src/greeting.txt
$ cargo build -v 2>&1 | grep -E "Dirty|Fresh|Compiling|Finished"
       Dirty hello v0.1.0 (…/hello): the file `src/greeting.txt` has changed (1789089784.519205528s, 2s after last build at 1789089782.904903643s)
   Compiling hello v0.1.0 (…/hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
$ ./target/debug/hello
Goodbye, Ada
$ touch src/main.rs
$ cargo build -v 2>&1 | grep -E "Dirty|Fresh|Compiling|Finished"
       Dirty hello v0.1.0 (…/hello): the file `src/main.rs` has changed (1789089786.057369000s, 2s after last build at 1789089784.566631000s)
   Compiling hello v0.1.0 (…/hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
```

- **`rustc` wrote the `.d` file**, because Cargo passes it `--emit=dep-info,link`. The [rustc book ↗](https://doc.rust-lang.org/rustc/command-line-arguments.html#--emit-specifies-the-types-of-output-files-to-generate) calls it a file "with Makefile syntax", and its last three lines are `-MP`'s empty rules.
- **It lists `greeting.txt`**, which no `mod` declares. `include_str!` read it, so it is in the graph, and changing it rebuilt the crate. The header trap cannot happen, because the compiler, not you, says what was read.
- **Cargo's reason is two timestamps.** After a `touch` that changed no byte, the file *has changed*: stable Cargo compares modification times for your own crate's files, as `make` does.

The same test has the same blind spot. Put back an older file and Cargo calls the crate `Fresh`, and the binary keeps the old greeting:

```text title="Real output — cargo 1.98.0, macOS (paths shortened)"
$ printf Hi > src/greeting.txt
$ touch -t 202001010000 src/greeting.txt
$ cargo build -v 2>&1 | grep -E "Dirty|Fresh|Compiling|Finished"
       Fresh hello v0.1.0 (…/hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
$ ./target/debug/hello
Goodbye, Ada
```

Cargo also leaves a copy beside the binary, `target/debug/hello.d`, with absolute paths — the [Cargo book ↗](https://doc.rust-lang.org/cargo/reference/build-cache.html#dep-info-files) says it is there for external build systems, which is to say for a Makefile deciding whether to run Cargo at all.

What stays Rust's own is the unit: `make` compiles files and Cargo compiles crates, so there is no `.o` per module to keep in step — [A build system is not a compiler](../build_systems_are_not_compilers/README.md) takes that up. When a Rust repository does ship a Makefile, read its `.PHONY` line first: if every target is on it, the file is a list of shortcuts for longer `cargo` commands, and none of `make`'s timestamp machinery is in use.

## If you are coming from another language

**Python.** A `.pyc` file in `__pycache__/` is a target with one prerequisite, and it carries its own timestamp check: the header of a default `.pyc` records the modification time and size of its `.py` — on Python 3.14.7 they match the source's exactly — and the import system recompiles when either stops matching. *Matching*, not *older*: `touch` recompiles a module, as it rebuilds an object file, and so does an older copy put back over one — on 3.14.7 the `.pyc` took the restored file's 2020 timestamp and its new contents, where `make` and Cargo both said up to date. What Python lacks is the header trap: there is no textual `#include`, so no file is ever compiled against another file's text; each module is compiled alone, and its imports are bound when the program runs. [PEP 552 ↗](https://peps.python.org/pep-0552/) added hash-checked `.pyc` files, for builds that must not depend on timestamps at all.

**ABAP.** There is no Makefile because the repository holds the graph. Activating a changed dictionary structure also activates the dictionary objects built on it, and a program's generated load is invalidated when its source or a dictionary type it uses changes, then regenerated the next time the program runs — so the where-used list is a query the system answers, not a file somebody maintains. The header trap has no ABAP form for the reason `-MMD` fixes it in C: the system, not the programmer, records what each program depends on.

## Two makes, on two machines

`/usr/bin/make` on macOS is GNU Make 3.81 — `make --version` dates it 2006. Linux distributions ship 4.x; every Linux run on this page is GNU Make 4.4.1 in Docker's `gcc:14` image, and Homebrew installs the same version as `gmake`. What differs between the two:

| | GNU Make 3.81, macOS | GNU Make 4.4.1, Debian |
|---|---|---|
| Nothing to do | `` make: `hello' is up to date. `` | `make: 'hello' is up to date.` |
| A recipe failed | `make: *** [greet.o] Error 1` | `make: *** [Makefile:11: greet.o] Error 1` |
| A built-in recipe failed | `make: *** [hello] Error 1` | `make: *** [<builtin>: hello] Error 1` |
| `.RECIPEPREFIX` | ignored | honoured |
| `CXX` if unset | `c++` | `g++` |
| Timestamps compared to | the whole second | a fraction of a second |

In `Error 1`, the 1 is the exit status of the command that failed — the compiler's or the linker's — and `make` itself then exits with 2. [The exit status](../../02_Errors/stderr_and_exit_status/README.md) is the byte it reads to decide whether to stop.

The last row is the one that bites. Build and edit within the same second, and 3.81 cannot see the edit:

```text title="Real output — GNU Make 3.81, macOS"
$ make && touch greet.c && make
cc -Wall -O2 -c hello.c
cc -Wall -O2 -c greet.c
cc -o hello hello.o greet.o
make: `hello' is up to date.
```

```text title="Real output — GNU Make 4.4.1, Debian (gcc:14 image)"
$ make && touch greet.c && make
cc -Wall -O2 -c hello.c
cc -Wall -O2 -c greet.c
cc -o hello hello.o greet.o
cc -Wall -O2 -c greet.c
cc -o hello hello.o greet.o
```

The filesystem knew better. In one of six such trials `stat` put `greet.o` at `…891.253720030` and `greet.c` at `…891.326716000`, 73 ms later, and 3.81 compared only the `…891`. Five of the six trials missed the edit; the sixth straddled a second boundary and caught it. A person at a keyboard is rarely that fast. A script, a test harness or a code generator is — which is why the other runs on this page leave a second before each edit.

## See also

- [A build system is not a compiler](../build_systems_are_not_compilers/README.md) — the same job with the graph inferred for you, and the rest of what Cargo's fingerprint covers
- [The linker](../the_linker/README.md) — the last recipe in every Makefile on this page, and the dialect its errors arrive in
- [Running a scratch program](../../15_First_Programs/rustc_without_cargo/README.md) — `rustc` with no build system at all
- [Standard error, and exit status](../../02_Errors/stderr_and_exit_status/README.md) — the byte `make` reads after every command
- [The GNU `make` manual ↗](https://www.gnu.org/software/make/manual/make.html) — in particular [phony targets ↗](https://www.gnu.org/software/make/manual/html_node/Phony-Targets.html), [automatic variables ↗](https://www.gnu.org/software/make/manual/html_node/Automatic-Variables.html), [the built-in rules ↗](https://www.gnu.org/software/make/manual/html_node/Catalogue-of-Rules.html), [the variables they read ↗](https://www.gnu.org/software/make/manual/html_node/Implicit-Variables.html), and [generating prerequisites automatically ↗](https://www.gnu.org/software/make/manual/html_node/Automatic-Prerequisites.html), the manual's own section on the header problem

---

*No generated output block on this page, deliberately: every transcript here is a property of `make`, a C compiler and a filesystem on one machine, not of a Rust program, and the answer-key runner compiles single `.rs` files. So each fence says which `make` and which machine printed it — macOS on x86_64 with its own `/usr/bin/make`, or Debian in Docker's `gcc:14` image — and nothing further is claimed.*

## Po polsku

Plik `Makefile` to lista **reguł** (*rules*): *ten plik powstaje z tamtych plików, tym poleceniem*. Każda reguła ma **cel** (*target*), **zależności** (*prerequisites*) i **polecenia** (*recipe*), a `make` uruchamia polecenia tylko wtedy, gdy któraś zależność jest **nowsza** od celu. Porównanie znaczników czasu modyfikacji (*modification time*, `mtime`) to cały silnik: `make` nie zagląda do plików i nie liczy sum kontrolnych — zna nazwy z Makefile'a i daty z systemu plików. Stąd samo `touch` wystarcza do przebudowy, a zmiana nagłówka (*header*), którego żadna reguła nie wymienia, nie przebuduje niczego: plik wykonywalny jest nieaktualny, a błędu nie ma.

Rozwiązanie jest takie samo w C i w Ruście — niech graf zależności wypisze **kompilator**, bo to on wie, co naprawdę otworzył. Flagi `-MMD -MP` każą `cc` zapisać obok każdego pliku obiektowego plik `.d` w składni Makefile'a; `rustc` robi to zawsze (`--emit=dep-info`), a `cargo` czyta ten plik i przebudowuje crate, gdy któryś z wymienionych plików — także wczytany przez `include_str!` — ma nowszy znacznik czasu. Dlatego projekt w Ruście nie potrzebuje Makefile'a.

Dwie pułapki widać dopiero w terminalu. Linia polecenia musi zaczynać się **tabulatorem** (bajt `0x09`), a nie spacjami — inaczej `missing separator`; `cat -et Makefile` pokazuje tabulator jako `^I`, a skopiowanie Makefile'a ze strony internetowej potrafi go po cichu zamienić na spacje (ta witryna też to robi, więc prawdziwe pliki leżą w katalogu `demo/`). Druga dotyczy macOS: systemowy `/usr/bin/make` to GNU Make 3.81 z 2006 roku, który porównuje czas z dokładnością do **sekundy** — zmiana w tej samej sekundzie co budowanie jest dla niego niewidoczna. Aktualną wersję Homebrew instaluje jako `gmake`.

**Szukaj po polsku:** plik Makefile · reguły make · zależności od plików nagłówkowych · `make missing separator` · `gcc -MMD -MP` · `make .PHONY`
