---
audience: developer
type: specification
updated: 2026-03-27
status: draft
---

# Polyglot Code — EBNF Grammar

<!-- @line-structure -->
<!-- @blocks -->
<!-- @identifiers -->
<!-- @types -->
<!-- @operators -->
<!-- @io -->
<!-- @packages -->
<!-- @comments -->
<!-- @pipelines -->
<!-- @collections -->
<!-- @variable-lifecycle -->
This document defines the complete formal grammar for Polyglot Code (`.pg` files) using Extended Backus-Naur Form (EBNF). Each section maps to spec files: [[line-structure]], [[blocks]], [[identifiers]], [[types]], [[operators]], [[io]], [[packages]], [[comments]], [[pipelines]], [[collections]], [[variable-lifecycle]]. For edge case testing, see [[EDGE-CASES]].

## Notation Conventions

| Symbol | Meaning |
|--------|---------|
| `::=` | Definition |
| `\|` | Alternative |
| `[ ... ]` | Optional (0 or 1) |
| `{ ... }` | Repetition (0 or more) |
| `( ... )` | Grouping |
| `" ... "` | Terminal string literal |
| `'...'` | Terminal character literal |
| `(* ... *)` | EBNF comment |

---

## 1. File Structure

```ebnf
file                ::= package_block { definition } ;

definition          ::= data_def
                      | pipeline_def
                      | macro_def
                      | queue_def
                      | error_def
                      | array_def
                      | comment_block ;
```

---

## 2. Lexical Elements

### 2.1 Indentation & Lines

```ebnf
indent              ::= { "   " } ;                  (* 3 spaces per level *)

line                ::= indent [ block_element ] expression NEWLINE ;

NEWLINE             ::= '\n' ;
```

**Rule:** Exactly one expression per line. No tabs. Indentation is 3 spaces per level. Scope is determined by indentation depth — no closing markers.

### 2.2 Character Classes

```ebnf
letter              ::= 'A'..'Z' | 'a'..'z' ;
digit               ::= '0'..'9' ;
name_char           ::= letter | digit ;
name                ::= letter { name_char } ;
```

### 2.3 Literals

```ebnf
string_literal      ::= '"' { string_content } '"' ;

string_content      ::= any_char - '"' - '{'
                      | interpolation
                      | "{{" | "}}" ;          (* escaped literal braces *)

interpolation       ::= '{' variable_id '}' ;  (* e.g., {$name}, {$user:location} *)
int_literal         ::= [ '-' ] digit { digit } ;
                      (* Runtime RE: ^-?[0-9]+$  — leading zeros allowed *)
float_literal       ::= [ '-' ] digit { digit } '.' digit { digit } ;
                      (* Runtime RE: ^-?[0-9]+\.[0-9]+$  — leading zeros allowed *)
bool_literal        ::= "#Boolean.True" | "#Boolean.False" ;

literal             ::= string_literal
                      | int_literal
                      | float_literal
                      | bool_literal ;
```

---

## 3. Identifiers

All identifiers require a prefix sigil. Field separators navigate within identifiers.

### 3.1 Prefixed Identifiers

```ebnf
package_id          ::= '@' package_address ;
data_id             ::= '#' dotted_name ;
pipeline_id         ::= '=' dotted_name ;
variable_id         ::= '$' field_path ;
error_id            ::= '!' dotted_name ;

identifier          ::= package_id
                      | data_id
                      | pipeline_id
                      | variable_id
                      | error_id ;
```

### 3.2 Field Separators

```ebnf
(* Fixed fields — predefined schema keys *)
fixed_sep           ::= '.' ;

(* Flexible fields — user-defined keys *)
flex_sep            ::= ':' ;

(* Metadata fields — read-only, Polyglot-managed *)
meta_sep            ::= '%' ;

dotted_name         ::= name { fixed_sep name } ;

flex_path           ::= name { flex_sep name } ;

(* Metadata access — query metadata on any named object *)
meta_access         ::= identifier meta_sep name ;

(* A field_path may use fixed OR flexible separators, but NOT both at the same sibling level *)
field_path          ::= name { field_separator name } ;

field_separator     ::= fixed_sep | flex_sep ;
```

