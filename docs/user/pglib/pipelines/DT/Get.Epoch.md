---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
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

## Related

- [[pglib/pipelines/DT/INDEX|=DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
