---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =File.Text.Write

Write text content to a file, replacing any existing content.

## Definition

```polyglot
{N} =File.Text.Write
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "FileTextWrite"
   [%] .description << "Write text to file"
   [=] <path#path
   [=] <content#string
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<path` | `#path` | Path to the file to write |
| `<content` | `#string` | Text content to write |

## Outputs

None. Success is signalled by `!NoError`.

## Errors

| Error | When |
|-------|------|
| `!File.NotFound` | Parent directory does not exist |
| `!File.WriteError` | Cannot write to path (permissions, disk full) |

## Permissions

Requires `File.Write` capability.

## Related

- [[pglib/pipelines/File/INDEX|=File.* File Pipelines]]
