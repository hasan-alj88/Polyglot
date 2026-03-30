---
audience: developer
type: reference
updated: 2026-03-30
---

<!-- @edge-cases/INDEX -->

## 2. Lexical Elements (S2)

### EC-2.1: Indentation depth — deeply nested scopes

<!-- @line-structure -->
**EBNF:** `indent ::= { "   " }` — unlimited nesting via 3-space repetition.

**What it tests:** 4+ levels of indentation (package -> pipeline -> expand -> conditional -> error). See [[line-structure]].

```polyglot
{=} =Deep
   [t] =T.Call
   [W] =W.Polyglot

   [r] ~ForEach.Array
   [~] <Array << $items
   [~] >item >> $item

      [?] $item >? 0
         [r] =SomeCall
         [=] <val << $item
         [=] >out >> $result
            [!] !Some.Error
               [r] $result << -1
```

### EC-2.2: Bool literals

<!-- @types -->
**EBNF:** `bool_literal ::= "#Boolean.True" | "#Boolean.False"` — booleans are data references, not keywords.

**What it tests:** Bool values are `#Boolean.True` / `#Boolean.False` (not `true`/`false`). See [[syntax/types/INDEX|types]].

```polyglot
[r] $flag#bool << #Boolean.True
[=] >enabled#bool ~> #Boolean.False
```

### EC-2.3: Negative numeric literals

**EBNF:** `int_literal ::= [ '-' ] digit { digit }` and `float_literal` — optional leading minus.

**What it tests:** Negative integers and floats as literal values.

```polyglot
[r] $offset#int << -1
[r] $threshold#float << -0.5
```

### EC-2.4: Empty string literal

**EBNF:** `string_literal ::= '"' { any_char - '"' } '"'` — zero characters between quotes is valid.

**What it tests:** `""` as a valid string literal.

```polyglot
[.] .name#string <~ ""
```
