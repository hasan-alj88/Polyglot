---
audience: automation-builder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.#:OS"
metadata_instance: "%#:OS:N"
---

# #OS Enum

<!-- @c:types -->

Runtime `##Enum` type available in every `.jm3` file. Uses `[#] ##Enum` (enum classification), `[#] ##Scalar` (depth 1), and `[#] ###ScalarEnum` (leaf content is variant selection).

---

## Definition

```aljam3
{#} #OS
   [%] .description << "Operating system enum"
   [%] .version << "1.0.0"
   [#] ##Enum
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "os"
   [.] .Unix
   [.] .Windows
```

---

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:OS` | Compile-time type template |
| Instance | `%#:OS:0` | Runtime instance (enum — one active field) |

---

## Related

- [[enums]] — other jm3lib enum types
- [[NativeKind]] — native kind enum
- [[syntax/types/INDEX|types]] — full type system specification
