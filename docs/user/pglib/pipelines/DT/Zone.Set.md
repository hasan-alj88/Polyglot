---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =DT.Zone.Set

Replaces the timezone label on a `#dt` without converting the instant. The wall-clock reading stays the same; the underlying instant changes.

## Definition

```polyglot
{N} =DT.Zone.Set
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtZoneSet"
   [%] .description << "Set timezone on DateTime"
   [=] <source#dt
   [=] <iana#string
   [=] >result#dt
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<source` | `#dt` | --- | DateTime to modify |
| `<iana` | `#string` | --- | IANA timezone identifier |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>result` | `#dt` | DateTime with replaced timezone label |

## Errors

None. Pure computation pipeline.

## Permissions

None required.

## Related

- [[pglib/pipelines/DT/INDEX|=DT.* DateTime Pipelines]]
- [[pglib/pipelines/DT/Zone.Convert|=DT.Zone.Convert]] -- converts instant to different timezone (wall-clock changes)
- [[pglib/types/datetime|DateTime types]]
