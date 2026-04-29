---
audience: automation-builder
type: specification
status: retired
updated: 2026-04-22
---

<!-- @concepts/pipelines/INDEX -->

## Chain Execution (Retired)

The `->` chain operator has been retired from Aljam3. Chains connected pipelines in sequence on a single `[-]` line (`-A->-B->-C`), with step-addressed IO and chain-specific error handling. This syntax has been replaced by **labeled `[-]` calls** using operation labels, which are strictly more powerful.

### Why Retired

Three existing mechanisms already handle sequential pipeline composition:

| Mechanism | Role |
|---|---|
| `[-]` marker | Declares sequential execution intent |
| `(-) $Label` + `$Label>output` | Names steps and addresses their IO |
| Trigger model (launched + inputs Final) | Orders execution by data readiness |

Chains were a redundant syntactic overlay. Labeled `[-]` calls support any-to-any wiring (not just linear N-to-N+1), use standard `[!]` error scoping (no special `.N!ErrorName` syntax), and require no additional EBNF grammar.

### Replacement Pattern

**Before (chain):**
```aljam3
[-] -File.Text.Read->-Text.Parse.CSV
   (-) >0.path#path << $file
   (-) <1.rows#string >> >content
   [!] !0.File.NotFound
      [-] >content << "Error: file not found"
   [!] !1.Parse.InvalidFormat
      [-] >content << "Error: invalid CSV"
```

**After (labeled calls):**
```aljam3
[ ] Step 1: read file
[-] -File.Text.Read
   (-) $Read
   (-) <path#path << $file
   (-) >content#string >> $fileContent
   [!] !File.NotFound
      [-] $fileContent << "Error: file not found"

[ ] Step 2: parse CSV — triggers when $fileContent is Final
[-] -Text.Parse.CSV
   (-) $Parse
   (-) <input#string << $fileContent
   (-) >rows#string >> >content
   [!] !Parse.InvalidFormat
      [-] >content << "Error: invalid CSV"
```

Each `[-]` call has its own `[!]` error blocks using standard scoping. No step indices, no chain-specific syntax. The trigger model ensures `-Text.Parse.CSV` runs only after `-File.Text.Read` completes, because `$fileContent` must be Final before the second call triggers.

### What Was Retired

| Retired Syntax | Replacement |
|---|---|
| `->` chain operator | Sequential `[-]` calls |
| `($)` / `(.)` step labels | `(-) $Label` operation labels |
| `>N.param` / `<N.param` step addressing | `$Label>output` / `$Label<input` |
| `.N!ErrorName` chain error addressing | Standard `[!]` blocks per call |
| Chain auto-wire | Explicit `(-)` wiring (wildcard auto-wire `<* << $A>*` in #345) |

### Related Compile Rules (Retired)

PGE07002, PGE08004, PGE08005, PGE08006, PGE08012, PGE10007 — all retired with chains.

## See Also

- [[syntax/operation-labels|Operation Labels]] — `(-) $Label` naming and `$Label>output` addressing
- [[concepts/pipelines/execution|Execution]] — execution body markers and rules
- [[concepts/pipelines/inline-calls|Inline Calls]] — inline pipeline calls in execution body
