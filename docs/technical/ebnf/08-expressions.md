---
audience: designer
type: spec
updated: 2026-04-16
---

<!-- @ebnf/INDEX -->

## 8. Expressions

```ebnf
expression          ::= assignment_expr
                      | comparison_expr
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
assignment_expr     ::= assign_target ( push_left | default_push_left ) value_expr
                      | value_expr ( push_right | default_push_right ) assign_target ;

assign_target       ::= typed_variable
                      | typed_field
                      | typed_io_param
                      | output_param          (* direct output port write *)
                      | "$*" ;                (* inline discard — final operators only (<<, >>); see PGE02010 *)

(* Self-assignment (same identifier both sides) is PGE08011. *)

value_expr          ::= literal
                      | identifier
                      | child_access
                      | io_param
                      | cross_pkg_enum
                      | inline_data
                      | inline_pipeline_call
                      | output_param ;       (* >pipelineOutput as source *)

inline_pipeline_call ::= pipeline_ref string_literal ;
                      (* e.g., -Path"/tmp/MyApp", -Path"{.}/logs"
                         The string literal is interpolated ({$var} resolved first),
                         then matched against the pipeline's %InlineString template.
                         The compiler extracts named values from placeholder positions
                         and wires them to the corresponding declared (-) inputs.
                         Pipelines must declare (-) %InlineString << "{template}"
                         to accept inline calls (PGE12003 if missing). *)
```

### 8.2 Comparison Expressions

```ebnf
comparison_expr     ::= value_expr comparison_op value_expr
                      | value_expr range_expr
                      | "*?" ;              (* wildcard catch-all *)
```

### 8.3 Inline Data

```ebnf
(* Flat subset of value_expr — no inline_data, no inline_pipeline_call. *)
inline_value        ::= literal
                      | identifier
                      | child_access
                      | io_param
                      | cross_pkg_enum
                      | output_param ;

inline_data         ::= '{' inline_value { ',' inline_value } '}'
                      | '{' '}' ;                          (* empty collection *)
```

---
