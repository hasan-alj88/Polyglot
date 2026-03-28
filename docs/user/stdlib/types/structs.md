---
audience: user
type: specification
updated: 2026-03-28
status: complete
---

# Struct Types

<!-- @types -->

Stdlib struct types available in every `.pg` file. These are non-enum, non-collection structs with typed value fields.

---

## #path

Cross-platform file system path with OS-specific subfields. At runtime, the Polyglot runtime resolves `$pathVar` to the correct subfield based on the current OS.

```polyglot
{#} #path
   [%] .description << "Cross-platform file system path"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [.] .Unix#string
   [.] .Windows#string
```

### Fields

| Field | Type | Purpose |
|-------|------|---------|
| `.Unix` | `#string` | Unix/macOS path |
| `.Windows` | `#string` | Windows path |

### Usage

Assign both subfields for cross-platform code:

```polyglot
[r] $AppDir#path
   [.] .Unix << "/tmp/MyApp"
   [.] .Windows << "C:\MyApp"
```

Or use `=Path"..."` inline notation for automatic OS normalization:

```polyglot
[r] $LogDir#path << =Path"/tmp/MyApp/logs"
```

See [[types#Path Type]] for full details including `=Path"..."`, path roots, and shorthands.

### Error Codes

- PGE-401 -- plain string assigned to `#path` (type mismatch)
- PGE-407 -- invalid path string
- PGE-408 -- missing path platform subfield
- PGW-408 -- single-platform path (warning)

---

## #Queue

Queue configuration struct used in pipeline `[Q]` blocks.

```polyglot
{#} #Queue
   [%] .description << "Queue configuration for pipeline execution"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [.] .strategy#QueueStrategy
   [.] .retrigger#RetriggerStrategy
```

### Fields

| Field | Type | Purpose |
|-------|------|---------|
| `.strategy` | `#QueueStrategy` | Queue ordering: FIFO, LIFO, or Priority |
| `.retrigger` | `#RetriggerStrategy` | Behavior on re-trigger while queued/running |

See [[enums#QueueStrategy]] and [[enums#RetriggerStrategy]] for the enum definitions.

## Related

- [[enums]] -- #OS, #QueueStrategy, #RetriggerStrategy, and other enums
- [[collections]] -- #Map, #Array, #Serial collection types
- [[types]] -- full type system specification
