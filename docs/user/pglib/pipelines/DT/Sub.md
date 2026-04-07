---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.=:DT.Sub"
metadata_instance: "%=:DT.Sub:N"
---

# =DT.Sub

Subtracts two `#dt` values and returns the `#Duration` between them.

## Definition

```polyglot
{N} =DT.Sub
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtSub"
   [%] .description << "Subtract two DateTimes to get Duration"
   [=] <a#dt
   [=] <b#dt
   [=] >result#Duration
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<a` | `#dt` | --- | First DateTime |
| `<b` | `#dt` | --- | Second DateTime |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>result` | `#Duration` | Duration between the two DateTimes |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.=:DT.Sub` | Compile-time pipeline template |
| Instance | `%=:DT.Sub:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/DT/INDEX|=DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
