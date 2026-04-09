---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.W:DB.Connection"
metadata_instance: "%W:DB.Connection:N"
---

# -W.DB.Connection

Opens DB connection on setup, closes on cleanup.

## Definition

```polyglot
{N} -W.DB.Connection
   [%] .Kind << #NativeKind.Wrapper
   [%] .Rust << "WDbConnection"
   [%] .description << "Opens DB connection on setup, closes on cleanup."
   [{] $connectionString#string   [ ] Database connection string
   [}] $dbConn                    [ ] Open database connection handle
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `$connectionString` | `#string` | Database connection string |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `$dbConn` | connection handle | Open database connection handle |

## Errors

None.

## Permissions

Database.Connect

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.W:DB.Connection` | Compile-time pipeline template |
| Instance | `%W:DB.Connection:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/W/INDEX|-W.* Wrappers]]
- [[pglib/pipelines/W/DB.Transaction|-W.DB.Transaction]]
