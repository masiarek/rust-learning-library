# Neovim with LazyVim: a working Rust setup, and the second way it silently does nothing

**Level:** 201 · working knowledge

**One line:** Four commands give you a complete Rust IDE in Neovim — and there are two independent ways for it to install perfectly, look healthy, highlight your code, and never once start a language server, neither of which announces itself.

[Choosing an editor](../editors/README.md) is where the comparison lives, and where the first of those two traps was first written down. This page is the install itself, run end to end on this machine, and the second trap — which is not an editor problem at all, and which the first trap's fix will not touch.

## The install

```sh
brew upgrade neovim                                      # see trap 1
git clone https://github.com/LazyVim/starter ~/.config/nvim
rm -rf ~/.config/nvim/.git
nvim --headless "+Lazy! sync" +qa                        # a few minutes, unattended
```

Then enable the Rust extra. Interactively that is `:LazyExtras`, tick `lang.rust`, restart; non-interactively it is one line in `~/.config/nvim/lua/config/lazy.lua`, before the `{ import = "plugins" }` entry:

```lua
{ import = "lazyvim.plugins.extras.lang.rust" },
```

That pulls in `rustaceanvim` (the Rust front end — macro expansion, MIR and HIR views, run-this-test), `crates.nvim` (dependency versions inline in `Cargo.toml`), and the Rust and RON tree-sitter parsers.

Measured on this machine, August 2026: **34 plugins, 153 MB, 29 tree-sitter parsers compiled.** `codelldb` aborts its download when a headless session exits — an artifact of installing this way, finished by the first interactive launch.

**Nothing here is destructive if `~/.config/nvim` does not already exist** — and it is worth checking that it does not, because the clone above would land on top of it. If you have a config you care about, install into a throwaway one instead and nothing is touched:

```sh
git clone https://github.com/LazyVim/starter ~/.config/lazyvim-trial
NVIM_APPNAME=lazyvim-trial nvim
```

To undo the whole thing: `rm -rf ~/.config/nvim ~/.local/share/nvim`.

## Trap 1: the Neovim version floor

`rustaceanvim` requires **Neovim 0.12 or above**. On 0.11 the extra installs perfectly, compiles all 29 parsers, and then never starts a server. You still get syntax highlighting, because tree-sitter does not care about the version, so the editor looks completely healthy while giving you no types, no diagnostics and no hints.

The only sign is one line, printed once, in two hundred lines of parser downloads:

```text
rustaceanvim requires Neovim 0.12 or above
```

This machine was on 0.11.5 — exactly the broken version — and Homebrew had 0.12.4 available. `brew upgrade neovim` is the fix, and *owning a version floor that moves underneath you* is the real cost of this path.

## Trap 2: a minimal toolchain has no `rust-analyzer`

This one is new, and it survives fixing trap 1.

If the project you open is [pinned](../pinning_the_toolchain/README.md) with `profile = "minimal"`, the toolchain contains `rustc`, `cargo`, `rust-std` and whatever components you listed. `rust-analyzer` is a **component**, and if it is not on that list it is not there — even though it is installed for `stable` and works everywhere else on the machine.

What happens then is worse than a missing binary, because the [rustup shim](../rustup/README.md) is on your `PATH` under that name. The shim looks for the component, does not find it, and falls back — to itself:

```text
info: `rust-analyzer` is unavailable for the active toolchain
info: falling back to "/usr/local/opt/rustup/bin/rust-analyzer"
info: `rust-analyzer` is unavailable for the active toolchain
...
error: infinite recursion detected
```

The editor reports only `Client rust-analyzer quit with exit code 1`. Every symptom matches trap 1 — highlighting fine, no server, no types — so having fixed the Neovim version you will conclude the fix did not work.

The fix is one word in the toolchain file:

```toml
[toolchain]
channel = "1.97.1"
profile = "minimal"
components = ["rustfmt", "clippy", "rust-analyzer"]
```

Measured cost of adding it: **657 MB → 697 MB.** Forty megabytes to make the editor work, which settles that argument. The general lesson is worth more than the fix: `profile = "minimal"` is a claim about what a *build* needs, and your editor is not a build.

## Verifying, rather than hoping

Both traps produce a healthy-looking editor, so "it opens and the code is coloured" proves nothing. Ask the editor directly instead — open a Rust file in a real Cargo project and check what attached:

```vim
:checkhealth lazyvim
:lua =vim.lsp.get_clients({ bufnr = 0 })
```

An empty list is the failure. A non-empty one naming `rust-analyzer` is the success, and the quickest confirmation is a deliberate type error: put `let wrong: u8 = "not a number";` in `main` and you should see three diagnostics, one of them from `rust-analyzer` itself:

```text
LSP clients: rust-analyzer
diagnostics: 3
  - rust-analyzer: expected u8, found &'static str
  - rustc: mismatched types
  - rustc: expected due to this
```

That is the state this setup reached after both fixes, on Neovim 0.12.4.

## Should you learn vim at all?

Worth separating from the install, because they are different decisions and only one of them is about editors.

**Age is not the variable.** The variable is what you are optimising for right now. Modal editing is a second skill, unrelated to Rust, with its own multi-week trough where you are slower at everything you used to be fast at. Learning two things at once means neither gets a clean signal: when something is frustrating, you will not know whether it was the borrow checker or the keystroke.

So the honest answer is not *too late*, it is **wrong order**. If you have a working [RustRover](../rustrover_setup/README.md), learn Rust in it, where the friction you feel is Rust's.

If you want the motions anyway — and they are genuinely good — there is a version with no project attached: **turn on vim keybindings inside the editor you already use.** RustRover has IdeaVim, VS Code has a vim mode. You get modal editing with the safety net, you can switch it off on a bad afternoon, and you are not simultaneously maintaining a Lua configuration and a version floor. If the motions stick after a month, *then* the full setup above is a small step rather than a leap.

## See also

- [Choosing an editor](../editors/README.md) — the six-way comparison, `rustaceanvim`'s capabilities, and the shim trap in its general form
- [RustRover setup](../rustrover_setup/README.md) — the same job in the IDE, including vim keybindings without the project
- [Pinning the toolchain](../pinning_the_toolchain/README.md) — where trap 2 comes from, and what `profile` really decides

## Po polsku

Cztery polecenia i masz w Neovimie kompletne środowisko do Rusta — z tym że obie opisane wyżej pułapki kończą się dokładnie tym samym obrazem: edytor wstaje, kod ładnie się koloruje, a serwer języka (*language server*) nigdy nie startuje. Warto rozdzielić te dwie warstwy raz na zawsze, bo ich mylenie jest źródłem całego nieporozumienia. **Kolorowanie składni** robi tree-sitter, lokalnie, nie pytając nikogo o zdanie, i działa nawet wtedy, gdy nie działa nic innego; **typy, diagnostyka i podpowiedzi w kodzie** przychodzą z `rust-analyzer` po LSP. „Ładnie się koloruje” nie jest więc żadnym dowodem — to najsłabszy sygnał, jaki ten edytor potrafi wysłać.

Pierwsza pułapka to próg wersji: `rustaceanvim` wymaga Neovima **0.12 lub nowszego**, a na 0.11 instaluje się bezbłędnie i po prostu milczy — jedyny ślad to jedna linijka w dwustu linijkach logu. Druga jest podstępniejsza, bo **przeżywa naprawienie pierwszej** i z edytorem nie ma nic wspólnego. Jeśli projekt ma przypięty toolchain z `profile = "minimal"`, to `rust-analyzer` — który jest osobnym *komponentem*, a nie częścią kompilatora — w tym toolchainie po prostu nie istnieje, choć dla `stable` na tej samej maszynie działa bez zarzutu. Wtedy `shim` rustupa znajduje pod tą nazwą sam siebie i kończy na `error: infinite recursion detected`; to komunikat **rustupa**, nie twojego programu, więc nie szukaj przepełnienia stosu w kodzie. Morał jest ogólniejszy niż sama poprawka: `profile = "minimal"` mówi, czego potrzebuje *budowanie*, a edytor to nie budowanie — dopisz `"rust-analyzer"` do `components` i zapłać za to czterdziestoma megabajtami.

Skoro oba objawy wyglądają jak zdrowy edytor, pytaj wprost: `:lua =vim.lsp.get_clients({ bufnr = 0 })` na otwartym pliku w prawdziwym projekcie Cargo — pusta lista to porażka. I jeszcze pytanie, które wśród polskich programistów wraca regularnie: czy w ogóle warto uczyć się vima. Odpowiedź tej strony nie brzmi „za późno”, tylko **„zła kolejność”** — edycja modalna to osobna umiejętność z własnym kilkutygodniowym dołkiem, a ucząc się jej równolegle z Rustem nie odróżnisz, czy dana frustracja pochodzi od borrow checkera, czy od skrótu klawiszowego. Same ruchy vima można włączyć w edytorze, który już masz (IdeaVim w RustRoverze), i to jest rozsądniejsza kolejność.

**Szukaj po polsku:** edycja modalna · konfiguracja Neovima pod Rusta · `rustaceanvim requires Neovim 0.12` · `rust-analyzer is unavailable for the active toolchain` · `rustup infinite recursion detected`
