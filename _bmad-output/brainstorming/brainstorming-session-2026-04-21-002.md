---
stepsCompleted: [1, 2, 3, 4]
inputDocuments: []
session_topic: 'Chain operator retirement — retire -> in favor of labeled [-] calls'
session_goals: 'Design new vertical chain syntax replacing horizontal -> chains; define compiler parsing rules'
selected_approach: 'ai-recommended'
techniques_used: ['First Principles Thinking', 'Morphological Analysis', 'Constraint Mapping']
ideas_generated: [18]
context_file: 'GitHub Issue #340'
session_active: false
workflow_completed: true
---

# Brainstorming Session Results

**Facilitator:** Hasan AlJamea
**Date:** 2026-04-21

## Session Overview

**Topic:** Vertical chain pipeline syntax with markers
**Goals:** Design new vertical syntax for multi-step chain pipelines; define compiler parsing rules

### Context Guidance

_GitHub Issue #340: The chain operator `->` connects pipelines in sequence, but long chains become unwieldy on a single line. `[~]` continuation can break the line but feels like a workaround. Need native vertical chain syntax that preserves step labels `($)`, step IO `$Step<input`/`$Step>output`, and chain error addressing `.N!ErrorName`._

### Session Setup

_Design tension between horizontal chain expressiveness and vertical readability at scale. Must integrate with Polyglot's existing marker vocabulary and indentation semantics while remaining unambiguous for the compiler._

## Technique Selection

**Approach:** AI-Recommended Techniques
**Analysis Context:** Technical syntax design with compiler parsing constraints

**Recommended Techniques:**

- **First Principles Thinking:** Strip away horizontal assumption, identify what a chain fundamentally IS
- **Morphological Analysis:** Systematically enumerate marker/indentation/wiring combinations
- **Constraint Mapping:** Stress-test candidates against parser ambiguity and existing syntax

## Technique Execution Results

### Phase 1: First Principles Thinking

**Key Ideas Generated:**

