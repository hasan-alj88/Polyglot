---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.=:DT.From.Parts"
metadata_instance: "%=:DT.From.Parts:N"
---

# =DT.From.Parts

Constructs a `#dt` from explicit date-time components. Hour, minute, second, nanosecond default to `0`; zone defaults to `"UTC"`.

## Definition

```polyglot
{N} =DT.From.Parts
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtFromParts"
   [%] .description << "Explicit components to DateTime"
   [=] <year#int
   [=] <month#int
   [=] <day#int
   [=] <hour#int <~ 0
   [=] <minute#int <~ 0
   [=] <second#int <~ 0
   [=] <nano#int <~ 0
   [=] <zone#string <~ "UTC"
   [=] >dt#dt
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<year` | `#int` | --- | Year component |
| `<month` | `#int` | --- | Month component |
| `<day` | `#int` | --- | Day component |
| `<hour` | `#int` | `0` | Hour component |
| `<minute` | `#int` | `0` | Minute component |
| `<second` | `#int` | `0` | Second component |
| `<nano` | `#int` | `0` | Nanosecond component |
| `<zone` | `#string` | `"UTC"` | Timezone IANA identifier |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>dt` | `#dt` | Constructed DateTime value |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.=:DT.From.Parts` | Compile-time pipeline template |
| Instance | `%=:DT.From.Parts:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/DT/INDEX|=DT.* DateTime Pipelines]]
- [[pglib/types/datetime|DateTime types]]
