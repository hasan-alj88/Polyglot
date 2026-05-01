---
audience: developer
rule: "3.8"
code: PGE03008
name: Collect Operator IO Mismatch
severity: error
---

# Rule 3.8 — Collect Operator IO Mismatch
`PGE03008`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/operators -->

**Statement:** Each collect operator (`*Into.*`, `*Agg.*`) requires specific IO inputs and outputs. If the provided IO does not match the operator's signature, PGE03008 fires.
**Rationale:** Collect operators aggregate items into specific shapes. Wrong IO (e.g., providing `<string` to `*Agg.Sum` which expects `<number`) would produce type errors or undefined aggregation behavior. Compile-time validation ensures correctness.
**Detection:** The compiler checks the `(*)` IO lines under the collect operator against the required signature:

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

Missing, extra, or misnamed IO lines fire PGE03008.

**See also:** PGE03007 (expand operator input mismatch), PGE03006 (race collector type homogeneity)

**VALID:**
```aljam3
[ ] ✓ correct IO for *Into.Array
[=] *Into.Array
   (*) <item << $doubled
   (*) >Array >> $DoubledNumbers
```

```aljam3
[ ] ✓ correct IO for *Agg.Sum
[=] *Agg.Sum
   (*) <number << $doubled
   (*) >sum >> $TotalSum
```

```aljam3
[ ] ✓ correct IO for *Into.Serial
[-] *Into.Serial
   (*) <key << $k
   (*) <value << $v
   (*) >Serial >> $result
```

**INVALID:**
```aljam3
[ ] ✗ PGE03008 — *Into.Serial missing <value input
[-] *Into.Serial
   (*) <item << $v                    [ ] ✗ PGE03008 — expected <key and <value, got <item
   (*) >Serial >> $result
```

```aljam3
[ ] ✗ PGE03008 — *Agg.Sum wrong input name
[=] *Agg.Sum
   (*) <string << $text               [ ] ✗ PGE03008 — expected <number, got <string
   (*) >sum >> $total
```

**Open point:** None.

## See Also

- [[concepts/collections/expand|Expand]] — documents collect operator IO contract (references PGE03008)
- [[concepts/collections/examples|Collection Examples]] — error table mapping PGE03008 to example scenarios
