---
audience: automation-builder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.#:ResourceTag"
metadata_instance: "%#:ResourceTag:N"
---

# #ResourceTag Enum

<!-- @c:types -->

Runtime `##Enum` type available in every `.jm3` file. Uses `[#] ##Enum` (enum classification), `[#] ##Scalar` (depth 1), and `[#] ###ScalarEnum` (leaf content is variant selection).

---

## Definition

```aljam3
{#} #ResourceTag
   [%] .description << "Resource tag for pipeline dispatch constraints"
   [%] .version << "1.0.0"
   [#] ##Enum
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "resourcetag"
   [.] .GPU
   [.] .HighRAM
   [.] .HighCPU
   [.] .HighIO
   [.] .Network
```

Used in `#Queue.resourceTags` for dispatch constraint checking. The Dispatch Coordinator enforces resource exclusion — e.g., only one `#GPU`-tagged pipeline executes at a time.

---

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:ResourceTag` | Compile-time type template |
| Instance | `%#:ResourceTag:0` | Runtime instance (enum — one active field) |

---

## Related

- [[enums]] — other jm3lib enum types
- [[Queue]] — queue configuration
- [[syntax/types/INDEX|types]] — full type system specification
