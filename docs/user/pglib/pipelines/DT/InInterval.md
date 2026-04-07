---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =DT.InInterval

Returns `#bool` -- true when `source` falls within the given `#Interval`.

## Definition

```polyglot
{N} =DT.InInterval
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtInInterval"
   [%] .description << "Check if DateTime is within interval"
   [=] <source#dt
   [=] <interval#Interval
   [=] >result#bool
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | DateTime to check |
| `<interval` | `#Interval` | --- | Time interval to test against |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>result` | `#bool` | True if `source` falls within the interval |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Related

- [[pglib/pipelines/DT/INDEX|=DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
