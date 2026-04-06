---
issue: 150
group: 1
group_name: "EBNF / Compiler Rule Gaps"
priority: P2-high
status: brief-ready
---

# Issue #150: PGW01002 and PGE01021 duplicate -- warning vs error ambiguity

## Inconsistency
Both PGW01002 (warning) and PGE01021 (error) cover empty `{#}` data definitions. PGE01021 explicitly states it "supersedes" PGW01002, and the COMPILE-RULES.md index marks PGW01002 as "superseded by PGE01021". However, PGW01002 still exists as an active file with full examples, diagnostic text, and its own "Open point" note acknowledging the supersession. STATE.md records the decision "PGW01002 superseded by PGE01021" from the #99-#106 batch. The continued presence of PGW01002 as a seemingly active rule file creates ambiguity about whether empty `{#}` blocks produce a warning or an error.

## Affected Files
| File | What's Wrong |
|------|-------------|
| `docs/technical/compile-rules/PGW/PGW01002-empty-data-definition.md` | Still exists as a full rule file with examples, despite being superseded; frontmatter has `superseded_by: PGE01021` but file is not marked deprecated or archived |
| `docs/technical/compile-rules/PGE/PGE01021-empty-data-definition.md` | Correctly states it supersedes PGW01002; frontmatter has `supersedes: PGW01002` |
| `docs/technical/COMPILE-RULES.md` | Lists both rules: PGE01021 at line ~50 and `PGW01002 \| Empty Data Definition *(superseded by PGE01021)*` at line ~242 |
| `docs/technical/ebnf/09-definition-blocks.md` | References PGE01021 only (correct): `(* At least one body line required -- empty {#} is PGE01021 *)` |

## Example
**Source A** (`docs/technical/compile-rules/PGW/PGW01002-empty-data-definition.md`, line ~10-11):
> ### Rule 1.2w -- Empty Data Definition *(superseded)*
> `PGW01002` -> **Superseded by [PGE01021](../PGE/PGE01021-empty-data-definition.md)**

**Source B** (same file, line ~44-46):
> **WARNING:**
> ```polyglot
> [ ] PGW01002 -- no field declarations
> {#} #UserRecord
> ```

**Source C** (`docs/technical/compile-rules/PGE/PGE01021-empty-data-definition.md`, line ~17):
> **Supersedes:** PGW01002 -- Empty Data Definition (warning). The grammar now rejects empty `{#}` at parse time.

**Source D** (`docs/technical/COMPILE-RULES.md`, line ~242):
> `| PGW01002 | Empty Data Definition *(superseded by PGE01021)* |`

The EBNF correctly references only PGE01021, but the compile-rules index still lists PGW01002 as an entry (albeit marked superseded), and the PGW01002 file itself still contains full warning examples that contradict PGE01021's error classification.

## Prior Related Work
- Issues #99-#106 -- EBNF edge cases batch; STATE.md records: "PGW01002 superseded by PGE01021" and "Empty {#} upgraded from warning to error; EBNF tightened"
- Issue #88 -- Schema properties work that established the `{#}` definition structure
- STATE.md decision (2026-03-30): "PGW01002 superseded by PGE01021"

## Recommendation
Since STATE.md explicitly records the decision and PGE01021 is the authoritative rule, PGW01002 should be either: (1) Deleted entirely, with PGE01021 kept as the sole reference, and the COMPILE-RULES.md index entry for PGW01002 replaced with a "Retired" note pointing to PGE01021. Or (2) Reduced to a stub file containing only a redirect notice (no examples, no diagnostic) to preserve the rule code for historical traceability. The COMPILE-RULES.md warning table should mark PGW01002 as retired/removed rather than listing it with its original name. Either way, no file should contain warning-level examples for empty `{#}` definitions -- the grammar now rejects them as errors.

## Discussion Prompts
1. Should superseded rules be deleted or kept as historical stubs?
2. Does the COMPILE-RULES.md index need a "Retired Rules" section for superseded codes?
3. Are there any other PGW rules that have been similarly superseded but not cleaned up?

---
*Brief prepared: 2026-04-06*
*Start work: /paul:work-issue 150*
