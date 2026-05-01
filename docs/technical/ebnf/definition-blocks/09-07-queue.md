---
audience: design
type: spec
updated: 2026-04-09
---

<!-- @ebnf/INDEX -->

## 9.7 Queue Definition

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

```aljam3
{Q} #Queue:GPUQueue
   [%] .description << "Queue for GPU-intensive work"
   [.] .strategy#QueueStrategy << #LIFO
   [.] .host#String << "gpu-server-01"
   [.] .maxInstances#UnsignedInt << 1
   [.] .killPropagation#KillPropagation << #Downgrade
   [.] .resourceTags#array:ResourceTag << [#GPU]
   [.] .maxWaitTime#String << "30m"
   [Q] -Q.Kill.Graceful.Time.MoreThan
      (-) <duration << "4h"
```

**Rule:** `{Q}` is both a data definition (`#Queue:*` struct) and a runtime instantiation — unlike `{#}` which only defines a type. `-Q.Default` is the jm3lib-provided queue and does not require a `{Q}` definition.

**Dual-purpose:** `{Q}` serves two roles based on the identifier prefix. The grammar above covers the **data definition** form (`{Q} #Queue:Name`). The **pipeline operation** form (`{Q} -Q.*`) is syntactic sugar for `{-}[Q]` and follows the pipeline definition grammar in 9.3 — it defines a queue control pipeline invocable via `[Q]`. Examples: `{Q} -Q.Default`, `{Q} -Q.Pause.Hard`, `{Q} -Q.Kill.Graceful`.

## Related User Documentation

| Section | User Doc |
|---------|----------|
| 9.7 `{Q}` Queue | [[concepts/pipelines/INDEX\|pipelines]] |
