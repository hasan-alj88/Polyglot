---
rule: "3.9"
code: PGE-309
name: Nested Expand Without Collect
severity: error
---

### Rule 3.9 — Nested Expand Without Collect
`PGE-309`

**Statement:** Every `~ForEach` expand operator must have a corresponding collector (`*Into`, `*Agg`, or `*Ignore`) within its own scope. A nested `~ForEach` inside another `~ForEach` without an intermediate collector for the inner expand is a compile error. If the inner expand's items are intentionally discarded, an explicit ignore collector must be used.
**Rationale:** Expand/collect scopes do not nest transparently — inner items cannot flow upward to an outer collector. Without a paired collector, inner items have no defined destination. Requiring explicit pairing (including explicit ignore) prevents silent data loss and makes intent clear.
**Detection:** The compiler tracks expand/collect scope depth. When a `~ForEach` is encountered, it opens a scope that must be closed by a corresponding collector before the scope exits or another expand begins at the same level. An unpaired expand triggers PGE-309.

**See also:** PGE-304 (section-boundary pairing for `[p]`/`[*]`), PGE-307 (expand operator input mismatch), PGE-308 (collect operator IO mismatch)

**VALID:**
```polyglot
[ ] ✓ nested expand with inner collector
[p] ~ForEach.Array
   [~] <Array << $outerList
   [~] >item >> $outerItem

   [p] ~ForEach.Array
      [~] <Array << $outerItem
      [~] >item >> $innerItem

      [r] =Transform
         [=] <input << $innerItem
         [=] >output >> $transformed

      [r] *Into.Array
         [*] <item << $transformed
         [*] >Array >> $innerResult

   [r] *Into.Array
      [*] <item << $innerResult
      [*] >Array >> $outerResult
```

```polyglot
[ ] ✓ inner results intentionally discarded with ignore collector
[p] ~ForEach.Array
   [~] <Array << $files
   [~] >item >> $file

   [p] ~ForEach.Array
      [~] <Array << $file
      [~] >item >> $line

      [b] =LogLine
         [=] <msg << $line

      [r] *Ignore

   [r] *Into.Array
      [*] <item << $file
      [*] >Array >> $processed
```

**INVALID:**
```polyglot
[ ] ✗ PGE-309 — inner expand has no collector
[p] ~ForEach.Array
   [~] <Array << $outerList
   [~] >item >> $outerItem

   [p] ~ForEach.Array                        [ ] ✗ PGE-309 — no collector for inner expand
      [~] <Array << $outerItem
      [~] >item >> $innerItem

      [r] =Process
         [=] <input << $innerItem

   [r] *Into.Array
      [*] <item << $outerItem
      [*] >Array >> $result
```

**Open point:** None.

### See Also

- [[concepts/collections/expand|Expand]] — documents nested expand/collect pairing requirement (references PGE-309)
- [[concepts/collections/examples|Collection Examples]] — error table mapping PGE-309 to example scenarios
