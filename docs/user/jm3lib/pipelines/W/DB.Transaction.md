---
audience: automation-builder
type: specification
updated: 2026-05-04
status: complete
metadata_definition: "%definition.W:DB.Transaction"
metadata_instance: "%W:DB.Transaction:N"
---

# -W.DB.Transaction

Establishes a secure Database Proxy session managed by the Rust Task Manager (TM). Opens the connection and begins a `BEGIN TRANSACTION` block on setup. If the pipeline succeeds, the TM commits the transaction and closes the connection on cleanup. If the pipeline errors, the TM automatically rolls back the transaction.

## Definition

```aljam3
{N} -W.DB.Transaction
   [%] .Kind << #NativeKind.Wrapper
   [%] .Rust << "WDbTransactionProxy"
   [%] .description << "Establishes secure Rust TM proxy for database capabilities within a Transaction block."
   (-) <_dbCapability                 [ ] The #DatabasePermission capability handle
   (-) >_dbTransactionSession         [ ] Active TM transaction proxy session delegated to the pipeline
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<_dbCapability` | `#DatabasePermission` | The strict atomic capability (e.g., `.InsertRecords` bound by `.TableNameRegex`) |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>_dbTransactionSession` | proxy handle | The secure proxy object that enforces the capability constraints during execution, bound to an active SQL Transaction |

## Execution Model
The pipeline is entirely shielded from raw TCL (Transaction Control Language) commands like `BEGIN`, `COMMIT`, or `ROLLBACK`. The Rust TM exclusively manages the transaction state, ensuring that a buggy pipeline cannot accidentally leave a transaction open or force an unauthorized commit.

## Errors

*   Throws if the Database connection defined in the capability's `.connectionUri` cannot be reached by the Rust TM.
*   Throws if the `##OneActive` constraint is violated.
*   Automatically triggers `ROLLBACK` if the inner pipeline raises a fault.

## Permissions

*   Any `#DatabasePermission` atomic action (`.QueryRecords`, `.InsertRecords`, `.UpdateRecords`, `.DeleteRecords`, `.ExecuteProcedure`, `.ExecuteRawQuery`).

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.W:DB.Transaction` | Compile-time pipeline template |
| Instance | `%W:DB.Transaction:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[jm3lib/pipelines/W/INDEX|-W.* Wrappers]]
- [[jm3lib/pipelines/W/DB.Connection|-W.DB.Connection]]
