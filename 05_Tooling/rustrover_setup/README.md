# RustRover: wiring the IDE to the rest of these pages

**Level:** 101 → 201 · working knowledge

**One line:** Everything the other tooling pages set up in files — the pinned compiler, the lint policy, the workspace — RustRover reads on its own; the three things it does *not* do by default are run clippy instead of `cargo check`, know which package in a workspace you meant, and print a path in a warning that names the file. Two more are the window's own defaults rather than Rust's: a documentation panel that renders itself unreadable, and a Build panel that reopens on every run.

This page is setup, not selection: [Choosing an editor](../editors/README.md) is where the comparison lives, along with the toolchain-location trap that costs everyone an hour once. What follows assumes you have picked RustRover and want it agreeing with the [practice tree](../practice_workspace/README.md).

## 1. Make clippy the linter, not `cargo check`

**Settings → Rust → External Linters.** Five controls, of which three are decisions:

| Control | What it decides |
|---|---|
| **Run external linter on the fly** | whether it runs in the background, or only when you ask |
| **External tool** | **Cargo Check** or **Clippy** — the default is Cargo Check |
| **Additional arguments** | *what gets linted* — appended to `cargo check` / `cargo clippy` |
| **Channel** | which toolchain runs it; `[default]` is whatever `rustup` resolves |
| **Environment variables** | passed to the linter process only |

Switch the tool to Clippy. `cargo check` answers *does this compile*, which rust-analyzer is already telling you inside the editor; clippy answers *is this right*, which nothing else in the window is saying. If you have adopted a [strict lint policy](../strict_lints/README.md), this is the setting that makes the IDE enforce it — otherwise you write against one standard all afternoon and meet the other at commit time.

To check what is actually active without reopening settings, hover the **linter widget in the status bar** at the bottom of the window; it names the linter and whether on-the-fly is on.

One caveat worth knowing before you turn on-the-fly analysis on: it runs a real `cargo clippy`, so it takes a build lock and can save files to do it. On a large project that is a noticeable background cost, and turning it off does not lose you the feature — it becomes manual instead.

Whichever linter is selected, it also reports rustc's own warnings — so a scratch file that binds five names to show five forms gets five `unused_variable`s in the console before clippy has said anything. That is a project-file decision rather than an IDE one: [the four lints a practice tree turns off](../scaffolding/README.md), and the one in the same group it must not.

### What goes in *Additional arguments*

`--all-targets`, and probably nothing else.

That box is **scope, not policy**. It decides which code the linter compiles, and no manifest key can do that job — while everything you might otherwise be tempted to type there is policy, and policy belongs in a file your colleagues and CI can also read:

| The question | Where it is answered |
|---|---|
| *which* lints, at what level | `[lints.clippy]` in `Cargo.toml` — or `[workspace.lints.clippy]` plus `lints.workspace = true` in each member |
| a lint's threshold or carve-out | `clippy.toml` — `allow-unwrap-in-tests`, `too-many-arguments-threshold`, `msrv` |
| *which code* is looked at | **this box** |

The two are genuinely orthogonal, which is easiest to see by setting a policy and then watching it not reach your tests. With `len_zero = "deny"` in `[lints.clippy]`, and one offending line in the library and another inside `#[cfg(test)] mod tests`:

```text
$ cargo clippy
error: length comparison to zero
 --> src/lib.rs:2:5
error: could not compile `clippy_targets` (lib) due to 1 previous error

$ cargo clippy --all-targets
error: length comparison to zero
 --> src/lib.rs:2:5
error: could not compile `clippy_targets` (lib) due to 1 previous error
error: length comparison to zero
  --> src/lib.rs:12:17
error: could not compile `clippy_targets` (lib test) due to 2 previous errors
```

The policy was in force both times. The second run compiled a target the first one never built — note the `(lib test)` in the last line, which is the test target appearing for the first time. Denying a lint harder cannot reach code the linter is not looking at, so this flag is the only thing that makes the carve-outs in `clippy.toml` mean anything: [an `unwrap` allowed in tests](../strict_lints/README.md) is a rule about code your editor is otherwise not reading.

