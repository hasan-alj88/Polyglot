---
audience: design
type: spec
updated: 2026-04-23
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

### 7.4 Wildcard IO (Auto-Wire)

```ebnf
wildcard_output     ::= variable_id ">*" ;
wildcard_input      ::= "<*" ;
```

`wildcard_output` (`$Label>*`) denotes **all outputs** of a labeled operation. `wildcard_input` (`<*`) denotes **all inputs** of the target pipeline call. Together they form the wildcard auto-wire construct `<* << $Label>*` used in `call_io_line` (see §10.2).

**Rule:** `<* << $Label>*` requires **bijective type-topology matching**. The compiler pairs each output of `$Label` with exactly one input of the target pipeline by [[type-identity|Type Identity]]. The match must be bijective and onto: every output maps to exactly one input, every input receives exactly one output. If the match cannot be determined uniquely (ambiguous types, mismatched types, or port-count mismatch), the compiler raises PGE08001, PGE08002, or PGE08003 — the developer must fall back to explicit per-port wiring.

**Rule:** `<* << $Label>*` implicitly requires **all** of `$Label`'s outputs to be Final before the target pipeline triggers. This naturally produces completion-wait behavior between the two operations without any explicit synchronization marker.

---
