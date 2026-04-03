---
audience: pg-coder
type: concept
updated: 2026-03-31
---

# Metadata

<!-- @glossary:Polyglot Service -->
<!-- @data-is-trees -->

Every Polyglot object carries metadata — descriptive fields you declare and runtime fields the system manages. You interact with metadata through two mechanisms:

1. **`[%]` block element** — declare metadata inside `{#}`, `{=}`, or `{M}` definitions
2. **`%` accessor** — query runtime state from any expression context

For the full metadata tree architecture, path grammar, and instance rules, see [[metadata-tree/INDEX|technical/spec/metadata-tree]].

## Declaring Metadata with `[%]`

The `[%]` block element lives inside any `{x}` definition. One definition = one metadata set.

### Fixed Fields

| Field | Type | Description |
|-------|------|-------------|
| `.description` | `#string` | Human-readable description |
| `.version` | `#string` | Semantic version |
| `.authors` | `#array:string` | Author list |
| `.license` | `#string` | License identifier |
| `.deprecated` | `#bool` | Deprecation flag |
| `.deprecatedMessage` | `#string` | Reason for deprecation and suggested replacement |
| `.baseCode` | `#BaseCode.*` | Links to native implementation. Present only on stdlib base pipelines — see [[concepts/pipelines/INDEX#Base vs Derived\|Base vs Derived]] |

Pipelines with `.baseCode` are **base pipelines** — they have no execution body. The compiler resolves `.baseCode` to the native implementation. Pipelines without `.baseCode` are **derived pipelines** with full Polyglot execution bodies. See [[stdlib/types/BaseCode\|#BaseCode enum]] for the variant tree.

### Flexible Fields

| Field | Type | Description |
|-------|------|-------------|
| `%alias` | `#Array.NestedKeyString` | Shorthand names — multiple aliases per definition. Each alias is a `#NestedKeyString` (allows `.` and `:` for nested paths). Must be globally unique (PGE12002) |
| `:info` | `#serial` | Opens a flexible scope for custom key-value tooling data |

### Example

```polyglot
{= =MyPipeline}
   [%] .description << "Processes incoming invoices"
   [%] .version << "2.1.0"
   [%] .authors << ["Alice", "Bob"]
   [%] .deprecated << false
   [%] %alias
      [:] "ProcessInvoice"
      [:] "InvoiceProcessor"
   [%] :info
      :team << "payments"
      :priority << "high"
```

User-declared fields follow normal variable lifecycle rules ([[variable-lifecycle]]).

## Querying Runtime State with `%`

`live` fields are populated by the Polyglot runtime automatically. Users read them via `%` but cannot push into them (PGE02006). See [[syntax/types/hierarchy#Live Type Modifier]].

### Pipeline (`{=}`)

| Accessor | Type | Description |
|----------|------|-------------|
| `=Name%status` | `#live.#PipelineStatus` | AwaitTrigger, Disabled, Running, Failed |
| `=Name%errors` | `#live.array:error` | Accumulated errors |
| `=Name%isSuccess` | `#live.#Boolean` | Last run completed without error |
| `=Name%instanceCount` | `#live.int` | Number of active instances |
| `=Name%lastRun` | `#live.string` | Timestamp of last execution |
| `=Name%duration` | `#live.string` | Duration of current/last run |
| `=Name%triggerCount` | `#live.int` | Total times triggered |

### Variable (`$`)

| Accessor | Type | Description |
|----------|------|-------------|
| `$name%state` | `#live.#VarState` | Declared, Default, Final, Failed, Released |
| `$name%sourceError` | `#live.error` | Error that triggered a `<!` fallback, or `!NoError` if no error. See [[errors#Error Fallback Operators]] |

### Data (`{#}`)

| Accessor | Type | Description |
|----------|------|-------------|
| `#Name%lastModified` | `#live.string` | Last modification timestamp |
| `#Name%files` | `#live.array:path` | Associated file paths |
| `#Name%errors` | `#live.array:error` | Accumulated errors |
| `#Name%usageCount` | `#live.int` | Usage count |

### Wrapper (`{W}`)

| Accessor | Type | Description |
|----------|------|-------------|
| `=W.Name%status` | `#live.#WrapperStatus` | Setup, Active, Cleanup, Complete, Failed |
| `=W.Name%errors` | `#live.array:error` | Accumulated wrapper errors |
| `=W.Name%setupDuration` | `#live.string` | Duration of setup phase |
| `=W.Name%cleanupDuration` | `#live.string` | Duration of cleanup phase |

### Queue (`{Q}`)

| Accessor | Type | Description |
|----------|------|-------------|
| `#Queue:Name%pendingCount` | `#live.int` | Queued pipelines awaiting dispatch |
| `#Queue:Name%activeCount` | `#live.int` | Currently executing pipeline instances |
| `#Queue:Name%totalProcessed` | `#live.int` | Total pipelines processed |
| `#Queue:Name%strategy` | `#live.#QueueStrategy` | Current queue strategy |

### Macro (`{M}`)

Live fields for macros are not yet defined.

## Rules

- **PGE02006:** `live` fields are pull-only — any push is a compile error
- **Non-live** `[%]` fields follow normal [[variable-lifecycle]] rules
- Prefer reactive patterns (error blocks, IO triggers) over polling `live` fields when possible. Use `%` when you genuinely need runtime introspection

## Related

- [[data-is-trees]] — conceptual overview of the unified tree
- [[metadata-tree/INDEX|technical/spec/metadata-tree]] — full tree architecture, path grammar, instance rules
- [[enums]] — `#PipelineStatus`, `#VarState`, `#QueueStrategy` enum definitions
- [[variable-lifecycle]] — lifecycle stages that `$name%state` reports
- [[permissions]] — `[%]` metadata that precedes permission declarations
