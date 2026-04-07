---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.Q:Resume"
metadata_instance: "%Q:Resume:N"
---

# =Q.Resume

Direct command: move from Suspended Set to Resume Queue. Signal: `command.resume`.

## Definition

```polyglot
{N} =Q.Resume
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "QResume"
   [%] .description << "Move from Suspended Set to Resume Queue."
```

## Inputs

None.

## Outputs

None.

## Errors

None.

## Conditional Variants

Resume when a resource condition recovers. Used as nested `[Q]` lines in `{Q}` definitions or pipeline `[Q]` sections.

| Pipeline | IO | Purpose |
|----------|-----|---------|
| `=Q.Resume.RAM.MoreThan` | `<mb#Float` | Resume when RAM recovers above threshold |
| `=Q.Resume.CPU.LessThan` | `<percent#Float` | Resume when CPU drops below threshold |
| `=Q.Resume.Disk.MoreThan` | `<mb#Float` | Resume when disk space recovers |
| `=Q.Resume.GPU.Free` | (none) | Resume when GPU becomes available |

## Permissions

None — pure computation (queue scheduling and resource management).

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.Q:Resume` | Compile-time pipeline template |
| Instance | `%Q:Resume:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/Q/Pause.Soft|=Q.Pause.Soft]]
- [[pglib/pipelines/Q/Pause.Hard|=Q.Pause.Hard]]
- [[pglib/pipelines/Q/INDEX|=Q.* Queue Pipelines]]
