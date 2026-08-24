# Which Rust UI, and whether you want one

**Level:** reference · orientation

**One line:** Three names come up — [Leptos](https://leptos.dev/), [Dioxus](https://dioxuslabs.com/) and [Tauri](https://tauri.app/) — they are not competing for the same job, and for a learning project the right first answer is usually "a CLI", because the UI layer is where you stop writing Rust and start writing a framework.

## The three, and what each is actually for

| | What it is | Reach | Reach for it when |
|---|---|---|---|
| **Leptos** | A full-stack web framework with fine-grained reactivity. Signals, server functions, SSR or pure client-side WASM. | the browser | you want a **web app**, and you would rather write Rust than TypeScript |
| **Dioxus** | One codebase targeting web, desktop, **iOS and Android**. React-shaped: `rsx!`, `use_signal`. | browser, desktop, phone | you want the **same app on a phone and a laptop** |
| **Tauri** | Not a UI framework. A shell that puts a *webview* window around a frontend and gives it a Rust backend. | desktop (and mobile) | your UI is already HTML/JS and you want the **backend in Rust** |

The distinction people trip over: Tauri is orthogonal to the other two. You can run Leptos or Dioxus *inside* Tauri. "Leptos vs Tauri" is not a choice; "Leptos vs Dioxus" is.

## Does Dioxus support iPhone and Android?

**Yes, both** — and it is a real feature rather than an aspiration. Since Dioxus 0.6 the CLI does it directly:

```sh
dx serve --platform ios
dx serve --platform android
```

Simulators and devices both work, with hot-reloading, fast rebuilds and asset bundling — the same loop as desktop. Two caveats worth knowing before you plan around it:

- **Mobile renders through a WebView**, not native widgets. Your UI is HTML and CSS in a system web view, so it will not look or feel like a UIKit app. A fully native renderer is on the roadmap and explicitly distant.
- **The setup is the hard part, and it is not Rust's fault.** iOS needs macOS with Xcode and the iOS SDK; Android needs the SDK *and* the NDK. Dioxus's own docs call that "a substantial amount of setup", which is fair.

So: cross-platform reach, at the cost of a WebView and a toolchain afternoon.

## Would a STAR voting toy app be a good project?

Yes — but stage it, because the first stage is where nearly all the learning is.

**Stage 1: a CLI, and no framework at all.** Read ballots, run the score round, run the automatic runoff, print the winner. This is the whole of STAR, and every hard part of it is ordinary Rust: modelling a ballot so an invalid one cannot be constructed, deciding what a tie *is*, handling malformed input at the boundary. You already have the pieces — [`clap`](../../05_Tooling/practice_workspace/README.md) for the arguments, `serde` for the ballots, `anyhow` for the failures. Nothing here needs a UI, and if the counting is wrong a UI only makes it wrong in colour.

**Stage 2: Leptos, client-side only.** If you then want people to click on it, Leptos compiled to WASM with Trunk produces **static files** — no server, no database, no deployment story. That drops onto GitHub Pages beside the voting library itself, which is a genuinely nice place for it to live. The counting code from stage 1 becomes a plain library crate that both the CLI and the web app call, which is the moment a workspace stops being a filing convention and starts paying for itself.

**Stage 3: only if you want it on a phone.** That is the Dioxus case, and it is a different project with a toolchain of its own.

The honest warning about starting at stage 2 or 3: reactive UI is a substantial subject with its own vocabulary — signals, effects, ownership across closures — and none of it teaches you Rust. It teaches you the framework. Borrow-checker fights inside an event handler are the least instructive fights available, because the answer is usually "clone it" and you learn nothing about why.

## See also

- [A tree of practice projects](../../05_Tooling/practice_workspace/README.md) — where a staged project like this would live, and how a shared library crate works
- [An HTTP request](../../07_Clients/http_with_reqwest/README.md) — the other half of a client, if the app ever talks to something
- [The long way round to a STAR count](../../ROADMAP.md) — the lessons sequenced so the running example is a voting method
