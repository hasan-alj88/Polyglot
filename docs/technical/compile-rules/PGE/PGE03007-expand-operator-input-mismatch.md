---
audience: developer
rule: "3.7"
code: PGE03007
name: Expand Operator Input Mismatch
severity: error
---

# Rule 3.7 — Expand Operator Input Mismatch
`PGE03007`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->

**Statement:** Each expand operator (`=ForEach.*`) requires specific IO inputs and outputs. If the provided IO does not match the operator's signature, PGE03007 fires.
**Rationale:** Expand operators iterate over specific collection shapes. Providing the wrong input type (e.g., a serial to `=ForEach.Array`) would produce undefined iteration behavior. Compile-time validation ensures the data shape matches the operator.
**Detection:** The compiler checks the `(=)` IO lines under the expand operator against the required signature:

| Operator | Required Inputs | Required Outputs |
|----------|----------------|-----------------|
| `=ForEach.Array` | `<Array` | `>item` |
| `=ForEach.Array.Enumerate` | `<Array` | `>index`, `>item` |
| `=ForEach.Map` | `<Map` | `>key`, `>item` |
| `=ForEach.Serial` | `<Serial` | `>key`, `>item` |
| `=ForEach.Level` | `<level` | `>key`, `>item` |

Missing or extra IO lines fire PGE03007.

**See also:** PGE03008 (collect operator IO mismatch), PGE03002 (parallel output must be collected)

**VALID:**
```aljam3
[ ] ✓ correct IO for =ForEach.Array
[=] =ForEach.Array
   (=) <Array << $InputNumbers
   (=) >item >> $num
```

```aljam3
[ ] ✓ correct IO for =ForEach.Array.Enumerate
[=] =ForEach.Array.Enumerate
   (=) <Array << $InputNumbers
   (=) >index >> $idx
   (=) >item >> $num
```

```aljam3
[ ] ✓ correct IO for =ForEach.Serial
[-] =ForEach.Serial
   (=) <Serial << $config
   (=) >key >> $k
   (=) >item >> $v
```

**INVALID:**
```aljam3
[ ] ✗ PGE03007 — =ForEach.Array missing <Array input
[=] =ForEach.Array
   (=) <Serial << $data              [ ] ✗ PGE03007 — expected <Array, got <Serial
   (=) >item >> $num
```

```aljam3
[ ] ✗ PGE03007 — =ForEach.Array.Enumerate missing >index output
[=] =ForEach.Array.Enumerate
   (=) <Array << $InputNumbers
   (=) >item >> $num                  [ ] ✗ PGE03007 — missing required >index output
```

**Open point:** None.

## See Also

- [[concepts/collections/expand|Expand]] — documents expand operator IO signature requirements (references PGE03007)
- [[concepts/collections/examples|Collection Examples]] — error table mapping PGE03007 to example scenarios
