---
audience: developer
type: spec
updated: 2026-03-30
---

<!-- @ebnf/INDEX -->

## 9. Definition Blocks

### 9.1 Package Declaration

```ebnf
package_block       ::= "{@}" package_id NEWLINE
                         { indent import_line NEWLINE }
                         { indent comment_line NEWLINE } ;

import_line         ::= "[@]" '@' name final_push package_id ;
```

**Rule:** `{@}` must be the first block in every `.pg` file. Exactly one `{@}` per file — multiple `{@}` blocks are not allowed. Multiple `{#}` and `{=}` definitions are allowed.

### 9.2 Data Definition

```ebnf
data_def            ::= "{#}" data_id NEWLINE
                         { indent data_body_line NEWLINE } ;

data_body_line      ::= schema_inheritance
                      | schema_composition
                      | field_type_composition
                      | schema_property
                      | field_type_property
                      | macro_invoke
                      | data_field
                      | metadata_line
                      | comment_line ;

data_field          ::= enum_field | value_field | flex_data_field | typed_flex_wildcard | field_expansion ;

enum_field          ::= "[.]" fixed_sep name NEWLINE
                         { indent ( data_field | metadata_line ) NEWLINE } ;  (* enum fields can nest sub-fields and metadata *)

value_field         ::= "[.]" fixed_sep name type_annotation [ assignment_op value_expr ] ;

(* Flexible-field data declaration *)
flex_data_field     ::= "[:]" flex_sep name type_annotation [ assignment_op value_expr ] ;

(* Typed flexible wildcard — all flex siblings at this level share this type *)
typed_flex_wildcard ::= "[:]" flex_sep "*" type_annotation ;
                      (* e.g., [:] :*#Handler — every :key at this level is #Handler.
                         Compiler infers type on new keys; no explicit annotation needed.
                         Contradicting annotation → PGE-401.
                         Absent wildcard → untyped (#serial). *)

field_expansion    ::= "[.]" fixed_sep "*" type_param_ref type_annotation ;
                      (* e.g., [.] .*ColumnEnum#$CellType — expands enum fields from macro parameter.
                         Macro param must satisfy ##EnumLeafs (PGE-928).
                         Compiler stamps out one [.] per enum variant, each with annotated type.
                         Used inside {M} macros to expand fields from type parameters. *)
```

**Rules:**
- No `#type` annotation implies an **enum field**.
- With `#type` implies a **value field**.
- All siblings at the same level must be the same kind (all enum or all value).
- Enum fields always use `[.]` fixed fields.

### 9.3 Pipeline Definition

```ebnf
pipeline_def        ::= "{=}" pipeline_id NEWLINE
                         pipeline_body ;

pipeline_body       ::= { ( metadata_line | comment_line ) NEWLINE }
                         trigger_io_section
                         queue_section
                         wrapper_section
                         execution_section ;

(* Trigger, IO, and error declarations form one section — order between [t], [=], and error decls is not strict.
   IO inputs are implicit triggers; some triggers produce inputs. Error declarations mark the pipeline as failable. *)
trigger_io_section  ::= { indent ( trigger_line | io_decl_line | error_decl_line | comment_line ) NEWLINE } ;

error_decl_line     ::= "[=]" error_id ;
```

### 9.3.1 Trigger Section

```ebnf
trigger_section     ::= indent "[t]" trigger_ref NEWLINE ;

trigger_ref         ::= pipeline_ref [ string_literal ] ;
```

**Examples:** `[t] =T.Call`, `[t] =T.Daily"3AM"`, `[t] =T.Folder.NewFiles`

### 9.3.2 IO Section

```ebnf
io_section          ::= { indent ( io_decl_line | type_input_line ) NEWLINE } ;

io_decl_line        ::= "[=]" typed_io_param [ assignment_op value_expr ] ;

type_input_line     ::= "[=]" "<#" identifier ;
                      (* Type definition as data tree input — extends <# from {M} macros to {=} pipeline IO.
                         The pipeline receives the type's % metadata tree. Works with #, ##, ### tiers.
                         Example: [=] <#type -- any type definition; [=] <#Config -- specific type *)
```

