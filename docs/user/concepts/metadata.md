---
audience: developer
type: spec
updated: 2026-03-22
---

# Metadata

<!-- @glossary:Polyglot Service -->
<!-- @data-is-trees -->

The `%` accessor reads metadata from named objects — pipelines, variables, data types, macros, operators, errors, and packages. All Polyglot objects live on one unified tree (see [[data-is-trees]] for the conceptual overview). This document covers the detailed field listings.

Metadata is organized as a tree: `%{type}:{ref}:{instance}.{fields}`. Two categories exist:
- **User-declared** — written via `[%]` block elements inside `{x}` definitions, follow normal lifecycle rules
- **Polyglot-managed (`live`)** — populated by the runtime, read-only (PGE-206 applies)

For the formal path grammar and instance rules, see [[metadata-tree|technical/spec/metadata-tree]].

## Metadata Tree

```
%
├── #  (Structs)
│   └── :<type ref>:<instance>
│       ├── .lastModified    ;live.string
│       ├── .files           ;live.array.path
│       ├── .errors          ;live.array.error
│       ├── .usageCount      ;live.int
│       ├── .description     ;string           (user-declared)
│       ├── .version         ;string           (user-declared)
│       ├── .authors         ;array.string     (user-declared)
│       ├── .license         ;string           (user-declared)
│       ├── .deprecated      ;bool             (user-declared)
│       ├── .alias                             (user-declared)
│       └── :info            ;serial           (user-declared, flexible)
├── =  (Pipelines)
│   └── :<name>:<instance>
│       ├── .status          ;live.#PipelineStatus
│       ├── .errors          ;live.array.error
│       ├── .isSuccess       ;live.#Boolean
│       ├── .instanceCount   ;live.int
│       ├── .lastRun         ;live.string
│       ├── .duration        ;live.string
│       ├── .triggerCount    ;live.int
│       ├── .description     ;string           (user-declared)
│       ├── .version         ;string           (user-declared)
│       ├── .authors         ;array.string     (user-declared)
│       ├── .license         ;string           (user-declared)
│       ├── .deprecated      ;bool             (user-declared)
│       ├── .deprecatedMessage ;string         (user-declared)
│       ├── .alias                             (user-declared)
│       ├── .<               (input ports)
│       ├── .>               (output ports)
│       └── :info            ;serial           (user-declared, flexible)
├── ~  (Expanders)
│   └── :<name>:<instance>
│       ├── .<               (expand inputs)
│       └── .>               (expand outputs)
├── *  (Collectors)
│   └── :<name>:<instance>
│       ├── .<               (collect inputs)
│       └── .>               (collect outputs)
├── $  (Variables)
│   └── :<name>:<instance>
│       ├── .state           ;live.#VarState
│       └── .sourceError     ;live.error        (NEW — !NoError if no error)
├── Q  (Queues)
│   └── :<name>:<instance>
│       ├── .strategy        ;live.#QueueStrategy
│       ├── .retrigger       ;live.#RetriggerStrategy
│       ├── .state           ;live.#QueueState
│       ├── .pendingCount    ;live.int
│       ├── .activeCount     ;live.int
│       ├── .description     ;string           (user-declared)
│       ├── .version         ;string           (user-declared)
│       └── .alias                             (user-declared)
├── M  (Macros)
│   └── :<name>:<instance>
│       ├── .description     ;string           (user-declared)
│       ├── .version         ;string           (user-declared)
│       ├── .authors         ;array.string     (user-declared)
│       ├── .license         ;string           (user-declared)
│       ├── .deprecated      ;bool             (user-declared)
│       ├── .deprecatedMessage ;string         (user-declared)
│       ├── .alias                             (user-declared)
│       ├── :info            ;serial           (user-declared, flexible)
│       └── (live fields TBD)
├── !  (Errors)
│   └── :<namespace>
│       └── .<error path>    (fixed hierarchy)
├── @  (Packages)
│   └── :<address>
│       └── (package metadata)
└── definition              (compile-time schema templates)
    └── .{type}:{ref}       (structural template for all instances)
```

## String Subtypes in the Tree

String subtypes live under `%#:String:*` at a flexible level. Each subtype uses the `#String` schema with `.re` pre-filled:

| Subtype | Tree path | `.re` pattern |
|---------|-----------|---------------|
| `int` | `%#:String:int` | `^-?[0-9]+$` |
| `float` | `%#:String:float` | `^-?[0-9]+\.[0-9]+$` |
| custom | `%#:String:<name>` | User-defined pattern |

