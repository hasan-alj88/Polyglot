---
audience: developer
type: reference
updated: 2026-03-30
---

<!-- @edge-cases/INDEX -->

## 6. Operators (S6)

### EC-6.1: All four assignment operators

<!-- @operators -->
<!-- @variable-lifecycle -->
**What it tests:** Each operator used in its correct context. See [[operators]], [[variable-lifecycle]].

```polyglot
[.] .name#string <~ "default"
[=] >count#int ~> 0
[r] $x#int << 42
[=] >item >> $result
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

### EC-6.4: Arithmetic in assignment

**EBNF:** `arithmetic_expr ::= value_expr arithmetic_op value_expr`

**What it tests:** `+`, `-`, `*`, `/` used in assignments.

```polyglot
[r] $total#int << $price * $quantity
[r] $name#string << "{$first} {$last}"
[r] $avg#float << $sum / $count
[r] $diff#int << $a - $b
```
