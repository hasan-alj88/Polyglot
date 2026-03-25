---
audience: user
type: specification
updated: 2026-03-25
status: draft
---

# =Sys — System Pipelines

System information pipelines. No `[@]` import needed.

## Permissions

<!-- @permissions -->
System pipelines that read environment state require `[_]` permission declarations. See [[permissions]] for the permission system and [[errors#Built-in Error Namespaces]] for `!Permission.*` errors.

| Pipeline | Permission | Type |
|----------|-----------|------|
| `=Sys.OS` | `_System.env` | Inline |

```
=Sys
   .OS
      >os;OS
      [ ] Returns the current operating system as #OS enum.
```
