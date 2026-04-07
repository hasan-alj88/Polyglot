---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =DT.To.Japanese

Projects a `#dt` value into a Japanese date structure. Projections are not cached -- store results in a `$variable` if you need the value again.

## Definition

```polyglot
{N} =DT.To.Japanese
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtToJapanese"
   [%] .description << "DateTime to Japanese date"
   [=] <source#dt
   [=] >japanese#JapaneseDate
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | DateTime to project |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>japanese` | `#JapaneseDate` | Japanese date projection |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Related

- [[pglib/pipelines/DT/INDEX|=DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
