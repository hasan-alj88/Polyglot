---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:DT.Zone.Convert"
metadata_instance: "%-:DT.Zone.Convert:N"
---

# -DT.Zone.Convert

Converts a `#dt` to a different timezone. The underlying instant stays the same; the wall-clock reading changes.

## Definition

```polyglot
{N} -DT.Zone.Convert
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtZoneConvert"
   [%] .description << "Convert DateTime to different timezone"
   (-) <source#dt
   (-) <iana#string
   (-) >result#dt
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | DateTime to convert |
| `<iana` | `#string` | --- | Target IANA timezone identifier |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>result` | `#dt` | DateTime in the target timezone |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:DT.Zone.Convert` | Compile-time pipeline template |
| Instance | `%-:DT.Zone.Convert:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/DT/INDEX|-DT.* DateTime Pipelines]]
- [[pglib/pipelines/DT/Zone.Set|-DT.Zone.Set]] -- replaces timezone label (instant changes)
- [[pglib/types/datetime|DateTime types]]