Two things not to put in there. **`-D warnings`** is the CI form — in the IDE it repaints every warning as an error, and it is a lint level, so it belongs in the manifest anyway. And **individual `-W clippy::…` flags**, for the same reason: they are invisible to everyone who does not use your IDE, including you at the next `git clone`.

**Channel** is best left at `[default]`, so the editor runs the same clippy as [your `rust-toolchain.toml`](../pinning_the_toolchain/README.md) and as CI. Pointing it at [nightly](../nightly/README.md) means the window enforces lints that stable does not, which is the [formatting disagreement](../formatting/README.md) again in a different tool.

*The two transcripts above are real clippy output, captured from a throwaway `cargo new --lib` project on stable. Like the ones on the strict-lints page they are verified once rather than on every commit, since `tools/run_examples.py` compiles single `.rs` files and does not invoke clippy.*

## 2. Or run it in the terminal, which is the same thing

The IDE is calling these; there is no separate "IDE clippy":

```sh
cargo clippy                       # the library/binary
cargo clippy --all-targets         # ...and tests, benches, examples
cargo clippy --fix                 # apply the mechanical suggestions
cargo clippy -- -D warnings        # the CI form: any warning fails
```

`--all-targets` is the one to get in your fingers. Without it, clippy skips your test code — which is exactly where the [test carve-outs](../strict_lints/README.md) are supposed to be proving themselves.

## 3. Teach it which package you meant

This is the one real friction of a [workspace](../practice_workspace/README.md). `cargo run` at the root does not know whether you meant `ex01_hello` or `ex07_lifetimes`, and RustRover inherits the ambiguity.

RustRover creates a **run configuration** per binary it finds, so after the first build there is a dropdown in the toolbar with every exercise in it, and switching exercises is choosing from that list rather than typing `-p`. Two things make it pleasant:

- **Pin the one you are working on** and use the run shortcut, rather than picking from the dropdown each time.
- A configuration is just a saved `cargo` command — if you want lint-then-test-then-run on one key, make one whose command is `run -p ex01_hello` and add a **Before launch** step for clippy.

The terminal equivalent, for the same reason:

```sh
cargo run -p ex01_hello
```

## 4. Make a warning name the file, not `src/main.rs`

A warning in the run console points at a span like this:

```text
warning: unused variable: `a`
 --> src/main.rs:5:9
```

`src/main.rs` is not where the file is. It is where the file is *relative to rustc's own working directory*, and Cargo sets that to the workspace root no matter which directory you invoked it from — so every project on the machine reports the same seven characters, and two IDE windows say `src/main.rs` about two different files.

One line in `.cargo/config.toml`, at the root of the tree, spells it out instead:

```toml
[build]
rustflags = ["--remap-path-prefix==/Users/you/RustroverProjects/untitled/"]
```

```text
warning: unused variable: `a`
 --> /Users/you/RustroverProjects/untitled/src/main.rs:5:9
```

