---
audience: automation-builder
type: specification
updated: 2026-04-22
status: complete
metadata_definition: "%definition.#:MIME"
metadata_instance: "%#:MIME:N"
---

# #MIME — Media Type

<!-- @c:types -->
<!-- @c:pglib/types/string -->

`#MIME` is a pglib type representing an Internet media type (RFC 6838). It stores the type and subtype components separately, enabling type-safe MIME handling without string parsing at runtime.

See [[syntax/constructors]] for the `$MIME` constructor.

## Definition

```polyglot
{#} #MIME
   [%] .description << "Internet media type (RFC 6838)"
   [#] ##Scalar
   [#] %##Alias << "mime"
   [.] .type#string
   [.] .subtype#string
```

## Fields

| Field | Type | Default | Purpose |
|-------|------|---------|---------|
| `.type` | `#string` | (none) | Top-level type (e.g., `"application"`, `"text"`, `"image"`) |
| `.subtype` | `#string` | (none) | Subtype (e.g., `"json"`, `"plain"`, `"png"`) |

## Schema Properties

- `[#] ##Scalar` -- sets `%##Depth.Max << 1`, marking `#MIME` as a scalar type
- `%##Alias << "mime"` -- lets users write `#mime` (lowercase) as shorthand

## Well-Known Values

| MIME String | `.type` | `.subtype` | Common Use |
|---|---|---|---|
| `application/json` | `application` | `json` | API payloads |
| `application/xml` | `application` | `xml` | XML documents |
| `application/octet-stream` | `application` | `octet-stream` | Binary data |
| `text/plain` | `text` | `plain` | Plain text |
| `text/html` | `text` | `html` | HTML pages |
| `text/csv` | `text` | `csv` | CSV data |
| `image/png` | `image` | `png` | PNG images |
| `image/jpeg` | `image` | `jpeg` | JPEG images |
| `multipart/form-data` | `multipart` | `form-data` | File uploads |

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:MIME` | Compile-time type template |
| Instance | `%#:MIME:N` | Runtime instance (N = instance number) |

## Related

- [[pglib/constructors/MIME\|$MIME constructor]] -- compile-time MIME construction
- [[pglib/pipelines/MIME.Parse\|-MIME.Parse]] -- runtime MIME string parsing
- [[scalars]] -- scalar subtypes overview
- [[string]] -- `#String` foundation type
- [[syntax/types/INDEX\|types]] -- full type system specification
