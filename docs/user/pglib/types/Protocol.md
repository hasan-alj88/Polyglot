---
audience: pg-coder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.#:Protocol"
metadata_instance: "%#:Protocol:N"
---

# #Protocol Enum

<!-- @types -->

Permission enum used in `{_}` permission objects. See [[concepts/permissions|Permissions]].

## Definition

```polyglot
{#} #Protocol
   [%] .description << "IO protocol for permission resource descriptors"
   [%] .version << "1.0.0"
   [#] ##Enum
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "protocol"
   [.] .File
   [.] .TCP
   [.] .UDP
   [.] .HTTPS
```

| Variant | Description |
|---------|-------------|
| `.File` | File system protocol |
| `.TCP` | TCP transport |
| `.UDP` | UDP transport |
| `.HTTPS` | HTTPS transport |

> **Note:** Used in __ResourceDescriptor to specify transport protocol.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:Protocol` | Compile-time type template |
| Instance | `%#:Protocol:0` | Runtime instance (enum — one active field) |

## Related

- [[concepts/permissions|Permissions]] -- permission system overview
- [[enums]] -- all enum types
- [[syntax/types/INDEX|types]] -- full type system specification
