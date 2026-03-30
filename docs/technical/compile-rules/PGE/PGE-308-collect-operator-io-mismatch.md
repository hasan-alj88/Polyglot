---
rule: "3.8"
code: PGE-308
name: Collect Operator IO Mismatch
severity: error
---

### Rule 3.8 — Collect Operator IO Mismatch
`PGE-308`

**Statement:** Each collect operator (`*Into.*`, `*Agg.*`) requires specific IO inputs and outputs. If the provided IO does not match the operator's signature, PGE-308 fires.
**Rationale:** Collect operators aggregate items into specific shapes. Wrong IO (e.g., providing `<string` to `*Agg.Sum` which expects `<number`) would produce type errors or undefined aggregation behavior. Compile-time validation ensures correctness.
**Detection:** The compiler checks the `[*]` IO lines under the collect operator against the required signature:

**`*Into` collectors:**

| Operator | Required Inputs | Required Outputs |
|----------|----------------|-----------------|
| `*Into.Array` | `<item` | `>Array` |
| `*Into.Map` | `<key`, `<value` | `>Map` |
| `*Into.Serial` | `<key`, `<value` | `>Serial` |
| `*Into.Level` | `<key`, `<value` | `>Serial` |

**`*Agg` collectors:**

| Operator | Required Inputs | Required Outputs |
|----------|----------------|-----------------|
| `*Agg.Sum` | `<number` | `>sum` |
| `*Agg.Count` | `<item` | `>count` |
| `*Agg.Average` | `<number` | `>average` |
| `*Agg.Max` | `<number` | `>max` |
| `*Agg.Min` | `<number` | `>min` |
| `*Agg.Concatenate` | `<string` | `>result` |

Missing, extra, or misnamed IO lines fire PGE-308.

**See also:** PGE-307 (expand operator input mismatch), PGE-306 (race collector type homogeneity)

**VALID:**
```polyglot
[ ] ✓ correct IO for *Into.Array
[p] *Into.Array
   [*] <item << $doubled
   [*] >Array >> $DoubledNumbers
```

```polyglot
[ ] ✓ correct IO for *Agg.Sum
[p] *Agg.Sum
   [*] <number << $doubled
   [*] >sum >> $TotalSum
```

```polyglot
[ ] ✓ correct IO for *Into.Serial
[r] *Into.Serial
   [*] <key << $k
   [*] <value << $v
   [*] >Serial >> $result
```

**INVALID:**
```polyglot
[ ] ✗ PGE-308 — *Into.Serial missing <value input
[r] *Into.Serial
   [*] <item << $v                    [ ] ✗ PGE-308 — expected <key and <value, got <item
   [*] >Serial >> $result
```

```polyglot
[ ] ✗ PGE-308 — *Agg.Sum wrong input name
[p] *Agg.Sum
   [*] <string << $text               [ ] ✗ PGE-308 — expected <number, got <string
   [*] >sum >> $total
```

**Open point:** None.

### See Also

- [[concepts/collections/expand|Expand]] — documents collect operator IO contract (references PGE-308)
- [[concepts/collections/examples|Collection Examples]] — error table mapping PGE-308 to example scenarios
