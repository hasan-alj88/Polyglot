---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
---

# Struct Types

<!-- @c:types -->

aj3lib struct types available in every `.aj3` file. These are non-enum, non-collection structs with typed value fields.

| Type | Description | File |
|------|-------------|------|
| `#path` | Cross-platform file system path | [[path]] |
| `#Queue` | Queue configuration for pipeline dispatch | [[Queue]] |
| `#Job` | Runtime job state within a pipeline instance | [[Job]] |

## Related

- [[enums]] -- #OS, #QueueStrategy, #KillPropagation, #ResourceTag, and other enums
- [[collections]] -- #Map, #Array, #Serial collection types
- [[syntax/types/INDEX|types]] -- full type system specification
- [[glossary]] -- Job vs Instance distinction
