---
audience: automation-builder
type: specification
updated: 2026-04-22
status: complete
metadata_definition: "%definition.#:URL"
metadata_instance: "%#:URL:N"
---

# #URL — Uniform Resource Locator

<!-- @c:types -->
<!-- @c:jm3lib/types/string -->

`#URL` is a jm3lib type representing a URL (RFC 3986). It decomposes a URL string into its scheme, host, port, path, query, and fragment components. URL parsing requires an RFC 3986-compliant parser — not regex — so `#URL` is backed by a native `PgUrl` class.

See [[scalars]] for scalar subtypes and [[syntax/constructors]] for the `$URL` constructor.

## Definition

```aljam3
{#} #URL
   [%] .description << "Uniform Resource Locator (RFC 3986)"
   [#] ##Scalar
   [#] %##Alias << "url"
   [.] .scheme#string
   [.] .host#string
   [.] .port#int
      <~ 0
   [.] .path#path
      <~ $Path""
   [.] .query#string
      <~ ""
   [.] .fragment#string
      <~ ""
   [%] %Native.Class
      [.] .Rust << "PgUrl"
      [.] .Validate << #True
```

## Fields

| Field | Type | Default | Purpose |
|-------|------|---------|---------|
| `.scheme` | `#string` | (none) | URL scheme (e.g., `"https"`, `"ssh"`) |
| `.host` | `#string` | (none) | Host name or IP address |
| `.port` | `#int` | `0` | Port number (0 = scheme default) |
| `.path` | `#path` | `$Path""` | URL path component |
| `.query` | `#string` | `""` | Query string (without leading `?`) |
| `.fragment` | `#string` | `""` | Fragment identifier (without leading `#`) |

## Schema Properties

- `[#] ##Scalar` -- sets `%##Depth.Max << 1`, marking `#URL` as a scalar type
- `%##Alias << "url"` -- lets users write `#url` (lowercase) as shorthand

## Native Class

`#URL` is backed by a native Rust class (`PgUrl`) declared via `[%] %Native.Class`. URL syntax (RFC 3986) cannot be fully validated by regex due to nested authority components, percent-encoding rules, and scheme-specific constraints. The compiler invokes the `PgUrl` parser to validate and decompose URL strings at compile time.

| Field | Value | Meaning |
|-------|-------|---------|
| `.Rust` | `"PgUrl"` | Name of the Rust class backing `#URL` |
| `.Validate` | `#True` | The class exposes a validation function |

## Well-Known Schemes

| Scheme | Default Port | Common Use |
|--------|-------------|------------|
| `http` | 80 | Web pages (unencrypted) |
| `https` | 443 | Web pages (TLS) |
| `ftp` | 21 | File transfer |
| `ssh` | 22 | Secure shell |
| `ws` | 80 | WebSocket |
| `wss` | 443 | WebSocket (TLS) |
| `file` | --- | Local filesystem |
| `mailto` | --- | Email addresses |

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:URL` | Compile-time type template |
| Instance | `%#:URL:N` | Runtime instance (N = instance number) |
| Native class | `%definition.#:URL.%Native.Class` | Rust class backing (`PgUrl`) |

## Related

- [[jm3lib/constructors/URL\|$URL constructor]] -- compile-time URL construction
- [[jm3lib/pipelines/URL.Parse\|-URL.Parse]] -- runtime URL string parsing
- [[scalars]] -- scalar subtypes overview
- [[string]] -- `#String` foundation type
- [[syntax/types/INDEX\|types]] -- full type system specification
