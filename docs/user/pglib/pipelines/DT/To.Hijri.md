---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.=:DT.To.Hijri"
metadata_instance: "%=:DT.To.Hijri:N"
---

# =DT.To.Hijri

Projects a `#dt` value into a Hijri date structure. Projections are not cached -- store results in a `$variable` if you need the value again.

The `<authority` input selects the Hijri calendar authority. Defaults to `#HijriAuthority.UmmAlQura`.

## Definition

```polyglot
{N} =DT.To.Hijri
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtToHijri"
   [%] .description << "DateTime to Hijri date"
   [=] <source#dt
   [=] <authority#HijriAuthority <~ #HijriAuthority.UmmAlQura
   [=] >hijri#HijriDate
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | DateTime to project |
| `<authority` | `#HijriAuthority` | `#HijriAuthority.UmmAlQura` | Hijri calendar authority |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>hijri` | `#HijriDate` | Hijri date projection |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.=:DT.To.Hijri` | Compile-time pipeline template |
| Instance | `%=:DT.To.Hijri:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/DT/INDEX|=DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
