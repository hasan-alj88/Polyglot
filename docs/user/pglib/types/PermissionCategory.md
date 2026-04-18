---
audience: automation-builder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.#:PermissionCategory"
metadata_instance: "%#:PermissionCategory:N"
---

# #PermissionCategory Enum

<!-- @c:types -->

Permission enum used in `{_}` permission objects. See [[concepts/permissions|Permissions]].

## Definition

```polyglot
{#} #PermissionCategory
   [%] .description << "Permission IO category"
   [%] .version << "1.0.0"
   [#] ##Enum
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "permissioncategory"
   [.] .File
   [.] .Web
   [.] .Database
   [.] .System
   [.] .Crypto
   [.] .IPC
   [.] .Device
   [.] .Memory
   [.] .RAM
   [.] .CPU
   [.] .GPU
   [.] .IO
   [.] .Processes
   [.] .Duration
```

| Variant | Description |
|---------|-------------|
| `.File` | File system operations |
| `.Web` | Network/web operations |
| `.Database` | Database operations |
| `.System` | System-level operations |
| `.Crypto` | Cryptographic operations |
| `.IPC` | Inter-process communication |
| `.Device` | Device access operations |
| `.Memory` | Memory operations |
| `.RAM` | Memory resource limits (cgroups v2) |
| `.CPU` | CPU resource limits (cgroups v2) |
| `.GPU` | GPU resource limits (device controller) |
| `.IO` | IO bandwidth/IOPS limits (cgroups v2) |
| `.Processes` | Process count limits (cgroups v2) |
| `.Duration` | Execution time limits (timer-based) |

> **Note:** Each category maps to a permission namespace in !Permission errors.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:PermissionCategory` | Compile-time type template |
| Instance | `%#:PermissionCategory:0` | Runtime instance (enum — one active field) |

## Related

- [[concepts/permissions|Permissions]] -- permission system overview
- [[enums]] -- all enum types
- [[syntax/types/INDEX|types]] -- full type system specification
