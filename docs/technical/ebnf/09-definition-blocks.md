---
audience: designer
type: spec
updated: 2026-04-05
---

<!-- @ebnf/INDEX -->

## 9. Definition Blocks

### 9.1 Package Declaration

```ebnf
package_block       ::= "{@}" package_id NEWLINE
                         { indent import_line NEWLINE }
                         { indent comment_line NEWLINE } ;

import_line         ::= "[@]" '@' name push_left package_id ;
```

**Rule:** `{@}` must be the first block in every `.pg` file. Exactly one `{@}` per file — multiple `{@}` blocks are not allowed. Multiple `{#}` and `{=}` definitions are allowed.

### 9.2 Data Definition

```ebnf
data_def            ::= "{#}" data_id NEWLINE
                         indent data_body_line NEWLINE
                         { indent data_body_line NEWLINE } ;
                      (* At least one body line required — empty {#} is PGE01021 *)

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
                         Contradicting annotation → PGE04001.
                         Absent wildcard → untyped (#serial). *)

field_expansion    ::= "[.]" fixed_sep "*" type_param_ref type_annotation ;
                      (* e.g., [.] .*ColumnEnum#$CellType — expands enum fields from macro parameter.
                         Macro param must satisfy ##EnumLeafs (PGE04022).
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
pipeline_def        ::= "{=}" marker_decl? pipeline_id NEWLINE
                         pipeline_body ;

marker_decl         ::= "[exe]" | "[b]" | "[r]" | "[p]"
                       | "[rp]" | "[rb]" | "[pb]" ;

pipeline_body       ::= { ( metadata_line | comment_line ) NEWLINE }
                         trigger_io_section
                         queue_section
                         wrapper_section
                         execution_section ;

(* Trigger, IO, and error declarations form one section — order IS strict: [=] IO declarations and error
   declarations come first, then [T] trigger lines (PGE01002). IO inputs are implicit triggers; some triggers
   produce inputs. Error declarations mark the pipeline as failable. *)
trigger_io_section  ::= { indent ( io_decl_line | error_decl_line | comment_line ) NEWLINE }
                         { indent ( trigger_line | comment_line ) NEWLINE } ;

error_decl_line     ::= "[=]" error_id ;
```

**Marker declarations:**
- `{=}` without `marker_decl` defaults to `{=}[exe]` — no warning, no error.
- `[exe]` = execution pipeline, invocable via `[r]` (sequential), `[p]` (parallel), or `[b]` (background).
- Subsets restrict invocation: `{=}[b]` means background-only (no outputs allowed — fire-and-forget), `{=}[rp]` means sequential or parallel only (no background).
- `{T}`, `{W}`, and `{Q}` already have implicit markers (`[T]`, `[W]`, `[Q]`) — they cannot take `marker_decl`.

**Example** — background-only pipeline:

```polyglot
{=}[b] =LogEvent
   [T] =T.Call
   [=] <message#string
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] =File.Text.Append"{$logPath}"
      [=] <text << $message
```

### 9.3.1 Trigger Section

```ebnf
trigger_section     ::= indent "[T]" trigger_ref NEWLINE ;

trigger_ref         ::= pipeline_ref [ string_literal ] ;
```

**Rules:**
- `trigger_ref` must reference an operation that declares `[T]` marker compatibility (PGE01024). Stdlib trigger pipelines (`=T.*`) are the canonical trigger operations.
- Multiple `[T]` lines in one pipeline have **AND** semantics — all triggers must fire before the pipeline executes.
- For **OR** semantics (any trigger fires the pipeline), use `[|]` to scope alternative triggers.

**Examples:** `[T] =T.Call`, `[T] =T.Daily"3AM"`, `[T] =T.Folder.NewFiles`

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
                      (* Same [=] IO marker as top-level io_line — scoped to the parent
                         [Q] operator via indentation, not a different marker. *)

queue_control_line  ::= "[Q]" pipeline_ref NEWLINE
                         { indent queue_io_line NEWLINE } ;
