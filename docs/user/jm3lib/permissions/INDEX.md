---
audience: automation-builder
type: specification
updated: 2026-04-12
status: complete
---

# jm3lib Generic Permissions

<!-- @c:permissions -->

jm3lib provides `__` generic permission templates — syntax sugar that interacts with the compiler to produce concrete `_` permission objects at compile time.

In Aljam3, authorized capability handles (`_`) are strictly organized into the `{_}` **Permissions Data Tree**, which is governed by the `##Permissions` schema. 

The `__` operators consume values from the `{;}` Environments tree, verify the resource (e.g., File, Database) exists and is accessible, and yield authorized capability handles (`_`) into the `{_}` tree using the `(_)` output marker.

Example:
```aljam3
{_}
   (<) <ProdDB << __Database.Connect
      (<) <uri << {;}<DbUri          
```

## Operator Levels
- **Category-level** (`__File`, `__Web`, ...) — takes capability + scope as inputs
- **Capability-level** (`__File.Read`, `__Web.Request`, ...) — takes only scope, sugar over category-level

All generics produce `_` objects with `#Grant` intent. For `#Ceiling` objects, write a full `{_}` block manually instead.

## Category-Level Generics

| Generic | Inputs | Category |
|---------|--------|----------|
| [[File/INDEX\|__File]] | `<capability;FileCapability`, `<scope;path` | File system operations |
| [[Web/INDEX\|__Web]] | `<capability;WebCapability`, `<scope;string` | Network/web operations |
| [[Database/INDEX\|__Database]] | `<capability;DatabaseCapability`, `<scope;string` | Database operations |
| [[System/INDEX\|__System]] | `<capability;SystemCapability`, `<scope;string` | System-level operations |
| [[Crypto/INDEX\|__Crypto]] | `<capability;CryptoCapability`, `<scope;string` | Cryptographic operations |
| [[IPC/INDEX\|__IPC]] | `<capability;IPCCapability`, `<scope;string` | Inter-process communication |
| [[Device/INDEX\|__Device]] | `<capability;DeviceCapability`, `<scope;string` | Device access operations |
| [[Memory/INDEX\|__Memory]] | `<capability;MemoryCapability`, `<scope;string` | Memory operations |

## Capability-Level Generics

| Generic | Input | Description |
|---------|-------|-------------|
| [[File/Read\|__File.Read]] | `<scope;path` | Read files |
| [[File/Write\|__File.Write]] | `<scope;path` | Write files |
| [[File/Execute\|__File.Execute]] | `<scope;path` | Execute files |
| [[File/Delete\|__File.Delete]] | `<scope;path` | Delete files |
| [[File/Create\|__File.Create]] | `<scope;path` | Create files |
| [[Web/Request\|__Web.Request]] | `<scope;string` | HTTP requests |
| [[Web/Socket\|__Web.Socket]] | `<scope;string` | WebSocket connections |
| [[Web/Listen\|__Web.Listen]] | `<scope;string` | Listen on ports |
| [[Database/QueryRecords\|__Database.QueryRecords]] | `<scope;string` | Retrieve records matching TableRegex |
| [[Database/InsertRecords\|__Database.InsertRecords]] | `<scope;string` | Insert records matching TableRegex |
| [[Database/UpdateRecords\|__Database.UpdateRecords]] | `<scope;string` | Update records matching TableRegex |
| [[Database/DeleteRecords\|__Database.DeleteRecords]] | `<scope;string` | Delete records matching TableRegex |
| [[Database/ExecuteProcedure\|__Database.ExecuteProcedure]] | `<scope;string` | Run stored procedure |
| [[Database/ExecuteRawQuery\|__Database.ExecuteRawQuery]] | `<scope;string` | High privilege arbitrary SQL |
| [[System/Env\|__System.Env]] | `<scope;string` | Access environment variables |
| [[System/Process\|__System.Process]] | `<scope;string` | Spawn/manage processes |
| [[System/Signal\|__System.Signal]] | `<scope;string` | Send/receive signals |
| [[System/Shell\|__System.Shell]] | `<scope;string` | Execute shell commands |
| [[Crypto/Key\|__Crypto.Key]] | `<scope;string` | Key management |
| [[Crypto/Sign\|__Crypto.Sign]] | `<scope;string` | Signing operations |
| [[Crypto/Encrypt\|__Crypto.Encrypt]] | `<scope;string` | Encryption operations |
| [[IPC/Send\|__IPC.Send]] | `<scope;string` | Send messages |
| [[IPC/Receive\|__IPC.Receive]] | `<scope;string` | Receive messages |
| [[IPC/Subscribe\|__IPC.Subscribe]] | `<scope;string` | Subscribe to channels |
| [[Device/Camera\|__Device.Camera]] | `<scope;string` | Camera access |
| [[Device/Microphone\|__Device.Microphone]] | `<scope;string` | Microphone access |
| [[Device/Location\|__Device.Location]] | `<scope;string` | Location access |
| [[Device/Bluetooth\|__Device.Bluetooth]] | `<scope;string` | Bluetooth access |
| [[Memory/Allocate\|__Memory.Allocate]] | `<scope;string` | Memory allocation |
| [[Memory/Shared\|__Memory.Shared]] | `<scope;string` | Shared memory access |

## Related

- [[concepts/permissions]] -- permission system overview
- [[jm3lib/types/PermissionCategory]] -- 8-category enum
- [[jm3lib/types/PermissionIntent]] -- Ceiling vs Grant intent
