---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.W:File.Lock"
metadata_instance: "%W:File.Lock:N"
---

# -W.File.Lock

Acquires file lock on setup, releases on cleanup.

## Definition

```polyglot
{N} -W.File.Lock
   [%] .Kind << #NativeKind.Wrapper
   [%] .Rust << "WFileLock"
   [%] .description << "Acquires file lock on setup, releases on cleanup."
   (-) <lockPath;path   [ ] Path to the file to lock
   (-) >lock            [ ] Active file lock handle
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `$lockPath` | `#path` | Path to the file to lock |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `$lock` | lock handle | Active file lock handle |

## Errors

None.

## Permissions

File.Read

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.W:File.Lock` | Compile-time pipeline template |
| Instance | `%W:File.Lock:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/W/INDEX|-W.* Wrappers]]
- [[pglib/pipelines/W/File.TempDir|-W.File.TempDir]]