```

`[Q]` references either `=Q.Default` or `=Q.Assign"QueueName"` (for user-defined queues). Nested `[Q]` lines declare pipeline-specific active queue controls (pause, resume, kill). These extend the queue's `{Q}` defaults for this pipeline only — contradictions raise PGE01013.

**Dual context:** `[Q]` appears in two locations: (1) the pipeline header `queue_section` — scoped to all jobs in the pipeline; (2) nested under `[r]`/`[p]`/`[b]` execution markers via `queue_control_line` in `pipeline_call` (§10.2) — scoped to that specific job and its sub-jobs. Job-level `[Q]` extends pipeline-level `[Q]`, it does not replace it.

**Examples:**

Simple: `[Q] =Q.Default`

With active controls:
```polyglot
[Q] =Q.Assign"GPUQueue"
   [Q] =Q.Pause.Hard.RAM.LessThan
      [=] <mb << 3072.0
   [Q] =Q.Resume.RAM.MoreThan
      [=] <mb << 5120.0
```

### 9.3.4 Wrapper Section

```ebnf
wrapper_section     ::= indent "[W]" pipeline_ref NEWLINE
                         { indent wrapper_io_line NEWLINE } ;

wrapper_io_line     ::= "[=]" variable_id assignment_op value_expr ;
                      (* Same [=] IO marker as top-level io_line — scoped to the parent
                         [W] operator via indentation, not a different marker. *)
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
- Macros overload by signature (parameter count + kind). Two overloads with identical signature = PGE01019.
- Type macros do NOT contain `[\]`, `[/]`, `[{]`, `[}]`, `[T]`, `[=]` IO, or `[Q]` — those belong to wrappers or pipelines.

### 9.4a Trigger Definition (`{T}`)

```ebnf
trigger_def         ::= "{T}" trigger_pipeline_id NEWLINE
                         trigger_def_body ;

trigger_pipeline_id ::= '=' 'T' '.' dotted_name ;

trigger_def_body    ::= { ( metadata_line | comment_line ) NEWLINE }
                         { indent ( io_decl_line | error_decl_line ) NEWLINE } ;
                      (* Trigger definitions contain ONLY metadata, IO declarations,
                         and error declarations. No execution body, no [Q], no [W]. *)
```

**Rules:**
- `{T}` defines a trigger pipeline — a subtype of `{=}` constrained to IO-only bodies.
- Trigger identifier must use the `=T.` prefix.
- Must include `>IsTriggered#bool` output (mandatory). May include additional outputs.
- No execution body, no `[Q]`, no `[W]` — triggers define event sources, not execution logic.
- `[T]` invokes a trigger inside a pipeline (see §9.3.1).

**Example:**

```polyglot
{T} =T.Folder.NewFiles
   [%] .description << "Fires when new files appear in watched directory"
   [=] <path#path
   [=] >IsTriggered#bool
   [=] >NewFiles#array:path
```

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
- Wrappers do NOT contain `{#}` definitions, `[T]`, `[=]` pipeline-level IO, or `[Q]` — those belong to pipelines. Type macros (`{M}`) are a separate construct for compile-time type generation.
- Execution order: `[=],[T]` → `[Q]` → `[\]` → Execution Body → `[/]`.
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

```polyglot
[W] =W.DB.Connection
   [=] $connectionString << $connStr     (* wrapper input *)
   [=] $dbConn >> $dbConn                (* wrapper output *)
```

**Examples:** `[W] =W.Polyglot` (no IO, no-op wrapper), `[W] =W.DB.Transaction` (with IO wiring)

### 9.4c Native Definition (`{N}`)

```ebnf
native_def          ::= "{N}" native_pipeline_id NEWLINE
                         native_def_body ;

native_pipeline_id  ::= '=' dotted_name ;

native_def_body     ::= { ( native_metadata_line | comment_line ) NEWLINE }
                         { indent ( io_decl_line | error_decl_line ) NEWLINE } ;
                      (* Native definitions contain ONLY metadata and IO declarations.
                         No [T], [Q], [W], or execution body. *)

native_metadata_line ::= "[%]" '.' native_field assignment_op value_expr ;

native_field        ::= "Kind"                   (* #NativeKind enum *)
                      | language_name             (* e.g., "Rust", "Cpp" *)
                      | "description" ;           (* human-readable description *)

language_name       ::= upper_letter { letter } ; (* Rust, Cpp, etc. *)
```