**Rule (sibling homogeneity):** All siblings at the same depth level must use the same separator type. Mixing `.` and `:` among siblings is invalid.

### 3.3 Package Addresses

```ebnf
package_address     ::= registry_type flex_sep registry_id
                         fixed_sep package_name
                         { fixed_sep sub_package }
                         [ flex_sep version ] ;

registry_type       ::= "Local" | "Community" | "Registry" ;

(* Registry ID format depends on registry_type:
   Local      — port number (unused port, e.g., :999, :042)
   Community  — username.ProjectName (e.g., :devops.NotificationHub)
   Registry   — registered company name (e.g., :Acme) *)
registry_id         ::= name | digit { digit } ;
package_name        ::= name ;
sub_package         ::= name ;
version             ::= 'v' digit '.' digit '.' digit [ '.' digit ] ;
```

**Example:** `@Local:999.MyPackage.Sub:v1.2.3.2`

### 3.4 Cross-Package References

```ebnf
cross_pkg_data      ::= '@' name data_id ;            (* @alias#DataName *)
cross_pkg_pipeline  ::= '@' name pipeline_id ;         (* @alias=PipelineName *)
cross_pkg_enum      ::= '@' name '#' dotted_name ;     (* @alias#DataName.EnumField *)
```

---

## 4. Type System

### 4.1 Type Annotations

```ebnf
type_annotation     ::= '#' type_expr ;

type_expr           ::= basic_type
                      | collection_type
                      | wildcard_type
                      | user_type
                      | live_type ;

live_type           ::= "live" type_expr ;    (* Polyglot-managed, read-only *)

basic_type          ::= "RawString" | "string" | "int" | "uint" | "float"
                      | "sci" | "eng" | "dim" | "bool" | "path" ;
                      (* RawString: compiler intrinsic, literal raw chars, no interpolation.
                         string (#String): struct with .string#RawString + .re#RawString.
                         int, uint, float, sci, eng, dim: #String subtypes with pre-set .re patterns.
                         bool (#Boolean): separate enum struct, not a #String subtype.
                         path (#path): struct with .Unix#string + .Windows#string. *)

collection_type     ::= array_type | dict_type | dataframe_type | serial_type ;

array_type          ::= "array" [ flex_sep type_param ] [ flex_sep dimension ] ;
                      (* e.g., #array:int, #array:float:2D, #array:Person *)
dict_type           ::= "dict" flex_sep type_param flex_sep type_param ;
                      (* e.g., #dict:string:int — key type : value type *)
dataframe_type      ::= "dataframe" flex_sep type_param flex_sep type_param ;
                      (* e.g., #dataframe:string:float — column key type : cell value type *)
serial_type         ::= "serial" ;

type_param          ::= basic_type | dimension | user_type | wildcard_type ;
                      (* Nested type refs drop the # prefix within type context *)
dimension           ::= digit { digit } "D" ;
                      (* e.g., :2D, :3D — omitted defaults to 1D *)

wildcard_type       ::= "*" ;                 (* #* — any type; used in generic constraints *)

user_type           ::= dotted_name ;         (* e.g., Person — no # prefix in type annotations *)
```

**Rule:** `#` starts a type context. Within that context, nested type references separated by `:` **drop the `#` prefix** — the compiler resolves them. Examples: `$score#int`, `$users#array:Person`, `$map#dict:string:int`, `$matrix#array:float:2D`.

### 4.2 Typed Variable

```ebnf
typed_variable      ::= variable_id [ type_annotation ] ;
typed_field         ::= field_ref [ type_annotation ] ;
typed_io_param      ::= io_param [ type_annotation ] ;
```

### 4.3 Generic Type Parameters in {#} Definitions

```ebnf
generic_param       ::= '<' name ;
                      (* Type parameter input — e.g., <ValueType, <Dim *)

generic_def_header  ::= data_id { generic_param } ;
                      (* e.g., #Array<ValueType<Dim, #Dict<KeyType<ValueType *)

schema_inheritance  ::= "[#]" "<~" data_id ;
                      (* e.g., [#] <~ #String — inherit schema, can specialize *)

schema_property     ::= "[#]" '%' dotted_name assignment_op expression ;
                      (* e.g., [#] %Key.Type << #UnsignedInt *)
                      (* e.g., [#] %Alias << "int" *)
                      (* e.g., [#] %Depth.Max << 0 *)

type_constraint     ::= "[<]" '%' dotted_name assignment_op expression ;
                      (* Nested under [#] <param — constrains the type parameter *)
                      (* e.g., [<] %Depth.Max << 0 — param must be scalar *)
```

