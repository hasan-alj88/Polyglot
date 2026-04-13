---
audience: pg-coder
type: specification
updated: 2026-04-13
status: draft
---

# -Run — Foreign Code Execution

<!-- @c:pipelines -->
<!-- @c:glossary#Runner -->
Foreign code execution pipelines run native code (Python, Rust, etc.) within Polyglot pipelines. Each `-Run.<Lang>.*` pipeline takes a language-specific environment handle from `-W.Env` and executes code in that runtime.

No `[@]` import needed.

**PRIMITIVE** — pglib runtime pipelines are direct language runtime integrations. They are implemented by the Polyglot runtime and cannot be reimplemented in user `.pg` files.

`<Lang>` is a placeholder for the target language (Python, Rust, etc.). The actual pipeline name uses the concrete language: `-Run.Python.Function`, `-Run.Rust.Script`, etc.

> **Supersedes:** `-RT.*` pipeline family. See [[pglib/pipelines/RT/INDEX|@d:-RT.*]] for the deprecated specification.

## Permissions

<!-- @c:permissions -->
All `-Run.*` pipelines require a `{_}` permission object granting System.Process. See [[permissions]] for the permission system.

| Pipeline | Required Capability | Category |
|----------|-------------------|----------|
| `-Run.<Lang>.*` | System.Process | System |

## Pipelines

| Pipeline | Description |
|----------|-------------|
| [[pglib/pipelines/Run/Function\|-Run.\<Lang\>.Function]] | Call a named function in foreign code |
| [[pglib/pipelines/Run/Script\|-Run.\<Lang\>.Script]] | Run code with Record-typed variable bindings |
| [[pglib/pipelines/Run/CLI\|-Run.\<Lang\>.CLI]] | Invoke compiled binary with string arguments |
| [[pglib/pipelines/Run/Bind\|-Run.\<Lang\>.Bind]] | Foreign code imports polyglot lib for data flow |

## Code Source — `<code#Code:Source`

<!-- @c:types -->
Three of the four pipelines (`.Function`, `.Script`, `.Bind`) accept a `<code` input. Code source is an enum-style struct with `%##Active` = one — the caller provides **either** inline code or a file path, never both:

```polyglot
{#} #Code:Source
   [#] %##Active << #ActiveKind.One
   [.] .inline#string
   [.] .file#path
```

**Inline usage** (with `[C]` blocks):
```polyglot
(-) <code.inline <<
   [C] import os
   [C] result = os.listdir(target_dir)
```

**File usage:**
```polyglot
(-) <code.file#path << "/scripts/process.py"
```

The compiler enforces `%##Active` one — providing both `.inline` and `.file` is PGE01038.

## Binding Modes

<!-- @u:syntax/blocks#Foreign Code -->
Four modes define **who controls data flow** between Polyglot and foreign code:

### `.Function` — Structured Call

Call a named function with positional and keyword arguments. The compiler validates that `<func` exists as a function definition in the code.

| IO | Type | Purpose |
|----|------|---------|
| `<env` | `#<Lang>Env` | Runtime environment from `-W.Env` |
| `<func` | `#string` | Function name (compiler-validated) |
| `<arg` | `#Record` | Positional arguments — field order = argument order |
| `<kwarg` | `#Record` | Keyword arguments — field names = parameter names |
| `>Bind` | `#Record` | Return value fields captured by name |
| `>output` | `#Code:<Lang>.Output` | `.stdout`, `.stderr` capture |
| `<code` | `#Code:Source` | Function definition (inline `[C]` or file) |

### `.Script` — Polyglot-Controlled Binding

Run code with Record-typed variable bindings. `<Bind#Record` field names become native local variables. `>Bind#Record` field names are read back after execution. The compiler validates that all field names exist as identifiers in the code.

| IO | Type | Purpose |
|----|------|---------|
| `<env` | `#<Lang>Env` | Runtime environment from `-W.Env` |
| `<Bind` | `#Record` | Input bindings — field names = native variable names |
| `>Bind` | `#Record` | Output bindings — field names = native variable names |
| `>output` | `#Code:<Lang>.Output` | `.stdout`, `.stderr` capture |
| `<code` | `#Code:Source` | Script code (inline `[C]` or file) |

### `.CLI` — Binary Invocation

Invoke a compiled binary with string arguments. No code validation — the binary is opaque.

| IO | Type | Purpose |
|----|------|---------|
| `<binary` | `#path` | Path to the executable |
| `<arg` | `#Record` | Positional arguments — all fields `#string`, order = argument order |
| `<kwarg` | `#Record` | Named flags — all fields `#string`, field names = `--flag` names |
| `>Bind` | `#Record` | Output capture (e.g., exit code) |
| `>output` | `#Code:<Lang>.Output` | `.stdout`, `.stderr` capture |

### `.Bind` — Foreign-Code-Controlled

Foreign code imports the polyglot lib and calls `pull("name")`/`push("name", value)` at arbitrary points. The compiler cannot validate these — they are opaque runtime strings.

