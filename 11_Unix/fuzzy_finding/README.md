# Fuzzy finding: three key bindings, and the setup that is not the install

**Level:** 101 → 201 · working knowledge

**One line:** `fzf` replaces *type the exact path* with *type four letters and pick from a list* — but installing it gives you a command, not the key bindings, and the key bindings are the entire reason to install it.

## The shape of it

Press **Ctrl-T** in the middle of typing a command. A filtered list of files opens, you type a fragment of the name — not a prefix, any fragment, in any order — and the path you pick is pasted into the command line where your cursor was.

```text
rustc --edition 2024 ▊          <- press Ctrl-T here, type "optvsres", pick
rustc --edition 2024 01_Foundations/option_vs_result/examples/option_vs_result.rs▊
```

Nothing is remembered, nothing is configured per project, and the list is the current directory tree. That is the whole tool: it reads lines on stdin, lets you narrow them interactively, and writes the chosen one to stdout. Everything else is a wrapper around that.

## Install

```sh
brew install fzf          # macOS
```

On Debian or Ubuntu it is `apt install fzf`; on Arch, `pacman -S fzf`. It is a single static binary with no runtime dependencies, which is also the answer for a machine where you cannot install anything: drop the release binary into `~/bin` and it works.

## The step that is not the install

`brew install fzf` puts `fzf` on your `PATH` and stops. The key bindings are a separate opt-in, and this is the point at which most people conclude fzf is "just a menu you have to pipe things into".

Add one line to your shell's startup file:

```fish
# ~/.config/fish/config.fish
fzf --fish | source
```

```bash
# ~/.bashrc
eval "$(fzf --bash)"
```

```zsh
# ~/.zshrc
source <(fzf --zsh)
```

Then open a new shell. Older fzf releases have no `--fish`/`--bash`/`--zsh` flag and ship the same code as files instead — `source (brew --prefix)/opt/fzf/shell/key-bindings.fish` and its `.bash` / `.zsh` siblings, which are still installed alongside the newer flag.

## The three bindings

Three keys, and each one answers a different question. They work *while you are typing a command* — you do not run fzf, you interrupt yourself with it.

| Key | Answers | What happens when you pick |
|---|---|---|
| **Ctrl-T** | "where is that file?" | the path is pasted where your cursor was, and you carry on typing |
| **Ctrl-R** | "what was that command I ran?" | the whole command line is replaced with the one you picked |
| **Alt-C** | "where is that directory?" | you `cd` there immediately — no pasting, it just moves you |

### Ctrl-T, step by step

You want to compile an example but cannot remember the folder.

1. Type `rustc --edition 2024 ` and stop, cursor still at the end.
2. Press **Ctrl-T**. The bottom half of the terminal fills with a list of every file under the current directory, and a `>` prompt appears.
3. Type `optvs`. The list narrows as you type. These are not the first letters of the filename — fzf matches those characters *in order, anywhere in the path*, so `optvs` finds `01_Foundations/option_vs_result/examples/option_vs_result.rs`.
4. Move with the arrow keys if more than one line survived, and press **Enter**.
5. The list disappears and your command line now reads `rustc --edition 2024 01_Foundations/option_vs_result/examples/option_vs_result.rs`, cursor after it. Nothing has run yet — you finish the command and press Enter yourself.

**Esc** at any point closes the list and leaves your command line untouched.

### Ctrl-R, step by step

This one replaces a habit rather than a command, and it is the one that pays for the install fastest.

1. Press **Ctrl-R** on an empty line.
2. Your shell history opens, most recent first.
3. Type `carg nex`. Both fragments have to appear, in that order, but nothing has to be adjacent — so this finds `cargo nextest run --workspace` even though the command does not start with either fragment.
4. **Enter** puts that command on your command line.

What this replaces depends on your shell, which is worth knowing before you decide it is a revelation. In **bash and zsh** the built-in Ctrl-R walks backwards through history matching a *substring*, one candidate at a time, and you press Ctrl-R again to step further back — fzf is a large win there. **fish already has a history pager** on that key (`history-pager`, a list you can filter), so what fzf adds is fuzzy matching instead of substring matching, and its ranking. You can check which one you have with `bind ctrl-r`: it names `fzf-history-widget` once the integration is loaded.

### Alt-C, step by step

1. Press **Alt-C** (hold Option on a Mac, press C).
2. A list of directories opens — directories only, no files.
3. Type a fragment, press **Enter**, and you are in that directory. There is no command to finish; the `cd` has already happened.

If nothing at all happens when you press it, that is the next section and it is not your fault.

## Point it at `fd`

