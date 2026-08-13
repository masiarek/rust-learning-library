# Glossary

Short definitions. Every entry links to the page that explains it properly — a definition that dead-ends hides the lesson that already exists.

**`and_then`** — Transform a value with a closure that can *itself* come up empty or fail, flattening the result instead of nesting it. The counterpart to `map` when the closure returns another `Option`/`Result`. → [`Option` vs `Result`](01_Foundations/option_vs_result/README.md)

**`anyhow`** — A crate giving applications one catch-all error type with good ergonomics and backtraces. The application-side counterpart to `thiserror`. → [`Option` vs `Result`](01_Foundations/option_vs_result/README.md)

**`Box<dyn Error>`** — A type-erased error: any error can convert into it, so unrelated failures can flow through one function. What applications reach for when nothing downstream will `match` on the cause. → [`Option` vs `Result`](01_Foundations/option_vs_result/README.md)

**Enum** — A type that is exactly one of several named variants, each optionally carrying data. `Option` and `Result` are both ordinary enums; nothing about them is built into the language.

**`expect`** — Panic with a message you wrote. Preferred over `unwrap` everywhere, because the message records *why* you believed this could not fail. → [`Option` vs `Result`](01_Foundations/option_vs_result/README.md)

**`From`** — The conversion trait. `?` calls it implicitly to turn one error type into the function's own, which is what makes custom error enums and `Box<dyn Error>` ergonomic. → [`Option` vs `Result`](01_Foundations/option_vs_result/README.md)

**`let … else`** — Bind a pattern or leave the current scope. The idiomatic guard clause; it keeps the happy path unindented. → [`Option` vs `Result`](01_Foundations/option_vs_result/README.md)

**`map`** — Transform the value inside a wrapper, leaving the wrapper alone. Nests rather than flattens if the closure returns another wrapper — that is when you want `and_then`. → [`Option` vs `Result`](01_Foundations/option_vs_result/README.md)

**`#[must_use]`** — An attribute making the compiler warn when a returned value is discarded. Both `Option` and `Result` carry it, which is why an ignored error is a warning rather than a silent bug. → [`Option` vs `Result`](01_Foundations/option_vs_result/README.md)

**Null-pointer optimization** — The compiler representing `None` with a bit pattern the happy variant cannot use, so `Option<Box<T>>` is the same size as `Box<T>`. Null safety at zero runtime cost. → [`Option` vs `Result`](01_Foundations/option_vs_result/README.md)

**`Option<T>`** — Either `Some(T)` or `None`. Models a value that might not be there, when "why not?" has exactly one possible answer. → [`Option` vs `Result`](01_Foundations/option_vs_result/README.md)

**Prelude** — The set of names in scope in every Rust file without an import. `Option`, `Result`, and their variants live there, which is why you write `Some(x)` and not `Option::Some(x)`.

**`?`** — Unwrap the happy value, or return the sad one from the current function — converting the error via `From` on the way out. → [`Option` vs `Result`](01_Foundations/option_vs_result/README.md)

**`Result<T, E>`** — Either `Ok(T)` or `Err(E)`. Models an operation that might fail, when the caller could reasonably ask *why* it failed. → [`Option` vs `Result`](01_Foundations/option_vs_result/README.md)

**`thiserror`** — A crate that derives the `Display`/`Error`/`From` boilerplate for a custom error enum. The library-side counterpart to `anyhow`. → [`Option` vs `Result`](01_Foundations/option_vs_result/README.md)

**`transpose`** — Flip `Option<Result<T, E>>` into `Result<Option<T>, E>` and back. → [`Option` vs `Result`](01_Foundations/option_vs_result/README.md)
