---
audience: automation-builder
type: specification
updated: 2026-04-18
status: complete
metadata_definition: "%definition.#:RAMCapability"
metadata_instance: "%#:RAMCapability:N"
---

# #RAMCapability Enum

<!-- @c:types -->

Capability enum for `#RAM` permission category. See [[concepts/permissions|Permissions]].

## Definition

```aljam3
{#} #RAMCapability
   [%] .description << "RAM resource limit capabilities"
   [%] .version << "1.0.0"
   [#] ##Enum
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "ramcapability"
   [.] .Limit
```

| Variant | Description |
|---------|-------------|
| `.Limit` | Set maximum memory allocation for the job |

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:RAMCapability` | Compile-time type template |
| Instance | `%#:RAMCapability:0` | Runtime instance (enum — one active field) |

## Related

- [[PermissionCategory]] -- permission category enum
- [[concepts/permissions|Permissions]] -- permission system overview
