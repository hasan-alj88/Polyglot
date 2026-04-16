---
audience: designer
type: reference
updated: 2026-04-16
---

<!-- @edge-cases/INDEX -->

## 8. Expressions (S8)

### EC-8.1: Inline data — multiple elements

<!-- @u:types:Inline Data Shorthand -->
**EBNF:** `inline_data ::= '{' inline_value { ',' inline_value } '}'`

**What it tests:** Non-empty inline data with mixed types. See [[syntax/types/structs#Inline Data Shorthand]].

```polyglot
[-] $nums#array << {1, 2, 3, 4, 5}
[-] $services#array:string << {"AD", "Email", "Slack"}
```

### EC-8.2: Inline data — empty collection

**EBNF:** `inline_data ::= '{' '}'`

**What it tests:** Empty `{}` as valid collection initializer.

```polyglot
(-) >results#array:string ~> {}
```

### EC-8.3: String interpolation

<!-- @u:types:String Interpolation -->
**EBNF:** `interpolation ::= '{' variable_id '}'`

**What it tests:** Variable interpolation inside string literals using `{$var}` syntax. See [[syntax/types/strings#String Interpolation]].

```polyglot
[-] $msg#string << "Hello {$first} {$last}!"
[-] $path#string << "/users/{$userId}/profile"
[ ] Escaped literal braces
[-] $json#string << "{{\"key\": \"{$val}\"}}"
```

### EC-8.4: Default assignment to discard (INVALID)

<!-- @u:EBNF:assign_target -->
**EBNF:** `assign_target ::= ... | "$*"` — discard accepts final operators only (`<<`, `>>`).

**What it tests:** `$*` is immediately released — setting a default (`<~`/`~>`) implies the value persists for later override, which is meaningless for a discard. See [[technical/compile-rules/PGE/PGE02010-discard-default-assignment|PGE02010]].

```polyglot
[ ] INVALID — DefaultPushLeft into discard
[-] $*#string <~ "never used"           [ ] ✗ PGE02010 — discard cannot hold a default

[ ] INVALID — DefaultPushRight into discard
[-] -Compute
   (-) >result ~> $*                    [ ] ✗ PGE02010 — discard cannot hold a default

[ ] VALID — PushLeft into discard
[-] $*#string << "discarded"            [ ] ✓ final is the only valid discard operator

[ ] VALID — PushRight into discard
[-] -Compute
   (-) >result >> $*                    [ ] ✓ explicit discard of output
```

### EC-8.5: Self-assignment via output params (INVALID)

<!-- @u:EBNF:assignment_expr -->
**EBNF:** `assign_target ::= ... | output_param` and `value_expr ::= ... | output_param`

**What it tests:** Same output parameter on both sides of an assignment within the same operation scope — no state change. See [[technical/compile-rules/PGE/PGE08011-self-assignment|PGE08011]].

```polyglot
[ ] INVALID — same output param on both sides
(-) >result#string
[-] >result << >result                  [ ] ✗ PGE08011 — self-assignment, no state change

[ ] VALID — different output ports (cross-port wiring)
(-) >out1#string
(-) >out2#string
[-] >out1 << >out2                      [ ] ✓ different ports, valid wiring
```

### EC-8.6: Variable self-assignment (INVALID)

<!-- @u:EBNF:assignment_expr -->
**EBNF:** `assign_target ::= ... | typed_variable` and `value_expr ::= ... | identifier`

**What it tests:** Same variable on both sides of an assignment — a no-op. See [[technical/compile-rules/PGE/PGE08011-self-assignment|PGE08011]].

```polyglot
[ ] INVALID — same variable on both sides
[-] $name#string << $name               [ ] ✗ PGE08011 — self-assignment, no-op

[ ] VALID — different variables
[-] $a#string << $b                     [ ] ✓ different variables, valid assignment
```

### EC-8.7: Arithmetic precedence — moot (X.35)

<!-- @u:compile-rules/PGE/PGE04010-invalid-arithmetic-operator -->
**What it tests:** Multi-operator arithmetic expressions that would require precedence rules. Moot because raw arithmetic tokens are compile errors ([[PGE04010|PGE04010]]). The `arithmetic_expr` and `arithmetic_op` productions have been removed from the EBNF.

```polyglot
[ ] ✗ PGE04010 — precedence question is irrelevant
[-] $result#int << $a + $b * $c         [ ] ✗ PGE04010 — use -Math.Add / -Math.Multiply

[ ] ✗ PGE04010 — associativity question is irrelevant
[-] $result#int << $a - $b - $c         [ ] ✗ PGE04010 — use -Math.Subtract

[ ] ✓ explicit pipeline composition replaces operator precedence
[-] -Math.Multiply
   (-) << $b
   (-) << $c
   (-) >> $bc
[-] -Math.Add
   (-) << $a
   (-) << $bc
   (-) >> $result
```

### EC-8.8: Nested inline data (INVALID — X.36)

<!-- @u:EBNF:inline_data -->
**EBNF:** `inline_data ::= '{' inline_value { ',' inline_value } '}'` — `inline_value` excludes `inline_data`, breaking the cycle.

**What it tests:** Nested `{}` inside inline data. The `inline_data` production uses `inline_value` (not `value_expr`), which excludes nested braces. See [[technical/compile-rules/PGE/PGE08013-nested-inline-data|PGE08013]].

```polyglot
[ ] ✗ PGE08013 — nested inline data
[-] $matrix#array:array:int << {{1, 2, 3}, {4, 5, 6}}   [ ] ✗ PGE08013

[ ] ✗ PGE08013 — deeply nested
[-] $deep << {{{1}}}                                     [ ] ✗ PGE08013

[ ] ✗ PGE08013 — mixed nesting
[-] $mixed << {1, {2, 3}, 4}                             [ ] ✗ PGE08013
```

```polyglot
[ ] ✓ flat inline data — valid
[-] $nums#array:int << {1, 2, 3, 4, 5}
[-] $names#array:string << {"Alice", "Bob"}
(-) >results#array:string ~> {}
```
