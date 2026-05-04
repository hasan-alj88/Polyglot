---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
---

# -RT — Runtime Execution

<!-- @c:pipelines -->
Runtime execution pipelines run foreign code (Python, Rust, etc.) within Aljam3 pipelines. Each `-RT.<Lang>.*` pipeline takes a language-specific environment handle from `-W.RT` and executes code in that runtime.

No `[@]` import needed.

**PRIMITIVE** — jm3lib runtime pipelines are direct language runtime integrations. They are implemented by the Aljam3 runtime and cannot be reimplemented in user `.jm3` files.

`<Lang>` is a placeholder for the target language (Python, Rust, etc.). The actual pipeline name uses the concrete language: `-RT.Python.Function.Inline`, `-RT.Rust.CLI`, etc.

## Permissions

<!-- @c:permissions -->
All `-RT.*` pipelines require a `{_}` permission object granting System.Process. See [[permissions]] for the permission system.

| Pipeline | Required Capability | Category |
|----------|-------------------|----------|
| `-RT.<Lang>.*` | System.Process | System |

## Categories

### Function Call

| Pipeline | Description |
|----------|-------------|
| [[jm3lib/pipelines/RT/Function.Inline\|-RT.\<Lang\>.Function.Inline]] | Call a named function in inline foreign code |
| [[jm3lib/pipelines/RT/Function.File\|-RT.\<Lang\>.Function.File]] | Call a named function in a source file |

### Script Execution

| Pipeline | Description |
|----------|-------------|
| [[jm3lib/pipelines/RT/Script.Inline\|-RT.\<Lang\>.Script.Inline]] | Run inline code with variable bindings |
| [[jm3lib/pipelines/RT/Script.File\|-RT.\<Lang\>.Script.File]] | Run source file with variable bindings |

### Binary Invocation

| Pipeline | Description |
|----------|-------------|
| [[jm3lib/pipelines/RT/CLI\|-RT.\<Lang\>.CLI]] | Invoke compiled binary |

### Native Binding

| Pipeline | Description |
|----------|-------------|
| [[jm3lib/pipelines/RT/Bind.Inline\|-RT.\<Lang\>.Bind.Inline]] | Native code imports aljam3 lib (inline) |
| [[jm3lib/pipelines/RT/Bind.File\|-RT.\<Lang\>.Bind.File]] | Native code imports aljam3 lib (file) |

## `.Script` vs `.Bind` — Binding Origin

Both `.Script` and `.Bind` execute code without a named function entry point. The difference is **who controls the data flow**:

- **`.Script`** — Aljam3-controlled binding. `<Bind#serial` injects Aljam3 data as named variables before execution. `>Bind#serial` captures their final state after execution. The compiler can validate bound variable names exist in the code.
- **`.Bind`** — Foreign-code-controlled binding. Native code imports the aljam3 lib and calls `pull("name")`/`push("name", value)` at arbitrary points during execution. The compiler cannot validate these — they are opaque runtime strings.

## Multiple `[W]` Wrappers

A pipeline may declare multiple `[W]` wrappers (e.g., one Python + one Rust). Setup runs in declaration order; cleanup runs in reverse order (bracket semantics).

## IO Summary

| Mode | `<env` | `<func` | `<arg` | `<kwarg` | `<Bind` | `>Bind` | `>output` | `>return` | `<code`/`<file` |
|------|--------|---------|--------|----------|---------|---------|-----------|-----------|-----------------|
| `.Function.Inline` | yes | yes | yes | opt | — | — | yes | yes | `<code` |
| `.Function.File` | yes | yes | yes | opt | — | — | yes | yes | `<file` |
| `.Script.Inline` | yes | — | — | — | opt | opt | yes | — | `<code` |
| `.Script.File` | yes | — | — | — | opt | opt | yes | — | `<file` |
| `.CLI` | — | — | opt | opt | — | — | yes | — | `<binary` |
| `.Bind.Inline` | yes | — | — | — | — | — | yes | — | `<code` |
| `.Bind.File` | yes | — | — | — | — | — | yes | — | `<file` |

## Compiler Validation

| Mode | What the compiler validates |
|------|---------------------------|
| `.Function` | `<func` name exists as a function definition in the `[C]` block or source file |
| `.Script` | `<Bind` variable names exist as identifiers in the code or file |
| `.CLI` | No code validation — binary is opaque |
| `.Bind` | No validation — `pull()`/`push()` calls are opaque runtime strings |

## Related

- [[jm3lib/pipelines/W/INDEX|-W]] — `-W.RT` wrapper that manages runtime environments
- [[jm3lib/types/rt|Runtime types]] — `#Code`, `#PyEnv`, `#RsEnv`
- [[jm3lib/errors/errors|errors]] — `!RT` error namespace
- [[syntax/blocks|blocks]] — `[C]` inline foreign code element

NOTE: Retry/timeout/rate-limiting are `[Q]` queue strategies, not runtime pipeline concerns.
