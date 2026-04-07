---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.#:FileAccess"
metadata_instance: "%#:FileAccess:N"
---

# #FileAccess Enum

<!-- @types -->

Runtime `##Enum` type available in every `.pg` file. Uses `[#] << ##Enum` (enum classification), `[#] << ##Scalar` (depth 1), and `[#] << ###ScalarEnum` (leaf content is variant selection).

---

## Definition

```polyglot
{#} #FileAccess
   [%] .description << "File access state"
   [%] .version << "1.0.0"
   [#] << ##Enum
   [#] << ##Scalar
   [#] << ###ScalarEnum
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

- [[enums]] — other pglib enum types
- [[pglib/pipelines/File/INDEX|=File.*]] — file operations
- [[syntax/types/INDEX|types]] — full type system specification
