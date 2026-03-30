---
audience: developer
type: reference
updated: 2026-03-30
---

<!-- @edge-cases/INDEX -->

## 11. Control Flow (S11)

### EC-11.1: Conditional chain — multiple [?] branches

<!-- @blocks:Control Flow -->
**EBNF:** `conditional_line ::= "[?]" comparison_expr`

**What it tests:** Sequential `[?]` blocks acting as switch-case. See [[blocks#Control Flow]].

```polyglot
{#} #Status
   [.] .Ok
   [.] .Warn
   [.] .Fail

[ ] ...in pipeline execution...
[?] $status =? #Status.Ok
   [r] >result << "Success"

[?] $status =? #Status.Warn
   [r] >result << "Warning"

[?] $status =? #Status.Fail
   [r] >result << "Failure"
```

### EC-11.2: Error block — scoped under [r], not pipeline-level

**What it tests:** `[!]` indentation must be under the specific `[r]` call, after its `[=]` IO lines. See [[concepts/pipelines/error-handling#Error Handling]].

```polyglot
[ ] CORRECT — error scoped under call
[r] =SomeCall
[=] <in << $val
[=] >out >> $result
   [!] !Some.Error
      [r] $result << "fallback"

[ ] WRONG — error at pipeline level (NOT valid)
```

### EC-11.3: Logical operators in conditionals

**EBNF:** `logical_and ::= "[&]" comparison_expr` etc.

**What it tests:** Compound conditions using `[&]`, `[|]`, `[-]`, `[^]`.

```polyglot
[ ] AND: both conditions must be true
[?] $age >=? 18
[&] $verified =? #Boolean.True
   [r] $access << #AccessLevel.Granted

[ ] OR: either condition
[?] $role =? #Role.Admin
[|] $role =? #Role.Superuser
   [r] $elevated << #Boolean.True

[ ] Negation: insert ! before ? in comparison operator
[ ] <!? means "not less than", >=!? means "not greater-or-equal"
[?] $banned =? #Boolean.False
[&] $age <!? 13
   [r] $allowed << #Boolean.True
```

### EC-11.4: Match — numeric value mapping

**EBNF:** `match_line ::= "[r]" value_expr ">>" assign_target { match_arm }`

**What it tests:** Match syntax as sugar for repeated `[?]` conditional assignment. Source must be Final, arms are assignment-only.

```polyglot
[ ] Match: maps $code to $status via value >> result arms
[r] $code >> $status#string
   [?] 200 >> "ok"
   [?] 404 >> "not_found"
   [?] 500 >> "error"
   [?] * >> "unknown"
```

### EC-11.5: Match — enum exhaustive (no wildcard needed)

**EBNF:** `match_arm ::= "[?]" match_value ">>" value_expr`

**What it tests:** Enum match with all variants listed — no `*` required, same as verbose form (PGE06002).

```polyglot
[ ] All #Direction variants covered — no * needed
[r] $dir >> $label#string
   [?] #Direction.North >> "N"
   [?] #Direction.South >> "S"
   [?] #Direction.East >> "E"
   [?] #Direction.West >> "W"
```

### EC-11.6: Match header without arms — plain assignment

**EBNF:** `run_line ::= "[r]" exec_expr`

**What it tests:** `[r] $x >> $y` without indented `[?]` children is a plain assignment, not a match header.

```polyglot
[ ] No [?] children — this is a plain assignment, not a match
[r] $source >> $target#string
```

### EC-11.7: Wildcard-only match — tautological

**EBNF ref:** `match_line ::= "[r]" value_expr ">>" assign_target { match_arm }`
**What it tests:** Match with only `[?] *` — always produces the same result. PGE06014 fires.

```polyglot
[ ] ✗ PGE06014 — wildcard-only match is tautological
[r] $code >> $msg#string
   [?] * >> "always this"
```

### EC-11.8: Variable as match arm value — must be Final

**EBNF ref:** `match_value ::= literal | identifier | cross_pkg_enum`
**What it tests:** Runtime variable in match arm — valid but must be Final state. If the variable could be in Failed state without a fallback, PGE fires.

```polyglot
[ ] ✓ Final variable as match value
[r] $threshold#int << 100
[r] $input >> $label#string
   [?] $threshold >> "at threshold"
   [?] * >> "other"
```

### EC-11.9: Pipeline identifier in comparison — non-value type

**EBNF ref:** `comparison_expr ::= value_expr comparison_op value_expr`
**What it tests:** Pipeline identifiers are not value types. PGE04024 fires.

```polyglot
[ ] ✗ PGE04024 — pipeline identifiers cannot be compared
[?] =Pipeline.A =? =Pipeline.B
   [r] $same << #Boolean.True
```
