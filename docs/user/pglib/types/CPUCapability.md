---
audience: automation-builder
type: specification
updated: 2026-04-18
status: complete
metadata_definition: "%definition.#:CPUCapability"
metadata_instance: "%#:CPUCapability:N"
---

# #CPUCapability Enum

<!-- @c:types -->

Capability enum for `#CPU` permission category. See [[concepts/permissions|Permissions]].

## Definition

```polyglot
{#} #CPUCapability
   [%] .description << "CPU resource limit capabilities"
   [%] .version << "1.0.0"
   [#] ##Enum
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "cpucapability"
   [.] .Limit
   [.] .Weight
```

| Variant | Description |
|---------|-------------|
| `.Limit` | Set maximum CPU allocation |
| `.Weight` | Set CPU scheduling weight/priority |

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:CPUCapability` | Compile-time type template |
| Instance | `%#:CPUCapability:0` | Runtime instance (enum — one active field) |

## Related

- [[PermissionCategory]] -- permission category enum
- [[concepts/permissions|Permissions]] -- permission system overview