**Rule:** Generic type parameters use `<` prefix (consistent with IO input semantics — the type is an "input" to the definition). Schema properties (`[#] %`) declare compile-time metadata. Type constraints (`[<]`) restrict what types may bind to a parameter.

---

## 5. Block Elements

Block elements are square-bracket markers that begin each line within a block.

```ebnf
block_element       ::= registry_elem
                      | data_flow_elem
                      | execution_elem
                      | control_flow_elem
                      | scope_elem
                      | data_access_elem
                      | logical_elem
                      | continuation_elem
                      | foreign_code_elem
                      | metadata_elem
                      | comment_elem ;

(* Registry *)
registry_elem       ::= "[@]" ;

(* Data Flow *)
data_flow_elem      ::= "[=]" | "[~]" | "[*]" | "[>]" | "[<]" ;

(* Execution *)
execution_elem      ::= "[r]" | "[p]" | "[b]" | "[#]" ;

(* Control Flow *)
control_flow_elem   ::= "[?]" | "[!]" | "[t]" | "[Q]" | "[W]" ;

(* Scope *)
scope_elem          ::= "[\]" | "[/]" | "[{]" | "[}]" ;

(* Data Access *)
data_access_elem    ::= "[.]" | "[:]" ;

(* Logical *)
logical_elem        ::= "[&]" | "[|]" | "[-]" | "[^]" ;

(* Line Continuation *)
continuation_elem   ::= "[+]" ;

(* Foreign Code *)
foreign_code_elem   ::= "[c]" ;

(* Metadata *)
metadata_elem       ::= "[%]" ;

(* Comment *)
comment_elem        ::= "[ ]" ;
```

**Rule:** `[>]` (output fallback) and `[<]` (input fallback) are scoped under `[=]` IO lines — they use the `<!` fallback operator to provide error-recovery values (see §10.2). `[<]` also appears nested under `[#] <param` in `{#}` definitions as a type parameter constraint block (see §4.3).

---

## 6. Operators

### 6.1 Assignment Operators

```ebnf
final_push          ::= "<<" ;     (* right-to-left final assignment *)
final_pull          ::= ">>" ;     (* left-to-right final assignment *)
default_push        ::= "<~" ;     (* right-to-left default assignment *)
default_pull        ::= "~>" ;     (* left-to-right default assignment *)
fallback_push       ::= "<!" ;     (* right-to-left fallback — error recovery *)
fallback_pull       ::= "!>" ;     (* left-to-right fallback — error recovery *)

assignment_op       ::= final_push | final_pull | default_push | default_pull
                      | fallback_push | fallback_pull ;
```

**Rule:** `<!` and `!>` are fallback assignment operators for error recovery. They provide a value when the source pipeline errors, preventing the target variable from entering the Failed state. Fallback operators only activate when an error occurs — they are not evaluated on the success path. See `[>]`/`[<]` block markers (§5) and fallback line syntax (§10.2).

### 6.2 Comparison Operators

```ebnf
comparison_op       ::= "=?"       (* equal *)
                      | ">?"       (* greater than *)
                      | "<?"       (* less than *)
                      | ">=?"      (* greater or equal *)
                      | "<=?"      (* less or equal *)
                      | "=!?"      (* not equal *)
                      | "<!?"      (* not less than *)
                      | ">!?"      (* not greater than *)
                      | "<=!?"     (* not less-or-equal *)
                      | ">=!?"     (* not greater-or-equal *)
                      | "*?" ;     (* wildcard / else / catch-all *)

(* Negation pattern: insert ! before ? to negate any comparison.
   This replaces the need for a standalone [-] NOT logical operator. *)

```

### 6.3 Range Operators

