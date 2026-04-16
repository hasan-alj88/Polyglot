---
audience: designer
type: reference
updated: 2026-04-16
---

<!-- @edge-cases/INDEX -->

## 11. Control Flow (S11)

### EC-11.1: Conditional chain — multiple [?] branches

<!-- @u:blocks:Control Flow -->
**EBNF:** `conditional_line ::= "[?]" comparison_expr`

**What it tests:** Sequential `[?]` blocks acting as switch-case. See [[blocks#Control Flow]].

```polyglot
{#} #Status
   [.] .Ok
   [.] .Warn
   [.] .Fail

[ ] ...in pipeline execution...
[?] $status =? #Status.Ok
   [-] >result << "Success"

[?] $status =? #Status.Warn
   [-] >result << "Warning"

[?] $status =? #Status.Fail
   [-] >result << "Failure"
```

### EC-11.2: Error block — scoped under [-], not pipeline-level

**What it tests:** `[!]` indentation must be under the specific `[-]` call, after its `(-)` IO lines. See [[concepts/pipelines/error-handling#Error Handling]].

```polyglot
[ ] CORRECT — error scoped under call
[-] -SomeCall
(-) <in << $val
(-) >out >> $result
   [!] !Some.Error
      [-] $result << "fallback"

[ ] WRONG — error at pipeline level (NOT valid)
```

### EC-11.3: Logical operators in conditionals

**EBNF:** `logical_and ::= "[&]" comparison_expr` etc.

**What it tests:** Compound conditions using `[&]`, `[+]`, `[-]`, `[^]`.

```polyglot
[ ] AND: both conditions must be true
[?] $age >=? 18
[&] $verified =? #Boolean.True
   [-] $access << #AccessLevel.Granted

[ ] OR: either condition
[?] $role =? #Role.Admin
[+] $role =? #Role.Superuser
   [-] $elevated << #Boolean.True

[ ] Negation: insert ! before ? in comparison operator
[ ] <!? means "not less than", >=!? means "not greater-or-equal"
[?] $banned =? #Boolean.False
[&] $age <!? 13
   [-] $allowed << #Boolean.True
```

### EC-11.4: Match — numeric value mapping

**EBNF:** `match_line ::= "[-]" value_expr ">>" assign_target { match_arm }`

**What it tests:** Match syntax as sugar for repeated `[?]` conditional assignment. Source must be Final, arms are assignment-only.

```polyglot
[ ] Match: maps $code to $status via value >> result arms
[-] $code >> $status#string
   [?] 200 >> "ok"
   [?] 404 >> "not_found"
   [?] 500 >> "error"
   [?] *? >> "unknown"
```

### EC-11.5: Match — enum exhaustive (no wildcard needed)

**EBNF:** `match_arm ::= "[?]" match_value ">>" value_expr`

**What it tests:** Enum match with all variants listed — no `*` required, same as verbose form (PGE06002).

```polyglot
[ ] All #Direction variants covered — no *? needed
[-] $dir >> $label#string
   [?] #Direction.North >> "N"
   [?] #Direction.South >> "S"
   [?] #Direction.East >> "E"
   [?] #Direction.West >> "W"
```

### EC-11.6: Match header without arms — plain assignment

**EBNF:** `run_line ::= "[-]" exec_expr`

**What it tests:** `[-] $x >> $y` without indented `[?]` children is a plain assignment, not a match header.

```polyglot
[ ] No [?] children — this is a plain assignment, not a match
[-] $source >> $target#string
```

### EC-11.7: Wildcard-only match — tautological

**EBNF ref:** `match_line ::= "[-]" value_expr ">>" assign_target { match_arm }`
**What it tests:** Match with only `[?] *?` — always produces the same result. PGE06014 fires.

```polyglot
[ ] ✗ PGE06014 — wildcard-only match is tautological
[-] $code >> $msg#string
   [?] *? >> "always this"
```

### EC-11.8: Variable as match arm value — must be Final

**EBNF ref:** `match_value ::= literal | identifier`
**What it tests:** Runtime variable in match arm — valid but must be Final state. If the variable could be in Failed state without a fallback, PGE fires.

```polyglot
[ ] ✓ Final variable as match value
[-] $threshold#int << 100
[-] $input >> $label#string
   [?] $threshold >> "at threshold"
   [?] *? >> "other"
```

### EC-11.9: Pipeline identifier in comparison — non-value type

**EBNF ref:** `comparison_expr ::= value_expr comparison_op value_expr`
**What it tests:** Pipeline identifiers are not value types. PGE04024 fires.

```polyglot
[ ] ✗ PGE04024 — pipeline identifiers cannot be compared
[?] -Pipeline.A =? -Pipeline.B
   [-] $same << #Boolean.True
```

### EC-11.10: Cross-package enum in match — via identifier

**EBNF ref:** `match_value ::= literal | identifier`
**What it tests:** `cross_pkg_enum` (`@alias#DataName.EnumField`) is matched through `identifier` — it was removed from `match_value` as a separate alternative because `identifier` already covers it via `data_id` and `package_id` in §3.1. See [[ebnf/03-identifiers#3.4 Cross-Package References]].

```polyglot
[ ] ✓ Cross-package enum matched through identifier
[-] $status >> $label#string
   [?] @auth#Role.Admin >> "admin"
   [?] @auth#Role.User >> "user"
   [?] *? >> "unknown"
```
