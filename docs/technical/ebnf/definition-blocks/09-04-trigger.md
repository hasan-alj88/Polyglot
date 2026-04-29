---
audience: design
type: spec
updated: 2026-04-09
---

<!-- @ebnf/INDEX -->

## 9.4 Trigger Definition (`{T}`)

```ebnf
trigger_def         ::= "{T}" trigger_pipeline_id NEWLINE
                         trigger_def_body ;

trigger_pipeline_id ::= '-' 'T' '.' dotted_name ;

trigger_def_body    ::= { ( metadata_line | comment_line ) NEWLINE }
                         { indent ( io_decl_line | error_decl_line ) NEWLINE } ;
                      (* Trigger definitions contain ONLY metadata, IO declarations,
                         and error declarations. No execution body, no [Q], no [W]. *)
```

**Rules:**
- `{T}` defines a trigger pipeline — a subtype of `{-}` constrained to IO-only bodies.
- Trigger identifier must use the `-T.` prefix.
- Must include `>IsTriggered#bool` output (mandatory). May include additional outputs.
- No execution body, no `[Q]`, no `[W]` — triggers define signal sources, not execution logic.
- `[T]` invokes a trigger inside a pipeline (see 9.3.1).

**Example:**

```aljam3
{T} -T.Folder.NewFiles
   [%] .description << "Fires when new files appear in watched directory"
   (-) <path#path
   (-) >IsTriggered#bool
   (-) >NewFiles#array:path
```

## Related User Documentation

| Section | User Doc |
|---------|----------|
| 9.4 `{T}` Trigger | [[concepts/pipelines/io-triggers\|io-triggers]] |