User code `;int` is an alias for `;String.int`. See [[types#Numeric Types — #String Subtypes]].

## Enum Active-Field-Only

An enum instance collapses to ONE active field. The definition lists all valid branches, but a specific instance has only the active one:

```
%definition.#:Boolean       ← lists .True and .False
%#:Boolean:0.True           ← instance 0: .True is active, .False does NOT exist
```

Push atomically clears the previous field and sets the new one. Reading a non-active field returns no path. See [[data-is-trees#Enum Instances — Active-Field-Only]].

## User-Declared Metadata

The `[%]` block element ([[blocks#Metadata]]) lives inside any `{x}` definition (`{#}`, `{=}`, `{M}`). One definition = one metadata set.

### Fixed Fields

| Field | Type | Description |
|-------|------|-------------|
| `.description` | `;string` | Human-readable description |
| `.version` | `;string` | Semantic version |
| `.authors` | `;array.string` | Author list |
| `.license` | `;string` | License identifier |
| `.deprecated` | `;bool` | Deprecation flag |
| `.deprecatedMessage` | `;string` | Reason for deprecation and suggested replacement |
| `.alias` | — | Shorthand name. Preserves type prefix (`#` for data, `=` for pipelines) |

### Flexible Field

| Field | Type | Description |
|-------|------|-------------|
| `:info` | `;serial` | Opens a flexible scope for custom key-value tooling data |

### Assignment

User-declared fields follow normal variable lifecycle rules ([[variable-lifecycle]]):

```polyglot
{= =MyPipeline}
   [%] .description << "Processes incoming invoices"
   [%] .version << "2.1.0"
   [%] .authors << ["Alice", "Bob"]
   [%] .deprecated << false
   [%] .alias << =ProcessInvoice
   [%] :info
      :team << "payments"
      :priority << "high"
```

## Live Metadata Fields

`live` fields are implicit on every `{=}` pipeline, `$` variable, and `{#}` struct. The runtime populates them automatically. Users read them via `%` but cannot push into them (PGE-206). See [[types#Live Type Modifier]].

### Pipeline (`{=}`)

| Accessor | Type | Description |
|----------|------|-------------|
| `=Name%status` | `;live.#PipelineStatus` | AwaitTrigger, Disabled, Running, Failed |
| `=Name%errors` | `;live.array.error` | Accumulated errors |
| `=Name%isSuccess` | `;live.#Boolean` | Last run completed without error |
| `=Name%instanceCount` | `;live.int` | Number of active instances |
| `=Name%lastRun` | `;live.string` | Timestamp of last execution |
| `=Name%duration` | `;live.string` | Duration of current/last run |
| `=Name%triggerCount` | `;live.int` | Total times triggered |

### Variable (`$`)

| Accessor | Type | Description |
|----------|------|-------------|
| `$name%state` | `;live.#VarState` | Declared, Default, Final, Failed, Released |
| `$name%sourceError` | `;live.error` | Error that triggered a `<!` fallback, or `!NoError` if no error. See [[errors#Error Fallback Operators]] |

### Data (`{#}`)

| Accessor | Type | Description |
|----------|------|-------------|
| `#Name%lastModified` | `;live.string` | Last modification timestamp |
| `#Name%files` | `;live.array.path` | Associated file paths |
| `#Name%errors` | `;live.array.error` | Accumulated errors |
| `#Name%usageCount` | `;live.int` | Usage count |

### Macro (`{M}`)

Live fields for macros are not yet defined.

## Instance References

One pipeline definition may spawn multiple concurrent instances. Each instance has its own metadata set — same schema, different values. The schema is **fixed** per block type.

Access pattern: `%=MyPipeline.<instance>.<field>` or shorthand `=MyPipeline%field` (defaults to current instance).

## Related Types

<!-- @glossary:Polyglot Service -->

| Type | Values | Defined in |
|------|--------|------------|
| `#PipelineStatus` | AwaitTrigger, Disabled, Running, Failed | [[STDLIB]] |
| `#VarState` | Declared, Default, Final, Failed, Released | [[STDLIB]] |

## Rules

- **PGE-206:** `live` fields are pull-only — any push is a compile error
- **Non-live** `[%]` fields follow normal [[variable-lifecycle]] rules
- Prefer reactive patterns (error blocks, IO triggers) over polling `live` fields when possible. Use `%` when you genuinely need runtime introspection
