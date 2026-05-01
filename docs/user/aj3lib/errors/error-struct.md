---
audience: automation-builder
type: specification
updated: 2026-04-10
status: complete
---

# Error Struct

<!-- @c:errors -->
<!-- @c:blocks -->

## `#NullableRecord`

A `#Record` that can be null — used for optional structured data:

```aljam3
{#} #NullableRecord
   [#] ##Nullable
      (#) <#Record
```

## `#Error` Struct

All errors — jm3lib and user-defined — share the same struct:

```aljam3
{#} #Error
   [.] .Name#RawString
   [.] .ErrorAlias#RawString
   [.] .MessageTemplate#RawString
   [.] .Info#Record
   [.] .Stderr#NullableRecord
   [.] .StackTrace#NullableRecord
   [.] .ExitCode#NullableRecord
```

| Field | Filled by | Compiler enforced? | Purpose |
|-------|-----------|-------------------|---------|
| `.Name` | Runtime (auto) | — | Full error identifier (e.g., `"File.NotFound"`) |
| `.ErrorAlias` | Author | — | Short display name for logging/UI |
| `.MessageTemplate` | Author (at definition site) | Required on every `{!}` terminal; `{key}` must exist in `.Info` (PGE07008) | Template with `{key}` interpolation (e.g., `"Name exceeds {maxLength} chars"`) |
| `.Info` | Schema at definition site, values at raise site | Raise site must provide all declared keys (PGE07009); extra keys allowed | Structured context data with typed key schema |
| `.Stderr` | Runtime (auto) | — | Captured standard error output (null when not applicable) |
| `.StackTrace` | Runtime (auto) | — | Execution stack trace (null when not applicable) |
| `.ExitCode` | Runtime (auto) | — | Process exit code (null when not applicable) |

`#Error` content is split across two sites:

- **Definition site (`{!}` block):** `.MessageTemplate` is required on every terminal. `.Info` declares a typed key schema — each key with its type (e.g., `[:] :path#path`). The compiler enforces that every `{key}` in `.MessageTemplate` has a matching key in the `.Info` schema (PGE07008).
- **Raise site (`[!] >>` block):** Fills `.Info` values for the declared keys. Must provide all keys from the definition schema (PGE07009). May add extra keys beyond the schema for additional context.

The resolved message is computed at runtime by interpolating `.Info` values into `.MessageTemplate`.
