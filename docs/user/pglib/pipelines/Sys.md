---
audience: pg-coder
type: specification
updated: 2026-03-25
status: draft
metadata_definition: "%definition.-:Sys"
metadata_instance: "%-:Sys:N"
---

# -Sys — System Pipelines

System information pipelines. No `[@]` import needed.

## Permissions

<!-- @permissions -->
System pipelines that read environment state require a `{_}` permission object granting the listed capabilities. See [[permissions]] for the permission system and [[errors#Built-in Error Namespaces]] for `!Permission.*` errors.

| Pipeline | Required Capability | Category |
|----------|-------------------|----------|
| `-Sys.OS` | System.Env | System |

```polyglot
-Sys
   .OS
      >os#OS
      [ ] Returns the current operating system as #OS enum.
```

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:Sys` | Compile-time pipeline template |
| Instance | `%-:Sys:N` | Runtime pipeline instance (N = instance number) |

