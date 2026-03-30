---
rule: "7.2"
code: PGE-702
name: Chain Error Scoping
severity: error
---

### Rule 7.2 — Chain Error Scoping
`PGE-702`

**Statement:** In chain execution (`[r] =A=>=B=>=C`), each step's `[!]` error handler is scoped to that step only — as if each step were a separate `[r]` call. Error references use the `.N!ErrorName` syntax: a step prefix (`.N` numeric or `.LeafName` name-based) followed by `!` and the error path. The `!` separates the step reference from the error name, eliminating dotted-path ambiguity. Name-based step references must be unambiguous per PGE-804. The handler sees only its step's IO and can provide a replacement value for the chain's output variable.
**Rationale:** Chain steps are logically separate pipeline calls. Scoping error handlers to their producing step keeps error handling local and explicit. The `.N!Error` syntax mirrors how IO lines use `.N` for step addressing while clearly delimiting the step reference from the error path.
**Detection:** The compiler checks that every `[!]` block under a chain `[r]` uses `.N!` or `.LeafName!` syntax. If a chain `[!]` uses the non-chain form (`!ErrorName` without step prefix), PGE-702 fires. If a name-based step reference is ambiguous, PGE-804 fires.

#### Addressing syntax

| Form | Example | When to use |
|------|---------|-------------|
| Numeric | `[!] .0!File.NotFound` | Always valid — step by position |
| Name-based | `[!] .Read!File.NotFound` | When leaf name is unique across chain steps |

Non-chain `[r]` calls (single step) continue to use `[!] !ErrorName` — no step prefix needed.

#### Scope

The error handler for step N can:
- Read step N's IO (inputs and outputs)
- Read `$variables` from the enclosing pipeline scope (same as non-chain `[!]` handlers under PGE-701)
- Provide a replacement value for the chain's output variable (PGE-205 recovery)
- Write to the enclosing pipeline's output ports (same as non-chain `[!]` handlers)

The handler **cannot** access other steps' IO (steps other than N).

**VALID:**
```polyglot
[ ] ✓ numeric step addressing
[r] =File.Text.Read=>=Text.Parse.CSV
   [=] >0.path#path << $path
   [=] <1.rows#string >> >content
   [!] .0!File.NotFound
      [r] >content << "Error: file not found"
   [!] .1!Parse.InvalidFormat
      [r] >content << "Error: invalid CSV"
```

```polyglot
[ ] ✓ name-based — unique leaf names
[r] =File.Text.Read=>=Text.Parse.CSV
   [=] >Read.path#path << $path
   [=] <CSV.rows#string >> >content
   [!] .Read!File.NotFound
      [r] >content << "Error: file not found"
   [!] .CSV!Parse.InvalidFormat
      [r] >content << "Error: invalid CSV"
```

```polyglot
[ ] ✓ error handler reads $variable from enclosing pipeline scope
[r] =File.Text.Read=>=Text.Parse.CSV
   [=] >0.path#path << $path
   [=] <1.rows#string >> >content
   [!] .0!File.NotFound
      [r] =File.Text.Read                [ ] ✓ handler reads $fallbackPath from pipeline scope
         [=] <path << $fallbackPath
         [=] >content >> >content
```

```polyglot
[ ] ✓ error handler with *Continue fallback
[r] =File.Text.Read=>=Text.Parse.CSV
   [=] >0.path#path << $path
   [=] <1.rows#string >> >content
   [!] .0!File.NotFound
      [r] =LogError
         [=] <msg << "file not found"
      [*] *Continue >FallBack << ""
```

**INVALID:**
```polyglot
[ ] ✗ PGE-702 — chain error without step prefix
[r] =File.Text.Read=>=Text.Parse.CSV
   [=] >0.path#path << $path
   [=] <1.rows#string >> >content
   [!] !File.NotFound              [ ] ✗ PGE-702 — must use .0!File.NotFound
      [r] >content << "fallback"
```

```polyglot
[ ] ✗ PGE-804 — ambiguous name-based step ref
[r] =Text.Read=>=Data.Read
   [=] >0.input#string << $text
   [=] <1.output#string >> >result
   [!] .Read!NotFound              [ ] ✗ PGE-804 — "Read" matches both steps
      [r] >result << "error"
```

**See also:**
- [PGE-701 — Error Block Scoping](PGE-701-error-block-scoping.md) — basic `[!]` must be under `[r]`
- [PGE-804 — Ambiguous Step Reference](PGE-804-ambiguous-step-reference.md) — disambiguation rules
- [PGE-205 — Failed Is Terminal](PGE-205-failed-is-terminal.md) — recovery via replacement value
- [PGE-207 — Continue After Error](PGE-207-continue-after-error.md) — `*Continue` fallback in chain errors
- [[user/concepts/errors|Errors]] — references PGE-702 in chain error addressing
