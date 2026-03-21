---
audience: user
type: specification
updated: 2026-03-20
status: draft
---

# =File — File Operations

Stdlib pipelines for file system operations. No `[@]` import needed.

```
=File
   .Text
      .Read
         <path;path
         >content;string
      .Write
         <path;path
         <content;string
         >written;bool (?)
      .Append
         <path;path
         <content;string
         >written;bool (?)
   .Copy (?)
      <source;path
      <destination;path
      >copied;bool (?)
   .Move (?)
      <source;path
      <destination;path
      >moved;bool (?)
   .Delete (?)
      <path;path
      >deleted;bool (?)
   .List (?)
      <folder;path
      >files;array.path (?)
```

## Errors

```
=File.Text.Read
   !File.NotFound
   !File.ReadError

=File.Text.Write
   !File.NotFound
   !File.WriteError

=File.Text.Append
   !File.NotFound
   !File.WriteError
```
