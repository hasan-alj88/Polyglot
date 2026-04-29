---
audience: automation-builder
type: specification
updated: 2026-04-17
status: complete
---

# __Permission Schema

The `__Permission` descriptor defines the full structure of a permission object:

```aljam3
__Permission
+-- .target                    __PermissionTarget
|   +-- .category              #PermissionCategory     <- enum: File, Web, Database, System, Crypto, IPC, Device, Memory
|   +-- .capability            #Capability             <- per-category enum
|   +-- .scope                 __PermissionScope
|       +-- .pattern           #GlobPattern (ceiling) or specific value (grant)
|       +-- .direction         #IODirection            <- enum: Inbound, Outbound, Both
+-- .grant                     __PermissionGrant
|   +-- .level                 #AccessLevel            <- enum: Allow, Deny
|   +-- .authority             #GrantAuthority         <- enum: Package, Pipeline
|   +-- .intent                #PermissionIntent       <- enum: Ceiling, Grant
+-- .resource                  __ResourceDescriptor
|   +-- .os                    #OSTarget               <- enum: Any, Linux, Windows, MacOS
|   +-- .protocol              #Protocol               <- enum: File, TCP, UDP, HTTPS, IPC, SharedMemory, USB, Bluetooth
|   +-- .handle                #HandleKind             <- enum: Path, ConnectionString, Descriptor, Address, DeviceID
|   +-- .locator               __ResourceLocator       <- category-dependent resource fields
|       +-- .path              #path                   <- File: required — filesystem path to the resource
|       +-- .format            #SerialFormat           <- File: optional — enum: YAML, JSON, TOML
|       +-- .host              #string                 <- Database/Web: host address
|       +-- .port              #int                    <- Database/Web: port number
|       +-- .endpoint          #string                 <- Web: API endpoint path
|       +-- .credentials       #path                   <- Database: path to credentials file (also content-hashed)
|       +-- .database          #string                 <- Database: database name
|       +-- .table             #string                 <- Database: table name
|       +-- .command           #string                 <- System: command to execute
|       +-- .args              #string                 <- System: command arguments
|       +-- .max              #string                 <- RAM/CPU/GPU/Processes/Duration: resource limit value
|       +-- .weight           #int                    <- CPU: scheduling weight (1-10000)
|       +-- .device           #string                 <- GPU: device identifier
|       +-- .maxBps           #string                 <- IO: max bytes per second
|       +-- .maxIops          #int                    <- IO: max IO operations per second
+-- .audit                     __PermissionAudit
    +-- .log                   #AuditLevel             <- enum: None, OnUse, OnDeny, All
    +-- .alert                 #AlertLevel             <- enum: None, OnDeny, OnEscalation
```

All fields use `.` fixed-field navigation — permission schemas are Aljam3-defined, not user-extensible.

## Resource Locator Fields

The `.resource.locator` section carries **category-dependent** fields that identify the specific external resource. Not all fields apply to every category — the compiler validates that only relevant fields are present.

| Category | Required Fields | Optional Fields |
|----------|----------------|-----------------|
| File | `.path` | `.format` |
| Database | `.host`, `.database` | `.port`, `.credentials`, `.table` |
| Web | `.host` | `.port`, `.endpoint` |
| System | `.command` | `.args` |
| Crypto | — | — |
| IPC | — | — |
| Device | — | — |
| Memory | — | — |
| RAM | `.max` | — |
| CPU | `.max` | `.weight` |
| GPU | `.max` | `.device` |
| IO | — | `.maxBps`, `.maxIops` |
| Processes | `.max` | — |
| Duration | `.max` | — |

The permission object is the **sole gateway to external resources**. Pipelines receive the whole `_` object via IO and extract the fields they need — `-Yaml.LoadFile` reads `.path` and `.format`, `-DB.Query` reads `.host`, `.port`, `.credentials`, and `.database`. No hardcoded resource identifiers exist outside `{_}` definitions.

## Compile-Time Resolution

All resource locator fields are resolved at compile time:

- **File resources:** The compiler reads the file at `.path` and computes a content hash. If the file changes after compilation, the associated permission is revoked. See [[enforcement#Compile-Time File Binding]].
- **Credentials files:** `.credentials` paths are also content-hashed — credential rotation requires recompilation.
- **Templates:** `{_}` definitions with `(_)` inputs resolve all interpolated fields (e.g., `.path "{<file}"`) before hashing.

## Shorthand

In `{_}` blocks, users write decomposed fields directly:

```aljam3
{_} _LogReader
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/var/log/app/*.log"
   [.] .path "/var/log/app/current.log"
```

The compiler maps these to the full `__Permission` tree at compile time. All fields are validated against the `##Permission` schema — unknown fields are a compile error.
