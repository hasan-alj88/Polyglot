---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =W.DB.Transaction

Opens connection + begins transaction on setup. Commits transaction + closes connection on cleanup. If execution body errors, transaction rolls back.

## Definition

```polyglot
{N} =W.DB.Transaction
   [%] .Kind << #NativeKind.Wrapper
   [%] .Rust << "WDbTransaction"
   [%] .description << "Opens connection + begins transaction on setup. Commits + closes on cleanup."
   [{] $connectionString#string   [ ] Database connection string
   [}] $tx                        [ ] Active transaction handle
   [}] $dbConn                    [ ] Open database connection handle
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

## Related

- [[pglib/pipelines/W/INDEX|=W.* Wrappers]]
- [[pglib/pipelines/W/DB.Connection|=W.DB.Connection]]
