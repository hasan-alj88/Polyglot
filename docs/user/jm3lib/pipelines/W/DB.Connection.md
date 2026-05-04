---
audience: automation-builder
type: specification
updated: 2026-05-04
status: complete
metadata_definition: "%definition.W:DB.Connection"
metadata_instance: "%W:DB.Connection:N"
---

# -W.DB.Connection

Establishes a secure Database Proxy session managed by the Rust Task Manager (TM). Opens the raw connection on setup, and securely delegates the capability to the inner pipeline. Closes connection on cleanup.

## Definition

```aljam3
{N} -W.DB.Connection
   [%] .Kind << #NativeKind.Wrapper
   [%] .Rust << "WDbConnectionProxy"
   [%] .description << "Establishes secure Rust TM proxy for database capabilities."
   (-) <_dbCapability                 [ ] The #DatabasePermission capability handle
   (-) >_dbProxySession               [ ] Active TM proxy session delegated to the pipeline
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<_dbCapability` | `#DatabasePermission` | The strict atomic capability (e.g., `.InsertRecords` bound by `.TableNameRegex`) |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>_dbProxySession` | proxy handle | The secure proxy object that enforces the capability constraints during execution |

## Errors

*   Throws if the Database connection defined in the capability's `.connectionUri` cannot be reached by the Rust TM.
*   Throws if the `##OneActive` constraint is violated.

## Permissions

*   Any `#DatabasePermission` atomic action (`.QueryRecords`, `.InsertRecords`, `.UpdateRecords`, `.DeleteRecords`, `.ExecuteProcedure`, `.ExecuteRawQuery`).

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.W:DB.Connection` | Compile-time pipeline template |
| Instance | `%W:DB.Connection:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[jm3lib/pipelines/W/INDEX|-W.* Wrappers]]
- [[jm3lib/pipelines/W/DB.Transaction|-W.DB.Transaction]]