| IO | Type | Purpose |
|----|------|---------|
| `<env` | `#<Lang>Env` | Runtime environment from `-W.Env` |
| `>output` | `#Code:<Lang>.Output` | `.stdout`, `.stderr` capture |
| `<code` | `#Code:Source` | Code with polyglot lib imports (inline `[C]` or file) |

## Record Binding — The `%InlineString` Pattern

<!-- @c:types/generic-types -->
`<Bind`, `<arg`, `<kwarg`, and `>Bind` all use `#Record` types. Record field names map **exactly** to native variable/parameter names — the same principle as `%InlineString` where template variables match Polyglot `$` variables exactly.

| Mechanism | Polyglot Side | Foreign Side | Compiler Validates |
|-----------|---------------|--------------|-------------------|
| `%InlineString` | `{$varName}` | String substitution | `$varName` exists |
| `<Bind#Record` | `.field_name#type` | Native variable `field_name` | `field_name` in code |
| `<arg#Record` | `.field_name#type` | Positional argument | `field_name` in `<func` params |
| `<kwarg#Record` | `.field_name#type` | Keyword argument | `field_name` in `<func` params |
| `>Bind#Record` | `.field_name#type` | Native variable `field_name` | `field_name` in code |

**Inline Record** (anonymous, at call site) is standard collection assignment syntax — valid as long as the assigned object matches the schema topology:

```polyglot
(-) <Bind#Record
   [.] .input_path#path << $imageFile
   [.] .target_w#int << $targetWidth
```

**Named Record** (reusable `{#}` definition):

```polyglot
{#} #ResizeInputs
   [.] .input_path#path
   [.] .target_w#int

[-] -Run.Python.Script
   (-) <Bind#ResizeInputs << $bindData
```

## Type Marshalling

<!-- @c:spec/native-dispatch#Serialization Protocol -->
Record field types drive marshalling through the native dispatch JSON wire format. Each field serializes to the native type per its annotation:

| Polyglot Field Type | Python | Rust |
|---------------------|--------|------|
| `#path` | `str` (OS path) | `PathBuf` |
| `#int` | `int` | `i64` |
| `#float` | `float` | `f64` |
| `#string` | `str` | `String` |
| `#bool` | `bool` | `bool` |
| `#serial` | `dict` | `serde_json::Value` |
| `#array:T` | `list[T]` | `Vec<T>` |
| Named `#Record` | `dict` | `HashMap<String, _>` |

## Compiler Validation

| Mode | What the compiler validates |
|------|---------------------------|
| `.Function` | `<func` name exists as function in code; `<arg`/`<kwarg` fields match parameters |
| `.Script` | `<Bind` / `>Bind` field names exist as identifiers in code |
| `.CLI` | `<arg` / `<kwarg` fields are all `#string` (PGE01039) |
| `.Bind` | No validation — `pull()`/`push()` are opaque runtime strings |

**Note:** Binding validation (PGE01033–PGE01036) applies at compile time for `<code.inline` only. When `<code.file` is used, binding validation is deferred to runtime (the file content is not available at compile time).

**File binding rule:** When `<code.file` is used, the compiler records a content hash of the referenced file. If the file changes after compilation, the Polyglot Service revokes the pipeline's permission grant and refuses to execute until the developer recompiles. A file change watcher trigger monitors the referenced path and notifies the developer. See [[concepts/permissions#Compile-Time File Binding|c:Compile-Time File Binding]].

### Binding Compiler Errors

| Code | Name | Condition |
|------|------|-----------|
| PGE01033 | Unbound Script Variable | `<Bind#Record` field name not found in code |
| PGE01034 | Unbound Script Output | `>Bind#Record` field name not found in code |
| PGE01035 | Unbound Function Argument | `<arg#Record` field not found as `<func` parameter |
| PGE01036 | Unbound Function Kwarg | `<kwarg#Record` field not found as `<func` keyword parameter |
| PGE01037 | Bind Schema Mismatch | Assigned value doesn't match Record schema topology |
| PGE01038 | Code Source Conflict | Both `<code.inline` and `<code.file` provided |
| PGE01039 | CLI Non-String Argument | `.CLI` `<arg`/`<kwarg` Record field is not `#string` |

## Scope Isolation

Each `-Run.*` call gets a fresh variable scope within the `-W.Env` environment. `<Bind` fields are injected before execution; `>Bind` fields are read back after. Variables from one `-Run.*` call do **not** leak into another.

The `-W.Env` wrapper manages the **environment** (interpreter process, installed packages), but each call gets its own variable scope within that environment.

## Related

- [[pglib/pipelines/W/Env|-W.Env]] — wrapper that manages runtime environments
- [[pglib/types/rt|Runtime types]] — `#Code`, `#PyEnv`, `#RsEnv`
- [[pglib/errors/errors|errors]] — `!Run` error namespace
- [[syntax/blocks|blocks]] — `[C]` inline foreign code element

NOTE: Retry/timeout/rate-limiting are `[Q]` queue strategies, not runtime pipeline concerns.
