---
audience: automation-builder
type: specification
updated: 2026-04-10
status: complete
metadata_definition: "%!.Permission"
---

# `!Permission`

No `[@]` import needed — pglib built-in.

```aljam3
{!} !Permission
   [.] .File.Denied#Error
      (-) .MessageTemplate << "File permission denied: {path}"
      (-) .Info
         [:] :path#path
   [.] .Web.Denied#Error
      (-) .MessageTemplate << "Web permission denied: {url}"
      (-) .Info
         [:] :url#string
   [.] .Database.Denied#Error
      (-) .MessageTemplate << "Database permission denied: {connection}"
      (-) .Info
         [:] :connection#string
   [.] .System.Denied#Error
      (-) .MessageTemplate << "System permission denied: {operation}"
      (-) .Info
         [:] :operation#string
   [.] .Crypto.Denied#Error
      (-) .MessageTemplate << "Crypto permission denied: {operation}"
      (-) .Info
         [:] :operation#string
   [.] .IPC.Denied#Error
      (-) .MessageTemplate << "IPC permission denied: {target}"
      (-) .Info
         [:] :target#string
   [.] .Device.Denied#Error
      (-) .MessageTemplate << "Device permission denied: {device}"
      (-) .Info
         [:] :device#string
   [.] .Memory.Denied#Error
      (-) .MessageTemplate << "Memory permission denied: {operation}"
      (-) .Info
         [:] :operation#string
```

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Tree path | `%!.Permission` | Fixed error namespace (no instances) |

See [[metadata-tree/FULL-TREE|Full Metadata Tree]] for the authoritative tree.
