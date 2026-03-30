---
rule: "1.22"
code: PGE01022
name: Empty Error Namespace
severity: error
---

### Rule 1.22 — Empty Error Namespace
`PGE01022`

**Statement:** A `{!}` error namespace must contain at least one `[.]` error leaf. An error namespace with no leaves is a compile error.
**Rationale:** An error namespace exists to define catchable error types. Without any leaves, it cannot be referenced in `[!]` handlers and serves no purpose. The EBNF requires at least one leaf line.
**Detection:** The compiler checks that each `{!}` block contains at least one `[.]` error leaf line. Comment-only and metadata-only blocks still trigger this error.

**VALID:**
```polyglot
[ ] ✓ error namespace with leaves
{!} !Validation
   [.] .Empty#Error
   [.] .TooLong#Error
```

**INVALID:**
```polyglot
[ ] ✗ PGE01022 — no error leaves
{!} !EmptyErrors
```

```polyglot
[ ] ✗ PGE01022 — metadata-only is still empty
{!} !Placeholder
   [%] .description << "no errors defined"
```

**Diagnostic:** "Empty error namespace `!Name` — requires at least one `.Error` leaf"
