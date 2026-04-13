---
audience: automation-builder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.#:IODirection"
metadata_instance: "%#:IODirection:N"
---

# #IODirection Enum

<!-- @c:types -->

Permission enum used in `{_}` permission objects. See [[concepts/permissions|Permissions]].

## Definition

```polyglot
{#} #IODirection
   [%] .description << "IO direction for permission scoping"
   [%] .version << "1.0.0"
   [#] ##Enum
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "iodirection"
   [.] .Inbound
   [.] .Outbound
   [.] .Both
```

| Variant | Description |
|---------|-------------|
| `.Inbound` | Reading/receiving direction |
| `.Outbound` | Writing/sending direction |
| `.Both` | Both inbound and outbound |

> **Note:** Scopes whether permission covers reading, writing, or both.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:IODirection` | Compile-time type template |
| Instance | `%#:IODirection:0` | Runtime instance (enum — one active field) |

## Related

- [[concepts/permissions|Permissions]] -- permission system overview
- [[enums]] -- all enum types
- [[syntax/types/INDEX|types]] -- full type system specification
