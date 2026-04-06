---
issue: 145
group: 2
group_name: "Syntax Documentation Gaps"
priority: P3-medium
status: brief-ready
---

# Issue #145: Wait/collect markers [<]/[>] vs [*]<</>>/>> undocumented relationship

## Inconsistency
The blocks.md Data Flow registry lists two overlapping marker sets for wait/collect semantics that serve completely different purposes but share visual similarity. The `[*] <<` and `[*] >>` forms are collect-all/race IO markers (wait for Final, collect output). The `[<]` and `[>]` markers are error fallback operators scoped under `[=]` IO lines. Despite the superficial resemblance (`[<]` vs `[*] <<`), these are unrelated features -- but the blocks.md table lists them adjacent to each other in the same "Data Flow" section with no disambiguation note. The memory lesson `pg_lesson_wait_collect_markers.md` states "[<] wait marker keeps var accessible; [>] collect-into cancels inputs" which conflates the fallback markers with the collector semantics, further confusing the relationship.

## Affected Files
| File | What's Wrong |
|------|-------------|
| `docs/user/syntax/blocks.md` | Lists `[*] <<`/`[*] >>` and `[<]`/`[>]` in same "Data Flow" section (lines 66-69) with no disambiguation |
| `docs/user/syntax/io.md` | Documents `[*] <<`/`[*] >>` under "Wait and Collect IO" (line 142) and `[>]`/`[<]` under "Fallback IO" (line 176) -- correct but no cross-reference to prevent confusion |
| `docs/user/concepts/collections/collect.md` | Documents `[*] <<`/`[*] >>` thoroughly but never mentions that `[<]`/`[>]` exist as separate features |

## Example
**Source A** (`docs/user/syntax/blocks.md`, lines ~66-69):
> | `[*] <<` | Wait input -- wait for variable to be Final (used inside `[*]` blocks). See [[concepts/collections/collect#Collect-All & Race Collectors]] |
> | `[*] >>` | Collect output -- in race blocks, losing inputs cancelled, output receives winner. See [[concepts/collections/collect#Collect-All & Race Collectors]] |
> | `[>]` | Output fallback -- scoped under `[=]` output line. See [[errors#Error Fallback Operators]] |
> | `[<]` | Input fallback -- scoped under `[=]` input line. See [[errors#Error Fallback Operators]] |

**Source B** (`docs/user/syntax/io.md`, lines ~147-148):
> | `[*] << $var` | **Wait input** -- waits for `$var` to be Final. Variable **stays accessible** after. |
> | `[*] >> $var` | **Collect output** -- in race collectors, losing inputs are **cancelled**; only the `>>` output survives. |

**Source C** (`docs/user/syntax/io.md`, lines ~198-199):
> | `[>] <! value` | **Generic fallback** -- activates for any unhandled error |
> | `[>] <!Error.Name value` | **Error-specific fallback** -- activates only for the named error |

## Prior Related Work
- Issue #126 -- `[=]` IO marker scoping rule (closed 2026-04-05). Clarified that `[=]`/`[~]`/`[*]` scope to parent operator via indentation. This established the principle that IO markers inherit context from their parent, which applies to `[*] <<`/`[*] >>` but not to `[<]`/`[>]` (which scope under `[=]` lines instead).
- Issue #125 -- `*All` collect-all terminology (closed 2026-04-05). Renamed "sync barrier" to "collect-all"; subcategorized `%*` into Data/Collect-all/Race.

## Recommendation
Add a disambiguation note to blocks.md's Data Flow section that explicitly states: `[*] <<` / `[*] >>` are collector IO markers used inside `*All`/`*First`/`*Nth` blocks (concurrency), while `[<]` / `[>]` are error fallback markers scoped under `[=]` IO lines (error recovery). These are unrelated features despite visual similarity. Additionally, the memory lesson `pg_lesson_wait_collect_markers.md` should be reviewed -- its description conflates the two marker sets. Consider splitting the Data Flow section in blocks.md into two subsections: "Collection IO" (`[*] <<`, `[*] >>`) and "Fallback IO" (`[<]`, `[>]`) to make the separation visually clear.

## Discussion Prompts
1. Should blocks.md's Data Flow section be split into "Collection IO" and "Fallback IO" subsections, or is a disambiguation note sufficient?
2. Is the memory lesson `pg_lesson_wait_collect_markers.md` conflating the two features, and if so, should it be corrected or split into two separate lessons?
3. Is there a deeper design relationship between these marker sets (both deal with "waiting for values"), or are they genuinely orthogonal features that happen to share visual patterns?

---
*Brief prepared: 2026-04-06*
*Start work: /paul:work-issue 145*
