---
audience: automation-builder
type: specification
updated: 2026-04-10
status: draft
---

# IO Parameter Handling

<!-- @u:errors:Error Fallback Operators -->
<!-- @u:operators -->
<!-- @u:variable-lifecycle#Failed -->
<!-- @u:technical/ebnf/07-io-parameters -->
The `(>)` (output) and `(<)` (input) block markers handle IO parameters scoped under `(-)` IO lines (see [[blocks#Data Flow]]). Currently, fallback is the primary use case: providing a value when a pipeline call errors, preventing the variable from entering the Failed state. Fallback uses the `!<` / `!>` operators (see [[operators#Assignment Operators]]). The `!` error sigil always leads, with the direction arrow (`<` or `>`) following.

## Fallback Line Pattern

```polyglot
(>) !> value_expr
(>) !Error.Name> value_expr
```

`(>)` lines are indented under the `(-)` output line they belong to — the output reference is implicit from indentation scope:

```polyglot
[-] -File.Text.Read
   (-) <path << $file
   (-) >content >> $out
      (>) !> "generic fallback"
      (>) !File.NotFound> "file not found"
      (>) !File.ReadError> "read error"
```

| Form | Meaning |
|------|---------|
| `(>) !> value` | **Generic fallback** — activates for any unhandled error |
| `(>) !Error.Name> value` | **Error-specific fallback** — activates only for the named error |

When a fallback activates, the target variable becomes **Final** with the fallback value (not Failed). The error is accessible via `$var%sourceError` metadata. See [[errors#Error Fallback Operators]] for the full execution model and [[variable-lifecycle#Fallback]] for lifecycle semantics.

## Scoping Rules

- `(>)` / `(<)` must be **indented under** an `(-)` IO line — they inherit the output/input reference
- One generic `!>` per output — duplicates are PGE07003
- One `!Error.Name>` per specific error per output — duplicates are PGE07003
- Fallback values can be any `value_expr`: literals, `$` variables, inline pipeline calls

## Chain Execution Exception

In chain execution, fallback uses the `(-)` explicit form with step addressing (since `(>)`/`(<)` cannot carry step references):

```polyglot
[-] -File.Text.Read->-Text.Parse.CSV
   (-) >0.path << $file
   (-) <1.rows >> $rows
   (-) <0.content !< ""
   (-) <1.rows !< ""
   [!] !0.File.NotFound
      (-) <0.content !< "missing"
```
