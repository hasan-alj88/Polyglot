---
audience: developer
type: spec
updated: 2026-03-30
---

<!-- @ebnf/INDEX -->

## 8. Expressions

```ebnf
expression          ::= assignment_expr
                      | comparison_expr
                      | arithmetic_expr
                      | call_expr
                      | chain_call
                      | io_line
                      | field_decl
                      | import_line
                      | trigger_line
                      | queue_line
                      | wrapper_line
                      | error_handler
                      | error_raise
                      | conditional_expr
                      | expand_expr
                      | collect_expr
                      | data_load
                      | comment_text
                      | identifier
                      | literal ;
```

### 8.1 Assignment Expressions

```ebnf
(* Right-to-left: target << source  or  target <~ source *)
assignment_expr     ::= assign_target ( final_push | default_push ) value_expr
                      | value_expr ( final_pull | default_pull ) assign_target ;

assign_target       ::= typed_variable
                      | typed_field
                      | typed_io_param
                      | output_param          (* direct output port write *)
                      | "$*" ;                (* inline discard — output immediately released *)

value_expr          ::= literal
                      | identifier
                      | child_access
                      | io_param
                      | cross_pkg_enum
                      | inline_data
                      | inline_pipeline_call
                      | arithmetic_expr
                      | output_param ;       (* >pipelineOutput as source *)

inline_pipeline_call ::= pipeline_ref string_literal ;
                      (* e.g., =Path"/tmp/MyApp", =Path"{.}/logs"
                         The string literal is interpolated ({$var} resolved first),
                         then auto-wired into the pipeline's <InlineStringLiteral#string
                         parameter (must be declared in [=] IO, defaults to "").
                         Each pipeline defines its own parsing logic for the string.
                         For =Path, separators / and \ are normalized per OS. *)
```

### 8.2 Comparison Expressions

```ebnf
comparison_expr     ::= value_expr comparison_op value_expr
                      | value_expr range_expr
                      | "*?" ;              (* wildcard catch-all *)
```

### 8.3 Arithmetic Expressions

```ebnf
arithmetic_expr     ::= value_expr arithmetic_op value_expr ;
```

### 8.4 Inline Data

```ebnf
inline_data         ::= '{' value_expr { ',' value_expr } '}'
                      | '{' '}' ;                          (* empty collection *)
```

---
