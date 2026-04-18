---
audience: automation-builder
type: specification
updated: 2026-04-18
status: complete
metadata_definition: "%definition.#:IOCapability"
metadata_instance: "%#:IOCapability:N"
---

# #IOCapability Enum

<!-- @c:types -->

Capability enum for `#IO` permission category. See [[concepts/permissions|Permissions]].

## Definition

```polyglot
{#} #IOCapability
   [%] .description << "IO resource limit capabilities"
   [%] .version << "1.0.0"
   [#] ##Enum
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "iocapability"
   [.] .Limit
   [.] .Iops
```

| Variant | Description |
|---------|-------------|
| `.Limit` | Set maximum IO bandwidth |
| `.Iops` | Set maximum IO operations per second |

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:IOCapability` | Compile-time type template |
| Instance | `%#:IOCapability:0` | Runtime instance (enum — one active field) |

## Related

- [[PermissionCategory]] -- permission category enum
- [[concepts/permissions|Permissions]] -- permission system overview
