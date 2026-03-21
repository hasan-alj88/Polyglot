---
audience: user
type: specification
updated: 2026-03-20
status: draft
---

# Built-in Error Namespaces (!)

Errors use the `!` prefix and hierarchical dot names. They appear inside `[!]` blocks scoped under the `[r]` call that produces them.

No `[@]` import needed.

## Standard Error Namespaces

```
!File
   .NotFound
   .ReadError
   .WriteError (?)

!No
   .Input
   .Output (?)

!Timeout (?)
   :Connection (?)
      [ ] Uses flexible (:) field for specific timeout targets.

!Validation (?)
   .Error (?)
```

## Pipeline Error Associations

Each stdlib pipeline exposes the errors it can raise:

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
