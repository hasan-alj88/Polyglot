---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =Q.Kill.Hard

Direct command: immediate OS kill, no cleanup. Signal: `command.kill.hard`.

## Definition

```polyglot
{N} =Q.Kill.Hard
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "QKillHard"
   [%] .description << "Immediate OS kill, no cleanup."
```

## Inputs

None.

## Outputs

None.

## Errors

None.

## Conditional Variants

Terminate jobs based on time or resource conditions.

| Pipeline | IO | Purpose |
|----------|-----|---------|
| `=Q.Kill.Hard.Time.MoreThan` | `<duration#String` | Hard kill after execution time exceeds limit |
| `=Q.Kill.Hard.RAM.LessThan` | `<mb#Float` | Hard kill when RAM critically low |

## Permissions

None — pure computation (queue scheduling and resource management).

## Related

- [[pglib/pipelines/Q/Kill.Graceful|=Q.Kill.Graceful]]
- [[pglib/pipelines/Q/INDEX|=Q.* Queue Pipelines]]
