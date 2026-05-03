---
audience: design
type: spec
updated: 2026-04-17
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

**Rule:** The grammar permits `!<`/`!>` in all `assignment_op` positions, but the compiler enforces semantic validity: fallback operators require a **failable source** — the right-hand side must be a pipeline call (an expression that can error at runtime). If the RHS is a literal value or variable reference, `PGE07008` fires — literals and variables cannot error, so the fallback path is dead code. A fallback chain (`!< -Pipeline.A !< -Pipeline.B !< "terminal"`) must terminate at a non-failable expression (literal or variable); if the chain ends at a pipeline call, `PGE07009` fires.

### 6.2 Comparison Operators

```ebnf
comparison_op       ::= "?="       (* equal *)
                      | "?>"       (* greater than *)
                      | "?<"       (* less than *)
                      | "?>="      (* greater or equal *)
                      | "?<="      (* less or equal *)
                      | "?!="      (* not equal *)
                      | "?!<"      (* not less than *)
                      | "?!>"      (* not greater than *)
                      | "?!<="     (* not less-or-equal *)
                      | "?!>="     (* not greater-or-equal *)
                      | "?*" ;     (* wildcard / else / catch-all *)

(* Negation pattern: insert ! after ? to negate any comparison.
   This replaces the need for a standalone [-] NOT logical operator. *)

domain_op           ::= "?#"       (* Type Check *)
                      | "?##"      (* Schema Check *)
                      | "?_"       (* Permission Check *)
                      | "?@"       (* Provenance Check *)
                      | "?!"       (* Error Check *)
                      | "?-" ;     (* Source Check *)
```

### 6.3 Range Operators

```ebnf
range_open          ::= "?[" | "?(" ;     (* left bound: [ inclusive, ( exclusive *)
range_close         ::= ']' | ')' ;       (* right bound: ] inclusive, ) exclusive *)

range_expr          ::= value_expr range_open value_expr ',' value_expr range_close ;
```

**Lexer Disambiguation:** The `?[` and `?(` range tokens share the `?` character with comparison operators and the `[?]` block element. The lexer resolves this positionally:

- `[?]` is a **three-character block element** token, matched at line start after indentation (§5.1). The `[` precedes the `?`.
- `?[` and `?(` are **two-character range tokens**, matched in expression context after a `value_expr`.
- Comparison operators consume greedily: `?=[` tokenizes as `?=` (comparison) + `[` (unrelated), never as `?` + `=[`.

No grammar ambiguity exists — the token boundary is determined by whether `[` appears first, and whether the position is line-start (block element) or mid-expression (range/comparison).

---
