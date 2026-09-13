# Byte tools

**Level:** 201 · for anyone with a terminal

**One line:** `xxd`, `hexdump`, `od`, `strings` and `file` are the five programs a file inspector imitates, and the tools that claim to replace them fall onto four shelves — a prettier dump, a better `file`, a text auditor, and a reverse-engineering suite — of which three of the dumps and two of the type detectors are written in Rust; every row below was checked on 2026-09-13, and the install lines are for this Mac.

The five originals are documented in the encodings library, one measured page each, because what each *decides before printing a line* is the interesting part and none of them says: [`hexdump` ↗](https://masiarek.github.io/encodings-learning-library/11_Tools/hexdump/index.html), [`xxd` ↗](https://masiarek.github.io/encodings-learning-library/11_Tools/xxd/index.html), [`od` ↗](https://masiarek.github.io/encodings-learning-library/11_Tools/od/index.html), [`strings` ↗](https://masiarek.github.io/encodings-learning-library/11_Tools/strings/index.html), [`file` ↗](https://masiarek.github.io/encodings-learning-library/06_Terminal/file_guesses/index.html). This page is the shelf beside them: what else exists, what it is written in, what it costs, and the one line that installs it.

**Stars are a size, not a verdict.** They are GitHub's count on 2026-09-13, and they are here so that "a modern alternative to `xxd`" can be read as *56 people's* alternative or *10,000 people's* — which changes what a bug report gets you, and nothing else. Licences are what the repository or the Homebrew formula declares.

## Hex viewers, and three editors

| Tool | What it adds over `xxd` | Written in | Licence | Stars | Install |
|---|---|---|---|---|---|
| [hexyl ↗](https://github.com/sharkdp/hexyl) | colour by byte class (NUL, ASCII, non-ASCII, whitespace), borders, `--length`/`--skip` with `KiB` suffixes | Rust | Apache-2.0 or MIT | 10,270 | `brew install hexyl` — already here |
| [hevi ↗](https://codeberg.org/arnauc/hevi) | colour, and parsers that highlight the fields of an ELF or PE header | Zig | GPL-3.0 | 279 on the archived GitHub mirror; the code moved to Codeberg | `brew install hevi` |
| [hexxy ↗](https://github.com/sweetbbak/hexxy) | colour palettes, `xxd`-compatible flags | Go | MIT | 56 | `go install github.com/sweetbbak/hexxy@latest` |
| [tinyxxd ↗](https://github.com/xyproto/tinyxxd) | a drop-in `xxd` that does not come with vim, in one C file | C | the repository says MIT or GPL-2; GitHub cannot read it | 63 | build from source |
| [fasthex ↗](https://github.com/CallMeAlphabet/fasthex) | speed on large files, `xxd`-style output | Rust | Apache-2.0 | 13 | `cargo install fasthex` |
| [hextazy ↗](https://github.com/0xfalafel/hextazy) | a terminal *editor*, not a viewer: change the bytes in place | Rust | MIT | 70 | `cargo install hextazy` |
| [turbohex ↗](https://github.com/nmbr7/turbohex) | a TUI viewer with WASM decoder plugins | Rust | MIT | 2 | build from source |
| [ImHex ↗](https://github.com/WerWolv/ImHex) | a GUI editor for reverse engineers: a pattern language that parses a format and colours its fields | C++ | GPL-2.0 | 54,789 | `brew install --cask imhex` |
| [010 Editor ↗](https://www.sweetscape.com/010editor/) | the commercial GUI editor: *binary templates*, a scripting engine, a licence fee | — | commercial | — | `brew install --cask 010-editor` installs the trial |
| HxD | the Windows freeware editor | — | freeware | — | Windows only |

Three things a viewer can do that `xxd` cannot, and each is a decision the [inspector page](../../03_Command_Line/writing_a_file_inspector/README.md) asks about: colour a byte by what *kind* of byte it is (which is `IsTerminal` and a palette), parse a header and name its fields (which is a magic-number table with offsets), and write bytes back (which is `xxd -r`, and the only one of the three the original does).

## Type detectors

| Tool | What it adds over `file` | Written in | Licence | Stars | Install |
|---|---|---|---|---|---|
| [libmagic ↗](https://www.darwinsys.com/file/) | it *is* `file`: the library and the magic database the command reads | C | BSD-2-Clause | — | `brew install libmagic`; the command is on every Mac already |
| [wiza ↗](https://github.com/qjerome/magic-rs) | JSON output, its own rule format, and a report when a file matches two formats at once | Rust | BSD-2-Clause | 36 | `cargo install wiza` |
| [libmagic-rs ↗](https://github.com/EvilBit-Labs/libmagic-rs) | a pure-Rust reading of the magic database, as a library | Rust | Apache-2.0 | 6 | `cargo add libmagic-rs` in a project |
| [zmime ↗](https://github.com/avac74/zmime) | a dependency-free detector, as a Zig library | Zig | MIT | 4 | build from source |
| [Detect It Easy ↗](https://github.com/horsicq/Detect-It-Easy) | signatures for executables — compiler, packer, linker — and a GUI that draws the file's layout | C++ and JavaScript | MIT | 11,524 | a download from the releases page; no Homebrew cask |

The question they all answer is [*what is in it*, the third of four ↗](https://masiarek.github.io/encodings-learning-library/06_Terminal/file_type_is_four_questions/index.html); `file` answers it in English that [changes between versions ↗](https://masiarek.github.io/encodings-learning-library/06_Terminal/file_type_is_four_questions/index.html#the-english-wording-is-not-stable-the-mime-type-is), which is the case for JSON output.

## Text auditors

| Tool | What it does | Written in | Licence | Stars | Install |
|---|---|---|---|---|---|
| [enca ↗](https://cihar.com/software/enca/) | guesses the encoding of a text file by language statistics, and converts | C | GPL-2.0 | — | `brew install enca` |
| [uni ↗](https://github.com/arp242/uni) | names every code point in a string, and finds them by name — [the encodings library's tool of choice ↗](https://masiarek.github.io/encodings-learning-library/11_Tools/uni/index.html) | Go | MIT | — | `brew install uni` — already here |
| [straudit ↗](https://github.com/kriskimmerle/straudit) | one script that reports invisible characters, homoglyphs, BiDi overrides and mixed line endings | Python | MIT | 0 | copy the script |
| [ovaltinepy ↗](https://github.com/ghostescript/ovaltinepy) | encode, decode and hash from the command line | Python | none stated | 0 | `pip install ovaltinepy` |
| [trice ↗](https://github.com/rokath/trice) | not an auditor: a `printf` replacement for *embedded* C, where the format string stays on the host and only an id crosses the wire | Go | MIT | 993 | not for this shelf |

`enca` guesses where [`file` proves a negative ↗](https://masiarek.github.io/encodings-learning-library/06_Terminal/file_guesses/index.html) — it will say a file is Polish in ISO-8859-2 where `file` can only say it is not UTF-8. `trice` is on the list because it turns up in searches for "printf alternative"; it is a tracing tool, and a good one, for a different job.

## Reverse engineering

| Tool | What it is | Written in | Licence | Stars | Install |
|---|---|---|---|---|---|
| [Ghidra ↗](https://github.com/NationalSecurityAgency/ghidra) | a disassembler *and decompiler* for dozens of architectures, from the NSA | Java | Apache-2.0 | 74,890 | `brew install ghidra` — brings `openjdk@21` |
| [radare2 ↗](https://github.com/radareorg/radare2) | the command-line reverse-engineering framework: disassembly, patching, scripting | C | LGPL-3.0 | 24,791 | `brew install radare2` |
| [rizin ↗](https://github.com/rizinorg/rizin) | the 2020 fork of radare2, with the Cutter GUI | C | LGPL-3.0 | 3,892 | `brew install rizin` |
| [binwalk ↗](https://github.com/ReFirmLabs/binwalk) | scans a firmware image for embedded files by magic number and carves them out; rewritten in Rust for version 3 | Rust | MIT | 14,335 | `brew install binwalk` |
| [Binary Ninja ↗](https://binary.ninja/) | a commercial decompiler with a free tier | — | commercial | — | `brew install --cask binary-ninja-free` |
| [IDA ↗](https://hex-rays.com/ida-free) | the industry's disassembler; IDA Free for non-commercial use | — | commercial | — | a download from Hex-Rays; no Homebrew cask |

All six start where a dump ends: the bytes are machine code, and the question becomes *what does it do*, which no hex viewer answers. `binwalk` is the one that belongs to this arc — a magic-number scanner over a whole image, and version 3 is a Rust program.

## Installing them here

The open-source command-line tools, in one line each way; every one is reversible with `brew uninstall` or `cargo uninstall`:

```bash
brew install hevi enca radare2 binwalk
```

```bash
cargo install fasthex wiza hextazy
```

The GUI editors and the decompiler:

```bash
brew install --cask imhex binary-ninja-free && brew install ghidra
```

`hexyl`, `uni`, `xxd`, `hexdump`, `od`, `file` and `strings` are already on this machine. 010 Editor is a paid product with a trial cask; IDA Free and Detect It Easy are downloads from their vendors' pages; hexxy needs a Go toolchain and tinyxxd, turbohex and zmime a compiler for their language.

## See also

- [Writing a file inspector](../../03_Command_Line/writing_a_file_inspector/README.md) — the questions every tool on this shelf answered its own way
- [Feeding stdin](../feeding_stdin/README.md) — the shell side of getting bytes into any of them
- [What the Rust rewrites bought](../search_tools_in_rust/README.md) — `fd`, `rg` and `bat`: the same question for the search tools
- [The five worth installing ↗](https://masiarek.github.io/encodings-learning-library/11_Tools/worth_installing/index.html) — `hexyl` and `uni` measured against what a Mac already has
- [Inspecting a file ↗](https://masiarek.github.io/encodings-learning-library/06_Terminal/inspecting_a_file/index.html) — the originals inside a session

## Po polsku

`xxd`, `hexdump`, `od`, `strings` i `file` to pięć programów, które naśladuje każdy inspektor plików, a narzędzia, które chcą je zastąpić, układają się na czterech półkach: ładniejszy zrzut szesnastkowy (`hexyl`, `hevi`, `hexxy`, edytory ImHex i 010 Editor), lepszy `file` (`wiza`, `libmagic-rs`), audytor tekstu (`enca`, `uni`, `straudit`) i zestaw do inżynierii wstecznej (Ghidra, radare2, binwalk). Trzy ze zrzutów i dwa z detektorów typów są napisane w Ruście — `hexyl` i nowy `binwalk` to najczęściej instalowane z nich. Liczba gwiazdek w tabelach to rozmiar, nie ocena: „nowoczesna alternatywa dla `xxd`” może znaczyć 56 osób albo 10 tysięcy, a różnica jest w tym, co dostaniesz po zgłoszeniu błędu. Jedno narzędzie z tej listy — `enca` — potrafi to, czego `file` nie umie: powiedzieć, że plik jest po polsku w ISO-8859-2, tam gdzie `file` może tylko stwierdzić, że nie jest w UTF-8. Każda pozycja była sprawdzona 13 września 2026 (Homebrew, crates.io, GitHub), a linie instalacyjne są dla tego Maca.

**Szukaj po polsku:** przeglądarka szesnastkowa · edytor hex na Maca · alternatywa dla xxd · rozpoznawanie typu pliku · `hexyl` · `binwalk` · `ghidra`
