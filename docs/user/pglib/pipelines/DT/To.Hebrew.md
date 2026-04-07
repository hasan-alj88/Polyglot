---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =DT.To.Hebrew

Projects a `#dt` value into a Hebrew date structure. Projections are not cached -- store results in a `$variable` if you need the value again.

## Definition

```polyglot
{N} =DT.To.Hebrew
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtToHebrew"
   [%] .description << "DateTime to Hebrew date"
   [=] <source#dt
   [=] >hebrew#HebrewDate
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | DateTime to project |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>hebrew` | `#HebrewDate` | Hebrew date projection |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Related

- [[pglib/pipelines/DT/INDEX|=DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
