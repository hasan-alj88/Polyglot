---
audience: pg-coder
type: specification
updated: 2026-04-09
status: complete
---

# Enum Types

<!-- @types -->

Runtime and internal `##Enum` types available in every `.pg` file. All enums use `[#] ##Enum` (enum classification), `[#] ##Scalar` (depth 1), and `[#] ###ScalarEnum` (leaf content is variant selection).

See [[boolean]] for `#Boolean` (also a `##Enum` type, documented separately).

| Type | Description | File |
|------|-------------|------|
| `#OS` | Operating system enum | [[OS]] |
| `#PipelineStatus` | Pipeline instance status | [[PipelineStatus]] |
| `#QueueStrategy` | Queue ordering strategy | [[QueueStrategy]] |
| `#RetriggerStrategy` | Retrigger handling strategy | [[RetriggerStrategy]] |
| `#QueueState` | Pipeline state within queue system | [[QueueState]] |
| `#KillPropagation` | Kill signal propagation to sub-jobs | [[KillPropagation]] |
| `#ResourceTag` | Resource tag for dispatch constraints | [[ResourceTag]] |
| `#FileAccess` | File access state | [[FileAccess]] |
| `#VarState` | Variable lifecycle state | [[VarState]] |
| `#FieldKind` | Leaf content field type classifier | [[FieldKind]] |
| `#FieldsDescriptor` | Child field descriptor | [[FieldsDescriptor]] |
| `#ActiveKind` | Branch activation classifier | [[ActiveKind]] |

## Permission Enums

Permission enums used in `{_}` permission objects. See [[concepts/permissions|Permissions]].

| Type | Description | File |
|------|-------------|------|
| `#PermissionIntent` | Ceiling vs Grant intent | [[PermissionIntent]] |
| `#PermissionCategory` | IO category (File, Web, Database, ...) | [[PermissionCategory]] |
| `#IODirection` | Inbound, Outbound, or Both | [[IODirection]] |
| `#AccessLevel` | Allow or Deny | [[AccessLevel]] |
| `#GrantAuthority` | Package or Pipeline scope | [[GrantAuthority]] |
| `#OSTarget` | Target operating system | [[OSTarget]] |
| `#Protocol` | IO transport protocol | [[Protocol]] |
| `#HandleKind` | Resource handle type | [[HandleKind]] |
| `#AuditLevel` | Audit logging level | [[AuditLevel]] |
| `#AlertLevel` | Alert trigger level | [[AlertLevel]] |

## Related

- [[boolean]] -- #Boolean enum type
- [[structs]] -- #Queue struct (uses #QueueStrategy, #KillPropagation, #ResourceTag)
- [[syntax/types/INDEX|types]] -- full type system specification
