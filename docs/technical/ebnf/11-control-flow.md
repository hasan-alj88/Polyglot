---
audience: designer
type: spec
updated: 2026-04-09
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

### 11.1.1 Match Syntax

```ebnf
match_line          ::= "[-]" value_expr ">>" assign_target NEWLINE
                         indent match_value_arm NEWLINE
                         { indent match_arm NEWLINE } ;
                      (* At least one non-wildcard arm required — PGE06014 *)

match_arm           ::= match_value_arm
                      | "[?]" "*?" ">>" value_expr ;       (* wildcard catch-all *)

match_value_arm     ::= "[?]" match_value ">>" value_expr ;

match_value         ::= literal
                      | identifier
                      | cross_pkg_enum ;
```

**Rule:** Match is syntactic sugar. `[-] $x >> $y` with indented `[?]` children desugars to a `[?]` chain where each arm becomes `[?] $x =? value` / `[-] $y << result`. All exhaustiveness rules (PGE06001 through PGE06013) apply to the desugared form. Match arms use `*?` for the wildcard catch-all, same as verbose conditionals.

**Rule:** If a `[-] value_expr >> assign_target` line has no indented `[?]` children, it is a plain assignment — not a match header.

**Rule:** Match arms are assignment-only. The source (`$x`) must be in Final state. The target (`$y`) receives the matched value. No side effects or complex logic in arms. PGE06009 does not apply to match arms (they use `value >> result` form, not `$var operator value`).

### 11.2 Error Handling

```ebnf
error_block         ::= "[!]" error_id NEWLINE
                         { indent exec_line NEWLINE } ;
```

**Rule:** `[!]` blocks are scoped to the specific `[-]` call that produces the error. They are indented under that call, after its `(-)` IO lines — never at pipeline level.

### 11.3 Error Raise

```ebnf
error_raise         ::= "[!]" ">>" error_id NEWLINE
                         { indent error_raise_line NEWLINE } ;

error_raise_line    ::= "(-)" fixed_field assignment_op value_expr          (* #Error field: .Message, .Info *)
                      | "(-)" io_param assignment_op value_expr              (* output fallback *)
                         { indent raise_fallback_meta NEWLINE }
                      | comment_line ;

raise_fallback_meta ::= "(>)" "%FallbackMessage" assignment_op string_literal ;
```

**Rule:** `[!] >>` raises a declared error in the execution body. The raise block fills `#Error` fields (`.Message`, `.Info`, etc.) via `(-)` lines. Output fallbacks (`(-) >outputName << value`) set specific outputs to Final instead of Failed. `(>) %FallbackMessage` documents the author's intent for each fallback — omitting it triggers PGW07003; callers overriding it see PGW07002.

**Rule:** `[!] >>` can only raise errors declared in the pipeline's `(-) !ErrorName` declarations. Raising an undeclared error is PGE07005.

### 11.4 Logical Operators (in conditionals)

```ebnf
logical_and         ::= "[&]" comparison_expr ;
logical_or          ::= "[|]" comparison_expr ;
logical_xor         ::= "[^]" comparison_expr ;

(* Note: Negation is expressed by modifying the comparison operator: <? → <!?, >=? → >=!? etc.
   [+] is the OR scope marker for triggers (§9.3.1) — distinct from [|] logical OR here. *)
```

### 11.5 Line Continuation

```ebnf
continuation_line   ::= "[~]" expression ;
```

**Rule:** The originating line keeps its normal block marker. Only continuation lines get `[~]`. The parser joins all `[~]` lines with the preceding logical line. `[~]` is only valid when the preceding expression is incomplete. Strings can span across `[~]` boundaries (multi-line string content preserved).

### 11.6 Foreign Code Injection

```ebnf
foreign_code_block  ::= foreign_code_line { foreign_code_line } ;
                      (* At least one code line required — PGE01027 *)
foreign_code_line   ::= "[C]" any_text ;
```

**Rule:** `[C]` lines embed foreign code passed to `-RT.*` runtime pipelines. Each `[C]` line is one line of foreign code — raw text, not parsed as Polyglot. The language is determined by which `-RT.*` pipeline is called (e.g., `-RT.Python.Script`, `-RT.JS.Script`). The block ends when a line without `[C]` appears.

---
