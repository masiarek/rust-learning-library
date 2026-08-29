# A build system is not a compiler

**Level:** 201 · working knowledge

**One line:** CMake, Make and Cargo compile nothing — they decide *what* to compile, in what order, and what can be skipped, and almost every "why did it rebuild everything?" question is theirs rather than the compiler's.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The one job: a dependency graph plus a staleness rule, and a driver that invokes the real compiler for the nodes that need it
- What Cargo decided *for* you that a C++ project decides by hand — the layout, the unit of compilation, the dependency resolution, the flags
- CMake's extra layer: it does not build, it *generates* a build (Ninja, Make, an IDE project), which is why its errors arrive at two different times
- The compilation unit is the crate, not the file — why touching one function rebuilds a whole crate, and why splitting a big crate can help a rebuild loop
- What actually invalidates a cache: fingerprints over source, features, flags, environment variables and build scripts. `build.rs` and its `cargo::rerun-if-changed`
- `sccache` and shared caches, and why the hit rate is worse than people expect
- Reading `cargo build --timings` as a graph rather than a list: what blocked what

## The trap it exists for

"Cargo is slow" is nearly always "this build recompiles more than it needs to", and the cause is upstream of any compiler flag — a build script with no `rerun-if-changed`, a feature that differs between two commands, an environment variable in the fingerprint. Optimizing the compiler cannot fix a graph that says everything is stale.

## See also

- [Compile times](../../05_Tooling/compile_times/README.md) — the phases inside one invocation, once the build system has decided to make it
- [Adding a dependency](../../05_Tooling/cargo_dependencies/README.md) — where the graph's nodes come from, and what a version range resolves to
- [A tree of practice projects](../../05_Tooling/practice_workspace/README.md) — one workspace, many crates, and what is shared between them
