---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =File.Copy

Copy a file from one path to another.

## Definition

```polyglot
{N} =File.Copy
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "FileCopy"
   [%] .description << "Copy file"
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
| `!File.CopyError` | Cannot complete copy (permissions, disk full) |

## Permissions

Requires `File.Read` + `File.Write` capability.

## Related

- [[pglib/pipelines/File/INDEX|=File.* File Pipelines]]
- [[pglib/pipelines/File/Move|=File.Move]]
