---
audience: design
type: spec
updated: 2026-04-09
---

<!-- @ebnf/INDEX -->

## 9.3 Pipeline Definition

```ebnf
pipeline_def        ::= "{-}" marker_decl? pipeline_id NEWLINE
                         pipeline_body ;

marker_decl         ::= "[exe]" | "[b]" | "[-]" | "[=]"
                       | "[-=]" | "[-b]" | "[=b]" ;

pipeline_body       ::= { ( metadata_line | comment_line ) NEWLINE }
                         trigger_io_section
                         queue_section
                         wrapper_section
                         execution_section ;

(* Trigger, IO, and error declarations form one section — order IS strict: (-) IO declarations and error
   declarations come first, then [T] trigger lines (PGE01002). IO inputs are implicit triggers; some triggers
   produce inputs. Error declarations mark the pipeline as failable. *)
trigger_io_section  ::= { indent ( io_decl_line | error_decl_line | comment_line ) NEWLINE }
                         { indent ( trigger_line | comment_line ) NEWLINE } ;

error_decl_line     ::= "(-)" error_id ;
```

**Marker declarations:**
- `{-}` without `marker_decl` defaults to `{-}[exe]` — no warning, no error.
- `[exe]` = execution pipeline, invocable via `[-]` (sequential), `[=]` (parallel), or `[b]` (background).
- Subsets restrict invocation: `{-}[b]` means background-only (no outputs allowed — fire-and-forget), `{-}[-=]` means sequential or parallel only (no background).
- `{T}`, `{W}`, and `{Q}` already have implicit markers (`[T]`, `[W]`, `[Q]`) — they cannot take `marker_decl`.

**Example** — background-only pipeline:

```aljam3
{-}[b] -LogEvent
   [T] -T.Call
   (-) <message#string
   [Q] -Q.Default
   [W] -W.Aljam3
   [ ]
   [-] -File.Text.Append"{$logPath}"
      (-) <text << $message
```

### 9.3.1 Trigger Section

```ebnf
trigger_section     ::= indent "[T]" trigger_ref NEWLINE ;

trigger_ref         ::= pipeline_ref [ string_literal ] ;
```

**Rules:**
- `trigger_ref` must reference an operation that declares `[T]` marker compatibility (PGE01024). aj3lib trigger pipelines (`-T.*`) are the canonical trigger operations.
- Multiple `[T]` lines in one pipeline have **AND** semantics — all triggers must fire before the pipeline executes.
- For **OR** semantics (any trigger fires the pipeline), use `[+]` to scope alternative triggers.

**Examples:** `[T] -T.Call`, `[T] -T.Daily"3AM"`, `[T] -T.Folder.NewFiles`

### 9.3.2 IO Section

```ebnf
io_section          ::= { indent ( io_decl_line | type_input_line ) NEWLINE } ;

io_decl_line        ::= "(-)" typed_io_param [ assignment_op value_expr ]
                      | "(-)" inline_template_decl ;

inline_template_decl ::= "%InlineString" push_left string_literal ;
                      (* Declares the inline call template for this pipeline.
                         Placeholders: {name} = required, {name?} = optional.
                         Each placeholder must match a declared <name input.
                         Optional placeholders require <~ default on the input. *)

type_input_line     ::= "(-)" "<#" identifier ;
                      (* Type definition as data tree input — same <# syntax as {#} generic params.
                         The pipeline receives the type's % metadata tree. Works with #, ##, ### tiers.
                         Example: (-) <#type -- any type definition; (-) <#Config -- specific type *)
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

queue_io_line       ::= "(-)" typed_io_param assignment_op value_expr ;
                      (* Same (-) IO bracket as top-level io_line — scoped to the parent
                         [Q] operator via indentation, not a different marker. *)

queue_control_line  ::= "[Q]" pipeline_ref NEWLINE
                         { indent queue_io_line NEWLINE } ;
```

`[Q]` references either `-Q.Default` or `-Q.Assign"QueueName"` (for user-defined queues). Nested `[Q]` lines declare pipeline-specific active queue controls (pause, resume, kill). These extend the queue's `{Q}` defaults for this pipeline only — contradictions raise PGE01013.

**Dual context:** `[Q]` appears in two locations: (1) the pipeline header `queue_section` — scoped to all jobs in the pipeline; (2) nested under `[-]`/`[=]`/`[b]` execution markers via `queue_control_line` in `pipeline_call` (10.2) — scoped to that specific job and its sub-jobs. Job-level `[Q]` extends pipeline-level `[Q]`, it does not replace it.

**Examples:**

Simple: `[Q] -Q.Default`

With active controls:
```aljam3
[Q] -Q.Assign"GPUQueue"
   [Q] -Q.Pause.Hard.RAM.LessThan
      (-) <mb << 3072.0
   [Q] -Q.Resume.RAM.MoreThan
      (-) <mb << 5120.0
```

### 9.3.4 Wrapper Section

```ebnf
wrapper_section     ::= indent "[W]" pipeline_ref NEWLINE
                         { indent wrapper_io_line NEWLINE } ;

wrapper_io_line     ::= "(-)" variable_id assignment_op value_expr ;
                      (* Same (-) IO bracket as top-level io_line — scoped to the parent
                         [W] operator via indentation, not a different marker. *)
```

**Rule:** `[W]` references a wrapper (`{W}`). Wrapper IO is wired using `(-)` with `$` variables. See 9.5 for wrapper definition syntax and IO wiring details.

**Examples:** `[W] -W.Aljam3`, `[W] -W.DB.Connection` with `(-) $connectionString << $connStr`

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

## Related User Documentation

| Section | User Doc |
|---------|----------|
| 9.3 `{-}` Pipeline | [[concepts/pipelines/INDEX\|pipelines]] |