The double `==` is not a typo, and it is the whole trick. [`--remap-path-prefix=OLD=NEW` ↗](https://doc.rust-lang.org/rustc/command-line-arguments.html#--remap-path-prefix-remap-source-names-in-output) is being given an **empty** OLD, and the match is by path *component* rather than by text: an empty prefix matches the start of every **relative** path and no absolute one at all. So your files get spelled out and the ones under `~/.cargo/registry` and in `std` — already absolute — pass through untouched. A trailing slash on the target is optional for the same reason.

It applies to clippy as much as to `cargo build`, which matters given section 1: `cargo clippy` reads `build.rustflags` like any other command, and `cargo fix` still applies its suggestions through the rewritten paths.

Three things to know before adopting it:

- **The path is written out because rustflags interpolate nothing** — there is no `$PWD` to reach for. Move or rename the tree and every diagnostic will confidently name the old location, which is worse than a relative path rather than better. That is why [`rust_scaffold.py`](../scaffolding/README.md) gitignores the file and its `doctor` fails when the two disagree.
- **A `RUSTFLAGS` environment variable replaces this wholesale** rather than adding to it, so the remap silently vanishes for any command run with one set. That is the usual explanation for a setting that appears not to apply.
- **`file!()` and panic locations become absolute too.** A backtrace naming a file you can open is an improvement; a recorded answer key containing a panic location is a re-record.

There is one honest caveat about when this is worth doing. Inside a [workspace](../practice_workspace/README.md) the relative form is less useless than it looks — the span reads `exercises/ex01_hello/src/main.rs`, because rustc runs from the workspace root and the member directory is already part of the path. It is the standalone `cargo new` project — the kind RustRover makes when you want to try something out, named `untitled` — where the span collapses to `src/main.rs` and names nothing at all.

What a fresh compile *does* print, and a cached one does not, is worth knowing alongside it:

```text
   Compiling untitled v0.1.0 (/Users/you/RustroverProjects/untitled)
```

That line carries the full path already. It appears only when something was actually compiled, so the run that surprises you — warnings replayed from cache, `Finished in 0.02s`, and no `Compiling` line above them — is exactly the run with nothing but `src/main.rs` on screen. `cargo build -v` prints `Fresh untitled v0.1.0 (…)` in its place, at the cost of the whole rustc command line with it.

## 5. Make the documentation window readable

`F1` on a symbol opens the documentation, and it can come up as a dark grey panel with the code samples inside it in dark red and dark blue. Nothing is broken: **the panel's background comes from the IDE theme, and the code fragments inside it from the editor color scheme.** Those are two settings, and only one of them follows macOS.

*Settings → Appearance & Behavior → Appearance*:

| Control | What it sets |
|---|---|
| **Theme:** | window chrome — menus, tabs, tool windows, and the documentation panel's background |
| **Editor color scheme:** | syntax colours, in the editor *and* inside the documentation panel |
| **Sync with OS** | whether the **theme** follows the system light/dark setting |

So: macOS in dark mode, *Sync with OS* on, and a light editor scheme picked deliberately — the theme half flips, the scheme half stays, and light-scheme syntax colours end up painted on a dark panel. Two files say which is which, faster than opening the dialog, both under `~/Library/Application Support/JetBrains/RustRover<version>/options/`:

```xml
<!-- laf.xml -->
<laf themeId="Islands Dark" />

<!-- colors.scheme.xml -->
<global_color_scheme name="_@user_Github" />
```

`_@user_Github` is a user-edited copy of the Github scheme, and its `parent_scheme` is `Default` — light. One line dark, one line light is the diagnosis.

**The trap is in the fix.** Choosing a new **Theme:** also rewrites **Editor color scheme:** to that theme's default — silently, in the same dialog, before you press Apply — so a scheme you chose on purpose goes away and takes its font size with it. Set the scheme back on the same visit. The pairing is then remembered per theme:

```xml
<!-- laf.xml -->
<lafs-to-previous-schemes>
  <laf-to-scheme laf="Islands Dark" scheme="_@user_Github" />
</lafs-to-previous-schemes>
```

To have both halves follow macOS rather than pinning one polarity, the cog beside **Sync with OS** opens *Preferred Theme and Editor Color Scheme*, which carries **For Light OS** and **For Dark OS** rows for the scheme as well as the theme. Filling in only the theme rows is the state above.

## 6. Install a theme the IDE did not ship with

Before installing anything, look at what is already inside, because the two settings section 5 separates ship two very different-sized lists.

**Themes** — eleven files in `Contents/lib/intellij.platform.ide.impl.jar`, of which the New UI's light ones are just **Light**, **Light with Light Header** and **Islands Light**. (`IntelliJ` and `IntelliJ Light` are their classic-UI ancestors; the dark side is **Dark**, **Darcula**, **Islands Dark**, **Islands Darcula** and **High Contrast**.) Three light choices is a short list, and it is the reason people go shopping.

**Editor color schemes** — fourteen more, and `parent_scheme` sorts them exactly as it did for `_@user_Github` above: `Default` is light, `Darcula` is dark.

| Light schemes, bundled | Dark schemes, bundled |
|---|---|
| Dawn · **Github** · Solarized Light · Xcode | All hallow's eve · Blackboard · Cobalt · Espresso · Monokai · Railscasts · Solarized Dark · Twilight · VibrantInk · WarmNeon |

Worth checking before you install anything, because three of those four light schemes are names people reach for the Marketplace to get. If what you actually dislike is the *syntax colours* rather than the window chrome, you already own the fix — and it costs no plugin and no restart.

### The install itself

*Settings → Plugins*, the **Marketplace** tab (its neighbour is **Installed**), search the name, **Install**. A restart is usually optional for a theme — Catppuccin's own instructions say *"(Optional) Restart your IDE"* — and the dialog offers a **Restart IDE** button when it is not.

The new theme then joins the dropdown at *Appearance & Behavior → Appearance* under **Theme:**, where section 5's trap applies to every install alike: picking it also rewrites **Editor color scheme:**, to whatever the plugin declares. A plugin carrying both the `Theme` and `Editor Color Schemes` tags on its Marketplace page ships both halves and looks right immediately; a `Theme`-only one changes the chrome and leaves your syntax colours behind. Catppuccin is explicit about the split and sends you to two different pages for it — *Appearance & Behaviour → Appearance* for the UI, *Editor → Color Scheme* for the code.

Two paths show what happened, in the folder section 5 already reads:

```text
~/Library/Application Support/JetBrains/RustRover2026.2/
├── plugins/<plugin>/     ← the theme itself, unpacked
└── options/laf.xml       ← <laf themeId="…" /> — the one now in force
```

### What the community actually installs

Downloads and ratings from the [JetBrains Marketplace ↗](https://plugins.jetbrains.com/search?tags=Theme) API on 2026-08-30, restricted to plugins that verifiably contain a light theme:

| Plugin | The light half | Downloads | Rating |
|---|---|---|---|
| Gerry Themes | `Gerry⟡ Light` — and an **`[Islands] Gerry⟡ Light`** | 3.72M | **4.94** |
| Solarized Theme | Solarized Light | 2.45M | 4.39 |
| Catppuccin Theme | **Latte**, one of four flavours | 2.13M | 4.81 |
| Atom One Theme | One Light | 1.02M | 4.85 |
| Xcode Theme | Xcode Light | 923k | 4.87 |
| Cyan Light Theme | the whole plugin | 574k | 4.83 |
| Gruvbox Theme | gruvbox light | 546k | 4.83 |
| macOS Light Theme | the whole plugin | 188k | 4.70 |
| Everest Theme | `Everest Nature - Light` and siblings | 117k | 4.80 |
| Falcon Relaxing-Eyes Islands Themes | `Islands Relax Light Green` | 89k | 4.82 |

**Read the rating column, not the download column.** Downloads rank *plugins*, and the largest plugins here are collections whose light theme is a minority variant — so that number is mostly counting people who installed it for the dark one. Two entries make the point from outside the table: **Hiberbee** is the 16th most-downloaded theme on the Marketplace (949k) and contains no light theme at all — its own description lists one as upcoming — and **GitHub Primer Theme** rates 4.90 while describing itself as getting as close to *GitHub's dark theme* as possible. A high placing in the overall theme chart is not evidence about a plugin's light half.

Two narrower notes for this IDE:

- **RustRover 2026.2 defaults to the Islands UI**, whose inset, rounded tool windows a pre-Islands theme knows nothing about. The plugins that have caught up say so in the listing: Gerry ships `[Islands]` variants of every theme it has, and Falcon's entire pitch is Islands light themes.
- **Solarized Light and Xcode are already bundled as schemes**, so those two plugins are buying you the window half of a look you can half-have for free. Set the scheme first and see whether the chrome still bothers you.

### Setting it in the files instead of the dialog

The dialog is not the only door, and the other one has an advantage: writing both halves yourself is the one way to change a theme **without** the reset above firing. Both settings are single lines, in the folder section 5 already reads:

```xml
<!-- options/laf.xml -->
<laf themeId="islandsGerryLight" />

<!-- options/colors.scheme.xml -->
<global_color_scheme name="_@user_Github" />
```

`themeId` is the display name for a **bundled** theme (`Islands Light`), but for a **plugin** theme it is the provider id out of the plugin's own `META-INF/plugin.xml`, which is not what the dropdown shows you:

```xml
<themeProvider id="islandsGerryLight" path="/resources/islands/islandsGerryLight.theme.json"/>
```

That file also declares an `editorScheme` — the scheme the dialog would have switched you to, and the one you are declining by leaving `colors.scheme.xml` alone. Reading it first tells you what you are turning down: `[Islands] Gerry⟡ Light` ships `Gerry Light`, whose `parent_scheme` is `Default`, so both routes here are light and the choice is taste rather than polarity.

**Do it with the IDE closed** — but not for the reason the phrase usually implies. Settings are *not* written only on exit: on the machine these pages were written on, RustRover had been running for five days and `laf.xml` carried that morning's timestamp, rewritten mid-session the moment a setting changed. The IDE holds this state in memory, never re-reads the file, and flushes its own copy over yours at the next save. So an edit made while it is running is both invisible now and gone later, which is a worse failure than being refused.

Plugins are equally inspectable, one directory up. A single-jar theme sits there whole:

```text
~/Library/Application Support/JetBrains/RustRover2026.2/plugins/
└── GerryThemes.jar      ← 224 KB, 27 themes in one file
```

The Marketplace button is the normal way to put it there and needs no help. Knowing the layout is worth it for the two things the button cannot do: telling you what is actually installed without opening a dialog, and pinning a specific version rather than the newest compatible one.

## 7. Stop the Build panel opening on every run

Run the program and a **Build** tool window opens across the bottom of the frame, with `Sync` and `Build Output` tabs. Closing it does not stick — the next run re-activates it.

It is not the run. There are two cargo invocations on screen, in two different tool windows:

```text
Build:  cargo build --color=always --message-format=json-diagnostic-rendered-ansi --package untitled --bin untitled --profile dev
Run:    cargo run --color=always --package untitled --bin untitled --profile dev
```

The first is a **Before launch → Build** task on the run configuration — the same list section 3 suggests hanging a clippy step on — and `--message-format=json-diagnostic-rendered-ansi` is what it is there for: the structured diagnostics the Build window turns into clickable entries.

Remove it and the panel stops opening: *Run → Edit Configurations… → Before launch →* select **Build** → **−**. The program still compiles, because `cargo run` builds; what moves is where a compile error is reported — cargo's own text in the Run console, rather than a structured entry in Build.

**Doing it once does it once.** The templates dialog says as much itself — *"Changing a template does not affect the existing configurations"* — and the reverse holds too, so both halves need doing: **Edit configuration templates…**, at the bottom-left of the same dialog, then Cargo, then the same **−**.

An empty `<method>` element is what *no before-launch task* looks like:

```xml
<!-- .idea/workspace.xml -->
<component name="RunManager">
  <configuration name="Run" type="CargoCommandRunConfiguration" factoryName="Cargo Command">
    …
    <method v="2" />
  </configuration>
  <configuration default="true" type="CargoCommandRunConfiguration" factoryName="Cargo Command">
    …
    <method v="2" />
  </configuration>
</component>
```

The second, carrying `default="true"`, is the template. Both sit in `workspace.xml` — under `RunManager`, the per-user half of `.idea/` — because the configuration's **Store as project file** box is unchecked; tick it and the configuration moves to `.idea/runConfigurations/<name>.xml` under `ProjectRunConfigurationManager`, which is the half that belongs in git.

Which is why a scaffolded tree never has this problem: the configurations [`rust_scaffold.py`](../scaffolding/README.md) writes there carry the same empty `<method v="2" />` already. It is the standalone `cargo new` project — `untitled` again, the one section 4 is also about — that ships with the step attached.

## 8. The things you do not have to configure

Worth knowing so you do not go looking:

- **The compiler.** `rust-toolchain.toml` is a rustup mechanism, and RustRover goes through rustup, so [the pin](../pinning_the_toolchain/README.md) applies in the IDE with nothing set. Open the practice tree and it is on 1.97.1 because the file says so.
- **The lint policy.** `[workspace.lints.clippy]` and `clippy.toml` are read by clippy itself, so once the external linter is Clippy the IDE shows exactly what the command line shows.
- **New projects.** There is no template to set up: inside a workspace, `cargo new` already writes the lint opt-in, and the toolchain and lints are inherited from the root. This is the answer to "how do I make these the defaults for every new project" — you do not, the workspace does.

## What about bacon, if the IDE already does this?

A fair question, and the honest answer is that the overlap is largest exactly here — RustRover with clippy on the fly is doing most of what [bacon](../bacon/README.md)'s `c` job does. What is left is the `t` job: a watcher that re-runs your **tests** on every save, which no IDE lint setting gives you. If you do not want a second pane, skip bacon; if the loop you want is test-driven, that is the gap it fills.

## See also

- [Choosing an editor](../editors/README.md) — the comparison, and the `rustup which` trap when a tool wants a real binary rather than a shim
- [Strict clippy lints](../strict_lints/README.md) — the policy this setting enforces
- [A tree of practice projects](../practice_workspace/README.md) — the layout the run configurations come from
- [Scaffolding a practice tree](../scaffolding/README.md) — writing those run configurations into `.idea/` before the IDE has opened the project, and why the External Linters setting above is the one thing no file can carry
- [A tree of practice projects](../practice_workspace/README.md) — why a workspace member's span already names it, and the standalone project where it does not

---

*Sections 1–4 have their settings paths verified against the [RustRover external linters documentation ↗](https://www.jetbrains.com/help/rust/rust-external-linters.html) rather than by driving the IDE; menu wording moves between releases, so treat those names as a route rather than a transcript. Sections 5 and 7 are the other way round — every label, dialog and file fragment in them was read off RustRover **2026.2.1** (`262.9437.161`) on macOS while making the changes, and the two `.idea/workspace.xml` fragments are that project's file after the edit. Section 6's two bundled lists were read out of that same build's application bundle rather than out of the dropdowns — the theme names from the `*.theme.json` files in `intellij.platform.ide.impl.jar`, the schemes and their polarity from `parent_scheme` in `colorSchemes/*.xml` — its download and rating figures came from the Marketplace API on 2026-08-30, so treat those two columns as a snapshot, and its `laf.xml` / `plugin.xml` fragments are one real installation of the theme named in them.*

## Po polsku

Najlepiej czytać tę stronę przez jedno pytanie: **co musi być w pliku, a co może mieszkać wyłącznie w oknie IDE.** RustRover sam czyta `rust-toolchain.toml`, `Cargo.toml` i `clippy.toml`, więc przypięty kompilator i polityka lintów działają w edytorze bez ustawiania czegokolwiek — a wszystko, co wyklikasz w oknie, jest niewidzialne dla współpracowników, dla CI i dla ciebie po następnym `git clone`. Pierwsza zmiana warta zrobienia to przestawienie zewnętrznego lintera z **Cargo Check** na **Clippy**, bo `cargo check` odpowiada na pytanie „czy to się kompiluje”, które i tak zadaje za ciebie `rust-analyzer`, a clippy odpowiada na „czy to jest dobrze napisane” — i tego w oknie nie mówi nic innego. Do pola **Additional arguments** wpisz `--all-targets` i najlepiej nic poza tym, bo to pole jest **zasięgiem, a nie polityką**:

- *które* linty i na jakim poziomie → `[lints.clippy]` w `Cargo.toml` (albo `[workspace.lints.clippy]` w korzeniu)
- próg linta albo wyjątek → `clippy.toml` (`allow-unwrap-in-tests`, `msrv`)
- *na jaki kod* linter w ogóle patrzy → **to pole**, i tylko ono
- którym toolchainem to uruchomić → **Channel**, zostaw `[default]`, żeby edytor sprawdzał tym samym clippy co CI

Rozróżnienie nie jest akademickie: przy `len_zero = "deny"` bez `--all-targets` clippy nie skompiluje w ogóle celu testowego, więc wyjątek „`unwrap` wolno w testach” dotyczy kodu, którego twój edytor nawet nie czyta. Zaostrzanie linta nie sięgnie tam, gdzie linter nie zagląda.

Sekcja o `--remap-path-prefix` wygląda na literówkę i nią nie jest: podwójne `==` to **puste OLD**. Dopasowanie idzie po składowych ścieżki, a nie po tekście, więc pusty przedrostek pasuje do początku każdej ścieżki **względnej** i do żadnej bezwzględnej — twoje pliki dostają pełną ścieżkę, a `std` i `~/.cargo/registry`, już bezwzględne, przechodzą nietknięte. Trzy rzeczy do zapamiętania, zanim to wpiszesz: w `rustflags` **nic się nie interpoluje** (nie ma żadnego `$PWD`), więc po przeniesieniu katalogu każdy komunikat będzie z przekonaniem podawał stare miejsce; zmienna środowiskowa `RUSTFLAGS` **zastępuje** te flagi w całości, a nie dokłada się do nich, i to jest zwykle wyjaśnienie ustawienia, które „nie działa”; a `file!()` i lokalizacje panik też zrobią się bezwzględne. Warto też wiedzieć, kiedy to się w ogóle opłaca: w workspace ścieżka względna i tak zawiera katalog członka, więc problem dotyczy głównie samotnego projektu z `cargo new` — tego domyślnego `untitled`, w którym cały komunikat kurczy się do `src/main.rs`.

Dwie ostatnie rzeczy nie są w ogóle o Ruście, tylko o domyślnych ustawieniach okna, i obie mają ładne polskie nazwy, które warto rozdzielić: **motyw** (*theme*) maluje chrom okna — menu, zakładki, panele, w tym tło panelu dokumentacji — a **schemat kolorów edytora** (*editor color scheme*) maluje składnię, i to zarówno w edytorze, jak i **wewnątrz** tego panelu. Przełącznik *Sync with OS* podąża za systemem tylko pierwszą z tych rzeczy, więc macOS w trybie ciemnym plus świadomie wybrany jasny schemat daje jasne kolory składni na ciemnym tle — nic nie jest zepsute, po prostu jedna połowa się przełączyła, a druga nie. Pułapka siedzi w naprawie: wybranie nowego **Theme:** po cichu nadpisuje też **Editor color scheme:**, jeszcze zanim klikniesz Apply, więc schemat ustaw z powrotem podczas tej samej wizyty w oknie. Jeśli wolisz edytować `laf.xml` i `colors.scheme.xml` ręcznie — to jedyna droga, która tego nadpisania unika — rób to **przy zamkniętym IDE**, ale nie z powodu, który zwykle się podaje: ustawienia wcale nie zapisują się „przy wyjściu”, zapisują się w trakcie pracy. Rzecz w tym, że IDE trzyma ten stan w pamięci, nigdy nie czyta pliku ponownie i przy najbliższym zapisie przykryje twoją zmianę własną kopią. To gorsza awaria niż zwykła odmowa, bo edycja jest niewidoczna teraz i znika później.

Panel **Build**, który otwiera się przy każdym uruchomieniu, to nie jest samo uruchomienie — to zadanie *Before launch → Build*, doklejone do konfiguracji uruchomieniowej po to, by dostarczyć strukturalne diagnostyki (`--message-format=json-diagnostic-rendered-ansi`). Usuń je znakiem **−**, a program dalej się zbuduje, bo `cargo run` buduje sam; zmieni się tylko miejsce, w którym zobaczysz błąd kompilacji. Tylko usuń je **dwa razy**: raz w konkretnej konfiguracji i raz w *Edit configuration templates…*, bo zmiana szablonu nie rusza istniejących konfiguracji ani odwrotnie. Na koniec warto zapamiętać listę rzeczy, których konfigurować **nie trzeba**: kompilatora (przypięcie działa przez rustupa), polityki lintów (czyta ją samo clippy) i szablonu nowych projektów — bo w workspace `cargo new` sam dopisuje zgodę na linty i wszystko dziedziczy się z korzenia.

**Szukaj po polsku:** motyw a schemat kolorów w IDE · `RustRover external linters clippy` · `cargo clippy --all-targets` · `--remap-path-prefix` · `rustrover before launch build task`
