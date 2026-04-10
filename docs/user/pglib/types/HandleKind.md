---
audience: pg-coder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.#:HandleKind"
metadata_instance: "%#:HandleKind:N"
---

# #HandleKind Enum

<!-- @types -->

Permission enum used in `{_}` permission objects. See [[concepts/permissions|Permissions]].

## Definition

```polyglot
{#} #HandleKind
   [%] .description << "Resource handle type for permission descriptors"
   [%] .version << "1.0.0"
   [#] ##Enum
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "handlekind"
   [.] .Path
   [.] .ConnectionString
```

| Variant | Description |
|---------|-------------|
| `.Path` | File system path handle |
| `.ConnectionString` | Connection string handle |

> **Note:** Specifies how the resource is addressed.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:HandleKind` | Compile-time type template |
| Instance | `%#:HandleKind:0` | Runtime instance (enum — one active field) |

## Related

- [[concepts/permissions|Permissions]] -- permission system overview
- [[enums]] -- all enum types
- [[syntax/types/INDEX|types]] -- full type system specification
