---
rule: "4.2w"
code: PGW04002
name: Leading Zeros in Literal
severity: warning
---

### Rule 4.2w — Leading Zeros in Literal
`PGW04002`

**Statement:** An `int` or `float` literal with unnecessary leading zeros produces a warning. Polyglot has no octal notation — `007` is decimal seven, not octal seven — but leading zeros may confuse developers familiar with languages that use `0`-prefixed octal.
**Rationale:** Leading zeros are syntactically valid per the EBNF but can cause confusion. The value is always interpreted as decimal. This warning encourages clearer literal formatting.
**Detection:** The compiler checks if an `int` or `float` literal starts with `0` followed by additional digits (excluding `0.` for floats, which is normal notation).

**VALID:**
```polyglot
[ ] ✓ no leading zeros
[r] $x#int << 7
[r] $y#float << 0.50
[r] $z#int << 0
```

**WARNING:**
```polyglot
[ ] ⚠ PGW04002 — leading zeros in int literal
[r] $x#int << 007

[ ] ⚠ PGW04002 — leading zeros in float literal
[r] $y#float << 00.50

[ ] ⚠ PGW04002 — multiple leading zeros
[r] $z#int << 0042
```

**Diagnostic:** "Literal `007` has leading zeros — Polyglot uses decimal only (not octal)"
