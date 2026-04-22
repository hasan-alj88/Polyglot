---
audience: design
type: spec
updated: 2026-04-09
---

<!-- @ebnf/INDEX -->

## 7. IO Parameters

```ebnf
io_param            ::= input_param | output_param ;

input_param         ::= '<' name { field_separator name } ;
output_param        ::= '>' name { field_separator name } ;
```

### 7.2 Operation Labels

```ebnf
operation_label     ::= "(-)" variable_id ;
step_label          ::= "(.)" variable_id ;
io_comment          ::= "( )" { any_char } ;
```

`(-) $Label` labels a pipeline call's IO, making outputs accessible via `$Label>outputParam` without intermediate variables. The `(-)` marker mirrors the `[-]` pipeline call context (IO mirroring rule). `(.)` labels individual chain steps, indented under `(-) $Label`, mapped by position. `( )` introduces an inline comment within IO blocks.

### 7.3 Label Accessors

```ebnf
label_output_access ::= variable_id '>' name { field_separator name } ;
label_input_access  ::= variable_id '<' name { field_separator name } ;
label_error_access  ::= variable_id '!' dotted_name ;
label_perm_access   ::= variable_id '_' name ;

label_access        ::= label_output_access
                      | label_input_access
                      | label_error_access
                      | label_perm_access ;
```

`label_access` is valid anywhere `value_expr` is valid. The `>`, `<`, `!`, `_` accessors mirror existing prefix conventions for outputs, inputs, errors, and permissions.

---
