---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
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

## Related

- [[pglib/pipelines/DT/INDEX|=DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
