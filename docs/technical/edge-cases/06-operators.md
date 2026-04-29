---
audience: design
type: reference
updated: 2026-04-17
---

<!-- @edge-cases/INDEX -->

## 6. Operators (S6)

### EC-6.1: All four assignment operators

<!-- @u:operators -->
<!-- @c:variable-lifecycle -->
**What it tests:** Each operator used in its correct context. See [[operators]], [[variable-lifecycle]].

```aljam3
[.] .name#string <~ "default"
(-) >count#int ~> 0
[-] $x#int << 42
(-) >item >> $result
```

### EC-6.2: All comparison operators

**EBNF:** `comparison_op ::= "=?" | ">?" | "<?" | ">=?" | "<=?" | "=!?"`

**What it tests:** Each comparison in a conditional.

```aljam3
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

```aljam3
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

```aljam3
[ ] ✗ PGE04010 — raw multiplication
[-] $total#int << $price * $quantity          [ ] ✗ use -Math.Multiply

[ ] ✗ PGE04010 — raw division
[-] $avg#float << $sum / $count               [ ] ✗ use -Math.Divide

[ ] ✗ PGE04010 — raw subtraction
[-] $diff#int << $a - $b                      [ ] ✗ use -Math.Subtract

[ ] ✗ PGE04010 — raw addition
[-] $total#int << $price + $tax               [ ] ✗ use -Math.Add
```

```aljam3
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

```aljam3
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

```aljam3
[ ] ✓ fallback on pipeline call — pipeline can fail
[-] -File.Text.Read
   (-) <path << $configPath
   (-) >content >> $data
      (<) !< "/default/path.txt"

[ ] ✓ data load with permission-mediated file access
[ ] Each {_} in the fallback chain is independently content-hashed
{_} _PrimaryConfig
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/config/config.json"
   [.] .path "/config/config.json"
   [.] .format #JSON

{_} _DefaultConfig
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/config/defaults.json"
   [.] .path "/config/defaults.json"
   [.] .format #JSON

[ ] ✓ — if the pipeline can fail, add fallback chain:
[#] $config#Settings !< -Json.LoadFile(_PrimaryConfig) !< -Json.LoadFile(_DefaultConfig) !< $hardcodedConfig
```

```aljam3
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

### EC-6.6: Range token `?[` vs conditional `[?]` (X.45)

<!-- @u:ebnf/06-operators -->
**EBNF ref:** `range_open ::= "?[" | "?("` (§6.3), `control_flow_elem ::= "[?]"` (§5.1)

**What it tests:** The `?[` range token and `[?]` conditional block element share `?` and `[` characters. The lexer disambiguates positionally: `[?]` is a three-character block element at line start; `?[`/`?(` are two-character range tokens in expression context; comparison operators consume `?` greedily.

**Decision:** Accept — lexer context disambiguates; no grammar change needed.

```aljam3
[ ] ✓ Range in assignment — ?[ is mid-expression after value_expr
[-] $ok#bool << $score ?[60, 100]

[ ] ✓ Conditional — [?] is line-start block element
[?] $score >=? 60
   [-] $grade << "pass"

[ ] ✓ Greedy operator parsing — =? consumed as one comparison token
[?] $x =? $y
   [-] $msg << "equal"
[ ] =?[ would tokenize as =? + [ — the ? is consumed by the comparison operator
[ ] and cannot start a ?[ range token
```
