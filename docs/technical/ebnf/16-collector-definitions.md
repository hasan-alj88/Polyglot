---
audience: design
type: spec
updated: 2026-04-11
---

<!-- @ebnf/INDEX -->
<!-- @u:technical/spec/collector-definitions -->

## 16. Collector Definitions

### 16.1 Collector Block

```ebnf
collector_def       ::= '{*}' collector_id NEWLINE
                         collector_metadata
                         collector_io
                         collector_body ;

collector_id        ::= '*' dotted_name ;
```

**Rule:** `{*}` defines a collector — a first-class definition block for collector logic. `{*}` blocks are self-contained: no `[T]` external triggers, no `[Q]`, no `[W]`. The collector's triggers are arrivals (`*Arrive`, `*Job.Arrive`).

**Rule:** `{*}` creates a branch on the `%` metadata tree at `%*`.

### 16.2 Collector Metadata

```ebnf
collector_metadata  ::= { '[%]' '.' metadata_field assignment_op value_expr NEWLINE } ;
```

**Rule:** Three mandatory `[%]` fields: `.category` (`#CollectorCategory`), `.scope` (`#CollectorScope`), `.overflow` (`#OverflowStrategy`). Optional: `.batchable` (`#bool`). Missing mandatory fields trigger PGE03013.

### 16.3 Collector IO

```ebnf
collector_io        ::= { '(*)' collector_io_line NEWLINE } ;
collector_io_line   ::= '<' port_name [ '#' type_ref ]
                      | '>' port_name [ '#' type_ref ]
                      | '!' error_ref ;
```

**Rule:** `(*) <Incoming#IncomingDataFrame` is mandatory — missing triggers PGE03018.

**Rule:** `(*)` IO uses the same bracket as collector invocation. Inputs use `<`, outputs use `>`, errors use `!`.

### 16.4 Collector Body

```ebnf
collector_body      ::= { collector_var_decl | collector_trigger } ;
collector_var_decl  ::= '[-]' variable_decl NEWLINE ;
```

**Rule:** Only `[-]` variable declarations and `[T]` trigger blocks are valid at the top level of a `{*}` body. Any other statements trigger PGE03020. `[=]` parallel execution is forbidden (PGE03021).

### 16.5 Arrival Triggers

```ebnf
collector_trigger   ::= '[T]' arrival_condition NEWLINE
                         trigger_io
                         { trigger_body_line }
                         { trigger_error_handler } ;

arrival_condition   ::= var_arrive | job_arrive ;
```

**Rule:** `[T]` inside `{*}` takes an arrival condition, not an external trigger reference. External trigger sources (`=T.*`) trigger PGE03022.

### 16.5.1 Trigger IO

```ebnf
trigger_io          ::= { '(T)' '>' port_name NEWLINE } ;
```

**Rule:** `(T)` declares arrival data as `$` variables inside the trigger block. For single arrivals: `>var` and `>job`. For range arrivals: `>vars` and `>jobs` (iterable collections).

### 16.5.2 Trigger Body

```ebnf
trigger_body_line   ::= sequential_call | expander_call | assignment
                      | output_push | release_line ;
release_line        ::= '[*]' ( job_release | var_job_release ) NEWLINE ;
```

**Rule:** `[*]` inside a `{*}` trigger block is a release command — it releases collector claims on jobs. Every code path must release all jobs `[0,N]` (PGE03025).

### 16.5.3 Trigger Error Handlers

```ebnf
trigger_error_handler ::= '[!]' error_ref NEWLINE
                           { indent handler_body_line NEWLINE } ;
```

**Rule:** `[T]` blocks CAN have `[!]` error handlers. If `$var.status == #JobStatus.Failed`, the trigger fires with error context.

### 16.6 Variable-Centric Operations

```ebnf
var_arrive          ::= '*Arrive' '"' index_expr '"' ;
var_job_release     ::= '*Arrive.Job.Release' '"' index_expr '"' ;
```

### 16.7 Job-Centric Operations

```ebnf
job_arrive          ::= '*Job.Arrive' '"' index_expr '"' ;
job_arrive_vars     ::= '*Job.Arrive.Vars' '"' index_expr '"' ;
job_release         ::= '*Job.Release' '"' index_expr '"' ;
```

### 16.8 Index Expressions

```ebnf
index_expr          ::= integer_literal
                      | 'N'
                      | '{' variable_ref '}'
                      | range_expr ;

range_expr          ::= '[' bound ',' bound ']'
                      | '(' bound ',' bound ')'
                      | '[' bound ',' bound ')'
                      | '(' bound ',' bound ']' ;
bound               ::= integer_literal | 'N' | '{' variable_ref '}' ;
```

**Rule:** `N` resolves to the total count — of variables for `*Arrive`, of jobs for `*Job.Arrive`. Range expressions use standard mathematical interval notation (inclusive `[]`, exclusive `()`).

---
