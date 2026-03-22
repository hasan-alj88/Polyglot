---
rule: "4.10"
code: PGE-410
name: Invalid Arithmetic Operator
severity: error
---

### Rule 4.10 — Invalid Arithmetic Operator
`PGE-410`

**Statement:** Raw arithmetic tokens (`+`, `-`, `*`, `/`, `%`) are not valid operators in Polyglot Code. Arithmetic is performed through `=Math.*` stdlib pipelines. The compiler rejects arithmetic tokens and suggests the equivalent stdlib pipeline.
**Rationale:** Polyglot Code uses pipelines for all operations, including arithmetic. Raw operators would bypass the pipeline execution model (trigger → queue → wrapper → body), break the metadata tree (`%=` tracking), and conflict with existing operator meanings (`*` is a collector prefix, `-` has no defined role).
**Detection:** The parser encounters an arithmetic token in an expression context (not inside a string literal). PGE-410 fires with a suggestion message pointing to the `=Math.*` equivalent.

**Suggestions:**

| Token | Suggestion |
|-------|-----------|
| `+` | Use `=Math.Add` |
| `-` | Use `=Math.Subtract` or `=Math.Negate` |
| `*` | Use `=Math.Multiply` |
| `/` | Use `=Math.Divide` |
| `%` | Use `=Math.Modulo` |

**See also:** [=Math stdlib](../../../user/stdlib/pipelines/Math.md)

**VALID:**
```polyglot
[ ] ✓ arithmetic through stdlib pipelines
[r] =Math.Add
   [=] << $price
   [=] << $tax
   [=] >> $total

[r] =Math.Multiply
   [=] << $quantity
   [=] << $unitPrice
   [=] >> $lineTotal
```

**INVALID:**
```polyglot
[ ] ✗ PGE-410 — raw arithmetic operator
[r] $total << $price + $tax                 [ ] ✗ PGE-410 — use =Math.Add
```

```polyglot
[ ] ✗ PGE-410 — raw arithmetic operator
[r] $result << $a * $b                      [ ] ✗ PGE-410 — use =Math.Multiply
```

**Open point:** None.
