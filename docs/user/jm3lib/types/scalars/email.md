---
audience: automation-builder
type: specification
updated: 2026-04-16
status: complete
metadata_definition: "%definition.#:Email"
metadata_instance: "%#:String:email"
---

# #Email

<!-- @c:types -->

```aljam3
{#} #Email
   [%] %alias << "email"
   [#] ##String
      (#) <regex << "^[a-zA-Z0-9+_.-]+@[a-zA-Z0-9.-]+$"
```

| Alias | `.regex` Pattern | Example Values |
|-------|---------------|----------------|
| `email` | `^[a-zA-Z0-9+_.-]+@[a-zA-Z0-9.-]+$` | `user@example.com`, `admin+tag@sub.domain.org` |

| Level | Name | Purpose |
|-------|------|---------|
| Definition | `%definition.#:Email` | Schema descriptor |
| Instance | `%#:String:email` | Tree path under `#String` |

See [[metadata-tree/FULL-TREE|Full Metadata Tree]] for the authoritative tree.

`#Email` validates email addresses by regex. Common in automation scenarios: notifications, user provisioning, Git author data. Used by `#Git.Author.email` in the Git event type tree.
