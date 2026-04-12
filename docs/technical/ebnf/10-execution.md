---
audience: designer
type: spec
updated: 2026-04-09
---

<!-- @ebnf/INDEX -->

## 10. Execution Statements

### 10.1 Sequential / Parallel / Background

```ebnf
run_line            ::= "[-]" exec_expr ;
parallel_line       ::= "[=]" exec_expr ;
background_line     ::= "[b]" exec_expr ;

exec_expr           ::= assignment_expr
                      | pipeline_call
                      | chain_call
                      | expand_invocation ;

(* Bare identifiers and literals are not valid exec_expr — see PGE01020. *)
```

### 10.2 Pipeline Call

```ebnf
pipeline_call       ::= pipeline_ref NEWLINE
                         { indent call_io_line NEWLINE }
                         { indent queue_control_line NEWLINE }
                         { indent error_block NEWLINE } ;

pipeline_ref        ::= pipeline_id                    (* local: -Pipeline.Name *)
                      | cross_pkg_pipeline ;            (* imported: @alias-Pipeline.Name *)

(* All pipeline references use the - prefix — no exceptions.
   pglib pipelines (-File.*, -T.*, -Q.*, -W.*) are pipeline_id: -File.Text.Read, -T.Call, etc.
   All Polyglot identifiers have a prefix; pipelines always use -. *)

call_io_line        ::= "(-)" io_param assignment_op value_expr
                         { indent fallback_line NEWLINE }
                      | operation_label ;

fallback_line       ::= "(>)" "!>" value_expr                   (* generic output fallback *)
                      | "(>)" "!" error_id ">" value_expr       (* error-specific output fallback *)
                      | "(<)" "!<" value_expr                   (* generic input fallback *)
                      | "(<)" "!" error_id "<" value_expr ;     (* error-specific input fallback *)
```

**Rule:** Fallback lines are indented under the `(-)` IO line they belong to — the output/input reference is inherited from the parent scope. `(>)` is used under output lines with `!>` direction, `(<)` under input lines with `!<` direction. The `!` error sigil always leads, with the direction arrow following — optionally with an error name between: `!Error.Name>` (output) or `!Error.Name<` (input). A generic `!>` / `!<` catches any unhandled error; `!Error.Name>` / `!Error.Name<` catches only the named error. Error-specific fallbacks take priority over the generic. Duplicate generic or duplicate error-specific fallbacks for the same error on the same output are PGE07003. When a fallback activates, `$var%sourceError` is set to the triggering error.

**Precedence:** `[!]` error blocks are checked before `!<` / `!>` fallbacks. If `[!]` pushes a replacement value, the fallback is not evaluated.

**Rule:** Standard library pipelines (`-File.*`, `-T.*`, `-Q.*`, `-W.*`) are built-in and do not require `[@]` import. Only user/external packages need import.

**Job-level `[Q]`:** `queue_control_line` (defined in §9.3.3) may appear nested under `[-]`, `[=]`, or `[b]` pipeline calls. This scopes queue conditions to that specific job and its sub-jobs, extending (not replacing) the pipeline-level `[Q]`. See [[concepts/pipelines/queue#Job-Level Queue Conditions]].

```polyglot
[=] -Transform
   (-) << $fetched
   (-) >> $transformed
   [Q] -Q.Pause.Soft.RAM.LessThan
      (-) <mb << 2048.0
```

### 10.3 Chain Execution

```ebnf
chain_call          ::= pipeline_ref "->" pipeline_ref { "->" pipeline_ref } NEWLINE
                         [ indent chain_label_block ]
                         { indent chain_io_line NEWLINE }
                         { indent chain_error_block NEWLINE } ;

chain_label_block   ::= operation_label NEWLINE
                         { indent step_label NEWLINE } ;

step_ref            ::= step_index | step_leaf_name | step_label_ref ;
step_label_ref      ::= variable_id ;
step_index          ::= digit { digit } ;              (* 0-based position in chain *)
step_leaf_name      ::= name ;                          (* last segment of pipeline dotted name *)

chain_io_param      ::= ( '<' | '>' ) step_ref fixed_sep name { field_separator name }
                         [ type_annotation ] ;

chain_io_line       ::= "(-)" chain_io_param assignment_op ( value_expr | chain_io_param )
                      | "(-)" chain_io_param "!<" value_expr ;

chain_error_block   ::= "[!]" '!' step_ref fixed_sep error_name NEWLINE
                         { indent exec_line NEWLINE } ;

error_name          ::= dotted_name ;
```

**Rules:**
- `>N.param` pushes into step N's input (pipeline perspective).
- `<N.param` pulls from step N's output (pipeline perspective).
- `step_leaf_name` is the final segment of a pipeline's dotted name and must be unambiguous within the chain.
- **Auto-wire:** When step N has exactly one output and step N+1 has exactly one input of the same type, the `chain_io_line` between them may be omitted.
- Type annotations on `chain_io_param` are optional — types are inferred from pipeline definitions.
- Errors reference the step that produces them: `!0.File.NotFound` or `!Read.File.NotFound`.
- **Chain fallback:** In chains, fallback uses `!<` directly on `(-)` chain IO lines (not `(>)`/`(<)` block markers, since those cannot carry step references). Example: `(-) <0.content !< ""`. Same precedence and duplicate rules (PGE07003) apply.

### 10.4 Data Load

```ebnf
data_load           ::= "[#]" assign_target assignment_op ( pipeline_call | data_id ) ;
                      (* Source must be a pipeline call or data reference, not a literal — PGE02011 *)
```

**In execution:** `[#] $hire#NewHire << $payload` — deserialize serialized data into a typed structure.

**In `{#}` definitions:** `[#]` can load external serialized files:

```polyglot
[#] #file1 << -Json.LoadFile"/config/appsettings.json"
[#] #file2 << -Yaml.LoadFile"/config/appsettings.yaml"
```

Fields can then reference loaded file data: `.dbConnection#string <~ #file1.db.connectionString`. Default error handling raises a compile error if the file is missing. Value changes propagate across the codebase where referenced.

---
