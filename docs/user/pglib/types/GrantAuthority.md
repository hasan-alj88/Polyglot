---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.#:GrantAuthority"
metadata_instance: "%#:GrantAuthority:N"
---

# #GrantAuthority Enum

<!-- @types -->

Permission enum used in `{_}` permission objects. See [[concepts/permissions|Permissions]].

## Definition

```polyglot
{#} #GrantAuthority
   [%] .description << "Permission grant authority level"
   [%] .version << "1.0.0"
   [#] << ##Enum
   [#] << ##Scalar
   [#] << ###ScalarEnum
   [#] %##Alias << "grantauthority"
   [.] .Package
   [.] .Pipeline
```

| Variant | Description |
|---------|-------------|
| `.Package` | Grant applies to all pipelines in the package |
| `.Pipeline` | Grant is scoped to one pipeline |

> **Note:** Package-level grants apply to all pipelines; pipeline-level grants are scoped to one pipeline.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:GrantAuthority` | Compile-time type template |
| Instance | `%#:GrantAuthority:0` | Runtime instance (enum — one active field) |

## Related

- [[concepts/permissions|Permissions]] -- permission system overview
- [[enums]] -- all enum types
- [[syntax/types/INDEX|types]] -- full type system specification
