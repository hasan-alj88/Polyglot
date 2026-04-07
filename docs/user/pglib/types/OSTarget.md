---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.#:OSTarget"
metadata_instance: "%#:OSTarget:N"
---

# #OSTarget Enum

<!-- @types -->

Permission enum used in `{_}` permission objects. See [[concepts/permissions|Permissions]].

## Definition

```polyglot
{#} #OSTarget
   [%] .description << "Target OS for permission constraints"
   [%] .version << "1.0.0"
   [#] << ##Enum
   [#] << ##Scalar
   [#] << ###ScalarEnum
   [#] %##Alias << "ostarget"
   [.] .Any
   [.] .Linux
   [.] .Windows
   [.] .MacOS
```

| Variant | Description |
|---------|-------------|
| `.Any` | Applies to all operating systems |
| `.Linux` | Linux only |
| `.Windows` | Windows only |
| `.MacOS` | macOS only |

> **Note:** Constrains which OS a permission applies to.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:OSTarget` | Compile-time type template |
| Instance | `%#:OSTarget:0` | Runtime instance (enum — one active field) |

## Related

- [[concepts/permissions|Permissions]] -- permission system overview
- [[enums]] -- all enum types
- [[syntax/types/INDEX|types]] -- full type system specification