**Rules:**
- `{N}` defines a compiler-native pipeline — implemented in the host language, not Polyglot.
- `[%]` metadata under `{N}` implicitly scopes to `%Native.*` — all fixed `.` fields.
- `.Kind` is mandatory — must be one of `#NativeKind.Trigger`, `.Queue`, `.Wrapper`, `.Execution`, `.Intrinsic`.
- At least one `.<Language>` binding is required — must match the configured host language.
- No execution body (`[r]`, `[p]`, `[b]`, `[s]`, `[?]`), no `[T]`, no `[Q]`, no `[W]`.
- `[=]` IO declarations define the public interface (inputs, outputs, errors) — same as any pipeline.
- `{N}` definitions can only appear in stdlib `.pg` files — user `.pg` files cannot define native pipelines.

**Example:**

```polyglot
{N} =File.Text.Read
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "FileTextRead"
   [%] .description << "Read text file contents"
   [=] <path#path
   [=] >content#string
   [=] !File.NotFound
   [=] !File.PermissionDenied
```

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

`{Q}` defines and instantiates a named queue. The identifier must use the `#Queue:` prefix (PGE01012). Fields set queue-level defaults (strategy, host, maxInstances, maxConcurrent, resourceTags, killPropagation, maxWaitTime, description). Nested `[Q]` lines set default active controls that apply to all pipelines on this queue.

**Example:**

```polyglot
{Q} #Queue:GPUQueue
   [%] .description << "Queue for GPU-intensive work"
   [.] .strategy;#QueueStrategy << #LIFO
   [.] .host;#String << "gpu-server-01"
   [.] .maxInstances;#UnsignedInt << 1
   [.] .killPropagation;#KillPropagation << #Downgrade
   [.] .resourceTags;#Array:ResourceTag << [#GPU]
   [.] .maxWaitTime;#String << "30m"
   [Q] =Q.Kill.Graceful.Time.MoreThan
      [=] <duration << "4h"
```

**Rule:** `{Q}` is both a data definition (`#Queue:*` struct) and a runtime instantiation — unlike `{#}` which only defines a type. `=Q.Default` is the stdlib-provided queue and does not require a `{Q}` definition.

**Dual-purpose:** `{Q}` serves two roles based on the identifier prefix. The grammar above covers the **data definition** form (`{Q} #Queue:Name`). The **pipeline operation** form (`{Q} =Q.*`) is syntactic sugar for `{=}[Q]` and follows the pipeline definition grammar in §9.3 — it defines a queue control pipeline invocable via `[Q]`. Examples: `{Q} =Q.Default`, `{Q} =Q.Pause.Hard`, `{Q} =Q.Kill.Graceful`.

### 9.6 Error Definition

```ebnf
error_def           ::= "{!}" error_namespace_id NEWLINE
                         indent error_body_line NEWLINE
                         { indent error_body_line NEWLINE } ;
                      (* At least one leaf required — empty {!} is PGE01022 *)

error_namespace_id  ::= '!' dotted_name ;

error_body_line     ::= "[.]" fixed_field "#Error"       (* terminal leaf *)
                      | "[:]" flexible_field              (* user-extensible branch — !Error only *)
                      | metadata_line
                      | comment_line ;
                      (* Siblings at same level must use same separator — PGE05001 *)
```

`{!}` defines an error tree. Each terminal leaf is typed `#Error`. The namespace uses the `!` prefix. Stdlib error namespaces (`!File`, `!No`, `!Timeout`, `!Math`, `!Validation`, `!Field`, `!Alias`, `!Permission`, `!RT`) are built-in and use `[.]` fixed leaves only.

User-defined `{!} !Name` implicitly nests under `!Error` in the metadata tree, creating `!Error:Name.*`. Only `{!} !Error` allows `[:]` flexible children for user-extensible branches. All other `{!}` namespaces use `[.]` fixed leaves only.

**Stdlib example** (runtime-defined, fixed leaves):
```polyglot
{!} !Validation
   [.] .Schema#Error
   [.] .Type#Error
   [.] .Regex#Error
```

**User example** (extensible branches under `!Error`):
```polyglot
{!} !Error
   [:] :MyApp
      [:] :Auth
         [.] .Expired#Error
         [.] .Invalid#Error
      [:] :Data
         [.] .Corrupt#Error
```

### 9.7 Array Definition

```ebnf
array_def           ::= "{Array}" variable_id type_annotation NEWLINE
                         { indent array_body_line NEWLINE } ;

array_body_line     ::= exec_line | comment_line ;
```

### 9.8 Permission Object Definition (`{_}`)

