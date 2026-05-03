---
audience: design
type: reference
updated: 2026-03-30
---

<!-- @edge-cases/INDEX -->

## 2. Lexical Elements (S2)

### EC-2.1: Indentation depth — deeply nested scopes

<!-- @u:line-structure -->
**EBNF:** `indent ::= { "   " }` — unlimited nesting via 3-space repetition.

**What it tests:** 4+ levels of indentation (package -> pipeline -> expand -> conditional -> error). See [[line-structure]].

```aljam3
{-} -Deep
   [T] -T.Call
   [W] -W.Aljam3

   [ ]
   [-] =ForEach.Array
   (=) <Array << $items
   (=) >item >> $item

      [?] $item >? 0
         [-] -SomeCall
         (-) <val << $item
         (-) >out >> $result
            [!] !Some.Error
               [-] $result << -1
```

### EC-2.2: Bool literals

<!-- @c:types -->
**EBNF:** `bool_literal ::= "$$True" | "$$False"` — booleans are built-in global instances (constants).

**What it tests:** Bool values use the `$$` constant prefix: `$$True` / `$$False` (not `true`/`false`). See [[syntax/types/INDEX|types]].

```aljam3
[-] $flag#bool << $$True
(-) >enabled#bool ~> $$False
```

### EC-2.3: Negative numeric literals

**EBNF:** `int_literal ::= [ '-' ] digit { digit }` and `float_literal` — optional leading minus.

**What it tests:** Negative integers and floats as literal values.

```aljam3
[-] $offset#int << -1
[-] $threshold#float << -0.5
```

### EC-2.4: Empty string literal

**EBNF:** `string_literal ::= '"' { any_char - '"' } '"'` — zero characters between quotes is valid.

**What it tests:** `""` as a valid string literal.

```aljam3
[.] .name#string <~ ""
```

### EC-2.5: Empty inline pipeline string argument

**EBNF ref:** `inline_pipeline_call ::= pipeline_ref string_literal`
**What it tests:** Empty string `""` as inline pipeline argument. Valid — the pipeline decides how to handle it.

```aljam3
[ ] ✓ empty inline string — pipeline responsibility
[-] $p#path << -Path""
```

### EC-2.6: Leading zeros in int/float literals

**EBNF ref:** `int_literal ::= [ '-' ] digit { digit }` — comment says "leading zeros allowed"
**What it tests:** `007`, `0042`, `00.50` — decimal only, no octal in Aljam3. PGW04002 warns.

```aljam3
[ ] ⚠ PGW04002 — leading zeros
[-] $x#int << 007
[-] $y#float << 00.50
```

### EC-2.7: Negative zero

**EBNF ref:** `int_literal`, `float_literal` — optional leading minus
**What it tests:** `-0` and `-0.0` — syntactically valid. Runtime normalizes to `0`/`0.0`.

```aljam3
[ ] ✓ negative zero — normalized at runtime
[-] $x#int << -0
[-] $y#float << -0.0
```
