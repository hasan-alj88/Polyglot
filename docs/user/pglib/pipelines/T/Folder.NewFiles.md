---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =T.Folder.NewFiles

Fires when new files appear in the specified folder. Folder path provided via inline call: `=T.Folder.NewFiles"/inbox/"`.

## Definition

```polyglot
{N} =T.Folder.NewFiles
   [%] .Kind << #NativeKind.Trigger
   [%] .Rust << "TFolderNewFiles"
   [%] .description << "Fires when new files appear in the specified folder."
   <Folder#path
   >NewFiles#array.path
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `Folder` | `#path` | Path to the folder to watch. Provided inline: `=T.Folder.NewFiles"/inbox/"`. |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `NewFiles` | `#array.path` | Array of paths for newly detected files. |

## Errors

None.

## Permissions

File.Read

## Related

- [[pglib/pipelines/T/INDEX|=T.* Trigger Pipelines]]
