---
audience: designer
type: spec
updated: 2026-03-30
---

<!-- @ebnf/INDEX -->

## 6. Operators

### 6.1 Assignment Operators

```ebnf
push_left           ::= "<<" ;     (* right-to-left final assignment *)
push_right          ::= ">>" ;     (* left-to-right final assignment *)
default_push_left   ::= "<~" ;     (* right-to-left default assignment *)
default_push_right  ::= "~>" ;     (* left-to-right default assignment *)
fallback_push_left  ::= "!<" ;     (* right-to-left fallback — error recovery *)
fallback_push_right ::= "!>" ;     (* left-to-right fallback — error recovery *)

assignment_op       ::= push_left | push_right | default_push_left | default_push_right
                      | fallback_push_left | fallback_push_right ;
```

**Rule:** `!<` and `!>` are fallback assignment operators for error recovery. They provide a value when the source pipeline errors, preventing the target variable from entering the Failed state. Fallback operators only activate when an error occurs — they are not evaluated on the success path. The `!` error sigil always leads, with the direction arrow (`<` or `>`) following — optionally with an error name between them (`!Error.Name<`, `!Error.Name>`). See `(>)`/`(<)` IO brackets (§5) and fallback line syntax (§10.2).

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
