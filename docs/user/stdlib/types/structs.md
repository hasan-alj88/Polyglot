---
audience: pg-coder
type: specification
updated: 2026-04-04
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

See [[syntax/types/strings#Path Type]] for full details including `=Path"..."`, path roots, and shorthands.

### Error Codes

- PGE04001 -- plain string assigned to `#path` (type mismatch)
- PGE04007 -- invalid path string
- PGE04008 -- missing path platform subfield
- PGW04001 -- single-platform path (warning)

---

## #Queue

Queue configuration struct used in `{Q}` queue definitions. Each `{Q} #Queue:Name` creates a Dispatch Queue at metadata path `%Queue.DispatchQueue:Name`.

```polyglot
{#} #Queue
   [%] .description << "Queue configuration for pipeline dispatch"
   [%] .version << "2.0.0"
   [#] << ##Scalar
   [.] .strategy;#QueueStrategy
   [.] .host;#String
   [.] .maxInstances;#UnsignedInt
   [.] .maxConcurrent;#UnsignedInt
   [.] .resourceTags;#Array:ResourceTag
   [.] .killPropagation;#KillPropagation
   [.] .maxWaitTime;#String
   [.] .description;#String
```

### Fields

| Field | Type | Purpose | Default |
|-------|------|---------|---------|
| `.strategy` | `#QueueStrategy` | FIFO, LIFO, Priority | (required) |
| `.host` | `#String` | Target host for job execution (1 queue = 1 host) | `"localhost"` |
| `.maxInstances` | `#UnsignedInt` | Max parallel instances per pipeline on this queue | unlimited |
| `.maxConcurrent` | `#UnsignedInt` | Max other pipelines alongside (queue-level default) | unlimited |
| `.resourceTags` | `#Array:ResourceTag` | Resource tags for constraint checking | empty |
| `.killPropagation` | `#KillPropagation` | How kill signals propagate to sub-jobs | `#Cascade` |
| `.maxWaitTime` | `#String` | Max time in queue before escalation (e.g., "30m") | unlimited |
| `.description` | `#String` | Human-readable queue description | empty |

See [[enums#QueueStrategy]], [[enums#KillPropagation]], [[enums#ResourceTag]] for enum definitions. `#RetriggerStrategy` is a queue configuration enforced by the Trigger Monitor — see [[concepts/pipelines/io-triggers#Retrigger Strategy|Triggers]].

## Related

- [[enums]] -- #OS, #QueueStrategy, #KillPropagation, #ResourceTag, and other enums
- [[concepts/collections/INDEX|collections]] -- #Map, #Array, #Serial collection types
- [[syntax/types/INDEX|types]] -- full type system specification
