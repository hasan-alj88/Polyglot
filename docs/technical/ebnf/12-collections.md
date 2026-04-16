---
audience: designer
type: spec
updated: 2026-04-17
---

<!-- @ebnf/INDEX -->

## 12. Collection Operations

### 12.1 Expand Operators (`=`)

```ebnf
expand_line         ::= ( "[-]" | "[=]" ) expand_invocation NEWLINE
                         { indent expand_io_line NEWLINE }
                         { indent exec_line NEWLINE } ;

expand_invocation   ::= '=' expand_operator ;

expand_operator     ::= "ForEach.Array"
                      | "ForEach.Array.Enumerate"
                      | "ForEach.Map"
                      | "ForEach.Serial"
                      | "ForEach.Level"
                      | "ForEach.Dataframe"
                      | "ForEach.Dataframe.Enumerate"
                      | "ForEach.Dataframe.Column" ;

expand_io_line      ::= "(=)" io_param assignment_op value_expr ;

level_input         ::= "<level" assignment_op type_path ".=" ;
                        (* .= marks the level iteration point, analogous to .* wildcard *)
```

**Execution marker on expand controls parallelism:**
- `[-]` — mini-pipelines run sequentially.
- `[=]` — mini-pipelines run in parallel.

**`=ForEach.Level` level iteration marker:** The `.=` suffix on the input path marks the level iteration point — analogous to `.*` wildcard, `.=` means "expand siblings at this level." Grammar: `level_input` production above. Example: `<level << #SomeData.SubField.=`

### Expand IO Signatures

| Operator | Inputs | Outputs |
|----------|--------|---------|
| `=ForEach.Array` | `<Array` | `>item` |
| `=ForEach.Array.Enumerate` | `<Array` | `>index`, `>item` |
| `=ForEach.Map` | `<Map` | `>key`, `>item` |
| `=ForEach.Serial` | `<Serial` | `>key`, `>item` |
| `=ForEach.Level` | `<level` (`.=` suffix) | `>key`, `>item` |
| `=ForEach.Dataframe` | `<Dataframe` | `>row` |
| `=ForEach.Dataframe.Enumerate` | `<Dataframe` | `>index`, `>row` |

### 12.2 Collect Operators (`*`)

```ebnf
collect_line        ::= ( "[-]" | "[=]" ) collect_invocation NEWLINE
                         { indent collect_io_line NEWLINE } ;

collect_invocation  ::= '*' collect_operator ;

collect_operator    ::= into_operator | agg_operator | sync_operator | race_operator
                      | discard_operator ;

discard_operator    ::= "Ignore" ;

into_operator       ::= "Into.Array"
                      | "Into.Map"
                      | "Into.Serial"
                      | "Into.Level"
                      | "Into.Dataframe" ;

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

collect_io_line     ::= "(*)" io_param assignment_op value_expr   (* named param: (*) <n#int << 2 *)
                      | "(*)" "<<" variable_ref                  (* wait input: (*) << $var *)
                      | "(*)" ">>" variable_ref ;                (* collect output: (*) >> $winner *)
```

**Rule:** Collector invocation uses an execution marker (`[-]` sequential, `[=]` parallel). `[=]` and `[b]` mean "run in parallel with the next `[=]` or `[b]` sibling at the same indentation level." A `[=]` or `[b]` line whose next sibling does not use `[=]` or `[b]` is a compile error (`PGE01040`) — there is nothing to parallelize against. `[-]` and `[*]` are inherently sequential. Use `[-]` for standalone collectors or when collectors depend on each other; use `[=]` when multiple sibling collectors are independent. Collector IO lines use `(*)` (matching the `*` operator prefix).

**Rule:** IO brackets always match the operator prefix: `(-)` for pipelines, `(=)` for expand, `(*)` for collect.

**Rule:** Collector outputs can write directly to a pipeline output port: `(*) >result >> >pipelineOutput`.

**Rule:** `(*) <<` = wait input — waits for variable to be Final; variable stays accessible after. `(*) >>` = collect output — in race collectors, losing inputs are cancelled; only the `>>` output survives. These are distinct from `(>)`/`(<)` IO parameter handling markers (see §5).

**Rule:** `(*) << $var` lines map to positional implicit input parameters (`<args.0`, `<args.1`, ...) — the compiler infers each parameter's type from the referenced variable. For single-output collectors (`*First`, `*Second`), the output type is also inferred from the input type.

**Rule:** `*All` uses `(*) <<` only (no `(*) >>`). `*First`/`*Second`/`*Nth` require both `(*) <<` inputs and `(*) >>` output.

### Collect IO Signatures

