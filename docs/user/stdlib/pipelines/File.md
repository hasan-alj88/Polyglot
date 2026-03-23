---
audience: user
type: specification
updated: 2026-03-23
status: draft
---

# =File — File Operations

Stdlib pipelines for file system operations. No `[@]` import needed.

Success is signalled by `!NoError`. Side-effect-only pipelines (Write, Append, Copy, Move, Delete) have no output — `!NoError` confirms completion.

```
=File
   .Text
      .Read
         <path;path
         >content;string
      .Write
         <path;path
         <content;string
      .Append
         <path;path
         <content;string
   .Copy
      <source;path
      <destination;path
   .Move
      <source;path
      <destination;path
   .Delete
      <path;path
   .Access
      <path;path
      >access;FileAccess
   .List
      <folder;path
      >files;array.path
```

## Errors

```
=File.Text.Read
   !NoError
   !File.NotFound
   !File.ReadError

=File.Text.Write
   !NoError
   !File.NotFound
   !File.WriteError

=File.Text.Append
   !NoError
   !File.NotFound
   !File.WriteError

=File.Copy
   !NoError
   !File.NotFound
   !File.CopyError

=File.Move
   !NoError
   !File.NotFound
   !File.MoveError

=File.Delete
   !NoError
   !File.NotFound
   !File.DeleteError

=File.List
   !NoError
   !Folder.NotFound
   !Folder.ReadError
```

## Implementation Status

| Pipeline | Status |
|---|---|
| `=File.Text.Read` | Deferred |
| `=File.Text.Write` | Deferred |
| `=File.Text.Append` | Deferred |
| `=File.Copy` | Deferred |
| `=File.Move` | Deferred |
| `=File.Delete` | Deferred |
| `=File.Access` | Deferred |
| `=File.List` | Deferred |
