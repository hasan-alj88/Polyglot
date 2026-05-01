---
audience: developer
rule: "3.9"
code: PGE03009
name: Nested Expand Without Collect
severity: error
---

# Rule 3.9 — Nested Expand Without Collect
`PGE03009`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->

**Statement:** Every `=ForEach` expand operator must have a corresponding collector (`*Into`, `*Agg`, or `*Ignore`) within its own scope. A nested `=ForEach` inside another `=ForEach` without an intermediate collector for the inner expand is a compile error. If the inner expand's items are intentionally discarded, an explicit ignore collector must be used.
**Rationale:** Expand/collect scopes do not nest transparently — inner items cannot flow upward to an outer collector. Without a paired collector, inner items have no defined destination. Requiring explicit pairing (including explicit ignore) prevents silent data loss and makes intent clear.
**Detection:** The compiler tracks expand/collect scope depth. When a `=ForEach` is encountered, it opens a scope that must be closed by a corresponding collector before the scope exits or another expand begins at the same level. An unpaired expand triggers PGE03009.

**See also:** PGE03004 (section-boundary pairing for `[=]`/`(*)`), PGE03007 (expand operator input mismatch), PGE03008 (collect operator IO mismatch)

**VALID:**
```aljam3
[ ] ✓ nested expand with inner collector
[=] =ForEach.Array
   (=) <Array << $outerList
   (=) >item >> $outerItem

   [=] =ForEach.Array
      (=) <Array << $outerItem
      (=) >item >> $innerItem

      [-] -Transform
         (-) <input << $innerItem
         (-) >output >> $transformed

      [-] *Into.Array
         (*) <item << $transformed
         (*) >Array >> $innerResult

   [-] *Into.Array
      (*) <item << $innerResult
      (*) >Array >> $outerResult
```

```aljam3
[ ] ✓ inner results intentionally discarded with ignore collector
[=] =ForEach.Array
   (=) <Array << $files
   (=) >item >> $file

   [=] =ForEach.Array
      (=) <Array << $file
      (=) >item >> $line

      [b] -LogLine
         (-) <msg << $line

      [-] *Ignore

   [-] *Into.Array
      (*) <item << $file
      (*) >Array >> $processed
```

**INVALID:**
```aljam3
[ ] ✗ PGE03009 — inner expand has no collector
[=] =ForEach.Array
   (=) <Array << $outerList
   (=) >item >> $outerItem

   [=] =ForEach.Array                        [ ] ✗ PGE03009 — no collector for inner expand
      (=) <Array << $outerItem
      (=) >item >> $innerItem

      [-] -Process
         (-) <input << $innerItem

   [-] *Into.Array
      (*) <item << $outerItem
      (*) >Array >> $result
```

**Open point:** None.

## See Also

- [[concepts/collections/expand|Expand]] — documents nested expand/collect pairing requirement (references PGE03009)
- [[concepts/collections/examples|Collection Examples]] — error table mapping PGE03009 to example scenarios