```ebnf
range_open          ::= "?[" | "?(" ;     (* left bound: [ inclusive, ( exclusive *)
range_close         ::= ']' | ')' ;       (* right bound: ] inclusive, ) exclusive *)

range_expr          ::= value_expr range_open value_expr ',' value_expr range_close ;
```

### 6.4 Arithmetic Operators

```ebnf
arithmetic_op       ::= '*' | '+' | '-' | '/' ;
```

---

## 7. IO Parameters

```ebnf
io_param            ::= input_param | output_param ;

input_param         ::= '<' name { field_separator name } ;
output_param        ::= '>' name { field_separator name } ;
```

---

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
                         { indent data_field NEWLINE } ;

data_field          ::= enum_field | value_field | flex_data_field | typed_flex_wildcard | metadata_line | comment_line ;

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

#### 9.3.1 Trigger Section

```ebnf
trigger_section     ::= indent "[t]" trigger_ref NEWLINE ;

trigger_ref         ::= pipeline_ref [ string_literal ] ;
```

**Examples:** `[t] =T.Call`, `[t] =T.Daily"3AM"`, `[t] =T.Folder.NewFiles`

#### 9.3.2 IO Section

```ebnf
io_section          ::= { indent io_decl_line NEWLINE } ;

io_decl_line        ::= "[=]" typed_io_param [ assignment_op value_expr ] ;
```

**IO as implicit triggers:**
- `<input << "value"` — constant, always satisfied.
- `<input <~ "value"` — default, used if nothing fills it.
- `<input` (no assignment) — must be filled externally; pipeline blocks until Final.

#### 9.3.3 Queue Section

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

#### 9.3.4 Wrapper Section

```ebnf
wrapper_section     ::= indent "[W]" pipeline_ref NEWLINE
                         { indent wrapper_io_line NEWLINE } ;

wrapper_io_line     ::= "[=]" variable_id assignment_op value_expr ;
```

**Rule:** `[W]` references a macro (`{M}`). Macro IO is wired using `[=]` with `$` variables. See §9.4 for macro definition syntax and wrapper IO wiring details.

**Examples:** `[W] =W.Polyglot`, `[W] =W.DB.Connection` with `[=] $connectionString << $connStr`

#### 9.3.5 Execution Section

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

### 9.4 Macro Definition

```ebnf
macro_def           ::= "{M}" '=' dotted_name NEWLINE
                         { indent macro_body_line NEWLINE } ;

macro_body_line     ::= scope_setup
                      | scope_cleanup
                      | from_outer
                      | to_outer
                      | exec_line
                      | comment_line ;

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
- `[{]` declares a macro input — a typed variable pulled from the calling pipeline's scope.
- `[}]` declares a macro output — a variable exposed back to the calling pipeline's scope.
- `[\]` runs before the pipeline execution body (setup). Can call a single pipeline or open a scope with multiple exec lines.
- `[/]` runs after the pipeline execution body (cleanup). Same structure as `[\]`.
- Macros do NOT contain `[t]`, `[=]` IO, or `[Q]` — those belong to the pipeline.
- Execution order: `[t],[=]` → `[Q]` → `[\]` → Execution Body → `[/]`.
- The wrapper unpacks the macro before and after the body like brackets.
- **Rule (parallel fork):** `[p]` inside `[\]` with no subsequent `[*] *All` in setup forks a parallel execution path. Setup completes and the body begins while the forked path is still running. `[/]` may use `[*] *All` with `[*] << $var` to synchronise with it before proceeding. `[b]` inside `[\]` is fire-and-forget — no collection in `[/]` is possible.
- Variables produced in `[\]` (including by `[p]`) remain accessible in `[/]`.

**Wrapper IO wiring at `[W]` usage site:**

```ebnf
wrapper_section     ::= indent "[W]" pipeline_ref NEWLINE
                         { indent wrapper_io_line NEWLINE } ;

wrapper_io_line     ::= "[=]" variable_id assignment_op value_expr ;
```

At the `[W]` line, macro IO is wired using `[=]` with `$` variables (not `<`/`>` IO params, not `[{]`/`[}]`):

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

(* Alias — binds short name to parent definition or field *)
metadata_alias      ::= fixed_sep "alias" assignment_op ( data_id | pipeline_id ) ;

(* Live fields — Polyglot-managed, read-only, implicit *)
metadata_live       ::= fixed_sep name ";" "live" type_expr ;
```

