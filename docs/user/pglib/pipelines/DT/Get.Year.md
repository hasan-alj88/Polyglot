---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =DT.Get.Year

Extracts the year component from a `#dt` value.

## Definition

```polyglot
{N} =DT.Get.Year
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtGetYear"
   [%] .description << "Extract year from DateTime"
   [=] <source#dt
   [=] >year#int
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | DateTime to extract from |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>year` | `#int` | Year component |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Related

- [[pglib/pipelines/DT/INDEX|=DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
