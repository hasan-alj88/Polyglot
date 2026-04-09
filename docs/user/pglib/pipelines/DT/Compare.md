---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:DT.Compare"
metadata_instance: "%-:DT.Compare:N"
---

# -DT.Compare

Compares two `#dt` values. Returns `-1`, `0`, or `1` as an `#int`.

## Definition

```polyglot
{N} -DT.Compare
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtCompare"
   [%] .description << "Compare two DateTimes"
   (-) <a#dt
   (-) <b#dt
   (-) >result#int
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<a` | `#dt` | --- | First DateTime |
| `<b` | `#dt` | --- | Second DateTime |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>result` | `#int` | `-1` if a < b, `0` if equal, `1` if a > b |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:DT.Compare` | Compile-time pipeline template |
| Instance | `%-:DT.Compare:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/DT/INDEX|-DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
