---
audience: automation-builder
type: specification
updated: 2026-04-15
status: deprecated
metadata_definition: "%definition.Q:Kill.Hard"
metadata_instance: "%Q:Kill.Hard:N"
---

# -Q.Kill.Hard

<!-- @d:pglib/pipelines/Q/Job.Kill.Now -->
**Deprecated.** Replaced by [[pglib/pipelines/Q/Job.Kill.Now|d:-Q.Job.Kill.Now]]. Conditions now use `[?]`/`[&]`/`[+]` conditional blocks inside `{Q} #JobRules` definitions instead of pipeline name suffixes.

---

*Original content preserved below for historical reference.*

Direct command: immediate OS kill, no cleanup. Signal: `command.kill.hard`.

## Definition

```aljam3
{N} -Q.Kill.Hard
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
| `-Q.Kill.Hard.Time.MoreThan` | `<duration#String` | Hard kill after execution time exceeds limit |
| `-Q.Kill.Hard.RAM.LessThan` | `<mb#Float` | Hard kill when RAM critically low |

## Permissions

None — pure computation (queue scheduling and resource management).

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.Q:Kill.Hard` | Compile-time pipeline template |
| Instance | `%Q:Kill.Hard:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/Q/Kill.Graceful|-Q.Kill.Graceful]]
- [[pglib/pipelines/Q/INDEX|-Q.* Queue Pipelines]]