**IO as implicit triggers:**
- `<input << "value"` — constant, always satisfied.
- `<input <~ "value"` — default, used if nothing fills it.
- `<input` (no assignment) — must be filled externally; pipeline blocks until Final.

### 9.3.3 Queue Section

```ebnf
queue_section       ::= indent "[Q]" queue_ref NEWLINE
                         { indent ( queue_io_line | queue_control_line ) NEWLINE } ;

queue_ref           ::= pipeline_ref | queue_id ;

queue_io_line       ::= "[=]" typed_io_param assignment_op value_expr ;

queue_control_line  ::= "[Q]" pipeline_ref NEWLINE
                         { indent queue_io_line NEWLINE } ;
```

`[Q]` references either a stdlib queue (`=Q.Default`) or a user-defined queue (`#Queue:Name`). Nested `[Q]` lines declare pipeline-specific active queue controls (pause, resume, kill). These override or extend the queue's `{Q}` defaults for this pipeline only — contradictions raise PGE-113.

**Examples:**

Simple: `[Q] =Q.Default`

With IO and active controls:
```polyglot
[Q] #Queue:GPUQueue
   [=] <maxConcurrent#int << 2
   [Q] =Q.Pause.Hard
      [=] <RAM.Available.LessThan#float << 3072.0
   [Q] =Q.Resume
      [=] <RAM.Available.MoreThan#float << 5120.0
```

### 9.3.4 Wrapper Section

```ebnf
wrapper_section     ::= indent "[W]" pipeline_ref NEWLINE
                         { indent wrapper_io_line NEWLINE } ;

wrapper_io_line     ::= "[=]" variable_id assignment_op value_expr ;
```

**Rule:** `[W]` references a wrapper (`{W}`). Wrapper IO is wired using `[=]` with `$` variables. See §9.4b for wrapper definition syntax and IO wiring details.

**Examples:** `[W] =W.Polyglot`, `[W] =W.DB.Connection` with `[=] $connectionString << $connStr`

### 9.3.5 Execution Section

```ebnf
execution_section   ::= { indent exec_line NEWLINE } ;

exec_line           ::= run_line
                      | match_line
                      | parallel_line
                      | background_line
                      | data_load
                      | conditional_line
                      | expand_line
                      | collect_line
                      | comment_line ;
```

### 9.4 Type Macro Definition (`{M}`)

```ebnf
(* Type macros generate {#} definitions at compile time.
   See §4.3 for macro_def, macro_param, macro_type_param, macro_invoke grammar. *)
```

**Rules:**
- `{M} #Name` defines a type macro. `[M] #Name` invokes it inside a `{#}` block.
- `[#] <Param` declares a value input parameter; `[#] <#Param` declares a type-as-data-tree input.
- Macro body contains `{#}` definitions that use `{$var}` interpolation from parameters.
- **[M] merge rule (identity):** The outer `{#}` names the result; the macro's internal `{#}` resolves to the same name. The macro fills the body. Any `[#]` lines after `[M]` in the outer `{#}` extend/override the macro's output.
- Macros overload by signature (parameter count + kind). Two overloads with identical signature = PGE-930.
- Type macros do NOT contain `[\]`, `[/]`, `[{]`, `[}]`, `[t]`, `[=]` IO, or `[Q]` — those belong to wrappers or pipelines.

### 9.4b Wrapper Definition (`{W}`)

