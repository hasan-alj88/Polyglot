---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =DT.Get.Weekday

Returns a `#Weekday` enum value from a `#dt`.

## Definition

```polyglot
{N} =DT.Get.Weekday
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtGetWeekday"
   [%] .description << "Extract weekday from DateTime"
   [=] <source#dt
   [=] >weekday#Weekday
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | DateTime to extract from |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>weekday` | `#Weekday` | Weekday enum value |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Related

- [[pglib/pipelines/DT/INDEX|=DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
