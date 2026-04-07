---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =DT.To.Hindu

Projects a `#dt` value into a Hindu date structure. Projections are not cached -- store results in a `$variable` if you need the value again.

The `<era` input selects the Hindu era. Defaults to `#HinduEra.VikramSamvat`.

## Definition

```polyglot
{N} =DT.To.Hindu
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtToHindu"
   [%] .description << "DateTime to Hindu date"
   [=] <source#dt
   [=] <era#HinduEra <~ #HinduEra.VikramSamvat
   [=] >hindu#HinduDate
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | DateTime to project |
| `<era` | `#HinduEra` | `#HinduEra.VikramSamvat` | Hindu era selection |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>hindu` | `#HinduDate` | Hindu date projection |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Related

- [[pglib/pipelines/DT/INDEX|=DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
