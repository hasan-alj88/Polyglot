---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.T:Daily"
metadata_instance: "%T:Daily:N"
---

# -T.Daily

Fires once per day at the specified time. Time string provided via inline call: `-T.Daily"3AM"`.

## Definition

```polyglot
{N} -T.Daily
   [%] .Kind << #NativeKind.Trigger
   [%] .Rust << "TDaily"
   [%] .description << "Fires once per day at the specified time."
   <InlineStringLiteral#string <~ ""
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `InlineStringLiteral` | `#string` | Time of day to fire (e.g. `"3AM"`). Provided inline: `-T.Daily"3AM"`. Defaults to `""`. |

## Outputs

| Name | Type | Description |
|------|------|-------------|

## Errors

None.

## Permissions

None.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.T:Daily` | Compile-time pipeline template |
| Instance | `%T:Daily:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/T/INDEX|-T.* Trigger Pipelines]]
