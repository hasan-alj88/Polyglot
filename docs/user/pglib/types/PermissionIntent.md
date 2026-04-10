---
audience: pg-coder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.#:PermissionIntent"
metadata_instance: "%#:PermissionIntent:N"
---

# #PermissionIntent Enum

<!-- @c:types -->

Permission enum used in `{_}` permission objects. See [[concepts/permissions|Permissions]].

## Definition

```polyglot
{#} #PermissionIntent
   [%] .description << "Permission object intent classifier"
   [%] .version << "1.0.0"
   [#] ##Enum
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "permissionintent"
   [.] .Ceiling
   [.] .Grant
```

| Variant | Description |
|---------|-------------|
| `.Ceiling` | Upper bound on permissions (package-level) |
| `.Grant` | Requests specific capabilities (pipeline-level) |

> **Note:** Ceiling sets upper bound (package-level), Grant requests specific capabilities (pipeline-level).

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:PermissionIntent` | Compile-time type template |
| Instance | `%#:PermissionIntent:0` | Runtime instance (enum — one active field) |

## Related

- [[concepts/permissions|Permissions]] -- permission system overview
- [[enums]] -- all enum types
- [[syntax/types/INDEX|types]] -- full type system specification
