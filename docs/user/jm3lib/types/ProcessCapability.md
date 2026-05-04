---
audience: automation-builder
type: specification
updated: 2026-04-18
status: complete
metadata_definition: "%definition.#:ProcessCapability"
metadata_instance: "%#:ProcessCapability:N"
---

# #ProcessCapability Enum

<!-- @c:types -->

Capability enum for `#Processes` permission category. See [[concepts/permissions|Permissions]].

## Definition

```aljam3
{#} #ProcessCapability
   [%] .description << "Process count limit capabilities"
   [%] .version << "1.0.0"
   [#] ##Enum
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "processcapability"
   [.] .Limit
```

| Variant | Description |
|---------|-------------|
| `.Limit` | Set maximum number of child processes |

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:ProcessCapability` | Compile-time type template |
| Instance | `%#:ProcessCapability:0` | Runtime instance (enum — one active field) |

## Related

- [[PermissionCategory]] -- permission category enum
- [[concepts/permissions|Permissions]] -- permission system overview
