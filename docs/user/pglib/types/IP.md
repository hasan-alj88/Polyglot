---
audience: automation-builder
type: specification
updated: 2026-04-22
status: complete
metadata_definition: "%definition.#:IP"
metadata_instance: "%#:IP:N"
---

# #IP — IP Address

<!-- @c:types -->
<!-- @c:pglib/types/string -->

`#IP` is a pglib type representing an IP address (IPv4 or IPv6). IP address validation requires range checking (e.g., octets 0-255 for IPv4, valid hex groups for IPv6) that cannot be expressed as regex constraints, so `#IP` is backed by a native `PgIP` class.

See [[scalars]] for scalar subtypes and [[syntax/constructors]] for the `$IP` constructor.

## Definition

```aljam3
{#} #IP
   [%] .description << "IP address (IPv4 or IPv6)"
   [#] ##Scalar
   [#] ###ScalarValue
   [#] %##Alias << "ip"
   [.] .address#RawString
   [%] %Native.Class
      [.] .Rust << "PgIP"
      [.] .Validate << #True
```

## Fields

| Field | Type | Default | Purpose |
|-------|------|---------|---------|
| `.address` | `#RawString` | (none) | The IP address string in standard notation (dotted-decimal for IPv4, colon-hex for IPv6) |

## Schema Properties

- `[#] ##Scalar` -- sets `%##Depth.Max << 1`, marking `#IP` as a scalar type
- `[#] ###ScalarValue` -- single-value scalar (no enum variants)
- `%##Alias << "ip"` -- lets users write `#ip` (lowercase) as shorthand

## Native Class

`#IP` is backed by a native Rust class (`PgIP`) declared via `[%] %Native.Class`. IPv4 validation requires checking that each octet is in range 0-255, and IPv6 validation requires proper hex group structure and abbreviation rules — neither is expressible as a simple regex constraint. The compiler invokes the `PgIP` parser to validate address strings at compile time.

| Field | Value | Meaning |
|-------|-------|---------|
| `.Rust` | `"PgIP"` | Name of the Rust class backing `#IP` |
| `.Validate` | `#True` | The class exposes a validation function |

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:IP` | Compile-time type template |
| Instance | `%#:IP:N` | Runtime instance (N = instance number) |
| Native class | `%definition.#:IP.%Native.Class` | Rust class backing (`PgIP`) |

## Related

- [[pglib/constructors/IP\|$IP constructor]] -- compile-time IP address construction
- [[pglib/pipelines/IP.Parse\|-IP.Parse]] -- runtime IP address string parsing
- [[scalars]] -- scalar subtypes overview
- [[string]] -- `#String` foundation type
- [[syntax/types/INDEX\|types]] -- full type system specification