**Rules:**
- `[%]` appears inside any `{x}` definition (`{#}`, `{=}`, `{M}`).
- One definition = one metadata set (class-level, not instance-level).
- All top-level `[%]` fields use `.` fixed separator. Only `.info#serial` opens a `:` flexible scope underneath (sibling homogeneity preserved).
- Under a `[.]` field, `[%] .alias << #AliasName` creates a shorthand that resolves to the fully qualified path (e.g., `#True` resolves to `#Boolean.True`).
- At pipeline definition level, `[%] .alias << =AliasName` aliases the whole pipeline.
- Aliases preserve their type prefix (`#` for data, `=` for pipelines) and participate in exhaustiveness checking when the variable carries the parent type annotation.
- `live` fields are implicit on all `{=}`, `$`, and `{#}` definitions. The runtime populates them. Users read via `%` accessor (e.g., `=Pipeline%status`, `$var%state`) but never assign.
- Prefer reactive alternatives (error blocks, IO triggers) over polling `live` fields when possible.

---

## 10. Execution Statements

### 10.1 Run / Parallel / Background

```ebnf
run_line            ::= "[r]" exec_expr ;
parallel_line       ::= "[p]" exec_expr ;
background_line     ::= "[b]" exec_expr ;

exec_expr           ::= assignment_expr
                      | pipeline_call
                      | chain_call
                      | expand_invocation
                      | identifier ;
```

### 10.2 Pipeline Call

```ebnf
pipeline_call       ::= pipeline_ref NEWLINE
                         { indent call_io_line NEWLINE }
                         { indent error_block NEWLINE } ;

pipeline_ref        ::= pipeline_id                    (* local: =Pipeline.Name *)
                      | cross_pkg_pipeline ;            (* imported: @alias=Pipeline.Name *)

(* All pipeline references use the = prefix — no exceptions.
   Stdlib pipelines (=File.*, =T.*, =Q.*, =W.*) are pipeline_id: =File.Text.Read, =T.Call, etc.
   All Polyglot identifiers have a prefix; pipelines always use =. *)

call_io_line        ::= "[=]" io_param assignment_op value_expr
                         { indent fallback_line NEWLINE } ;

fallback_line       ::= "[>]" "<!" value_expr                   (* generic fallback *)
                      | "[>]" "<!" error_id value_expr           (* error-specific fallback *)
                      | "[<]" "<!" value_expr                    (* generic input fallback *)
                      | "[<]" "<!" error_id value_expr ;         (* error-specific input fallback *)
```

**Rule:** Fallback lines are indented under the `[=]` IO line they belong to — the output/input reference is inherited from the parent scope. `[>]` is used under output lines, `[<]` under input lines. A generic `<!` catches any unhandled error; `<!Error.Name` catches only the named error. Error-specific fallbacks take priority over the generic. Duplicate generic or duplicate error-specific fallbacks for the same error on the same output are PGE-703. When a fallback activates, `$var%sourceError` is set to the triggering error.

**Precedence:** `[!]` error blocks are checked before `<!` fallbacks. If `[!]` pushes a replacement value, the fallback is not evaluated.

**Rule:** Standard library pipelines (`=File.*`, `=T.*`, `=Q.*`, `=W.*`) are built-in and do not require `[@]` import. Only user/external packages need import.

### 10.3 Chain Execution

```ebnf
chain_call          ::= pipeline_ref "=>" pipeline_ref { "=>" pipeline_ref } NEWLINE
                         { indent chain_io_line NEWLINE }
                         { indent chain_error_block NEWLINE } ;

step_ref            ::= step_index | step_leaf_name ;
step_index          ::= digit { digit } ;              (* 0-based position in chain *)
step_leaf_name      ::= name ;                          (* last segment of pipeline dotted name *)

chain_io_param      ::= ( '<' | '>' ) step_ref fixed_sep name { field_separator name }
                         [ type_annotation ] ;

chain_io_line       ::= "[=]" chain_io_param assignment_op ( value_expr | chain_io_param )
                      | "[=]" chain_io_param fallback_push value_expr ;

chain_error_block   ::= "[!]" '!' step_ref fixed_sep error_name NEWLINE
                         { indent exec_line NEWLINE } ;

error_name          ::= dotted_name ;
```

