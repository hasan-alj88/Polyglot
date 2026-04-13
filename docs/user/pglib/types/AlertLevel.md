---
audience: automation-builder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.#:AlertLevel"
metadata_instance: "%#:AlertLevel:N"
---

# #AlertLevel Enum

<!-- @c:types -->

Permission enum used in `{_}` permission objects. See [[concepts/permissions|Permissions]].

## Definition

```polyglot
{#} #AlertLevel
   [%] .description << "Permission alert level"
   [%] .version << "1.0.0"
   [#] ##Enum
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "alertlevel"
   [.] .None
   [.] .OnDeny
   [.] .OnEscalation
```

| Variant | Description |
|---------|-------------|
| `.None` | No alerts |
| `.OnDeny` | Alert on permission denial |
| `.OnEscalation` | Alert on privilege escalation |

> **Note:** Controls when permission violations trigger alerts.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:AlertLevel` | Compile-time type template |
| Instance | `%#:AlertLevel:0` | Runtime instance (enum — one active field) |

## Related

- [[concepts/permissions|Permissions]] -- permission system overview
- [[enums]] -- all enum types
- [[syntax/types/INDEX|types]] -- full type system specification
