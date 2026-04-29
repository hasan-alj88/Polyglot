---
audience: developer
rule: "3.12"
code: PGE03011
name: Orphaned Expand IO Marker
severity: error
---

# Rule 3.12 — Orphaned Expand IO Marker
`PGE03011`

<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->

**Statement:** The `(=)` block element marker is only valid within the scope of a `=ForEach` expand invocation. A `(=)` line outside any expand context is a compile error.
**Rationale:** `(=)` declares expand IO — the input collection being iterated and the per-item variable produced. Without an enclosing expand operator, these declarations have no meaning and cannot be wired.
**Detection:** The compiler tracks expand scope depth. When a `(=)` line is encountered at scope depth zero (no enclosing expand), PGE03011 fires.

**See also:** PGE03010 (collector without expand), PGE03007 (expand operator input mismatch)

**VALID:**
```aljam3
[ ] ✓ (=) inside expand scope
[=] =ForEach.Array
   (=) <Array << $items
   (=) >item >> $item
   [-] -Process
      (-) <data << $item
```

**INVALID:**
```aljam3
[ ] ✗ PGE03011 — (=) outside any expand context
(=) <Array << $items
(=) >item >> $item
[-] -Process
   (-) <data << $item
```

**Diagnostic:** "`(=)` expand IO marker outside expand scope — must be inside a `=ForEach` invocation"
