---
audience: design
type: spec
updated: 2026-04-23
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
   aj3lib pipelines (-File.*, -T.*, -Q.*, -W.*) are pipeline_id: -File.Text.Read, -T.Call, etc.
   All Aljam3 identifiers have a prefix; pipelines always use -. *)

call_io_line        ::= "(-)" io_param assignment_op value_expr
                         { indent fallback_line NEWLINE }
                      | "(-)" wildcard_input "<<" wildcard_output   (* wildcard auto-wire — §7.4 *)
                      | operation_label
                      | grouped_fallback ;

fallback_line       ::= "(>)" "!>" value_expr                   (* generic output fallback *)
                      | "(>)" "!" error_id ">" value_expr       (* error-specific output fallback *)
                      | "(<)" "!<" value_expr                   (* generic input fallback *)
                      | "(<)" "!" error_id "<" value_expr ;     (* error-specific input fallback *)

grouped_fallback    ::= "(-)" variable_id NEWLINE               (* (-) $label — pipeline IO scope *)
                         { indent grouped_fallback_line NEWLINE }
                         { indent error_block NEWLINE } ;        (* [!] blocks under label *)

grouped_fallback_line ::= "($)" ">" output_name "!>" value_expr           (* generic per-output fallback *)
                        | "($)" ">" output_name "!" error_id ">" value_expr ; (* error-specific per-output fallback *)
```

**Rule:** Fallback lines are indented under the `(-)` IO line they belong to — the output/input reference is inherited from the parent scope. `(>)` is used under output lines with `!>` direction, `(<)` under input lines with `!<` direction. The `!` error sigil always leads, with the direction arrow following — optionally with an error name between: `!Error.Name>` (output) or `!Error.Name<` (input). A generic `!>` / `!<` catches any unhandled error; `!Error.Name>` / `!Error.Name<` catches only the named error. Error-specific fallbacks take priority over the generic. Duplicate generic or duplicate error-specific fallbacks for the same error on the same output are PGE07003. When a fallback activates, `$var%sourceError` is set to the triggering error.

**Rule:** The `grouped_fallback` production provides an alternative to scattered `(>) !>` fallbacks for pipelines with multiple outputs. `(-) $label` declares the label in pipeline IO scope (the `(-)` marker mirrors the `[-]` pipeline call context); `($)` lines inside the group operate on the label's variable-scope accessors, referencing outputs by `>outputName`. `[!]` blocks may also appear under the label, scoped to the pipeline call. Both scattered and grouped forms are valid — the compiler unions all mechanisms for exhaustiveness checking (PGE07007). The same error cannot be declared in both forms (PGE07003).

**Precedence:** `[!]` error blocks are checked before `!<` / `!>` fallbacks. If `[!]` pushes a replacement value, the fallback is not evaluated.

**Wildcard auto-wire:** `(-) <* << $Label>*` passes **all** outputs of a labeled operation as inputs to the current pipeline call, resolved at compile time via bijective type-topology matching (see [[technical/ebnf/07-io-parameters#7.4 Wildcard IO (Auto-Wire)]]). The form requires all of `$Label`'s outputs to be Final before the target triggers, producing implicit completion-wait semantics. Compile failures surface as PGE08001 (type mismatch), PGE08002 (ambiguous type), or PGE08003 (port count mismatch). See [[user/syntax/io/auto-wire|Wildcard Auto-Wire]].

**Rule:** Standard library pipelines (`-File.*`, `-T.*`, `-Q.*`, `-W.*`) are built-in and do not require `[@]` import. Only user/external packages need import.

**Job-level `[Q]`:** `queue_control_line` (defined in §9.3.3) may appear nested under `[-]`, `[=]`, or `[b]` pipeline calls. This scopes queue conditions to that specific job and its sub-jobs, extending (not replacing) the pipeline-level `[Q]`. See [[concepts/pipelines/queue#Job-Level Queue Conditions]].

```aljam3
[=] -Transform
   (-) << $fetched
   (-) >> $transformed
   [Q] -Q.Pause.Soft.RAM.LessThan
      (-) <mb << 2048.0
```

### 10.3 Data Load

```ebnf
data_load           ::= "[#]" assign_target assignment_op ( pipeline_call | data_id ) ;
                      (* Source must be a pipeline call or data reference, not a literal — PGE02011 *)
```

**In execution:** `[#] $hire#NewHire << $payload` — deserialize serialized data into a typed structure.

**In `{#}` definitions:** `[#]` can load external serialized files. The file access is mediated through a `{_}` permission object declared via `(#)` IO on the definition. The compiler resolves the permission object's `.path` field, reads the file, and content-hashes it at compile time. See [[concepts/permissions/enforcement#Compile-Time File Binding]].

```aljam3
[ ] Permission objects define the file resources
{_} _AppConfig
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/config/appsettings.json"
   [.] .path "/config/appsettings.json"
   [.] .format #JSON

{_} _YAMLFile
   (_) <file#path
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "{<file}"
   [.] .path "{<file}"
   [.] .format #YAML

[ ] {#} definitions declare file dependencies via (#)
{#} #Config
   (#) _AppConfig
   [#] #file1 << -Json.LoadFile
      (-) <source << _AppConfig
   [.] .dbConnection#string <~ #file1.db.connectionString

{#} #Config2
   (#) _YAMLFile
      (_) <file << "/config/appsettings.yaml"
   [#] #file2 << -Yaml.LoadFile
      (-) <source << _YAMLFile
   [.] .reportFolder#string <~ #file2.report.folder
```

Fields reference loaded file data: `.dbConnection#string <~ #file1.db.connectionString`. If the file at `.path` is missing, the compiler raises PGE10010. If the file changes after compilation, the associated permission is revoked and the pipeline refuses to execute until recompiled.

---
