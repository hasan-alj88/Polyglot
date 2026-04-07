---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =DT.From.ISO

Parses an ISO-8601 string into a `#dt` value. Called implicitly by `=DT"..."` inline notation.

## Definition

```polyglot
{N} =DT.From.ISO
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtFromIso"
   [%] .description << "ISO-8601 string to DateTime"
   [=] <iso#string
   [=] >dt#dt
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

## Related

- [[pglib/pipelines/DT/INDEX|=DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
