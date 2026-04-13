---
audience: automation-builder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.#:FieldKind"
metadata_instance: "%#:FieldKind:N"
---

# #FieldKind Enum

<!-- @c:types -->

Runtime `##Enum` type available in every `.pg` file. Uses `[#] ##Enum` (enum classification), `[#] ##Scalar` (depth 1), and `[#] ###ScalarEnum` (leaf content is variant selection).

---

## Definition

```polyglot
{#} #FieldKind
   [%] .description << "Leaf content field type classifier"
   [%] .version << "1.0.0"
   [#] ##Enum
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "fieldkind"
   [.] .Value
   [.] .Enum
   [.] .None
```

Used by the `%##Leafs.Kind` schema property to constrain what `###` field type all leafs in a type must be. For example, `[#] %##Leafs.Kind << #FieldKind.Enum` requires all leafs to be `###Enum` (no type annotation). See [[syntax/types/schema-properties#Approved ## Schema Types]].

---

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:FieldKind` | Compile-time type template |
| Instance | `%#:FieldKind:0` | Runtime instance (enum — one active field) |

---

## Related

- [[enums]] — other pglib enum types
- [[scalars]] — scalar type system
- [[syntax/types/INDEX|types]] — full type system specification
