---
audience: designer
type: spec
updated: 2026-03-30
---

<!-- @ebnf/INDEX -->

## 12. Collection Operations

### 12.1 Expand Operators (`~`)

```ebnf
expand_line         ::= ( "[r]" | "[p]" ) expand_invocation NEWLINE
                         { indent expand_io_line NEWLINE }
                         { indent exec_line NEWLINE } ;

expand_invocation   ::= '~' expand_operator ;

expand_operator     ::= "ForEach.Array"
                      | "ForEach.Array.Enumerate"
                      | "ForEach.Map"
                      | "ForEach.Serial"
                      | "ForEach.Level"
                      | "ForEach.Dataframe"
                      | "ForEach.Dataframe.Enumerate"
                      | "ForEach.Dataframe.Column" ;

expand_io_line      ::= "[~]" io_param assignment_op value_expr ;
```

**Execution marker on expand controls parallelism:**
- `[r]` — mini-pipelines run sequentially.
- `[p]` — mini-pipelines run in parallel.

**`~ForEach.Level` special input syntax:** The `~` suffix on input marks the iteration point: `<level << #SomeData.SubField.~`

### Expand IO Signatures

| Operator | Inputs | Outputs |
|----------|--------|---------|
| `~ForEach.Array` | `<Array` | `>item` |
| `~ForEach.Array.Enumerate` | `<Array` | `>index`, `>item` |
| `~ForEach.Map` | `<Map` | `>key`, `>item` |
| `~ForEach.Serial` | `<Serial` | `>key`, `>item` |
| `~ForEach.Level` | `<level` | `>key`, `>item` |
| `~ForEach.Dataframe` | `<Dataframe` | `>row` |
| `~ForEach.Dataframe.Enumerate` | `<Dataframe` | `>index`, `>row` |

### 12.2 Collect Operators (`*`)

```ebnf
collect_line        ::= ( "[r]" | "[p]" ) collect_invocation NEWLINE
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

collect_io_line     ::= "[*]" io_param assignment_op value_expr   (* named param: [*] <n#int << 2 *)
                      | "[*]" "<<" variable_ref                  (* wait input: [*] << $var *)
                      | "[*]" ">>" variable_ref ;                (* collect output: [*] >> $winner *)
```

**Rule:** Collector invocation uses an execution marker (`[r]` sequential, `[p]` parallel) — same as expand. Collector IO lines use `[*]` (matching the `*` operator prefix). Use `[r]` when collectors depend on each other, `[p]` when independent.

**Rule:** IO markers always match the operator prefix: `[=]` for pipelines, `[~]` for expand, `[*]` for collect.

**Rule:** Collector outputs can write directly to a pipeline output port: `[*] >result >> >pipelineOutput`.

**Rule:** `[*] <<` = wait input — waits for variable to be Final; variable stays accessible after. `[*] >>` = collect output — in race collectors, losing inputs are cancelled; only the `>>` output survives. These are distinct from `[>]`/`[<]` IO parameter handling markers (see §5).

**Rule:** `[*] << $var` lines map to positional implicit input parameters (`<args.0`, `<args.1`, ...) — the compiler infers each parameter's type from the referenced variable. For single-output collectors (`*First`, `*Second`), the output type is also inferred from the input type.

**Rule:** `*All` uses `[*] <<` only (no `[*] >>`). `*First`/`*Second`/`*Nth` require both `[*] <<` inputs and `[*] >>` output.

### Collect IO Signatures

| Operator | Inputs | Outputs | Context |
|----------|--------|---------|---------|
| `*Into.Array` | `<item` | `>Array` | Inside `~ForEach` |
| `*Into.Map` | `<key`, `<value` | `>Map` | Inside `~ForEach` |
| `*Into.Serial` | `<key`, `<value` | `>Serial` | Inside `~ForEach` |
| `*Into.Level` | `<key`, `<value` | `>Serial` | Inside `~ForEach` |
| `*Into.Dataframe` | `<row` | `>Dataframe` | Inside `~ForEach` |
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
| `*Ignore` | `[*] << $var...` | none | Parallel `[p]` discard |

**Rule:** `*Ignore` is an explicit discard collector. It takes `[*] <<` wait inputs only and produces no outputs. Use for parallel output that exists for debugging but is intentionally unused. Prefer `$*` inline discard when the value is never needed.

---
