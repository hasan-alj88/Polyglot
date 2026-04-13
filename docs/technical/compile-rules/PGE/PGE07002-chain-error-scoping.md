---
audience: developer
rule: "7.2"
code: PGE07002
name: Chain Error Scoping
severity: error
---

# Rule 7.2 — Chain Error Scoping
`PGE07002`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->

**Statement:** In chain execution (`[-] -A->-B->-C`), each step's `[!]` error handler is scoped to that step only — as if each step were a separate `[-]` call. Error references use the `.N!ErrorName` syntax: a step prefix (`.N` numeric or `.LeafName` name-based) followed by `!` and the error path. The `!` separates the step reference from the error name, eliminating dotted-path ambiguity. Name-based step references must be unambiguous per PGE08004. The handler sees only its step's IO and can provide a replacement value for the chain's output variable.
**Rationale:** Chain steps are logically separate pipeline calls. Scoping error handlers to their producing step keeps error handling local and explicit. The `.N!Error` syntax mirrors how IO lines use `.N` for step addressing while clearly delimiting the step reference from the error path. This extends Polyglot's exhaustive error handling to chains — each step's failure modes must be addressed individually, ensuring the compiler can verify complete coverage across the entire chain.
**Detection:** The compiler checks that every `[!]` block under a chain `[-]` uses `.N!` or `.LeafName!` syntax. If a chain `[!]` uses the non-chain form (`!ErrorName` without step prefix), PGE07002 fires. If a name-based step reference is ambiguous, PGE08004 fires.

## Addressing syntax

| Form | Example | When to use |
|------|---------|-------------|
| Numeric | `[!] .0!File.NotFound` | Always valid — step by position |
| Name-based | `[!] .Read!File.NotFound` | When leaf name is unique across chain steps |

Non-chain `[-]` calls (single step) continue to use `[!] !ErrorName` — no step prefix needed.

## Scope

The error handler for step N can:
- Read step N's IO (inputs and outputs)
- Read `$variables` from the enclosing pipeline scope (same as non-chain `[!]` handlers under PGE07001)
- Provide a replacement value for the chain's output variable (PGE02005 recovery)
- Write to the enclosing pipeline's output ports (same as non-chain `[!]` handlers)

The handler **cannot** access other steps' IO (steps other than N).

**VALID:**
```polyglot
[ ] ✓ numeric step addressing
[-] -File.Text.Read->-Text.Parse.CSV
   (-) >0.path#path << $path
   (-) <1.rows#string >> >content
   [!] .0!File.NotFound
      [-] >content << "Error: file not found"
   [!] .1!Parse.InvalidFormat
      [-] >content << "Error: invalid CSV"
```

```polyglot
[ ] ✓ name-based — unique leaf names
[-] -File.Text.Read->-Text.Parse.CSV
   (-) >Read.path#path << $path
   (-) <CSV.rows#string >> >content
   [!] .Read!File.NotFound
      [-] >content << "Error: file not found"
   [!] .CSV!Parse.InvalidFormat
      [-] >content << "Error: invalid CSV"
```

```polyglot
[ ] ✓ error handler reads $variable from enclosing pipeline scope
[-] -File.Text.Read->-Text.Parse.CSV
   (-) >0.path#path << $path
   (-) <1.rows#string >> >content
   [!] .0!File.NotFound
      [-] -File.Text.Read                [ ] ✓ handler reads $fallbackPath from pipeline scope
         (-) <path << $fallbackPath
         (-) >content >> >content
```

```polyglot
[ ] ✓ error handler with !> fallback
[-] -File.Text.Read->-Text.Parse.CSV
   (-) >0.path#path << $path
   (-) <1.rows#string >> >content
      (>) !> ""                       [ ] catch-all fallback
   [!] .0!File.NotFound
      [-] -LogError
         (-) <msg << "file not found"
      [-] >content << ""
```

**INVALID:**
```polyglot
[ ] ✗ PGE07002 — chain error without step prefix
[-] -File.Text.Read->-Text.Parse.CSV
   (-) >0.path#path << $path
   (-) <1.rows#string >> >content
   [!] !File.NotFound              [ ] ✗ PGE07002 — must use .0!File.NotFound
      [-] >content << "fallback"
```

```polyglot
[ ] ✗ PGE08004 — ambiguous name-based step ref
[-] -Text.Read->-Data.Read
   (-) >0.input#string << $text
   (-) <1.output#string >> >result
   [!] .Read!NotFound              [ ] ✗ PGE08004 — "Read" matches both steps
      [-] >result << "error"
```

**See also:**
- [PGE07001 — Error Block Scoping](PGE07001-error-block-scoping.md) — basic `[!]` must be under `[-]`
- [PGE08004 — Ambiguous Step Reference](PGE08004-ambiguous-step-reference.md) — disambiguation rules
- [PGE02005 — Failed Is Terminal](PGE02005-failed-is-terminal.md) — recovery via replacement value
- [PGE02005 — Failed Must Resolve](PGE02005-failed-is-terminal.md) — compiler-enforced error handling
- [[user/concepts/errors|Errors]] — references PGE07002 in chain error addressing
