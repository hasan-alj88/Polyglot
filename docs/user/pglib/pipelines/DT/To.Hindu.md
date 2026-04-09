---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:DT.To.Hindu"
metadata_instance: "%-:DT.To.Hindu:N"
---

# -DT.To.Hindu

Projects a `#dt` value into a Hindu date structure. Projections are not cached -- store results in a `$variable` if you need the value again.

The `<era` input selects the Hindu era. Defaults to `#HinduEra.VikramSamvat`.

## Definition

```polyglot
{N} -DT.To.Hindu
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtToHindu"
   [%] .description << "DateTime to Hindu date"
   (-) <source#dt
   (-) <era#HinduEra <~ #HinduEra.VikramSamvat
   (-) >hindu#HinduDate
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

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:DT.To.Hindu` | Compile-time pipeline template |
| Instance | `%-:DT.To.Hindu:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/DT/INDEX|-DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