```ebnf
(* Wrappers provide setup/cleanup scope for pipelines.
   See §4.3 for wrapper_def grammar. *)

scope_setup         ::= "[\]" pipeline_ref NEWLINE
                         { indent exec_line NEWLINE }
                      | "[\]" NEWLINE
                         { indent exec_line NEWLINE } ;

scope_cleanup       ::= "[/]" pipeline_ref NEWLINE
                         { indent call_io_line NEWLINE }
                      | "[/]" NEWLINE
                         { indent exec_line NEWLINE } ;

from_outer          ::= "[{]" typed_variable ;
to_outer            ::= "[}]" variable_id ;
```

**Rules:**
- `{W} =W.Name` defines a wrapper. `[W] =W.Name` invokes it inside a pipeline.
- `[{]` declares a wrapper input — a typed variable pulled from the calling pipeline's scope.
- `[}]` declares a wrapper output — a variable exposed back to the calling pipeline's scope.
- `[\]` runs before the pipeline execution body (setup). Can call a single pipeline or open a scope with multiple exec lines.
- `[/]` runs after the pipeline execution body (cleanup). Same structure as `[\]`.
- Wrappers do NOT contain `{#}` definitions, `[t]`, `[=]` IO, or `[Q]` — those belong to type macros or pipelines.
- Execution order: `[t],[=]` → `[Q]` → `[\]` → Execution Body → `[/]`.
- The wrapper unpacks before and after the body like brackets.
- **Rule (parallel fork):** `[p]` inside `[\]` with no subsequent `[*] *All` in setup forks a parallel execution path. Setup completes and the body begins while the forked path is still running. `[/]` may use `[*] *All` with `[*] << $var` to synchronise with it before proceeding. `[b]` inside `[\]` is fire-and-forget — no collection in `[/]` is possible.
- Variables produced in `[\]` (including by `[p]`) remain accessible in `[/]`.

**Wrapper IO wiring at `[W]` usage site:**

```ebnf
wrapper_section     ::= indent "[W]" pipeline_ref NEWLINE
                         { indent wrapper_io_line NEWLINE } ;

wrapper_io_line     ::= "[=]" variable_id assignment_op value_expr ;
```

At the `[W]` line, wrapper IO is wired using `[=]` with `$` variables (not `<`/`>` IO params, not `[{]`/`[}]`):

```
[W] =W.DB.Connection
   [=] $connectionString << $connStr     (* macro input *)
   [=] $dbConn >> $dbConn                (* macro output *)
```

**Examples:** `[W] =W.Polyglot` (no IO, no-op macro), `[W] =W.DB.Transaction` (with IO wiring)

### 9.5 Queue Definition

```ebnf
queue_def           ::= "{Q}" queue_id NEWLINE
                         { indent queue_body_line NEWLINE } ;

queue_id            ::= "#Queue:" name ;

queue_body_line     ::= queue_field_line
                      | queue_control_line
                      | metadata_line
                      | comment_line ;

queue_field_line    ::= "[.]" fixed_field type_annotation assignment_op value_expr ;

queue_control_line  ::= "[Q]" pipeline_ref NEWLINE
                         { indent queue_io_line NEWLINE } ;
```

`{Q}` defines and instantiates a named queue. The identifier must use the `#Queue:` prefix (PGE-112). Fields set queue-level defaults (strategy, retrigger). Nested `[Q]` lines set default active controls that apply to all pipelines on this queue.

**Example:**

```polyglot
{Q} #Queue:GPUQueue
   [%] .description << "Queue for GPU-intensive work"
   [.] .strategy#QueueStrategy << #LIFO
   [.] .maxInstances#int << 1
   [.] .retrigger#RetriggerStrategy << #Disallow
   [Q] =Q.Kill.Graceful
      [=] <ExecutionTime.MoreThan#string << "2h"
```

**Rule:** `{Q}` is both a data definition (`#Queue:*` struct) and a runtime instantiation — unlike `{#}` which only defines a type. `=Q.Default` is the stdlib-provided queue and does not require a `{Q}` definition.

### 9.6 Error Definition

