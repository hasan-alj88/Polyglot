---
audience: developer
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
