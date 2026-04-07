---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =DT.Format

Formats a `#dt` using a pattern string (e.g. `"YYYY-MM-DD HH:mm:ss"`).

## Definition

```polyglot
{N} =DT.Format
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtFormat"
   [%] .description << "Format DateTime with pattern"
   [=] <source#dt
   [=] <pattern#string
   [=] >text#string
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | DateTime to format |
| `<pattern` | `#string` | --- | Format pattern string |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>text` | `#string` | Formatted date-time string |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Related

- [[pglib/pipelines/DT/INDEX|=DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
