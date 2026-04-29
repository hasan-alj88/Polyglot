---
audience: automation-builder
type: specification
updated: 2026-04-18
status: complete
metadata_definition: "%definition.#:DurationCapability"
metadata_instance: "%#:DurationCapability:N"
---

# #DurationCapability Enum

<!-- @c:types -->

Capability enum for `#Duration` permission category. See [[concepts/permissions|Permissions]].

## Definition

```aljam3
{#} #DurationCapability
   [%] .description << "Execution time limit capabilities"
   [%] .version << "1.0.0"
   [#] ##Enum
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "durationcapability"
   [.] .Limit
```

| Variant | Description |
|---------|-------------|
| `.Limit` | Set maximum execution time for the job |

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:DurationCapability` | Compile-time type template |
| Instance | `%#:DurationCapability:0` | Runtime instance (enum — one active field) |

## Related

- [[PermissionCategory]] -- permission category enum
- [[concepts/permissions|Permissions]] -- permission system overview
