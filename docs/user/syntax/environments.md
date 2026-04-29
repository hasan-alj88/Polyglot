---
audience: automation-builder
type: specification
updated: 2026-04-10
status: draft
---

# Environment Definitions (`{;}`)

<!-- @c:vision:Core Philosophy -->
<!-- @c:glossary:Aljam3 Service -->
<!-- @u:pipelines:Pipeline Structure -->
<!-- @u:blocks#Definition Elements -->

Environment definitions declare what a pipeline needs from its host system — language runtime, version, dependencies, and configuration. The compiler validates environment availability at every call site, ensuring no runtime surprises.

**Parallel:** Just as `{!}` defines errors so the compiler enforces handling them, `{;}` defines environments so the compiler can verify they're set up. Environment requirements are part of the pipeline's IO contract — same as inputs, outputs, and errors.

## The `;` Prefix

`;` is the environment identifier prefix — it names environments the same way `$` names variables or `#` names types. It is **not** a line marker. Line markers are bracket shapes (`{X}`, `[X]`, `(X)`) — see [[line-structure#Ground Truth: Every Line Has a Marker]].

| Prefix | Purpose |
|--------|---------|
| `@` | Packages |
| `#` | Data types |
| `-` | Pipelines |
| `$` | Variables |
| `!` | Errors |
| `%` | Metadata |
| `_` | Permissions |
| `;` | Environments |

## Defining Environments

`{;}` blocks are standalone definitions at the same level as `{#}`, `{!}`, etc. They are importable via `[@]` like any other definition.

```aljam3
{;} ;EnvName
   [.] .language << #BaseCode.*        {- mandatory -}
   [.] .version << ?[lo, hi)           {- version range, optional for Shell -}
   [.] .Dependency                      {- %##Active: .packages/.file/.None -}
   [.] .env                             {- optional custom env vars -}
      [:] :VAR_NAME << "value"
```

### `.language` — `#BaseCode` (mandatory)

Which runtime. Uses the `#BaseCode` enum:

```aljam3
[.] .language << #BaseCode.Python
[.] .language << #BaseCode.Rust
[.] .language << #BaseCode.JavaScript
[.] .language << #BaseCode.Shell
[.] .language << #BaseCode.Aljam3
```

### `.version` — `#string` (optional for Shell)

Version constraint using Aljam3 range syntax (see [[operators#Range Operators]]):

```aljam3
[.] .version << ?[3.10, 4.0)       {- Python >=3.10 <4.0 -}
[.] .version << ?[1.70, 2.0)       {- Rust >=1.70 <2.0 -}
[.] .version << ?[22.0, 99.0)      {- Node >=22 -}
```

### `.Dependency` — `%##Active` (mandatory)

How to resolve packages. Exactly one field active:

| Active Field | Type | When to use |
|-------------|------|-------------|
| `.packages` | `##Record` | Inline declarations — flexible `:` fields for name-version pairs |
| `.file` | `#path` | External dependency file (requirements.txt, Cargo.toml, package.json) |
| `.None` | `###None` | No external dependencies |

```aljam3
{- Inline packages -}
[.] .Dependency
   [.] .packages
      [:] :numpy << ">=1.24"
      [:] :pandas << ">=2.0"

{- External file -}
[.] .Dependency
   [.] .file << "requirements.txt"

{- No dependencies -}
[.] .Dependency
   [.] .None
```

### Language-Specific Fields

These optional fields apply only to specific languages:

| Field | Languages | Purpose |
|-------|-----------|---------|
| `.edition` | Rust | Rust edition: `"2021"`, `"2024"` |
| `.target` | Rust | Cross-compilation triple: `"aarch64-unknown-linux-musl"` |
| `.runtime` | JavaScript | Runtime variant: `"node"`, `"deno"`, `"bun"` |
| `.module` | JavaScript | Module system: `"esm"`, `"cjs"` |
| `.shell` | Shell | Shell binary: `"bash"`, `"sh"`, `"zsh"` |
| `.isolation` | Shell | Isolation level: `#Isolation.Process`, `#Isolation.Container` |

### `.env` — `##Record` (optional)

Custom environment variables injected into the runtime. Flexible `:` fields — user-defined key-value pairs:

```aljam3
[.] .env
   [:] :NODE_ENV << "production"
   [:] :API_KEY << "{$config.apiKey}"
   [:] :DB_HOST << "localhost"
```

Platform-managed vars (PATH, HOME, VIRTUAL_ENV, CARGO_HOME, etc.) are NOT declared here — the [[pglib/pipelines/W/Env|-W.Env]] wrapper sets those automatically.

## The `#;` Type

`#;` is the environment definition type — an alias for `##Env`. It represents a reference to a `{;}` definition and allows the [[pglib/pipelines/W/Env|-W.Env]] wrapper to receive the environment specification as input:

```aljam3
[W] -W.Env
   (-) <env#; << ;Python3.14
```

## User Declares vs Platform Resolves

| User declares in `{;}` | Platform resolves internally |
|------------------------|---------------------------|
| Language + version constraint | Binary path, toolchain installation |
| Package names + versions | Package resolution, lockfiles, install |
| Dependency source file path | File parsing, format detection |
| Custom env vars needed | PATH, HOME, system vars, activation |
| Module system (JS: ESM/CJS) | Loader configuration, flags |
| Edition (Rust: 2021/2024) | Compiler flags |

Resources (RAM, CPU, disk) are NOT in `{;}` — resource limits belong to `[Q]` queue configuration.

## Complete Examples

### Python ML Environment

```aljam3
{;} ;MLPython
   [.] .language << #BaseCode.Python
   [.] .version << ?[3.10, 4.0)
   [.] .Dependency
      [.] .packages
         [:] :numpy << ">=1.24"
         [:] :pandas << ">=2.0"
         [:] :scikit-learn << ">=1.3"
   [.] .env
      [:] :PYTHONUNBUFFERED << "1"
```

### Rust Cross-Compilation

```aljam3
{;} ;RustARM
   [.] .language << #BaseCode.Rust
   [.] .version << ?[1.75, 2.0)
   [.] .edition << "2024"
   [.] .target << "aarch64-unknown-linux-musl"
   [.] .Dependency
      [.] .file << "Cargo.toml"
```

### Node.js API Server

```aljam3
{;} ;NodeAPI
   [.] .language << #BaseCode.JavaScript
   [.] .version << ?[22.0, 99.0)
   [.] .runtime << "node"
   [.] .module << "esm"
   [.] .Dependency
      [.] .file << "package.json"
   [.] .env
      [:] :NODE_ENV << "production"
```

### Deno TypeScript Worker

```aljam3
{;} ;DenoWorker
   [.] .language << #BaseCode.JavaScript
   [.] .version << ?[2.0, 99.0)
   [.] .runtime << "deno"
   [.] .module << "esm"
   [.] .Dependency
      [.] .None
```

### Isolated Shell

```aljam3
{;} ;SecureShell
   [.] .language << #BaseCode.Shell
   [.] .shell << "bash"
   [.] .isolation << #Isolation.Process
   [.] .Dependency
      [.] .None
   [.] .env
      [:] :API_KEY << "{$secrets.apiKey}"
      [:] :TEMP_DIR << "/tmp/aljam3-work"
```

### Pure Aljam3 (built-in)

```aljam3
{;} ;Aljam3
   [.] .language << #BaseCode.Aljam3
   [.] .Dependency
      [.] .None
```

`;Aljam3` is built-in — no user definition needed. All pure Aljam3 pipelines declare `(-) ;Aljam3`.

## Compiler Rules

| Code | Rule |
|------|------|
| PGE01032 | `(-) ;Env` declared but no matching `[W] -W.Env` or `[W] -W.Aljam3` wrapper |
| PGE01033 | `[W] -W.Env` present but no `(-) ;Env` declaration |
| PGE01034 | Caller invokes pipeline whose `(-) ;Env` is not available at call site |
| PGE01035 | Multiple `(-) ;Env` declared but wrappers don't cover all |
| PGE01036 | Pipeline missing `(-) ;` environment declaration (mandatory) |

**PGE01034** is the key rule — the compiler tracks active environments through the call graph and validates availability at each call site. Either the caller's wrapper or the callee's own wrapper must provide the required environment.

## Related

- [[io#Environment Declaration]] — declaring `(-) ;` in IO contracts
- [[pglib/pipelines/W/Env|-W.Env]] — the environment wrapper
- [[pglib/errors/errors#Built-in Error Namespaces]] — `!Env.*` errors
- [[pglib/pipelines/W/Aljam3|-W.Aljam3]] — pure Aljam3 wrapper
