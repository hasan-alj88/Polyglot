---
audience: pg-coder
type: specification
updated: 2026-04-08
status: complete
metadata_definition: "%definition.#:FlexKind"
metadata_instance: "%#:FlexKind:N"
---

# #FlexKind Enum

<!-- @types -->

Runtime `##Enum` type available in every `.pg` file. Uses `[#] << ##Enum` (enum classification), `[#] << ##Scalar` (depth 1), and `[#] << ###ScalarEnum` (leaf content is variant selection).

---

## Definition

```polyglot
{#} #FlexKind
   [%] .description << "Branch flexibility classifier"
   [%] .version << "1.0.0"
   [#] << ##Enum
   [#] << ##Scalar
   [#] << ###ScalarEnum
   [#] %##Alias << "flexkind"
   [.] .Fixed       [ ] . fields, author-declared
   [.] .Flexible    [ ] : fields, user adds/removes
   [.] .Range       [ ] : fields, compiler-generated from %##Range
```

Used by the `%##Flexible` schema property to describe how a branch's children are accessed. `.Fixed` means the branch has `.` fixed fields declared by the type author. `.Flexible` means the branch has `:` flexible fields that users add and remove at runtime. `.Range` means the branch has `:` flexible fields whose keys are compiler-generated from the `%##Range` interval.

---

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:FlexKind` | Compile-time type template |
| Instance | `%#:FlexKind:0` | Runtime instance (enum -- one active field) |

---

## Related

- [[enums]] -- other pglib enum types
- [[syntax/types/schema-properties|Schema Properties]] -- `%##Flexible` property using this type
- [[syntax/types/INDEX|types]] -- full type system specification
