---
audience: designer
type: reference
updated: 2026-03-30
---

<!-- @edge-cases/INDEX -->

## 8. Expressions (S8)

### EC-8.1: Inline data — multiple elements

<!-- @types:Inline Data Shorthand -->
**EBNF:** `inline_data ::= '{' value_expr { ',' value_expr } '}'`

**What it tests:** Non-empty inline data with mixed types. See [[syntax/types/structs#Inline Data Shorthand]].

```polyglot
[r] $nums#array << {1, 2, 3, 4, 5}
[r] $services#array:string << {"AD", "Email", "Slack"}
```

### EC-8.2: Inline data — empty collection

**EBNF:** `inline_data ::= '{' '}'`

**What it tests:** Empty `{}` as valid collection initializer.

```polyglot
[=] >results#array:string ~> {}
```

### EC-8.3: String interpolation

<!-- @types:String Interpolation -->
**EBNF:** `interpolation ::= '{' variable_id '}'`

**What it tests:** Variable interpolation inside string literals using `{$var}` syntax. See [[syntax/types/strings#String Interpolation]].

```polyglot
[r] $msg#string << "Hello {$first} {$last}!"
[r] $path#string << "/users/{$userId}/profile"
[ ] Escaped literal braces
[r] $json#string << "{{\"key\": \"{$val}\"}}"
```

### EC-8.4: Default assignment to discard (INVALID)

<!-- @EBNF:assign_target -->
**EBNF:** `assign_target ::= ... | "$*"` — discard accepts final operators only (`<<`, `>>`).

**What it tests:** `$*` is immediately released — setting a default (`<~`/`~>`) implies the value persists for later override, which is meaningless for a discard. See [[technical/compile-rules/PGE/PGE02010-discard-default-assignment|PGE02010]].

```polyglot
[ ] INVALID — DefaultPushLeft into discard
[r] $*#string <~ "never used"           [ ] ✗ PGE02010 — discard cannot hold a default

[ ] INVALID — DefaultPushRight into discard
[r] =Compute
   [=] >result ~> $*                    [ ] ✗ PGE02010 — discard cannot hold a default

[ ] VALID — PushLeft into discard
[r] $*#string << "discarded"            [ ] ✓ final is the only valid discard operator

[ ] VALID — PushRight into discard
[r] =Compute
   [=] >result >> $*                    [ ] ✓ explicit discard of output
```

### EC-8.5: Self-assignment via output params (INVALID)

<!-- @EBNF:assignment_expr -->
**EBNF:** `assign_target ::= ... | output_param` and `value_expr ::= ... | output_param`

**What it tests:** Same output parameter on both sides of an assignment within the same operation scope — no state change. See [[technical/compile-rules/PGE/PGE08011-self-assignment|PGE08011]].

```polyglot
[ ] INVALID — same output param on both sides
[=] >result#string
[r] >result << >result                  [ ] ✗ PGE08011 — self-assignment, no state change

[ ] VALID — different output ports (cross-port wiring)
[=] >out1#string
[=] >out2#string
[r] >out1 << >out2                      [ ] ✓ different ports, valid wiring
```

### EC-8.6: Variable self-assignment (INVALID)

<!-- @EBNF:assignment_expr -->
**EBNF:** `assign_target ::= ... | typed_variable` and `value_expr ::= ... | identifier`

**What it tests:** Same variable on both sides of an assignment — a no-op. See [[technical/compile-rules/PGE/PGE08011-self-assignment|PGE08011]].

```polyglot
[ ] INVALID — same variable on both sides
[r] $name#string << $name               [ ] ✗ PGE08011 — self-assignment, no-op

[ ] VALID — different variables
[r] $a#string << $b                     [ ] ✓ different variables, valid assignment
```
