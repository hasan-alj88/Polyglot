---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.=:DT.IsBefore"
metadata_instance: "%=:DT.IsBefore:N"
---

# =DT.IsBefore

Returns `#bool` -- true when `a` is earlier than `b`.

## Definition

```polyglot
{N} =DT.IsBefore
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtIsBefore"
   [%] .description << "Check if DateTime is before another"
   [=] <a#dt
   [=] <b#dt
   [=] >result#bool
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<a` | `#dt` | --- | DateTime to check |
| `<b` | `#dt` | --- | DateTime to compare against |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>result` | `#bool` | True if `a` is earlier than `b` |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.=:DT.IsBefore` | Compile-time pipeline template |
| Instance | `%=:DT.IsBefore:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/DT/INDEX|=DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
