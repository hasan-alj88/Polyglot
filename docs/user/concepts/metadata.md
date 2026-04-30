---
audience: automation-builder
type: concept
updated: 2026-03-31
---

# Metadata

<!-- @c:glossary:Aljam3 Service -->
<!-- @c:data-is-trees -->
<!-- @u:technical/spec/metadata-tree/INDEX -->
<!-- @u:technical/edge-cases/15-metadata-blocks -->
<!-- @u:blocks#Metadata -->
<!-- @u:variable-lifecycle#Querying Lifecycle State -->

Every Aljam3 object carries metadata — descriptive fields you declare and runtime fields the system manages. You interact with metadata through two mechanisms:

1. **`[%]` block element** — declare metadata inside `{#}`, `{-}`, or other `{x}` definitions
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

### Flexible Fields

| Field | Type | Description |
|-------|------|-------------|
| `%alias` | `#Array.NestedKeyString` | Shorthand names — multiple aliases per definition. Each alias is a `#NestedKeyString` (allows `.` and `:` for nested paths). Must be globally unique (PGE12002) |
| `%Native.Class` | scope | Native class backing — `{#}` only. Declares that a host-language class backs this type. Fields: `.Rust` (class name), `.Validate` (`#True` if class provides validation). See [[aj3lib/types/string\|#String]] for example |
| `:info` | `#serial` | Opens a flexible scope for custom key-value tooling data |

### Example

```aljam3
{- -MyPipeline}
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

`live` fields are populated by the Aljam3 runtime automatically. Users read them via `%` but cannot push into them (PGE02006). See [[syntax/types/hierarchy#Live Type Modifier]].

### Pipeline (`{-}`)

| Accessor | Type | Description |
|----------|------|-------------|
| `-Name%status` | `#live.#PipelineStatus` | AwaitTrigger, Disabled, Running, Failed |
| `-Name%errors` | `#live.array:error` | Accumulated errors |
| `-Name%isSuccess` | `#live.#Boolean` | Last run completed without error |
| `-Name%instanceCount` | `#live.int` | Number of active instances |
| `-Name%lastRun` | `#live.string` | Timestamp of last execution |
| `-Name%duration` | `#live.string` | Duration of current/last run |
| `-Name%triggerCount` | `#live.int` | Total times triggered |

### Variable (`$`)

| Accessor | Type | Description |
|----------|------|-------------|
| `$name%state` | `#live.#VarState` | Declared, Default, Final, Failed, Released |
| `$name%sourceError` | `#live.error` | Error that triggered a `!<` fallback, or `!NoError` if no error. See [[errors#Error Fallback Operators]] |

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
| `-W.Name%status` | `#live.#WrapperStatus` | Setup, Active, Cleanup, Complete, Failed |
| `-W.Name%errors` | `#live.array:error` | Accumulated wrapper errors |
| `-W.Name%setupDuration` | `#live.string` | Duration of setup phase |
| `-W.Name%cleanupDuration` | `#live.string` | Duration of cleanup phase |

### Queue (`{Q}`)

| Accessor | Type | Description |
|----------|------|-------------|
| `#Queue:Name%pendingCount` | `#live.int` | Queued pipelines awaiting dispatch |
| `#Queue:Name%activeCount` | `#live.int` | Currently executing pipeline instances |
| `#Queue:Name%totalProcessed` | `#live.int` | Total pipelines processed |
| `#Queue:Name%strategy` | `#live.#QueueStrategy` | Current queue strategy |


## Advanced: Full Metadata Paths

The shorthand accessors shown above (`$name%state`, `-MyPipeline%status`) are syntactic sugar. The compiler resolves each to a full path in the `%` metadata tree.

### Shorthand Resolution

| You write | Compiler resolves to |
|-----------|---------------------|
| `$myVar%state` | `%$:myVar:<current>.state` |
| `-MyPipeline%status` | `%-:MyPipeline:<current>.status` |
| `#Record%lastModified` | `%#:Record:<current>.lastModified` |
| `-W.DB.Connection%status` | `%W:DB.Connection:<current>.status` |
| `#Queue:GPUQueue%activeCount` | `%Q:GPUQueue:<current>.activeCount` |

The full path follows the pattern: `%{type}:{name}:{instance}.{field}`

- **`%`** — metadata tree root
- **`{type}`** — object type prefix (`$`, `-`, `#`, `W`, `Q`, `T`)
- **`{name}`** — object reference name (flexible field, uses `:`)
- **`{instance}`** — which instance (flexible field, uses `:`)
- **`.{field}`** — fixed field within the instance

### `:<current>` — Implicit Instance

When you write `-MyPipeline%status`, the `:<current>` instance segment is implicit — the runtime resolves it to the instance executing in the current context. Most code never needs to specify an instance explicitly.

### Instance Addressing with `:N`

When a pipeline has multiple concurrent instances, you can target a specific one by instance number:

```aljam3
[-] $status << %-:MyPipeline:3.status
```

This reads the status of instance 3 of `-MyPipeline`. Explicit instance addressing is primarily useful for monitoring and debugging — normal pipeline code uses the implicit `:<current>` resolution.

For the complete path grammar including job paths and marker addressing, see [[metadata-tree/path-grammar|Path Grammar]].

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
