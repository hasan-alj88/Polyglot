---
audience: developer
rule: "4.10"
code: PGE04010
name: Invalid Arithmetic Operator
severity: error
---

# Rule 4.10 — Invalid Arithmetic Operator
`PGE04010`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->

**Statement:** Raw arithmetic tokens (`+`, `-`, `*`, `/`, `%`) are not valid operators in Aljam3 Code. Arithmetic is performed through `-Math.*` jm3lib pipelines. The compiler rejects arithmetic tokens and suggests the equivalent jm3lib pipeline.
**Rationale:** Aljam3 Code uses pipelines for all operations, including arithmetic. Raw operators would bypass the pipeline execution model (trigger → queue → wrapper → body), break the metadata tree (`%-` tracking), and conflict with existing operator meanings (`*` is a collector prefix, `-` has no defined role).
**Detection:** The parser encounters an arithmetic token in an expression context (not inside a string literal). PGE04010 fires with a suggestion message pointing to the `-Math.*` equivalent.

**Suggestions:**

| Token | Suggestion |
|-------|-----------|
| `+` | Use `-Math.Add` |
| `-` | Use `-Math.Subtract` or `-Math.Negate` |
| `*` | Use `-Math.Multiply` |
| `/` | Use `-Math.Divide` |
| `%` | Use `-Math.Modulo` |

**See also:** [-Math jm3lib](../../../user/jm3lib/pipelines/Math.md)

**VALID:**
```aljam3
[ ] ✓ arithmetic through jm3lib pipelines
[-] -Math.Add
   (-) << $price
   (-) << $tax
   (-) >> $total

[-] -Math.Multiply
   (-) << $quantity
   (-) << $unitPrice
   (-) >> $lineTotal
```

**INVALID:**
```aljam3
[ ] ✗ PGE04010 — raw arithmetic operator
[-] $total << $price + $tax                 [ ] ✗ PGE04010 — use -Math.Add
```

```aljam3
[ ] ✗ PGE04010 — raw arithmetic operator
[-] $result << $a * $b                      [ ] ✗ PGE04010 — use -Math.Multiply
```

**Open point:** None.

## See Also

- [[syntax/types/basic-types|Basic Types]] — regex validation catches invalid literals at compile time
- [[syntax/operators|Operators]] — arithmetic through `-Math.*` jm3lib pipelines
- [[technical/edge-cases/24-datatype-defs|EC-24.2: #Int leading zeros and negative zero]] — references PGE04010
- [[technical/edge-cases/24-datatype-defs|EC-24.4: #Eng exponent]] — references PGE04010
- [[technical/edge-cases/24-datatype-defs|EC-24.5: #KeyString excluded chars]] — references PGE04010
- [[technical/edge-cases/24-datatype-defs|EC-24.6: #NestedKeyString allows dot/colon]] — references PGE04010