**Rules:**
- `>N.param` pushes into step N's input (caller perspective).
- `<N.param` pulls from step N's output (caller perspective).
- `step_leaf_name` is the final segment of a pipeline's dotted name and must be unambiguous within the chain.
- **Auto-wire:** When step N has exactly one output and step N+1 has exactly one input of the same type, the `chain_io_line` between them may be omitted.
- Type annotations on `chain_io_param` are optional — types are inferred from pipeline definitions.
- Errors reference the step that produces them: `!0.File.NotFound` or `!Read.File.NotFound`.
- **Chain fallback:** In chains, fallback uses `<!` directly on `[=]` chain IO lines (not `[>]`/`[<]` block markers, since those cannot carry step references). Example: `[=] <0.content <! ""`. Same precedence and duplicate rules (PGE-703) apply.

### 10.4 Data Load

```ebnf
data_load           ::= "[#]" assignment_expr ;
```

**In execution:** `[#] $hire#NewHire << $payload` — deserialize serialized data into a typed structure.

**In `{#}` definitions:** `[#]` can load external serialized files:

```
[#] #file1 << =Json.LoadFile"/config/appsettings.json"
[#] #file2 << =Yaml.LoadFile"/config/appsettings.yaml"
```

Fields can then reference loaded file data: `.dbConnection#string <~ #file1.db.connectionString`. Default error handling raises a compile error if the file is missing. Value changes propagate across the codebase where referenced.

---

## 11. Control Flow

### 11.1 Conditional Switch

```ebnf
conditional_line    ::= "[?]" comparison_expr NEWLINE
                         { indent conditional_branch NEWLINE } ;

conditional_branch  ::= exec_line | comment_line ;

(* Exhaustiveness: All [?] chains must cover every case.
   If conditions are not exhaustive, a catch-all [?] *? branch is mandatory.
   PGE-601: Conditional must be exhaustive.
   PGE-609: Every [?] line must include a comparison operator — no bare subjects.
   PGE-610: Every [?] branch must contain at least one executable statement. *)
```

### 11.1.1 Match (Conditional Assignment Sugar)

```ebnf
match_line          ::= "[r]" value_expr ">>" assign_target NEWLINE
                         indent { match_arm NEWLINE } ;

match_arm           ::= "[?]" match_value ">>" value_expr
                      | "[?]" "*" ">>" value_expr ;        (* wildcard catch-all *)

match_value         ::= literal
                      | identifier
                      | cross_pkg_enum ;
```

**Rule:** Match is syntactic sugar. `[r] $x >> $y` with indented `[?]` children desugars to a `[?]` chain where each arm becomes `[?] $x =? value` / `[r] $y << result`. All exhaustiveness rules (PGE-601 through PGE-613) apply to the desugared form. `[?] *` in match context desugars to `[?] *?`.

**Rule:** If a `[r] value_expr >> assign_target` line has no indented `[?]` children, it is a plain assignment — not a match header.

**Rule:** Match arms are assignment-only. The source (`$x`) must be in Final state. The target (`$y`) receives the matched value. No side effects or complex logic in arms. PGE-609 does not apply to match arms (they use `value >> result` form, not `$var operator value`).

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

**Rule:** `[!] >>` raises a declared error in the execution body. The raise block fills `#Error` fields (`.Message`, `.Info`, etc.) via `[=]` lines. Output fallbacks (`[=] >outputName << value`) set specific outputs to Final instead of Failed. `[>] %FallbackMessage` documents the author's intent for each fallback — omitting it triggers PGW-703; callers overriding it see PGW-702.

**Rule:** `[!] >>` can only raise errors declared in the pipeline's `[=] !ErrorName` declarations. Raising an undeclared error is PGE-705.

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

## 12. Collection Operations

### 12.1 Expand Operators (`~`)

