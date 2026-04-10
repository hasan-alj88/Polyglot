---
audience: pg-coder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.#:AuditLevel"
metadata_instance: "%#:AuditLevel:N"
---

# #AuditLevel Enum

<!-- @types -->

Permission enum used in `{_}` permission objects. See [[concepts/permissions|Permissions]].

## Definition

```polyglot
{#} #AuditLevel
   [%] .description << "Permission audit logging level"
   [%] .version << "1.0.0"
   [#] ##Enum
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "auditlevel"
   [.] .None
   [.] .OnUse
   [.] .OnDeny
   [.] .All
```

| Variant | Description |
|---------|-------------|
| `.None` | No audit logging |
| `.OnUse` | Log on permission use |
| `.OnDeny` | Log on permission denial |
| `.All` | Log all permission events |

> **Note:** Controls when permission usage is logged.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:AuditLevel` | Compile-time type template |
| Instance | `%#:AuditLevel:0` | Runtime instance (enum — one active field) |

## Related

- [[concepts/permissions|Permissions]] -- permission system overview
- [[enums]] -- all enum types
- [[syntax/types/INDEX|types]] -- full type system specification
