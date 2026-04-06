---
issue: 142
group: 2
group_name: "Syntax Documentation Gaps"
priority: P3-medium
status: brief-ready
---

# Issue #142: Queue definition syntax ambiguous (#Queue: vs {Q} block)

## Inconsistency
PGE01012 requires the `#Queue:` prefix for `{Q}` definitions, and blocks.md documents `{Q}` as a "dual-purpose block" where `{Q} #Queue:Name` is a data definition and `{Q} =Q.*` is a pipeline operation. However, the SPEC-INDEX.md (user learning path) never mentions this dual-purpose distinction or the `#Queue:` prefix requirement, and the PGE01012 compile rule's INVALID example shows `{Q} =Q.MyQueue` as wrong, which contradicts the documented `{Q} =Q.*` pipeline operation form. The issue is that PGE01012 was written before the dual-purpose design was formalized in issue #113, and its INVALID examples now conflict with legitimate syntax.

## Affected Files
| File | What's Wrong |
|------|-------------|
| `docs/technical/compile-rules/PGE/PGE01012-queue-definition-prefix.md` | INVALID example shows `{Q} =Q.MyQueue` as error, but `{Q} =Q.*` is valid pipeline operation form per blocks.md and EBNF |
| `docs/user/syntax/blocks.md` | Dual-purpose correctly documented but dense; no cross-reference to PGE01012's scope limitation |
| `docs/user/SPEC-INDEX.md` | No mention of queue definition syntax or `{Q}` dual-purpose anywhere in the learning path |
| `docs/technical/ebnf/09-definition-blocks.md` | Section 9.5 documents both forms correctly but the grammar rule `queue_id` only covers `#Queue:name`, not the `=Q.*` form |

## Example
**Source A** (`docs/technical/compile-rules/PGE/PGE01012-queue-definition-prefix.md`, line ~33):
> ```polyglot
> [ ] wrong prefix
> {Q} =Q.MyQueue
>    [.] .strategy;#QueueStrategy << #LIFO
> ```

**Source B** (`docs/user/syntax/blocks.md`, line ~26):
> `{Q}` Queue -- dual-purpose block. `{Q} #Queue:Name` defines a queue instance (subtype of `{#}`, data definition). `{Q} =Q.*` defines a queue pipeline operation (subtype of `{=}`, equivalent to `{=}[Q]`). The identifier prefix (`#` vs `=`) disambiguates.

**Source C** (`docs/technical/ebnf/09-definition-blocks.md`, line ~387):
> **Dual-purpose:** `{Q}` serves two roles based on the identifier prefix. The grammar above covers the **data definition** form (`{Q} #Queue:Name`). The **pipeline operation** form (`{Q} =Q.*`) is syntactic sugar for `{=}[Q]` and follows the pipeline definition grammar in SS 9.3

## Prior Related Work
- Issue #113 -- Unified `{Q}` dual-purpose documentation (closed 2026-04-01). This is the issue that formalized the dual-purpose design, but PGE01012 was not updated to reflect the new understanding.

## Recommendation
Update PGE01012 to scope its rule specifically to the **data definition** form (`{Q} #Queue:Name`) and add a note that the pipeline operation form (`{Q} =Q.*`) is a separate grammar path governed by SS 9.3. Remove or relabel the `{Q} =Q.MyQueue` INVALID example (it is only invalid when used with `[.]` data fields, not as a pipeline definition). Optionally add a "Queue Syntax" entry to SPEC-INDEX.md's Phase 2 table linking to queue.md.

## Discussion Prompts
1. Should PGE01012's scope statement explicitly mention that it applies only to `{Q} #Queue:*` data definitions, not `{Q} =Q.*` pipeline definitions?
2. Does the EBNF need a separate `queue_pipeline_def` production rule, or is the current prose note in SS 9.5 sufficient?
3. Is this purely a text fix (PGE01012 was not updated after #113), or does the compiler implementation also need to distinguish the two forms?

---
*Brief prepared: 2026-04-06*
*Start work: /paul:work-issue 142*