```ebnf
expand_line         ::= ( "[r]" | "[p]" ) expand_invocation NEWLINE
                         { indent expand_io_line NEWLINE }
                         { indent exec_line NEWLINE } ;

expand_invocation   ::= '~' expand_operator ;

expand_operator     ::= "ForEach.Array"
                      | "ForEach.Array.Enumerate"
                      | "ForEach.Serial"
                      | "ForEach.Level" ;

expand_io_line      ::= "[~]" io_param assignment_op value_expr ;
```

**Execution marker on expand controls parallelism:**
- `[r]` — mini-pipelines run sequentially.
- `[p]` — mini-pipelines run in parallel.

**`~ForEach.Level` special input syntax:** The `~` suffix on input marks the iteration point: `<level << #SomeData.SubField.~`

#### Expand IO Signatures

| Operator | Inputs | Outputs |
|----------|--------|---------|
| `~ForEach.Array` | `<Array` | `>item` |
| `~ForEach.Array.Enumerate` | `<Array` | `>index`, `>item` |
| `~ForEach.Serial` | `<Serial` | `>key`, `>item` |
| `~ForEach.Level` | `<level` | `>key`, `>item` |

### 12.2 Collect Operators (`*`)

```ebnf
collect_line        ::= ( "[r]" | "[p]" ) collect_invocation NEWLINE
                         { indent collect_io_line NEWLINE } ;

collect_invocation  ::= '*' collect_operator ;

collect_operator    ::= into_operator | agg_operator | sync_operator | race_operator
                      | error_operator | discard_operator ;

error_operator      ::= "Continue" ;

discard_operator    ::= "Ignore" ;

into_operator       ::= "Into.Array"
                      | "Into.Serial"
                      | "Into.Level" ;

agg_operator        ::= "Agg.Sum"
                      | "Agg.Count"
                      | "Agg.Average"
                      | "Agg.Max"
                      | "Agg.Min"
                      | "Agg.Concatenate" ;

sync_operator       ::= "All" ;

race_operator       ::= "First"
                      | "Second"
                      | "Nth" ;

collect_io_line     ::= "[*]" io_param assignment_op value_expr   (* named param: [*] <n#int << 2 *)
                      | "[*]" "<<" variable_ref                  (* wait input: [*] << $var *)
                      | "[*]" ">>" variable_ref ;                (* collect output: [*] >> $winner *)
```

**Rule:** Collector invocation uses an execution marker (`[r]` sequential, `[p]` parallel) — same as expand. Collector IO lines use `[*]` (matching the `*` operator prefix). Use `[r]` when collectors depend on each other, `[p]` when independent.

**Rule:** IO markers always match the operator prefix: `[=]` for pipelines, `[~]` for expand, `[*]` for collect.

**Rule:** Collector outputs can write directly to a pipeline output port: `[*] >result >> >pipelineOutput`.

**Rule:** `[*] <<` = wait input — waits for variable to be Final; variable stays accessible after. `[*] >>` = collect output — in race collectors, losing inputs are cancelled; only the `>>` output survives.

**Rule:** `*All` uses `[*] <<` only (no `[*] >>`). `*First`/`*Second`/`*Nth` require both `[*] <<` inputs and `[*] >>` output.

#### Collect IO Signatures

| Operator | Inputs | Outputs | Context |
|----------|--------|---------|---------|
| `*Into.Array` | `<item` | `>Array` | Inside `~ForEach` |
| `*Into.Serial` | `<key`, `<value` | `>Serial` | Inside `~ForEach` |
| `*Into.Level` | `<key`, `<value` | `>Serial` | Inside `~ForEach` |
| `*Agg.Sum` | `<number` | `>sum` | Inside `~ForEach` |
| `*Agg.Count` | `<item` | `>count` | Inside `~ForEach` |
| `*Agg.Average` | `<number` | `>average` | Inside `~ForEach` |
| `*Agg.Max` | `<number` | `>max` | Inside `~ForEach` |
| `*Agg.Min` | `<number` | `>min` | Inside `~ForEach` |
| `*Agg.Concatenate` | `<string` | `>result` | Inside `~ForEach` |
| `*All` | `[*] << $var...` | none — vars stay accessible | Parallel `[p]` sync |
| `*First` | `[*] << $var...` | `[*] >> $winner` | Parallel `[p]` race |
| `*Second` | `[*] << $var...` | `[*] >> $winner` | Parallel `[p]` race |
| `*Nth` | `<n#int`, `[*] << $var...` | `[*] >> $winner` | Parallel `[p]` race |
| `*Continue` | none | `>IsFailed#bool` via `[*] >IsFailed >> $var` | Inside `[!]` error block |
| `*Ignore` | `[*] << $var...` | none | Parallel `[p]` discard |

