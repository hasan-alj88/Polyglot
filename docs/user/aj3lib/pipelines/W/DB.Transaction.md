---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.W:DB.Transaction"
metadata_instance: "%W:DB.Transaction:N"
---

# -W.DB.Transaction

Opens connection + begins transaction on setup. Commits transaction + closes connection on cleanup. If execution body errors, transaction rolls back.

## Definition

```aljam3
{N} -W.DB.Transaction
   [%] .Kind << #NativeKind.Wrapper
   [%] .Rust << "WDbTransaction"
   [%] .description << "Opens connection + begins transaction on setup. Commits + closes on cleanup."
   (-) <connectionString;string   [ ] Database connection string
   (-) >tx                        [ ] Active transaction handle
   (-) >dbConn                    [ ] Open database connection handle
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `$connectionString` | `#string` | Database connection string |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `$tx` | transaction handle | Active transaction handle |
| `$dbConn` | connection handle | Open database connection handle |

## Errors

None.

## Permissions

Database.Connect

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.W:DB.Transaction` | Compile-time pipeline template |
| Instance | `%W:DB.Transaction:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[aj3lib/pipelines/W/INDEX|-W.* Wrappers]]
- [[aj3lib/pipelines/W/DB.Connection|-W.DB.Connection]]
