---
audience: automation-builder
type: specification
updated: 2026-04-05
status: complete
---

# __Permission Schema

The `__Permission` descriptor defines the full structure of a permission object:

```polyglot
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
+-- .audit                     __PermissionAudit
    +-- .log                   #AuditLevel             <- enum: None, OnUse, OnDeny, All
    +-- .alert                 #AlertLevel             <- enum: None, OnDeny, OnEscalation
```

All fields use `.` fixed-field navigation — permission schemas are Polyglot-defined, not user-extensible.