**[First Principles #1]**: Retire Chain Operator `->` Entirely
_Concept:_ The `->` chain operator is redundant. Operation labels with `$Label>output` addressing already express sequential data flow. The compiler infers execution order from data dependencies. Multiple `[-]` calls with label wiring replace chains with zero new syntax.
_Novelty:_ Instead of redesigning chains, eliminate the concept. Less language surface, same expressiveness.

**[First Principles #2]**: Any-to-Any Wiring Supersedes Linear Chains
_Concept:_ Labels allow step 3 to reference step 1's output directly — `$Filter<extra << $Read>metadata` — something chains couldn't do without breaking the linear N-to-N+1 contract. Removing chains gains expressiveness.
_Novelty:_ The "fix" is strictly more powerful than what it replaces.

**[First Principles #3]**: Error Scoping Simplifies
_Concept:_ Each `[-]` has standard `[!]` error blocks scoped to that call. No `.N!ErrorName` chain error addressing needed — it was a workaround for bundling N steps into one unit.
_Novelty:_ Removes a special-case error syntax entirely.

**[First Principles #4]**: `[-]` Already Means Sequential
_Concept:_ The `[-]` marker is the sequential execution signal. `[=]` is parallel. Two `[-]` in order = sequential. `->` was redundant with the marker's own semantics.
_Novelty:_ The chain operator was solving a problem that the execution markers already solved at a more fundamental level.

**[First Principles #5]**: Labeled Sequential Calls ARE the Optimal Form
_Concept:_ The "improved" syntax is just standard `[-]` calls with operation labels. No special chain syntax needed. The existing IO marker pattern `(-)` handles labels, inputs, outputs, and fallbacks uniformly. Attempts to compress break mirroring or violate one-expression-per-line.
_Novelty:_ The best chain syntax is no chain syntax at all.

**[First Principles #6]**: Output Declarations Are Wiring Contracts
_Concept:_ `(-) >content` isn't just declaration — it's a referenceable endpoint that supports fallbacks, type annotations, and cross-step addressing. Outputs must be explicit.
_Novelty:_ Outputs pull double duty as documentation AND wiring endpoints.

### Phase 2: Morphological Analysis

**[Morphological #7]**: EBNF Grammar Impact
_Concept:_ Remove `chain_expression`, `chain_label`, `chain_error_ref` productions. `call_expression` and `operation_label` remain unchanged. `[~]` line continuation kept for general use.
_Novelty:_ Grammar shrinks by 4+ productions with no replacement needed.

**[Morphological #8]**: Compiler Phase Simplification
_Concept:_ Lexer removes `->` token. Parser removes chain production rules and `($)`/`(.)` parsing. Semantic analysis removes chain bundling logic and `.N!` resolution. Dependency graph and signal IR already handle `$Label>output` references.
_Novelty:_ Every compiler phase gets simpler. No phase gets more complex.

**[Morphological #9]**: `[~]` Stays, `->` Goes
_Concept:_ `[~]` is general-purpose line continuation — kept. `->` is chain-exclusive — fully retired from lexer.
_Novelty:_ Clean separation: general tools stay, chain-specific tools go.

**[Morphological #10]**: Full Retirement Inventory
_Concept:_ Retired: `->` operator, `($)`/`(.)` step labels, `.N!ErrorName` chain error addressing, chain-as-unit bundling. Kept: `[-]` calls, `(-) $Label` operation labels, `$Label>output`/`$Label<input` addressing, `[!]` standard error scoping, `[~]` continuation.
_Novelty:_ Complete accounting of what goes and what stays.

### Phase 3: Constraint Mapping

**[Constraint #11]**: Clean Retirement Path
_Concept:_ No existing `.pg` code uses chains. Impact is purely documentation and grammar — ~6 spec files to retire/rewrite, ~2 memory files to update, ~4 files to verify. No code migration needed.
_Novelty:_ Retirement cost is documentation-only.

**[Constraint #12]**: EBNF Simplification
_Concept:_ Removing chain productions eliminates `chain_call`, `step_ref`, `chain_io_param`, `chain_error_block` from grammar. Parser complexity drops.
_Novelty:_ Language grammar shrinks, compiler gets simpler.

### Edge Case Exploration

**[Edge Case #13]**: Explicit Wiring Over Auto-Wire
_Concept:_ Chain auto-wire hid data flow. Explicit `(-) IO` wiring makes every connection visible, debuggable, and type-annotated. Losing auto-wire is a feature, not a cost. Aligns with Polyglot's accountability philosophy.
_Novelty:_ Explicit is better — consistent with language philosophy.

**[Edge Case #14]**: Wildcard Auto-Wire `<* << $A>*`
_Concept:_ Explicit wildcard wiring — `$A>*` collects all outputs of step A, `<*` represents all inputs of step B. Compiler matches by data tree topology (bijective and onto type matching). Compile error on mismatch forces explicit per-port wiring.
_Novelty:_ Preserves chain auto-wire convenience as a general-purpose `[-]` feature — not locked behind `->`.

**[Edge Case #15]**: Auto-Wire Compile Error on Mismatch
_Concept:_ If `$A>*` outputs don't bijectively map onto `<*` inputs by type topology, compile error. No partial wiring, no ambiguous many-to-one. Either perfect type match or failure.
_Novelty:_ Error forces developer to explicit wiring — the right behavior when types diverge.

**[Edge Case #16]**: `*` Symbol Consistency
_Concept:_ `*` already means "collect" in Polyglot (`(*) <<`, `(*) >>`, `*All`, `*First`). Using `>*` for "all outputs" and `<*` for "all inputs" is semantically consistent — collection in the IO context.
_Novelty:_ No new symbol — existing `*` semantics extend naturally.

**[Edge Case #17]**: Wildcard Auto-Wire Implies Completion Wait
_Concept:_ `<* << $A>*` requires ALL of A's outputs to be Final before B triggers. This is semantically equivalent to the old chain's "wait for completion" — without a special operator. Strict-serial behavior emerges naturally from wildcard wiring combined with Polyglot's trigger model (launch when prev block launched AND inputs Final).
_Novelty:_ Chain serialization was never a unique capability — it was auto-wiring all outputs, which `<* << $A>*` does explicitly.

**[Edge Case #18]**: Trigger Model Makes Chains Redundant by Design
_Concept:_ Polyglot's trigger model — "launch when prev block launched AND inputs Final" — already encodes sequential dependency through data flow. The `->` chain was a syntactic overlay on a mechanism the runtime already handled. Retirement aligns syntax with the actual execution model.
_Novelty:_ The chain wasn't just redundant with labels — it was redundant with the trigger model itself.

## Idea Organization and Prioritization

### Thematic Organization

**Theme 1: Chain Retirement — Core Thesis (FP#1-6)**
The `->` chain operator is redundant with operation labels, `[-]` sequential semantics, and the trigger model. Labeled `[-]` calls are more powerful (any-to-any wiring), more explicit, and use existing syntax.

**Theme 2: Syntax Removal Inventory (M#7-10)**
Retire `->`, `($)`/`(.)`, `.N!ErrorName`. Keep `[-]`, `(-)`, `$Label>output`, `[!]`, `[~]`. EBNF grammar shrinks, every compiler phase simplifies.

**Theme 3: Constraint Validation (C#11-12)**
No `.pg` code uses chains. Impact is docs-only. Clean retirement path.

**Theme 4: Wildcard Auto-Wire — New Feature (EC#13-18)**
`<* << $A>*` provides chain-like auto-wiring as a general-purpose feature using bijective type-topology matching. `*` symbol is consistent with existing collection semantics. Wildcard wiring naturally produces completion-wait behavior through the trigger model.

### Prioritization Results

**Top Priority: Retire `->` chain operator**
- Resolution for Issue #340: don't refine chains — retire them
- Labeled `[-]` calls with `$Label>output` addressing replace all chain functionality
- Strictly more powerful, less syntax, simpler compiler

**Second Priority: Design wildcard auto-wire `<* << $A>*`**
- New feature that recovers chain auto-wire convenience
- Needs its own design spec (new GitHub issue)
- Bijective type-topology matching, compile error on mismatch

**Third Priority: Documentation and grammar cleanup**
- ~6 spec files to retire/rewrite
- ~2 memory/lesson files to update
- ~4 files to verify for indirect references
- EBNF grammar: remove 4+ chain productions

### Action Plan

1. **Update Issue #340** — Resolution: retire `->` chain operator, replace with labeled `[-]` calls
2. **Create new issue** — Wildcard auto-wire `<* << $A>*` design and spec
3. **Update documentation** — Retire chain spec files, update EBNF grammar
4. **Update memory/lessons** — Replace `pg_lesson_chain_operator_syntax.md` and `pg_lesson_chain_error_addressing.md`

## Summary: The Three-Line Rule

Polyglot doesn't need a chain operator because three existing mechanisms already handle sequential pipeline composition:

| Mechanism | Role |
|---|---|
| `[-]` marker | Declares sequential execution intent |
| `(-) $Label` + `$Label>output` | Names steps and addresses their IO |
| Trigger model (launched + inputs Final) | Orders execution by data readiness |

The answer to "how to improve chain syntax" is: **remove it**.
