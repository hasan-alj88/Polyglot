---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.Q:Kill.Graceful"
metadata_instance: "%Q:Kill.Graceful:N"
---

# =Q.Kill.Graceful

Direct command: finish work + `[/]` cleanup, then terminate. Signal: `command.kill.graceful`.

## Definition

```polyglot
{N} =Q.Kill.Graceful
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "QKillGraceful"
   [%] .description << "Finish work + cleanup, terminate."
```

## Inputs

None.

## Outputs

None.

## Errors

None.

## Conditional Variants

Terminate jobs based on time, state, or resource conditions.

| Pipeline | IO | Purpose |
|----------|-----|---------|
| `=Q.Kill.Graceful.Time.MoreThan` | `<duration#String` | Graceful kill after execution time exceeds limit |
| `=Q.Kill.Graceful.RAM.LessThan` | `<mb#Float` | Graceful kill when RAM critically low |
| `=Q.Kill.Graceful.Pipeline.Completed` | `<name#String` | Graceful kill when named pipeline completes |
| `=Q.Kill.Graceful.Pipeline.Failed` | `<name#String` | Graceful kill when named pipeline fails |

## Permissions

None — pure computation (queue scheduling and resource management).

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.Q:Kill.Graceful` | Compile-time pipeline template |
| Instance | `%Q:Kill.Graceful:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/Q/Kill.Hard|=Q.Kill.Hard]]
- [[pglib/pipelines/Q/INDEX|=Q.* Queue Pipelines]]
