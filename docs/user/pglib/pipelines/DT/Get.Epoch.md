---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.=:DT.Get.Epoch"
metadata_instance: "%=:DT.Get.Epoch:N"
---

# =DT.Get.Epoch

Returns epoch seconds as `#int` from a `#dt` value.

## Definition

```polyglot
{N} =DT.Get.Epoch
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtGetEpoch"
   [%] .description << "Extract epoch seconds from DateTime"
   [=] <source#dt
   [=] >epoch#int
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | DateTime to extract from |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>epoch` | `#int` | Epoch seconds |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.=:DT.Get.Epoch` | Compile-time pipeline template |
| Instance | `%=:DT.Get.Epoch:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/DT/INDEX|=DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
