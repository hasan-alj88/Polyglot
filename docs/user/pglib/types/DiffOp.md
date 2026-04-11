---
audience: pg-coder
type: specification
updated: 2026-04-11
status: draft
metadata_definition: "%definition.#:DiffOp"
metadata_instance: "%#:DiffOp:N"
---

# #DiffOp Enum

<!-- @c:types -->

Runtime `##Enum` type available in every `.pg` file. Classifies the operation type in a text diff entry — whether a line was added, deleted, or replaced relative to the original text.

---

## Definition

```polyglot
{#} #DiffOp
   [%] .description << "Text diff operation type"
   [#] ##Enum
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "diffop"
   [.] .Add
   [.] .Delete
   [.] .Replace
```

---

## Variants

| Variant | Description |
|---------|-------------|
| `.Add` | Line inserted at position (not present in original) |
| `.Delete` | Line removed from original |
| `.Replace` | Line content changed (same position, different content) |

---

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:DiffOp` | Compile-time type template |
| Instance | `%#:DiffOp:0` | Runtime instance (enum — one active field) |

---

## Related

- [[TextDiff]] — uses #DiffOp for operation classification
- [[enums]] — other pglib enum types
- [[syntax/types/INDEX|types]] — full type system specification
