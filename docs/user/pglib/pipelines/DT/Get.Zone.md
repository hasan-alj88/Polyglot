---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:DT.Get.Zone"
metadata_instance: "%-:DT.Get.Zone:N"
---

# -DT.Get.Zone

Returns the `#Zone` attached to a `#dt` value.

## Definition

```polyglot
{N} -DT.Get.Zone
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtGetZone"
   [%] .description << "Extract timezone from DateTime"
   (-) <source#dt
   (-) >zone#Zone
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | DateTime to extract from |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>zone` | `#Zone` | Timezone attached to the DateTime |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:DT.Get.Zone` | Compile-time pipeline template |
| Instance | `%-:DT.Get.Zone:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/DT/INDEX|-DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
