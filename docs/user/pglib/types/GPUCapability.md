---
audience: automation-builder
type: specification
updated: 2026-04-18
status: complete
metadata_definition: "%definition.#:GPUCapability"
metadata_instance: "%#:GPUCapability:N"
---

# #GPUCapability Enum

<!-- @c:types -->

Capability enum for `#GPU` permission category. See [[concepts/permissions|Permissions]].

## Definition

```aljam3
{#} #GPUCapability
   [%] .description << "GPU resource limit capabilities"
   [%] .version << "1.0.0"
   [#] ##Enum
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "gpucapability"
   [.] .Limit
   [.] .Device
```

| Variant | Description |
|---------|-------------|
| `.Limit` | Set maximum GPU memory allocation |
| `.Device` | Bind job to specific GPU device |

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:GPUCapability` | Compile-time type template |
| Instance | `%#:GPUCapability:0` | Runtime instance (enum — one active field) |

## Related

- [[PermissionCategory]] -- permission category enum
- [[concepts/permissions|Permissions]] -- permission system overview
