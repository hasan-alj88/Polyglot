---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =File.Move

Move or rename a file.

## Definition

```polyglot
{N} =File.Move
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "FileMove"
   [%] .description << "Move/rename file"
   [=] <source#path
   [=] <destination#path
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<source` | `#path` | Path to the source file |
| `<destination` | `#path` | Path to the destination file |

## Outputs

None. Success is signalled by `!NoError`.

## Errors

| Error | When |
|-------|------|
| `!File.NotFound` | Source file does not exist |
| `!File.MoveError` | Cannot complete move (permissions, cross-device, disk full) |

## Permissions

Requires `File.Read` + `File.Write` capability.

## Related

- [[pglib/pipelines/File/INDEX|=File.* File Pipelines]]
- [[pglib/pipelines/File/Copy|=File.Copy]]
- [[pglib/pipelines/File/Delete|=File.Delete]]
