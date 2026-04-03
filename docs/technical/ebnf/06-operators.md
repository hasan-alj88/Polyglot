---
audience: designer
type: spec
updated: 2026-03-30
---

<!-- @ebnf/INDEX -->

## 6. Operators

### 6.1 Assignment Operators

```ebnf
final_push          ::= "<<" ;     (* right-to-left final assignment *)
final_pull          ::= ">>" ;     (* left-to-right final assignment *)
default_push        ::= "<~" ;     (* right-to-left default assignment *)
default_pull        ::= "~>" ;     (* left-to-right default assignment *)
fallback_push       ::= "<!" ;     (* right-to-left fallback — error recovery *)
fallback_pull       ::= "!>" ;     (* left-to-right fallback — error recovery *)

assignment_op       ::= final_push | final_pull | default_push | default_pull
                      | fallback_push | fallback_pull ;
```

**Rule:** `<!` and `!>` are fallback assignment operators for error recovery. They provide a value when the source pipeline errors, preventing the target variable from entering the Failed state. Fallback operators only activate when an error occurs — they are not evaluated on the success path. See `[>]`/`[<]` block markers (§5) and fallback line syntax (§10.2).

### 6.2 Comparison Operators

```ebnf
comparison_op       ::= "=?"       (* equal *)
                      | ">?"       (* greater than *)
                      | "<?"       (* less than *)
                      | ">=?"      (* greater or equal *)
                      | "<=?"      (* less or equal *)
                      | "=!?"      (* not equal *)
                      | "<!?"      (* not less than *)
                      | ">!?"      (* not greater than *)
                      | "<=!?"     (* not less-or-equal *)
                      | ">=!?"     (* not greater-or-equal *)
                      | "*?" ;     (* wildcard / else / catch-all *)

(* Negation pattern: insert ! before ? to negate any comparison.
   This replaces the need for a standalone [-] NOT logical operator. *)

```

### 6.3 Range Operators

```ebnf
range_open          ::= "?[" | "?(" ;     (* left bound: [ inclusive, ( exclusive *)
range_close         ::= ']' | ')' ;       (* right bound: ] inclusive, ) exclusive *)

range_expr          ::= value_expr range_open value_expr ',' value_expr range_close ;
```

### 6.4 Arithmetic Operators

```ebnf
arithmetic_op       ::= '*' | '+' | '-' | '/' ;
```

---
