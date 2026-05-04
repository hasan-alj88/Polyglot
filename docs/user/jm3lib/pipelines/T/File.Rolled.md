---
audience: automation-builder
type: specification
updated: 2026-04-17
status: draft
metadata_definition: "%definition.T:File.Rolled"
metadata_instance: "%T:File.Rolled:N"
---

# -T.File.Rolled

Fires when a file rotates, delivering the **completed** (rolled-over) file — not the newly active one. Designed for log rotation workflows where the trigger target is the previous file that is now closed and ready for processing.

## Definition

```aljam3
{N} -T.File.Rolled
   [%] .Kind << #NativeKind.Trigger
   [%] .Rust << "TriggerFileRolled"
   [%] .description << "Fires when a file rotates, delivering the completed file"
   <Dir#path
   <Pattern#string
   <Strategy#RotationKind <~ #RotationKind.Auto
   >RolledFile#path
   >NewFile#path
   >RolledSize#int
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<Dir` | `#path` | — | Directory to watch for rotation events. |
| `<Pattern` | `#string` | — | Filename glob pattern to match (e.g. `"app-*.log"`). |
| `<Strategy` | `#RotationKind` | `#RotationKind.Auto` | Detection strategy. See [[jm3lib/types/RotationKind\|#RotationKind]]. |

### Inline Syntax

Dir and Pattern are provided inline:

```aljam3
[T] -T.File.Rolled"/var/log/app/" "app-*.log"
   (-) >RolledFile >> <logFile
   (-) >RolledSize >> <fileSize
```

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>RolledFile` | `#path` | Full path to the completed (rolled-over) file. |
| `>NewFile` | `#path` | Full path to the newly active file. |
| `>RolledSize` | `#int` | Final size in bytes of the rolled file. |

## Detection Strategy

The trigger abstracts OS-level file watching. There is no single "file rolled" OS event — the runtime infers rotation from lower-level signals.

### Algorithm

```
State:
    known_files = {files matching Pattern in Dir}
    active_file = newest by naming convention

On directory change event:
    current_files = scan Dir for Pattern matches
    new_files = current_files - known_files

    if Strategy = CopyTruncate:
        if active_file.size < previous_size:
            rolled_copy = newest file in current_files != active_file
            EMIT(RolledFile=rolled_copy, NewFile=active_file)

    else if new_files is not empty:
        rolled_file = active_file
        active_file = newest(new_files)
        wait_for CLOSE_WRITE on rolled_file (timeout: 5s)
        EMIT(RolledFile=rolled_file, NewFile=active_file)
```

### OS Primitives

| OS | API | Events Used |
|----|-----|-------------|
| Linux | `inotify` | `IN_CREATE`, `IN_CLOSE_WRITE`, `IN_MOVED_FROM`/`IN_MOVED_TO`, `IN_MODIFY` |
| macOS | `kqueue` | `EVFILT_VNODE`: `NOTE_WRITE`, `NOTE_RENAME` |
| Windows | `ReadDirectoryChangesW` | `FILE_NOTIFY_CHANGE_FILE_NAME`, `FILE_NOTIFY_CHANGE_LAST_WRITE` |
| NFS/CIFS | Polling fallback | Interval-based directory scan (inotify unavailable on network mounts) |

## Edge Cases

| Scenario | Detection | Handling |
|----------|-----------|----------|
| **CopyTruncate** (logrotate) | `IN_MODIFY` where file size decreases | Emit the copy as `RolledFile`; `NewFile` = same active file (truncated) |
| **Compressed rotation** (`app.log.1.gz`) | `IN_MOVED_FROM` on original, match by inode | Track inode across rename; emit pre-compression path |
| **Rapid rotation** | Multiple rotations before processing | Queue rolled files; emit one trigger per file |
| **Atomic rename** (`mv tmp newlog`) | `IN_MOVED_TO` instead of `IN_CREATE` | Treat `MOVED_TO` as equivalent to `CREATE` for pattern matching |
| **NFS/network mounts** | No inotify support | Fall back to polling with configurable interval |

## Errors

| Error | When |
|-------|------|
| `!File.NotFound` | `Dir` does not exist at trigger registration time |
| `!File.PermissionDenied` | Insufficient permissions to watch `Dir` |
| `!File.WatchError` | OS-level watch subsystem failure (e.g. inotify descriptor limit reached) |

## Permissions

File.Read

## Queue Composability

- `-Q.Debounce` — batch rapid rotations into a single pipeline run
- `-Q.Allow.One` — prevent concurrent processing of rolled files

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.T:File.Rolled` | Compile-time pipeline template |
| Instance | `%T:File.Rolled:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[jm3lib/pipelines/T/INDEX|-T.* Trigger Pipelines]]
- [[jm3lib/pipelines/T/Folder.NewFiles|-T.Folder.NewFiles]] — related trigger (fires on new files, not rolled files)
- [[jm3lib/types/RotationKind|#RotationKind]] — detection strategy enum
