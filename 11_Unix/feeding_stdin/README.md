# Feeding stdin

**Level:** 201 · for anyone with a terminal

**One line:** Your program reads file descriptor 0 and has no idea what put the bytes there — a keyboard, a pipe, a redirect, a here-document or another program — so the shell is where you make the bytes on purpose, look at exactly what they are, and end them when you mean to; every command below was run on this machine, and the encodings library has the page that measures the ones that differ on Linux.

Nothing on this page is Rust. It is the minute before [Reading a line from standard input](../../03_Command_Line/reading_stdin/README.md) runs, and the way to answer *"what did my program actually receive?"* without adding a `dbg!` to it. Every fence is from one session: macOS 26.6.2, bash 3.2.57, 2026-09-13.

## Five ways bytes reach fd 0

| Spelling | What arrives | Worth knowing |
|---|---|---|
| `printf 'x\n' \| prog` | whatever the left side wrote | a **pipe**: no size, no seek, may arrive in pieces |
| `prog < file` | the file's bytes | a **file** on fd 0: `read` sees it whole, and it can be rewound |
| `prog <<< 'abc'` | `abc` **and a newline** | a here-string adds `\n` — 4 bytes, not 3 |
| `prog <<'END' … END` | the lines between | a **here-doc**; quote the `END` or `$HOME` inside it expands |
| `prog <(cmd)` | a *path* like `/dev/fd/63` | **process substitution**: an argument, not stdin, but it is a pipe |

```text title="Measured 2026-09-13 — the here-string's extra byte, and the here-doc's quote"
$ printf 'abc' | wc -c | tr -d ' '
3

$ wc -c <<< 'abc' | tr -d ' '
4

$ cat <<'END'
$HOME is not expanded in a quoted here-doc
END
$HOME is not expanded in a quoted here-doc

$ cat <<END
$HOME is expanded in a bare one
END
/Users/amasa is expanded in a bare one
```

