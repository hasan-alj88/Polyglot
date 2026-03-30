---
rule: "3.7"
code: PGE-307
name: Expand Operator Input Mismatch
severity: error
---

### Rule 3.7 — Expand Operator Input Mismatch
`PGE-307`

**Statement:** Each expand operator (`~ForEach.*`) requires specific IO inputs and outputs. If the provided IO does not match the operator's signature, PGE-307 fires.
**Rationale:** Expand operators iterate over specific collection shapes. Providing the wrong input type (e.g., a serial to `~ForEach.Array`) would produce undefined iteration behavior. Compile-time validation ensures the data shape matches the operator.
**Detection:** The compiler checks the `[~]` IO lines under the expand operator against the required signature:

| Operator | Required Inputs | Required Outputs |
|----------|----------------|-----------------|
| `~ForEach.Array` | `<Array` | `>item` |
| `~ForEach.Array.Enumerate` | `<Array` | `>index`, `>item` |
| `~ForEach.Map` | `<Map` | `>key`, `>item` |
| `~ForEach.Serial` | `<Serial` | `>key`, `>item` |
| `~ForEach.Level` | `<level` | `>key`, `>item` |

Missing or extra IO lines fire PGE-307.

**See also:** PGE-308 (collect operator IO mismatch), PGE-302 (parallel output must be collected)

**VALID:**
```polyglot
[ ] ✓ correct IO for ~ForEach.Array
[p] ~ForEach.Array
   [~] <Array << $InputNumbers
   [~] >item >> $num
```

```polyglot
[ ] ✓ correct IO for ~ForEach.Array.Enumerate
[p] ~ForEach.Array.Enumerate
   [~] <Array << $InputNumbers
   [~] >index >> $idx
   [~] >item >> $num
```

```polyglot
[ ] ✓ correct IO for ~ForEach.Serial
[r] ~ForEach.Serial
   [~] <Serial << $config
   [~] >key >> $k
   [~] >item >> $v
```

**INVALID:**
```polyglot
[ ] ✗ PGE-307 — ~ForEach.Array missing <Array input
[p] ~ForEach.Array
   [~] <Serial << $data              [ ] ✗ PGE-307 — expected <Array, got <Serial
   [~] >item >> $num
```

```polyglot
[ ] ✗ PGE-307 — ~ForEach.Array.Enumerate missing >index output
[p] ~ForEach.Array.Enumerate
   [~] <Array << $InputNumbers
   [~] >item >> $num                  [ ] ✗ PGE-307 — missing required >index output
```

**Open point:** None.

### See Also

- [[concepts/collections/expand|Expand]] — documents expand operator IO signature requirements (references PGE-307)
- [[concepts/collections/examples|Collection Examples]] — error table mapping PGE-307 to example scenarios
