---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =DT.Format.ISO

Formats a `#dt` as an ISO-8601 string.

## Definition

```polyglot
{N} =DT.Format.ISO
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtFormatIso"
   [%] .description << "Format DateTime as ISO-8601"
   [=] <source#dt
   [=] >text#string
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | DateTime to format |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>text` | `#string` | ISO-8601 formatted string |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Related

- [[pglib/pipelines/DT/INDEX|=DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
