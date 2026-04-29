---
audience: design
type: spec
updated: 2026-04-09
---

<!-- @ebnf/INDEX -->

## 9.5 Wrapper Definition (`{W}`)

```ebnf
(* Wrappers provide setup/cleanup scope for pipelines.
   See 4.3 for wrapper_def grammar. *)

scope_setup         ::= "[\]" pipeline_ref NEWLINE
                         { indent exec_line NEWLINE }
                      | "[\]" NEWLINE
                         { indent exec_line NEWLINE } ;

scope_cleanup       ::= "[/]" pipeline_ref NEWLINE
                         { indent call_io_line NEWLINE }
                      | "[/]" NEWLINE
                         { indent exec_line NEWLINE } ;

wrapper_io_decl     ::= "(-)" typed_io_param ;
                      (* Wrapper IO uses (-) with <input and >output prefixes — same as pipeline IO *)
```

**Rules:**
- `{W} -W.Name` defines a wrapper. `[W] -W.Name` invokes it inside a pipeline.
- `(-)` declares wrapper IO — inputs use `<` prefix, outputs use `>` prefix, same as pipeline IO.
- `[\]` runs before the pipeline execution body (setup). Can call a single pipeline or open a scope with multiple exec lines.
- `[/]` runs after the pipeline execution body (cleanup). Same structure as `[\]`.
- Wrappers do NOT contain `{#}` definitions, `[T]`, or `[Q]` — those belong to pipelines.
- Execution order: `(-)`/`[T]` -> `[Q]` -> `[\]` -> Execution Body -> `[/]`.
- The wrapper unpacks before and after the body like brackets.
- **Rule (parallel fork):** `[=]` inside `[\]` with no subsequent `[*] *All` in setup forks a parallel execution path. Setup completes and the body begins while the forked path is still running. `[/]` may use `[*] *All` with `(*) << $var` to synchronise with it before proceeding. `[b]` inside `[\]` is fire-and-forget — no collection in `[/]` is possible.
- Variables produced in `[\]` (including by `[=]`) remain accessible in `[/]`.

**Wrapper IO wiring at `[W]` usage site:**

```ebnf
wrapper_section     ::= indent "[W]" pipeline_ref NEWLINE
                         { indent wrapper_io_line NEWLINE } ;

wrapper_io_line     ::= "(-)" variable_id assignment_op value_expr ;
```

At the `[W]` line, wrapper IO is wired using `(-)` with `$` variables:

```aljam3
[W] -W.DB.Connection
   (-) $connectionString << $connStr     (* wrapper input *)
   (-) $dbConn >> $dbConn                (* wrapper output *)
```

**Examples:** `[W] -W.Aljam3` (no IO, no-op wrapper), `[W] -W.DB.Transaction` (with IO wiring)

## Related User Documentation

| Section | User Doc |
|---------|----------|
| 9.5 `{W}` Wrapper | [[concepts/pipelines/wrappers\|wrappers]] |
