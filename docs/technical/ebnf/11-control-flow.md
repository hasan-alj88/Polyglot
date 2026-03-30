---
audience: developer
type: spec
updated: 2026-03-30
---

<!-- @ebnf/INDEX -->

## 11. Control Flow

### 11.1 Conditional Switch

```ebnf
conditional_line    ::= "[?]" comparison_expr NEWLINE
                         { indent conditional_branch NEWLINE } ;

conditional_branch  ::= exec_line | comment_line ;

(* Exhaustiveness: All [?] chains must cover every case.
   If conditions are not exhaustive, a catch-all [?] *? branch is mandatory.
   PGE06001: Conditional must be exhaustive.
   PGE06009: Every [?] line must include a comparison operator — no bare subjects.
   PGE06010: Every [?] branch must contain at least one executable statement. *)
```

### 11.1.1 Match (Conditional Assignment Sugar)

```ebnf
match_line          ::= "[r]" value_expr ">>" assign_target NEWLINE
                         indent match_value_arm NEWLINE
                         { indent match_arm NEWLINE } ;
                      (* At least one non-wildcard arm required — PGE06014 *)

match_arm           ::= match_value_arm
                      | "[?]" "*" ">>" value_expr ;        (* wildcard catch-all *)

match_value_arm     ::= "[?]" match_value ">>" value_expr ;

match_value         ::= literal
                      | identifier
                      | cross_pkg_enum ;
```

**Rule:** Match is syntactic sugar. `[r] $x >> $y` with indented `[?]` children desugars to a `[?]` chain where each arm becomes `[?] $x =? value` / `[r] $y << result`. All exhaustiveness rules (PGE06001 through PGE06013) apply to the desugared form. `[?] *` in match context desugars to `[?] *?`.

**Rule:** If a `[r] value_expr >> assign_target` line has no indented `[?]` children, it is a plain assignment — not a match header.

**Rule:** Match arms are assignment-only. The source (`$x`) must be in Final state. The target (`$y`) receives the matched value. No side effects or complex logic in arms. PGE06009 does not apply to match arms (they use `value >> result` form, not `$var operator value`).

### 11.2 Error Handling

```ebnf
error_block         ::= "[!]" error_id NEWLINE
                         { indent exec_line NEWLINE } ;
```

**Rule:** `[!]` blocks are scoped to the specific `[r]` call that produces the error. They are indented under that call, after its `[=]` IO lines — never at pipeline level.

### 11.3 Error Raise

```ebnf
error_raise         ::= "[!]" ">>" error_id NEWLINE
                         { indent error_raise_line NEWLINE } ;

error_raise_line    ::= "[=]" fixed_field assignment_op value_expr          (* #Error field: .Message, .Info *)
                      | "[=]" io_param assignment_op value_expr              (* output fallback *)
                         { indent raise_fallback_meta NEWLINE }
                      | comment_line ;

raise_fallback_meta ::= "[>]" "%FallbackMessage" assignment_op string_literal ;
```

**Rule:** `[!] >>` raises a declared error in the execution body. The raise block fills `#Error` fields (`.Message`, `.Info`, etc.) via `[=]` lines. Output fallbacks (`[=] >outputName << value`) set specific outputs to Final instead of Failed. `[>] %FallbackMessage` documents the author's intent for each fallback — omitting it triggers PGW07003; callers overriding it see PGW07002.

**Rule:** `[!] >>` can only raise errors declared in the pipeline's `[=] !ErrorName` declarations. Raising an undeclared error is PGE07005.

### 11.4 Logical Operators (in conditionals)

```ebnf
logical_and         ::= "[&]" comparison_expr ;
logical_or          ::= "[|]" comparison_expr ;
logical_xor         ::= "[^]" comparison_expr ;

(* Note: [-] NOT is not needed as a logical operator.
   Negation is expressed by modifying the comparison operator: <? → <!?, >=? → >=!? etc. *)
```

### 11.5 Line Continuation

```ebnf
continuation_line   ::= "[+]" expression ;
```

**Rule:** The originating line keeps its normal block marker. Only continuation lines get `[+]`. The parser joins all `[+]` lines with the preceding logical line. `[+]` is only valid when the preceding expression is incomplete. Strings can span across `[+]` boundaries (multi-line string content preserved).

### 11.6 Foreign Code Injection

```ebnf
foreign_code_block  ::= foreign_code_header { foreign_code_line } ;
foreign_code_header ::= "[c]" "#Code:" language_name ":" version ;
foreign_code_line   ::= "[c]" any_text ;
language_name       ::= name ;
version             ::= digit { ( digit | ":" ) } ;
```

**Rule:** The first `[c]` line declares the language via `#Code:<Language>:<Version>`. All body lines also get `[c]` prefix. Body content is raw text — not parsed as Polyglot. The block ends when a line without `[c]` appears.

---
