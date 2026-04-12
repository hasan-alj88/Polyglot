---
audience: pg-coder
type: spec
updated: 2026-03-30
---

<!-- @concepts/collections/INDEX -->

## Example: Expand, Transform, and Collect

Expand an array of integers in parallel, double each value, collect the doubled values into a new array, and compute the sum:

```polyglot
...
{-} -DoubleAndSum
[ ] Triggers, queue config, and wrapper assumed defined
...
[ ] Input: an array of integers
(-) <numbers#array << $InputNumbers
[ ] Output: doubled array and total sum
(-) >doubled#array >> $DoubledNumbers
(-) >total#int >> $TotalSum

[ ] Expand — one mini-pipeline per item, run in parallel
[=] =ForEach.Array.Enumerate
   (=) <Array << $InputNumbers
   (=) >index >> $idx
   (=) >item >> $num

   [ ] Double the number inside mini-pipeline scope
   [-] $doubled#int << $num * 2

   [ ] Collect doubled values back into array (one level up)
   [=] *Into.Array
      (*) <item << $doubled
      (*) >Array >> $DoubledNumbers

   [ ] Also aggregate the sum (one level up)
   [=] *Agg.Sum
      (*) <number << $doubled
      (*) >sum >> $TotalSum
...
```

## Example: Expand Map, Transform, and Collect

Expand a map of ticker->price pairs, multiply each price by 1.1 using `-Math.Multiply`, and collect into a new map:

```polyglot
...
{-} -AdjustPrices
[ ] Triggers, queue config, and wrapper assumed defined
...
[ ] Input: a map of ticker → price
(-) <prices#map:string:float << $InputPrices
[ ] Output: adjusted prices map
(-) >adjusted#map:string:float >> $AdjustedPrices

[ ] Expand — one mini-pipeline per key-value pair
[=] =ForEach.Map
   (=) <Map << $InputPrices
   (=) >key >> $ticker
   (=) >item >> $price

   [ ] Multiply the price by 1.1
   [-] -Math.Multiply
      (-) <a#float << $price
      (-) <b#float << 1.1
      (-) >result#float >> $newPrice

   [ ] Collect back into map (one level up)
   [-] *Into.Map
      (*) <key << $ticker
      (*) <value << $newPrice
      (*) >Map >> $AdjustedPrices
...
```

## Fallback in Expand Context

<!-- @u:errors:Error Fallback Operators -->
When a pipeline call inside an expand scope may error, use `(>) !>` fallback to provide a default value per iteration instead of failing the entire expand:

```polyglot
[=] =ForEach.Array
   (=) <Array << $files
   (=) >item >> $file

   [-] -File.Text.Read
      (-) <path << $file
      (-) >content >> $text
         (>) !> ""

   [-] *Into.Array
      (*) <item << $text
      (*) >Array >> $results
```

If any file fails to read, `$text` gets `""` for that iteration instead of entering the Failed state. The expand continues for all items. See [[errors#Error Fallback Operators]] for the full fallback model.

## See Also

- [[concepts/collections/expand|Expand Operators]] — `=ForEach` operator reference and IO signatures
- [[concepts/collections/collect|Collect Operators]] — `*Into` and `*Agg` collector reference
- [[concepts/pipelines/error-handling|Error Handling]] — `[!]` blocks and `!<` fallback operators

## Compile Rules

Parallel execution, expand/collect, and race collector rules enforced at compile time. See [[compile-rules/PGE/{code}|{code}]] for full definitions.

| Code | Name | Section |
|------|------|---------|
| PGE03001 | No Push Across Parallel Boundaries | Parallel Boundaries |
| PGE03002 | Parallel Output Must Be Collected | Discarding Parallel Output |
| PGE03003 | Pull Isolation Until Collection | Parallel Boundaries |
| PGE03004 | Section-Boundary Pairing | Parallel Boundaries |
| PGE03005 | `[b]` Has No Collectible Output | Discarding Parallel Output |
| PGE03006 | Race Collector Type Homogeneity | Race Collectors |
| PGE03007 | Expand Operator Input Mismatch | Expand Operators |
| PGE03008 | Collect Operator IO Mismatch | Collect Operators |
| PGE03009 | Nested Expand Without Collect | Expand Operators |
| PGE03010 | Collector Without Expand | Collect Operators |
| PGE11002 | Unbounded Collection Nesting | Nested Collection Safety |
| PGW03001 | `[b]` Called Pipeline Has Discarded Outputs | Discarding Parallel Output |
| PGW03002 | Error Handler on Fire-and-Forget | Discarding Parallel Output |
| PGW11003 | Unlimited Depth on User Type | Nested Collection Safety |
