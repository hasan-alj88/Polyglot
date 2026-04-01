---
audience: user
type: specification
updated: 2026-04-01
status: complete
---

# Runtime Types

<!-- @types -->

Types used by `=RT.*` runtime execution pipelines and `=W.RT` wrappers. No `[@]` import needed.

## #Code — Runtime Execution Output

Output struct for all `=RT.*` pipeline modes. Language is a flexible field (`:Python`, `:Rust`, etc.), `.Output` is fixed.

```polyglot
{#} #Code
   :Python
      .Output
         [.] .stdout#RawString
         [.] .stderr#RawString
         [.] .return#serial
   :Rust
      .Output
         [.] .stdout#RawString
         [.] .stderr#RawString
         [.] .return#serial
```

| Field | Type | Purpose |
|-------|------|---------|
| `.stdout` | `#RawString` | Standard output capture (raw bytes) |
| `.stderr` | `#RawString` | Standard error capture (raw bytes) |
| `.return` | `#serial` | Function return value (shape varies by code). Empty for `.Script` and `.Bind` modes. |

## #PyEnv — Python Environment Handle

Runtime environment handle produced by `=W.RT:Python:*` wrapper setup.

```polyglot
{#} #PyEnv
   [.] .version#string
   [.] .handle#RawString
```

| Field | Type | Purpose |
|-------|------|---------|
| `.version` | `#string` | Python version string (e.g., `"3.14"`) |
| `.handle` | `#RawString` | Opaque runtime handle used by `=RT.Python.*` pipelines |

## #RsEnv — Rust Environment Handle

Runtime environment handle produced by `=W.RT:Rust:*` wrapper setup.

```polyglot
{#} #RsEnv
   [.] .version#string
   [.] .handle#RawString
```

| Field | Type | Purpose |
|-------|------|---------|
| `.version` | `#string` | Rust version string (e.g., `"1.84"`) |
| `.handle` | `#RawString` | Opaque runtime handle used by `=RT.Rust.*` pipelines |

## Related

- [[stdlib/pipelines/RT|=RT]] — runtime execution pipelines that consume these types
- [[stdlib/pipelines/W|=W]] — `=W.RT` wrapper that produces `#PyEnv`/`#RsEnv`
- [[stdlib/errors/errors|errors]] — `!RT` error namespace for runtime failures
