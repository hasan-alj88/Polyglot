---
audience: developer
rule: "7.8"
code: PGE07008
name: Fallback on Non-Failable Source
severity: error
---

# Rule 7.8 — Fallback on Non-Failable Source
`PGE07008`

<!-- @u:syntax/operators -->
<!-- @c:compile-rules/PGE/PGE07003-duplicate-fallback-assignment -->
<!-- @c:compile-rules/PGW/PGW07004-fallback-on-non-failable-io -->

**Statement:** Fallback assignment operators (`!<` and `!>`) are only valid when the source expression (right-hand side) is a pipeline call — an expression that can fail at runtime. If the source is a literal value or variable reference, `PGE07008` fires. Literals and variables cannot error, so the fallback path can never activate.
**Rationale:** Fallback operators exist for error recovery — they provide an alternative value when a pipeline call produces an error instead of a result. Applying them to non-failable expressions is semantically meaningless: the fallback path is dead code that can never execute. Catching this at compile time prevents developer confusion about when fallback operators activate.
**Detection:** The compiler inspects the right-hand side of any assignment using `!<` or `!>`. If the RHS resolves to a literal value, a variable reference, or any other non-failable expression (not a pipeline call), PGE07008 fires.

## Applicability

This rule applies universally — the grammar permits `!<`/`!>` in all `assignment_op` positions, but the compiler enforces semantic validity based on what the source expression actually is:

| Source Expression | `<<`/`>>` | `<~`/`~>` | `!<`/`!>` |
|-------------------|-----------|-----------|-----------|
| Literal value | Valid | Valid | **PGE07008** |
| Variable reference | Valid | Valid | **PGE07008** |
| Inline pipeline call | Valid | Valid | Valid |

This means the same production (e.g., `expand_io_line`, `schema_property`, `collect_io_line`) may or may not trigger PGE07008 depending on its RHS — the constraint is on the source, not the syntactic context.

**Exception — collector output:** The `(*) >>` collect-output line is always paired with a collection process that can fail (timeouts, all-Failed race inputs). Fallback on `(*) >>` is valid even though the syntax uses a variable reference target, because the collection process itself is the failable source.

## Diagnostic Format

`Fallback operator '!<' used with non-failable source at line N — '!<'/'!>' only activates on error from a pipeline call; literals and variables cannot error`

---

## Examples

### VALID

```polyglot
[ ] ✓ fallback on pipeline call — pipeline can fail
[-] -File.Text.Read
   (-) <path << $configPath
   (-) >content >> $data
      (<) !< "/default/path.txt"
```

```polyglot
[ ] ✓ fallback on inline pipeline in data load
[#] $config#Settings !< -Json.LoadFile"/fallback.json"
```

```polyglot
[ ] ✓ collector output fallback — collection process can fail
[*] *First
   (*) << $rA
   (*) << $rB
   (*) >> $winner
      (>) !> $defaultResult
```

### INVALID

```polyglot
[ ] ✗ PGE07008 — schema property with fallback on literal
{#} #Sensor
   [#] %##Depth.Max !< 3
```

```polyglot
[ ] ✗ PGE07008 — expand IO with fallback on variable
[=] =ForEach.Array
   (=) <Array !< $items
   (=) >item >> $val
```

```polyglot
[ ] ✗ PGE07008 — collect IO input with fallback on variable
[-] *Into.Array
   (*) <item !< $val
   (*) >Array >> $result
```

```polyglot
[ ] ✗ PGE07008 — value field with fallback on literal
{#} #User
   [.] .name#string !< "anonymous"
```

```polyglot
[ ] ✗ PGE07008 — metadata field with fallback on literal
[%] .description !< "My pipeline"
```

```polyglot
[ ] ✗ PGE07008 — data load with fallback on literal (no pipeline source)
[#] $config#Settings !< "raw-string"
```

### See Also

- [[compile-rules/PGE/PGE07009-unterminated-fallback-chain|PGE07009]] — fallback chain must terminate at non-failable value
- [[compile-rules/PGE/PGE07003-duplicate-fallback-assignment|PGE07003]] — duplicate fallback on same output
- [[compile-rules/PGW/PGW07004-fallback-on-non-failable-io|PGW07004]] — fallback on non-failable IO (warning-level cousin)