```ebnf
error_def           ::= "{!}" error_namespace_id NEWLINE
                         { indent error_leaf_line NEWLINE } ;

error_namespace_id  ::= '!' dotted_name ;

error_leaf_line     ::= "[.]" fixed_field "#Error"
                      | metadata_line
                      | comment_line ;
```

`{!}` defines a custom error tree. Each leaf is typed `#Error`. The namespace uses the `!` prefix. Stdlib error namespaces (`!File`, `!No`, `!Timeout`, `!Math`, `!Validation`) are built-in.

**Example:**
```polyglot
{!} !Validation
   [.] .Empty#Error
   [.] .TooLong#Error
   [.] .InvalidEmail#Error
```

### 9.7 Array Definition

```ebnf
array_def           ::= "{Array}" variable_id type_annotation NEWLINE
                         { indent array_body_line NEWLINE } ;

array_body_line     ::= exec_line | comment_line ;
```

### 9.8 Comment Block (Definition Level)

```ebnf
comment_block       ::= "{ }" comment_text NEWLINE ;
```

### 9.9 Metadata Block

```ebnf
metadata_line       ::= "[%]" metadata_expr ;

metadata_expr       ::= metadata_fixed
                      | metadata_info
                      | metadata_alias
                      | metadata_live ;

(* Fixed schema fields — all use . fixed separator *)
metadata_fixed      ::= fixed_sep "description" [ type_annotation ] assignment_op string_literal
                      | fixed_sep "authors" [ type_annotation ] assignment_op inline_data
                      | fixed_sep "version" [ type_annotation ] assignment_op string_literal
                      | fixed_sep "license" [ type_annotation ] assignment_op string_literal
                      | fixed_sep "deprecated" assignment_op data_id ;

(* Info field — serial type, opens flexible scope *)
metadata_info       ::= fixed_sep "info" type_annotation NEWLINE
                         { indent "[%]" flex_sep name [ type_annotation ] assignment_op value_expr NEWLINE } ;

(* Alias — binds short names to parent definition or field *)
metadata_alias      ::= "%" "alias" NEWLINE
                         { indent flex_sep string_literal NEWLINE } ;

(* Live fields — Polyglot-managed, read-only, implicit *)
metadata_live       ::= fixed_sep name ";" "live" type_expr ;
```

**Rules:**
- `[%]` appears inside any `{x}` definition (`{#}`, `{=}`, `{M}`).
- One definition = one metadata set (class-level, not instance-level).
- All top-level `[%]` fields use `.` fixed separator. Only `.info#serial` opens a `:` flexible scope underneath (sibling homogeneity preserved).
- `[%] %alias` declares shorthand names for definitions or fields. Each `[:]` child is a `#NestedKeyString` alias name. Multiple aliases per definition are allowed. All aliases must be globally unique (PGE-1002).
- Aliases participate in exhaustiveness checking when the variable carries the parent type annotation.
- `live` fields are implicit on all `{=}`, `$`, and `{#}` definitions. The runtime populates them. Users read via `%` accessor (e.g., `=Pipeline%status`, `$var%state`) but never assign.
- Prefer reactive alternatives (error blocks, IO triggers) over polling `live` fields when possible.

## Related User Documentation

| Section | User Doc |
|---------|----------|
| §9.1 `{@}` Package | [[syntax/packages\|packages]] |
| §9.2 `{#}` Data | [[syntax/blocks\|blocks]], [[syntax/types/INDEX\|types]] |
| §9.3 `{=}` Pipeline | [[concepts/pipelines/INDEX\|pipelines]] |
| §9.4 `{M}` Macro | [[concepts/macros\|macros]], [[syntax/types/macro-types\|macro-types]] |
| §9.4b `{W}` Wrapper | [[concepts/pipelines/wrappers\|wrappers]] |
| §9.5 `{Q}` Queue | [[concepts/pipelines/INDEX\|pipelines]] |
| §9.6 `{!}` Error | [[concepts/errors\|errors]] |
| §9.9 `[%]` Metadata | [[concepts/metadata\|metadata]] |