The extra byte matters exactly once: when the program is counting. `printf` writes what you typed and nothing more, which is [why it is the tool for building a test file by hand ↗](https://masiarek.github.io/encodings-learning-library/06_Terminal/printf_writes_bytes/index.html); `echo` adds a newline, and `echo -n` is [not the same command in every shell](#the-traps).

## Making bytes on purpose

```text title="Measured 2026-09-13 — printf names bytes; xxd -r -p turns hex back into them"
$ printf 'a\0b\n' | xxd
00000000: 6100 620a                                a.b.

$ printf '\x48\x69\n' | xxd -p
48690a

$ printf '48690a' | xxd -r -p | cat -vet
Hi$

$ printf '%s\n' a b c
a
b
c

$ printf '%d\n' 0x41 "'A"
65
65
```

- `\0`, `\xHH` and `\NNN` name a **byte** and ask nothing of the locale. `\uHHHH` names a *code point* and [gives three different answers on three configurations ↗](https://masiarek.github.io/encodings-learning-library/02_Characters/writing_a_code_point/index.html) — one of them a euro sign.
- `printf` **reuses its format** for every remaining argument, so `'%s\n' a b c` is three lines. That is the loop most people write in `for`.
- `"'A"` — a quote then a character — is `printf`'s way of asking for a character's code, and `%d` of `0x41` is the same number the other way round.
- `xxd -r -p` is the only one of the dump tools that runs backwards, which is [the whole reason to install it ↗](https://masiarek.github.io/encodings-learning-library/11_Tools/xxd/index.html) on a machine that lacks it.
- A lot of one byte: `head -c 64 /dev/zero`, and a lot of one line: `yes | head -3`. `seq 3` for numbers.

## Seeing what you are about to feed

The program cannot tell you what it read, but the shell can show you what it is about to send. Put the inspection *before* the pipe:

```text title="Measured 2026-09-13 — the same bytes, five views"
$ printf 'a\tb\n' | cat -vet
a^Ib$

$ printf 'one\r\ntwo\r\n' | tr -d '\r' | cat -vet
one$
two$

$ printf 'a\nb' | wc -l | tr -d ' '
1

$ printf 'a\nb' | grep -c ''
2

$ printf 'hi\n' | file -
/dev/stdin: ASCII text

$ printf 'hi\n' | file --mime-type -
/dev/stdin: text/plain
```

- `cat -vet` draws the tab as `^I`, a carriage return as `^M`, and ends every line with `$`, so a trailing space and a missing final newline both become visible. It is the portable spelling; `cat -A` [does not exist on macOS ↗](https://masiarek.github.io/encodings-learning-library/06_Terminal/inspecting_a_file/index.html).
- `wc -l` counts newlines; `grep -c ''` counts lines. On a file whose last line has no newline they [differ by one ↗](https://masiarek.github.io/encodings-learning-library/06_Terminal/trailing_newline/index.html), and a Rust `lines()` loop agrees with `grep`.
- `file -` reads stdin and, having no name, prints `/dev/stdin`. Ask for `--mime-type`: the English wording [changes between versions and the MIME form does not ↗](https://masiarek.github.io/encodings-learning-library/06_Terminal/file_type_is_four_questions/index.html).
- `tee /dev/stderr` in the middle of a pipeline shows you the bytes going past without changing them — `printf 'x\n' | tee /dev/stderr | wc -l` prints `x` on the terminal and `1` on stdout.

## The first few bytes, and the first few lines

```text title="Measured 2026-09-13 — four spellings of the first four bytes of /etc/hosts, and od is the one that pads"
$ head -c 4 /etc/hosts | xxd
00000000: 2323 0a23                                ##.#

$ xxd -l 4 /etc/hosts
00000000: 2323 0a23                                ##.#

$ hexdump -n 4 -C /etc/hosts
00000000  23 23 0a 23                                       |##.#|
00000004

$ od -N 4 -An -tx1 /etc/hosts
           23  23  0a  23                                                
```

`head -c` cuts *bytes* and works on a pipe; `-l`, `-n` and `-N` are the same bound spelled three ways by three tools. Lines are `head -3` or `sed -n '1,3p'`, and both stop reading, which on a pipe means the producer sees a broken pipe — the event [Broken pipe](../../02_Errors/broken_pipe/README.md) is about, from the other side. The `od` line's padding is [not the same on Linux ↗](https://masiarek.github.io/encodings-learning-library/11_Tools/od/index.html), which is why the encodings library never records it raw.

## Transforming on the way in

```text title="Measured 2026-09-13 — the classic filters, one line each"
$ printf 'first\nsecond\nthird\n' | sed -n '1,2p'
first
second

$ printf 'hello\n' | sed 's/l/L/'
heLlo

$ printf 'hello\n' | sed 's/l/L/g'
heLLo

$ printf 'k=v\n' | cut -d= -f2
v

$ printf 'aXbXc' | tr X '\n'
a
b
c
$ printf '3\n1\n2\n' | sort -n | tr '\n' ' '
1 2 3 
$ seq 3 | paste -sd+ - | bc
6

$ printf 'a b\n' | xargs -n1 echo
a
b
```

- `sed 's/l/L/'` changes the *first* match on each line; `g` changes all of them. `sed -n 'N,Mp'` is `head` and `tail` with a range.
- `tr` maps **bytes** and `sed` matches **patterns**, which is the whole difference between them on anything outside ASCII — [`tr -d 'é'` damages `naïve` and `sed 's/é//'` does not ↗](https://masiarek.github.io/encodings-learning-library/11_Tools/sed/index.html). `cut -c` [counts characters on one platform and bytes on the other ↗](https://masiarek.github.io/encodings-learning-library/11_Tools/cut/index.html).
- Two of the outputs above have no trailing newline — `printf 'aXbXc'` had none and `tr` preserved that; `tr '\n' ' '` removed the last one — so the next `$` prompt lands on the same line. That is not a display glitch; it is the trailing-newline question again.
- `xargs` splits on spaces as well as newlines and [treats quotes as syntax ↗](https://masiarek.github.io/encodings-learning-library/11_Tools/xargs/index.html): `find -print0 | xargs -0` is the pairing for filenames, not a refinement.
- `grep` in a UTF-8 locale [may drop a line it cannot decode and exit 0 ↗](https://masiarek.github.io/encodings-learning-library/11_Tools/grep/index.html) on macOS; search a file of unknown encoding under `LC_ALL=C`.

## Ending the input

```text title="Measured 2026-09-13 — three ways the input is over, and one question about who is on the other end"
$ cat </dev/null | wc -c | tr -d ' '
0

$ printf 'x\n' | sh -c 'test -t 0 && echo terminal || echo not a terminal'
not a terminal

$ sh -c 'test -t 0 && echo terminal || echo not a terminal' </dev/null
not a terminal
```

- `</dev/null` is an input that is *already* over: the first read returns zero bytes. It is the way to run a program that reads stdin without giving it anything.
- At a keyboard, **Ctrl-D** at the start of a line ends the current read — the program's `read_line` returns `Ok(0)` — and a second Ctrl-D is needed if you typed something first, because the first one only flushes what is pending. On Windows it is **Ctrl-Z** and then Enter.
- **Ctrl-C** is not input at all. It is a signal, and the default action kills the program before it reads anything.
- `test -t 0` is the shell's `IsTerminal`: it says whether fd 0 is a terminal, and both fences above say *no* because both were fed. Run it bare at a prompt and it says yes. [A pipe is not a terminal ↗](https://masiarek.github.io/encodings-learning-library/06_Terminal/pipe_is_not_a_terminal/index.html) is what changes when the answer flips.

## stdin in the middle of a list

```text title="Measured 2026-09-13 — the - convention, and /dev/stdin as a path"
$ printf 'hi\n' | cat -
hi

$ printf 'mid\n' | cat <(printf 'top\n') - <(printf 'bottom\n')
top
mid
bottom

$ printf 'hi\n' | wc -l /dev/stdin
       1 /dev/stdin

$ diff <(printf 'a\nb\n') <(printf 'a\nc\n')
2c2
< b
---
> c
```

A lone `-` means stdin to nearly every tool that takes files, and it can sit *between* two real files. `/dev/stdin` is the same thing as a path, for the tool that insists on one. `<(cmd)` hands a tool a path that is secretly a pipe, which is how `diff` compares two commands' outputs without a temporary file — and it is [what your own tool should accept](../../03_Command_Line/a_file_or_stdin/README.md).

## The traps

```text title="Measured 2026-09-13 — echo -n is a flag in bash and two characters in sh"
$ echo -n hi | xxd -p
6869

$ sh -c 'echo -n hi' | xxd -p
2d6e2068690a
```

- **`echo -n` is not portable.** macOS's `/bin/sh` printed `-n hi` and a newline. `printf` is the fix, every time, and [the shell has no string type ↗](https://masiarek.github.io/encodings-learning-library/11_Tools/sh/index.html) is the page on why the shell is not the place to reason about bytes anyway.
- **A here-string adds a newline** (above). **`printf` does not** — so `printf abc | prog` sends a last line with no newline, which is the case the [101 page's kata](../../03_Command_Line/reading_stdin/README.md#practice) makes you handle.
- **`sed -i` has two spellings and no portable one** except `-i.bak` — [measured on BSD, GNU and busybox ↗](https://masiarek.github.io/encodings-learning-library/11_Tools/sed/index.html).
- **The locale is an input.** `LC_ALL=C` in front of a command makes `grep`, `tr`, `sed` and `sort` byte-oriented and identical on every machine; without it, [six variables decide what a character is ↗](https://masiarek.github.io/encodings-learning-library/06_Terminal/locale_and_lc_ctype/index.html), and the answer differs between a Mac and a container.
- **stdout is block-buffered when piped**, so a program's `dbg!` on stderr can appear *before* the `println!` that ran first — [What `dbg!` does](../../15_First_Programs/what_dbg_does/README.md), section 3.

## See also

- [Reading a line from standard input](../../03_Command_Line/reading_stdin/README.md) — the program on the other end of every pipe above
- [A file or stdin](../../03_Command_Line/a_file_or_stdin/README.md) — accepting `FILE`, `-` and nothing, from one function
- [Writing a file inspector](../../03_Command_Line/writing_a_file_inspector/README.md) — the tool these tricks feed and test
- [Byte tools](../byte_tools/README.md) — `xxd`, `hexdump`, `od`, `file` and their replacements, and what to install
- [Standard error, and exit status](../../02_Errors/stderr_and_exit_status/README.md) — the two streams going the other way, `2>&1`, and why order matters
- [The encodings library's toolbox chapter ↗](https://masiarek.github.io/encodings-learning-library/11_Tools/index.html) — every tool above, measured on macOS and Ubuntu

## Po polsku

Program czyta deskryptor 0 i nie ma pojęcia, co położyło tam bajty — klawiatura, potok (*pipe*), przekierowanie, dokument wbudowany (*here-document*) czy inny program — więc to powłoka jest miejscem, w którym bajty robi się celowo, ogląda dokładnie takimi, jakie są, i kończy wtedy, kiedy się chce. Pięć dróg na fd 0 różni się drobiazgami, które liczą się dokładnie wtedy, gdy program liczy: `<<< 'abc'` dokłada znak nowej linii (4 bajty, nie 3), `printf` nie dokłada niczego, a `echo -n` w `/bin/sh` na Macu wypisuje literalnie `-n`. `printf '\x48\x69'` nazywa **bajty** i nie pyta locale o nic, `xxd -r -p` zamienia szesnastkowy zapis z powrotem w bajty, a `cat -vet` pokazuje tabulator jako `^I`, powrót karetki jako `^M` i koniec każdego wiersza jako `$` — czyli to, czego żaden edytor nie rysuje. `wc -l` liczy znaki nowej linii, a `grep -c ''` liczy wiersze, i na pliku bez końcowego znaku nowej linii różnią się o jeden. `</dev/null` to wejście, które już się skończyło, Ctrl-D kończy bieżący odczyt, a Ctrl-C w ogóle nie jest wejściem, tylko sygnałem. `test -t 0` mówi, czy po drugiej stronie deskryptora 0 jest terminal — i w obu pomiarach powyżej mówi, że nie, bo oba były karmione potokiem.

**Szukaj po polsku:** przekierowanie standardowego wejścia · potok w powłoce · dokument wbudowany here-doc · `printf` bajty szesnastkowo · `cat -vet` niewidoczne znaki · `test -t 0 terminal` · `xxd -r -p`
