---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.#:path"
metadata_instance: "%#:path:N"
---

# #path Struct

<!-- @types -->

Cross-platform file system path with OS-specific subfields. At runtime, the Polyglot runtime resolves `$pathVar` to the correct subfield based on the current OS.

---

## Definition

```polyglot
{#} #path
   [%] .description << "Cross-platform file system path"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [.] .Unix#string
   [.] .Windows#string
```

---

## Fields

| Field | Type | Purpose |
|-------|------|---------|
| `.Unix` | `#string` | Unix/macOS path |
| `.Windows` | `#string` | Windows path |

---

## Usage

Assign both platform subfields explicitly:

```polyglot
[-] $AppDir#path
   [.] .Unix << "/tmp/MyApp"
   [.] .Windows << "C:\MyApp"
```

Use the `-Path"..."` inline notation for common paths:

```polyglot
[-] $LogDir#path << -Path"/tmp/MyApp/logs"
```

See [[syntax/types/strings#Path Type]] for full details including `-Path"..."`, path roots, and shorthands.

---

## Error Codes

| Code | Severity | Description |
|------|----------|-------------|
| PGE04001 | Error | Plain string assigned to `#path` (type mismatch) |
| PGE04007 | Error | Invalid path string |
| PGE04008 | Error | Missing path platform subfield |
| PGW04001 | Warning | Single-platform path |

---

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:path` | Compile-time type template |
| Instance | `%#:path:N` | Runtime instance (N = instance number) |

---

## Related

- [[structs]] — other pglib struct types
- [[string]] — string type
- [[syntax/types/INDEX|types]] — full type system specification
