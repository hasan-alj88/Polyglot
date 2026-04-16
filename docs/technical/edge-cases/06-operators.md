---
audience: designer
type: reference
updated: 2026-04-16
---

<!-- @edge-cases/INDEX -->

## 6. Operators (S6)

### EC-6.1: All four assignment operators

<!-- @u:operators -->
<!-- @c:variable-lifecycle -->
**What it tests:** Each operator used in its correct context. See [[operators]], [[variable-lifecycle]].

```polyglot
[.] .name#string <~ "default"
(-) >count#int ~> 0
[-] $x#int << 42
(-) >item >> $result
```

### EC-6.2: All comparison operators

**EBNF:** `comparison_op ::= "=?" | ">?" | "<?" | ">=?" | "<=?" | "=!?"`

**What it tests:** Each comparison in a conditional.

```polyglot
[?] $a =? 0
[?] $b >? 10
[?] $c <? 5
[?] $d >=? 100
[?] $e <=? -1
[?] $f =!? ""
```

### EC-6.3: Range operators

**EBNF:** `range_expr ::= value_expr range_open value_expr ',' value_expr range_close`

**What it tests:** All four range combinations — mixing `[` (inclusive) and `(` (exclusive) on each bound. Mathematical interval notation.

```polyglot
[ ] Inclusive-inclusive: 1 <= val <= 10
[?] $val ?[1,10]
[ ] Exclusive-exclusive: 0 < val < 100
[?] $val ?(0,100)
[ ] Inclusive-exclusive: 1 <= val < 10
[?] $val ?[1,10)
[ ] Exclusive-inclusive: 0 < val <= 10
[?] $val ?(0,10]
```

### EC-6.4: Arithmetic in assignment (INVALID)

<!-- @u:compile-rules/PGE/PGE04010-invalid-arithmetic-operator -->
**What it tests:** Raw arithmetic tokens (`+`, `-`, `*`, `/`) are compile errors. Arithmetic uses `-Math.*` pglib pipelines. See [[PGE04010|PGE04010]].

```polyglot
[ ] ✗ PGE04010 — raw multiplication
[-] $total#int << $price * $quantity          [ ] ✗ use -Math.Multiply

[ ] ✗ PGE04010 — raw division
[-] $avg#float << $sum / $count               [ ] ✗ use -Math.Divide

[ ] ✗ PGE04010 — raw subtraction
[-] $diff#int << $a - $b                      [ ] ✗ use -Math.Subtract

[ ] ✗ PGE04010 — raw addition
[-] $total#int << $price + $tax               [ ] ✗ use -Math.Add
```

```polyglot
[ ] ✓ arithmetic through pglib pipelines
[-] -Math.Multiply
   (-) << $price
   (-) << $quantity
   (-) >> $total

[ ] ✓ string interpolation is NOT arithmetic — still valid
[-] $name#string << "{$first} {$last}"
```

### EC-6.5: Fallback operators in non-error context (X.33)

**EBNF ref:** `assignment_op` includes `fallback_push_left` (`!<`) and `fallback_push_right` (`!>`), used by multiple productions.

**What it tests:** Fallback operators require a failable source (pipeline call). Using `!<`/`!>` with a literal or variable is PGE07008. A fallback chain must terminate at a non-failable value or PGE07009 fires.

```polyglot
[ ] ✗ PGE07008 — schema property with fallback on literal
{#} #Sensor
   [#] %##Depth.Max !< 3

[ ] ✗ PGE07008 — expand IO with fallback on variable
[=] =ForEach.Array
   (=) <Array !< $items
   (=) >item >> $val

[ ] ✗ PGE07008 — collect IO input with fallback on variable
[-] *Into.Array
   (*) <item !< $val
   (*) >Array >> $result

[ ] ✗ PGE07008 — value field definition with fallback on literal
{#} #User
   [.] .name#string !< "anonymous"

[ ] ✗ PGE07008 — metadata field with fallback on literal
[%] .description !< "My pipeline"
```

```polyglot
[ ] ✓ fallback on pipeline call — pipeline can fail
[-] -File.Text.Read
   (-) <path << $configPath
   (-) >content >> $data
      (<) !< "/default/path.txt"

[ ] ✓ data load with inline pipeline fallback
[#] $config#Settings << -Json.LoadFile"/config.json"
[ ] ✓ — if the pipeline can fail, add fallback chain:
[#] $config#Settings !< -Json.LoadFile"/config.json" !< -Json.LoadFile"/defaults.json" !< $hardcodedConfig
```

```polyglot
[ ] ✓ fallback chain terminates at literal
[-] -Fetch.Config
   (-) <url << $primary
   (-) >config >> $cfg
      (<) !< -Fetch.Config"/backup" !< "no config"

[ ] ✗ PGE07009 — fallback chain ends at pipeline call
[-] -Fetch.Config
   (-) <url << $primary
   (-) >config >> $cfg
      (<) !< -Fetch.Config"/backup" !< -Fetch.Config"/last-resort"
```