fzf builds its own file list when you have not told it otherwise, and that walker skips exactly two directories: `.git` and `node_modules`. It has never read `.gitignore` and does not know what a `target/` is.

Set three variables and it uses [`fd`](../search_tools_in_rust/README.md) instead, which reads every `.gitignore` on the way down.

```fish
# ~/.config/fish/config.fish
set -gx FZF_DEFAULT_COMMAND "fd --type f --hidden --exclude .git"
set -gx FZF_CTRL_T_COMMAND $FZF_DEFAULT_COMMAND
set -gx FZF_ALT_C_COMMAND "fd --type d --hidden --exclude .git"
```

The bash and zsh forms are the same three names with `export`. `--hidden` puts dotfiles back in — `fd` skips them by default, which is the opposite of fzf's walker — and `--exclude .git` then removes the one hidden directory you never want to pick a file from.

Measured on this repository, August 2026:

| Ctrl-T list is built by | Entries offered |
|---|---|
| fzf's own walker (hidden included, `.git` and `node_modules` skipped) | 18,770 |
| `fd` with the settings above | 443 |

Two directories account for almost the entire difference: `.venv/` (17,099 files) and the built `site/` (480). Both are in `.gitignore`, so `fd` never enters them and fzf's walker has no reason not to.

This is not a speed argument — fzf narrows 18,770 lines instantly. It is that a list of 443 real files can be narrowed to one by typing three characters, and a list of 18,770 cannot: every fragment you type still matches forty files you have never opened.

**How to tell which one you are on**, without checking any config: fzf's walker lists **directories as well as files**, with a trailing `/`. `fd --type f` lists only files. If your Ctrl-T shows `examples/` on a line of its own, the variables above have not reached that shell yet.

The variables are read when you press the key, not when the shell starts, so it does not matter whether these lines come before or after the `fzf --fish | source` line — but they do have to be in a shell that has started since you wrote them.

## The macOS trap: Alt-C types a letter instead

Ctrl-T and Ctrl-R work the moment the integration is sourced. **Alt-C** usually does not, and it fails in a way that looks like nothing to do with fzf: pressing it inserts an accented character into your command line.

```text
Option-C  →  ç        # US layout
Option-C  →  ć        # Polish layout
```

That is macOS working as designed. The Option key is a compose key, and each layout maps it to its own accented letters, so the terminal never sends the Meta-C that fzf is listening for.

The fix is one terminal setting:

- **iTerm2** — Settings (⌘,) → Profiles → Keys → General → *Left Option key* → **Esc+**
- **Terminal.app** — Settings → Profiles → Keyboard → tick *Use Option as Meta key*
- **Ghostty**, **WezTerm**, **Alacritty** — Meta is on by default; nothing to change

**If you type a language that needs those characters, change the right Option key instead and leave the left one alone.** iTerm2 configures the two keys separately, so *Right Option key → Esc+* with *Left Option key → Normal* gives you Meta shortcuts on one thumb and `ć`, `ó`, `ż` on the other. Terminal.app has the same split under the same panel.

Worth knowing before you spend the setting: Alt-C is the least valuable of the three bindings. It fuzzy-matches directory names under the current directory only, so it is useful for descending a tree you are already standing in and useless for getting to a project somewhere else. A directory jumper that ranks by where you have actually been solves that instead, and Alt-C does not try to.

## A preview pane

`bat` is a `cat` that syntax-highlights and knows about git. Wired into Ctrl-T it shows the file under the cursor while you narrow the list:

```fish
set -gx FZF_CTRL_T_OPTS "--preview 'bat -n --color=always {}'"
```

`{}` is the currently highlighted line, substituted by fzf before running the command. Any command works there — `head -50 {}` needs nothing extra installed at all.

Skip this if you would rather not add a dependency. The preview is a genuine convenience when you are picking among six similarly-named files and a waste of a pane the rest of the time.

## Where it does not help

fzf narrows a list you already have. It has no index, learns nothing between runs, and cannot answer *which files mention this function* — that is [`rg`](../search_tools_in_rust/README.md), and the two compose rather than compete:

```sh
rg -l "fn main" | fzf     # narrow the files that matched, interactively
```

It is also not a project switcher. A fuzzy list of every directory under `~` is not more useful than a fuzzy list of every file under `~`; both are too long to narrow by typing. The tools that solve *that* keep a ranked list of where you have actually been.

## See also

- [What the Rust rewrites bought](../search_tools_in_rust/README.md) — `fd` and `rg` measured against the tools they replace, and the honest breakdown of where the speed comes from
- [Choosing an editor](../../05_Tooling/editors/README.md) — the same question one layer up: what a window costs you before it shows you a type
