---
audience: user
type: specification
updated: 2026-03-25
status: complete
---

# =File — File Operations

<!-- @errors -->
Stdlib pipelines for file system operations. No `[@]` import needed. See [[errors#Pipeline Error Associations]] for error contracts.

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

## Permissions

<!-- @permissions -->
All `=File.*` pipelines perform filesystem IO and require `[_]` permission declarations. See [[permissions]] for the permission system and [[errors#Built-in Error Namespaces]] for `!Permission.*` errors.

| Pipeline | Permission | Type |
|----------|-----------|------|
| `=File.Text.Read` | `_File.read` | Inline |
| `=File.Text.Write` | `_File.write` | Inline |
| `=File.Text.Append` | `_File.write` | Inline |
| `=File.Copy` | `_File.read` + `_File.write` | Inline |
| `=File.Move` | `_File.read` + `_File.write` | Inline |
| `=File.Delete` | `_File.delete` | Inline |
| `=File.Access` | `_File.read` | Inline |
| `=File.List` | `_File.read` | Inline |

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
