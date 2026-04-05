---
audience: pg-coder
type: specification
updated: 2026-03-25
status: draft
---

# =Sys — System Pipelines

System information pipelines. No `[@]` import needed.

## Permissions

<!-- @permissions -->
System pipelines that read environment state require a `{_}` permission object granting the listed capabilities. See [[permissions]] for the permission system and [[errors#Built-in Error Namespaces]] for `!Permission.*` errors.

| Pipeline | Required Capability | Category |
|----------|-------------------|----------|
| `=Sys.OS` | System.Env | System |

```
=Sys
   .OS
      >os#OS
      [ ] Returns the current operating system as #OS enum.
```
