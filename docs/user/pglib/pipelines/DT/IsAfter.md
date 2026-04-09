---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:DT.IsAfter"
metadata_instance: "%-:DT.IsAfter:N"
---

# -DT.IsAfter

Returns `#bool` -- true when `a` is later than `b`.

## Definition

```polyglot
{N} -DT.IsAfter
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtIsAfter"
   [%] .description << "Check if DateTime is after another"
   (-) <a#dt
   (-) <b#dt
   (-) >result#bool
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<a` | `#dt` | --- | DateTime to check |
| `<b` | `#dt` | --- | DateTime to compare against |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>result` | `#bool` | True if `a` is later than `b` |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:DT.IsAfter` | Compile-time pipeline template |
| Instance | `%-:DT.IsAfter:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/DT/INDEX|-DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
