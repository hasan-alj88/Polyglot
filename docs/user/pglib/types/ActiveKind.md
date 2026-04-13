---
audience: automation-builder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.#:ActiveKind"
metadata_instance: "%#:ActiveKind:N"
---

# #ActiveKind Enum

<!-- @c:types -->

Runtime `##Enum` type available in every `.pg` file. Uses `[#] ##Enum` (enum classification), `[#] ##Scalar` (depth 1), and `[#] ###ScalarEnum` (leaf content is variant selection).

---

## Definition

```polyglot
{#} #ActiveKind
   [%] .description << "Branch activation classifier"
   [%] .version << "1.0.0"
   [#] ##Enum
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "activekind"
   [.] .All         [ ] Every branch must be present
   [.] .One         [ ] Exactly one branch active
   [.] .Partial     [ ] Any non-zero subset
```

Used by the `%##Active` schema property to describe how many of a branch's children must be present at runtime. `.All` means every declared branch must exist (structs, records). `.One` means exactly one branch is active (enums, tagged unions). `.Partial` means any non-zero subset of branches may be present (sparse containers).

---

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:ActiveKind` | Compile-time type template |
| Instance | `%#:ActiveKind:0` | Runtime instance (enum -- one active field) |

---

## Related

- [[enums]] -- other pglib enum types
- [[syntax/types/schema-properties|Schema Properties]] -- `%##Active` property using this type
- [[syntax/types/INDEX|types]] -- full type system specification