| Operator | Inputs | Outputs | Context |
|----------|--------|---------|---------|
| `*Into.Array` | `<item` | `>Array` | Inside `=ForEach` |
| `*Into.Map` | `<key`, `<value` | `>Map` | Inside `=ForEach` |
| `*Into.Serial` | `<key`, `<value` | `>Serial` | Inside `=ForEach` |
| `*Into.Level` | `<key`, `<value` | `>Serial` | Inside `=ForEach` |
| `*Into.Dataframe` | `<row` | `>Dataframe` | Inside `=ForEach` |
| `*Agg.Sum` | `<number` | `>sum` | Inside `=ForEach` |
| `*Agg.Count` | `<item` | `>count` | Inside `=ForEach` |
| `*Agg.Average` | `<number` | `>average` | Inside `=ForEach` |
| `*Agg.Max` | `<number` | `>max` | Inside `=ForEach` |
| `*Agg.Min` | `<number` | `>min` | Inside `=ForEach` |
| `*Agg.Concatenate` | `<string` | `>result` | Inside `=ForEach` |
| `*All` | `(*) << $var...` | none — vars stay accessible | Parallel `[=]` sync |
| `*First` | `(*) << $var...` | `(*) >> $winner` | Parallel `[=]` race |
| `*Second` | `(*) << $var...` | `(*) >> $winner` | Parallel `[=]` race |
| `*Nth` | `<n#int`, `(*) << $var...` | `(*) >> $winner` | Parallel `[=]` race |
| `*Ignore` | `(*) << $var...` | none | Parallel `[=]` discard |

**Rule:** `*Ignore` is an explicit discard collector. It takes `(*) <<` wait inputs only and produces no outputs. Use for parallel output that exists for debugging but is intentionally unused. Prefer `$*` inline discard when the value is never needed.

### 12.3 Reassemble Operators (`=*`)

```ebnf
reassemble_line         ::= ( "[-]" | "[=]" ) reassemble_invocation NEWLINE
                             indent reassemble_expand_io NEWLINE
                             indent reassemble_collect_io NEWLINE ;

reassemble_invocation   ::= '=' '*' reassemble_operator ;

reassemble_operator     ::= reassemble_agg | reassemble_into ;

reassemble_agg          ::= "Agg.Sum"
                          | "Agg.Count"
                          | "Agg.Average"
                          | "Agg.Max"
                          | "Agg.Min"
                          | "Agg.Concatenate" ;

reassemble_into         ::= "Into.Array"
                          | "Into.Map"
                          | "Into.Dataframe" ;

reassemble_expand_io    ::= "(=)" "<" param_name assignment_op value_expr ;
                            (* expander input — source collection *)

reassemble_collect_io   ::= "(*)" ">" param_name ">>" variable_ref ;
                            (* collector output — aggregated result *)
```

**Rule:** Reassemble operators combine an expander and collector into a single atomic operation. The `=*` prefix reads as "expand, then collect" — fan-out followed by fan-in with no intermediate body logic.

**Rule:** The compiler expands `=*` into the equivalent `=ForEach` + `*` pair. No new runtime instruction is created — `=*` is syntactic sugar.

**Rule:** `=*` has no intermediate body. The expander feeds directly into the collector. If per-item logic is needed (conditionals, pipeline calls, error handling), use the full `=` ... `*` form.

**Rule:** IO brackets follow the operator prefix convention: `(=)` for the expander input, `(*)` for the collector output. This matches the dual nature of the operator.

**Rule:** Execution marker controls the internal expand step: `[-]` runs items sequentially, `[=]` runs items in parallel.

### Reassemble IO Signatures

| Operator | Expander Input `(=)` | Collector Output `(*)` | Equivalent |
|----------|---------------------|----------------------|------------|
| `=*Agg.Sum` | `<array` | `>sum` | `=ForEach.Array` + `*Agg.Sum` |
| `=*Agg.Count` | `<array` | `>count` | `=ForEach.Array` + `*Agg.Count` |
| `=*Agg.Average` | `<array` | `>average` | `=ForEach.Array` + `*Agg.Average` |
| `=*Agg.Max` | `<array` | `>max` | `=ForEach.Array` + `*Agg.Max` |
| `=*Agg.Min` | `<array` | `>min` | `=ForEach.Array` + `*Agg.Min` |
| `=*Agg.Concatenate` | `<array` | `>result` | `=ForEach.Array` + `*Agg.Concatenate` |
| `=*Into.Array` | `<Map` \| `<Serial` \| `<Dataframe` | `>Array` | `=ForEach.*` + `*Into.Array` |
| `=*Into.Map` | `<Array` \| `<Serial` \| `<Dataframe` | `>Map` | `=ForEach.*` + `*Into.Map` |
| `=*Into.Dataframe` | `<Array` \| `<Map` \| `<Serial` | `>Dataframe` | `=ForEach.*` + `*Into.Dataframe` |

**Schema Enforcement:** The `(=)` expander input is typed to a specific collection schema — `<Collection.Array` requires `##Array`, `<Collection.Serial` requires `##Serial`, etc. Passing an incompatible collection type is caught by standard type checking (`PGE04001`). The per-iteration output shape derives from the collection schema's `%##Active << #ActiveKind.One` property: `##Array` yields a single `>item`, while `##Serial` and `##Record` yield `>key` + `>value` pairs. The compiler desugars `=*` into the equivalent `=ForEach.*` + `*` pair and validates the IO wiring between them using the same type rules as any other pipeline IO. This table documents what the schemas already enforce — no separate reassemble validation layer exists.

---