```ebnf
permission_object_def  ::= "{_}" permission_id NEWLINE
                            indent "[.]" ".intent" push_left ( "#Ceiling" | "#Grant" ) NEWLINE
                            { indent permission_field_line NEWLINE }
                            { indent comment_line NEWLINE } ;

permission_field_line  ::= "[.]" "." category_name "." capability_name string_literal ;

category_name          ::= "File" | "Web" | "Database" | "System"
                         | "Crypto" | "IPC" | "Device" | "Memory" ;
```

**Rules:**

- `{_}` defines a named, reusable permission object. The name uses the `_` prefix (e.g., `_DataCeiling`, `_ReportReader`).
- `.intent` must be the first field — either `#Ceiling` (allows glob patterns) or `#Grant` (requires specific narrow values).
- Each `permission_field_line` declares a capability: `.Category.Capability "scope"`. The `category_name` must be one of the 8 predefined categories. Each category has a per-category capability enum (e.g., `#FileCapability`: Read, Write, Execute, Delete, Create).
- **Fully filled** — every `{_}` object must have all leaf fields assigned. Empty leaves are a compile error.
- **No instances** — permissions are compile-time declarations. No `:{instance}` level exists in `%_`.
- **No inline declarations** — `[_]` in `{@}` and `{=}` always references a `{_}` object by name. Inline permission syntax is not valid.
- **Identifier tiers:** `_` = permission object, `__` = permission descriptor (schema), `___` = constraint descriptor. Mirrors `#`/`##`/`###`.

### 9.9 Comment Block (Definition Level)

```ebnf
comment_block       ::= "{ }" comment_text NEWLINE ;
```

### 9.10 Metadata Block

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
                         indent flex_sep string_literal NEWLINE
                         { indent flex_sep string_literal NEWLINE } ;
                      (* At least one alias name required — PGE12004 *)

(* Live fields — Polyglot-managed, read-only, implicit *)
metadata_live       ::= fixed_sep name ";" "live" type_expr ;
```

**Rules:**
- `[%]` appears inside any `{x}` definition (`{#}`, `{=}`, `{M}`).
- One definition = one metadata set (class-level, not instance-level).
- All top-level `[%]` fields use `.` fixed separator. Only `.info#serial` opens a `:` flexible scope underneath (sibling homogeneity preserved).
- `[%] %alias` declares shorthand names for definitions or fields. Each `[:]` child is a `#NestedKeyString` alias name. Multiple aliases per definition are allowed. All aliases must be globally unique (PGE12002).
- Aliases participate in exhaustiveness checking when the variable carries the parent type annotation.
- `live` fields are implicit on all `{=}`, `$`, and `{#}` definitions. The runtime populates them. Users read via `%` accessor (e.g., `=Pipeline%status`, `$var%state`) but never assign.
- Prefer reactive alternatives (error blocks, IO triggers) over polling `live` fields when possible.
- Native definitions use `{N}` — a separate block type (see §9.4c). `{N}` metadata implicitly scopes to `%Native.*` with fixed fields `.Kind` (`#NativeKind`), `.<Language>` (native function binding), and `.description`. `{N}` and `{=}` are mutually exclusive block types — a definition cannot be both native and derived (PGE01028).

## Related User Documentation

| Section | User Doc |
|---------|----------|
| §9.1 `{@}` Package | [[syntax/packages\|packages]] |
| §9.2 `{#}` Data | [[syntax/blocks\|blocks]], [[syntax/types/INDEX\|types]] |
| §9.3 `{=}` Pipeline | [[concepts/pipelines/INDEX\|pipelines]] |
| §9.4 `{M}` Macro | [[concepts/macros\|macros]], [[syntax/types/macro-types\|macro-types]] |
| §9.4a `{T}` Trigger | [[concepts/pipelines/io-triggers\|io-triggers]] |
| §9.4b `{W}` Wrapper | [[concepts/pipelines/wrappers\|wrappers]] |
| §9.4c `{N}` Native | [[concepts/pipelines/INDEX#Native vs Derived\|Native vs Derived]] |
| §9.5 `{Q}` Queue | [[concepts/pipelines/INDEX\|pipelines]] |
| §9.6 `{!}` Error | [[concepts/errors\|errors]] |
| §9.8 `{_}` Permission | [[concepts/permissions\|permissions]] |
| §9.10 `[%]` Metadata | [[concepts/metadata\|metadata]] |