**Rule:** `*Continue` is an error recovery collector used inside `[!]` blocks. It signals the pipeline to continue after an error instead of terminating. Its single output `>IsFailed#bool` must be captured and handled — if the compiler cannot verify the output is checked, it emits PGW-205. Syntax: `[*] *Continue >IsFailed >> $fetchFailed`.

**Rule:** `*Ignore` is an explicit discard collector. It takes `[*] <<` wait inputs only and produces no outputs. Use for parallel output that exists for debugging but is intentionally unused. Prefer `$*` inline discard when the value is never needed.

---

## 13. Comments

```ebnf
comment_line        ::= "[ ]" comment_text ;
comment_curly       ::= "{ }" comment_text ;

multiline_comment   ::= "[ ]<" NEWLINE
                         { any_text NEWLINE }
                         "[ ]>" ;

comment_text        ::= { any_char } ;
```

**Rule:** A bracket containing only whitespace is always a comment: `[ ]` or `{ }`.

---

## 14. Variable Lifecycle Constraints

These are semantic rules enforced by the grammar's type system, not syntactic productions:

1. **Declared** — variable appears without assignment. Holds no value.
2. **Default** — assigned via `<~` or `~>`. Allows one further reassignment.
3. **Final** — assigned via `<<` or `>>`. No further assignment permitted.
4. **Failed** — the pipeline responsible for producing the variable's value terminated with an error. The variable will never resolve; downstream pipelines waiting on it will not fire. **Fallback override:** if a `<!` fallback is declared on the IO line, the variable bypasses Failed and becomes Final with the fallback value. When fallback activates, `$var%sourceError` is set to the triggering error.
5. **Released** — scope ends (indentation returns to parent) or collected via `*`.

### Serialization Constraints

1. **Sibling separator homogeneity** — all siblings at the same level must use the same separator (all `.` or all `:`).
2. **Sibling kind homogeneity** — all siblings at the same level must be the same kind (all enum fields or all value fields). Assignment within value fields is individually optional — unassigned value fields remain in Declared state.
3. **Leaf-only assignment** — only leaf fields (those with no children) can have values assigned.

---

## 15. Complete File Example (Informative)

```
file
  └─ package_block          {@ } @Local:999.MyPkg:v1.0.0
  │    └─ import_line          [@] @utils << @Community:user.Utils:v2.0.0
  │
  ├─ data_def               {#} #Status
  │    ├─ metadata              [%] .description << "entity status"
  │    ├─ enum_field            [.] .Active
  │    │    └─ metadata          [%] .alias << #Active
  │    └─ enum_field            [.] .Inactive
  │         └─ metadata          [%] .alias << #Inactive
  │
  ├─ data_def               {#} #Record
  │    ├─ value_field           [.] .name#string <~ ""
  │    └─ value_field           [.] .count#int <~ 0
  │
  ├─ error_def              {!} !Processing
  │    └─ leaf                 [.] .InvalidRecord#Error
  │
  └─ pipeline_def           {=} =ProcessItems
       ├─ metadata              [%] .version << "1.0.0"
       ├─ trigger               [t] =T.Call
       ├─ io                    [=] <items#array:Record
       │                        [=] >total#int ~> 0
       ├─ error_decl            [=] !Processing.InvalidRecord
       ├─ queue                 [Q] =Q.Default
       ├─ wrapper               [W] =W.Polyglot
       └─ execution
            ├─ expand            [p] ~ForEach.Array
            │   ├─ io            [~] <Array << $items
            │   └─ io            [~] >item >> $rec
            │      └─ collect    [r] *Agg.Sum
            │          ├─ io     [*] <number << $rec.count
            │          └─ io     [*] >sum >> >total
            └─ run               [r] @utils=Report.Generate
                ├─ io            [=] <total << $total
                └─ error         [!] !Report.Failed
                                    [r] >total << -1
```
