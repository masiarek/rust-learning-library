# Flags by hand

**Level:** 201 · working knowledge

**One line:** A flag is only an argument that starts with a dash; parsing one takes about ten lines, and the value of writing those ten is finding out exactly where the eleventh turns into a re-implementation of [`clap`](../clap_derive/README.md).

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The vocabulary, because the finished page needs it: positional argument, short flag, long flag, an option that takes a value, and a flag that is just a boolean switch
- A ten-line hand parser: loop the arguments, match on `-w` / `--words`, push everything else onto a list of filenames
- The four conventions that cost the next hundred lines — `--` ending flag parsing, `-abc` meaning three short flags, `--out=file` beside `--out file`, and a `-` that means standard input
- Storing the result in a struct rather than a pile of `bool`s, which is where [`Default`](../the_default_trait/README.md) earns its place
- The honest conclusion: when hand-rolling is right (one flag, no dependencies) and when it stops being right

## The trap it exists for

Hand-rolled flag parsing does not fail loudly; it fails by *quietly not implementing a convention* users assume. A program that treats `-` as a filename, or that swallows `--` as a flag, is wrong in a way no test written by its author will find — because its author knows what they meant to type.

## See also

- [Command-line arguments](../command_line_arguments/README.md) — where the strings come from
- [Deriving a parser with `clap`](../clap_derive/README.md) — the same job, declared instead of written
- [Testing a command](../testing_a_command/README.md) — how you find out that `--` case is broken

## Po polsku

Flaga to po prostu argument zaczynający się od myślnika, a największą wartością tej lekcji jest **słownictwo**, bo po polsku ono nie jest ustalone: *positional argument* to argument pozycyjny, *short* i *long flag* to flaga krótka i długa, *switch* Microsoft tłumaczy jako „przełącznik”, a całą resztę nazywa się u nas zbiorczo „parametrami”. Dochodzi do tego pamięć DOS-u — pokolenie wychowane na `dir /w` ma w głowie ukośnik, nie myślnik — więc konwencji uniksowych trzeba się tu nauczyć świadomie, zamiast zakładać, że się je zna. Są cztery i żaden polski poradnik nie wylicza ich w jednym miejscu: `--` kończy parsowanie flag, `-abc` znaczy trzy krótkie flagi naraz, `--out=plik` znaczy to samo co `--out plik`, a samotny `-` oznacza wejście standardowe. Właśnie dlatego ręcznie pisany parser zawodzi po cichu: program, który potraktuje `-` jak nazwę pliku albo połknie `--`, jest zepsuty w sposób, którego autor własnym testem nie wykryje — bo autor wie, co chciał wpisać.

**Szukaj po polsku:** flagi i przełączniki · argument pozycyjny · ręczne parsowanie argumentów · `posix utility argument syntax` · `gnu long options double dash`
