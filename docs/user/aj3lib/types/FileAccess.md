---
audience: automation-builder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.#:FileAccess"
metadata_instance: "%#:FileAccess:N"
---

# #FileAccess Enum

<!-- @c:types -->

Runtime `##Enum` type available in every `.jm3` file. Uses `[#] ##Enum` (enum classification), `[#] ##Scalar` (depth 1), and `[#] ###ScalarEnum` (leaf content is variant selection).

---

## Definition

```aljam3
{#} #FileAccess
   [%] .description << "File access state"
   [%] .version << "1.0.0"
   [#] ##Enum
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "fileaccess"
   [.] .Available
   [.] .Locked
   [.] .NotFound
```

---

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:FileAccess` | Compile-time type template |
| Instance | `%#:FileAccess:0` | Runtime instance (enum — one active field) |

---

## Related

- [[enums]] — other jm3lib enum types
- [[jm3lib/pipelines/File/INDEX|-File.*]] — file operations
- [[syntax/types/INDEX|types]] — full type system specification
