---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:DT.From.ISO"
metadata_instance: "%-:DT.From.ISO:N"
---

# -DT.From.ISO

Parses an ISO-8601 string into a `#dt` value. Called implicitly by `-DT"..."` inline notation.

## Definition

```polyglot
{N} -DT.From.ISO
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtFromIso"
   [%] .description << "ISO-8601 string to DateTime"
   (-) <iso#string
   (-) >dt#dt
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<iso` | `#string` | --- | ISO-8601 formatted date-time string |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>dt` | `#dt` | Parsed DateTime value |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:DT.From.ISO` | Compile-time pipeline template |
| Instance | `%-:DT.From.ISO:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/DT/INDEX|-DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
