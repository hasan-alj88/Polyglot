---
phase: 356-audit-compiler-compile-rules
plan: 01
completed: 2026-04-23
status: complete
---

# Plan 356-01 Summary — Heading + Frontmatter Normalization

## What was done

**Task 1 — H3 → H1 rule-title promotion**
- Promoted `### Rule X.Y — Name` → `# Rule X.Y — Name` across 183 PGE/PGW files.
- Promoted `### See Also` → `## See Also` in the same set (105 of the 183 had a See Also trailer).
- **Scope deviation (minor):** also promoted `### See Also` → `## See Also` in 4 already-H1 outliers (PGE01040, PGE07007, PGE07008, PGE07009) where See Also was stuck at H3 inside an `## Examples` parent. AC-2 requires zero `### See Also` anywhere; the plan's Task 1 action was scoped to H3-first files only, so these 4 would have been missed. Fixed in-task to satisfy AC-2 literally.

**Task 2 — PGE12xxx rule-number correction**
- Fixed frontmatter `rule:` field and `# Rule X.Y — Name` heading text in 9 files (PGE12001, 12002, 12003, 12005–12010).
- Correct values (`rule:` and heading) now match the file code: `PGE12NNN` → `Rule 12.N`.
- PGE12004 already correct; PGE14xxx untouched (all 11 files verified correct during planning).

## Verification (all ACs pass)

| AC | Check | Result |
|----|-------|--------|
| AC-1 | `grep -l "^### Rule " PGE/*.md PGW/*.md` | 0 (was 183) |
| AC-1 | `grep -l "^# Rule " PGE/*.md PGW/*.md` | 217 |
| AC-2 | `grep -l "^### See Also" PGE/*.md PGW/*.md` | 0 (was 109) |
| AC-3 | Spot-check diffs (PGE01005, PGW02002) | Only heading swaps, no body changes |
| AC-4 | `grep -E '^rule: "10\.' PGE/PGE12*.md` | 0 |
| AC-4 | Per-file rule-code match loop | 10 OK / 10 files |
| AC-5 | `grep -E "^# Rule 10\." PGE/PGE12*.md` | 0 |
| Boundary | `git diff docs/technical/compile-rules/PGE/PGE14*.md` | empty |

## Diff stats

```
195 files changed, 309 insertions(+), 309 deletions(-)
```

- Task 1: 183 H3-first files + 4 See Also outliers = 187 distinct files.
- Task 2: PGE12001, 12003, 12005–12010 = 8 files not in Task 1; PGE12002 overlaps Task 1 (1 file).
- Total distinct: 187 + 8 = **195** ✓ matches git stats.

## Anomalies noted (but not addressed — out of scope)

1. Four already-H1 files had structurally-nested `### See Also` inside `## Examples` — promoted to `## See Also` (new top-level trailer section). Might want to also move the section out of the Examples subsection semantically, but text order unchanged — cosmetic improvement for later.
2. PGE12xxx legacy tracking: `split_from:` frontmatter on some PGE12xxx files may still reference the old 10-range. Not in scope for this plan; revisit if Plan 356-02 index generation surfaces issues.

## Baseline state for follow-on plans

Plan 356-02 (directory indexes) and Plan 356-03 (glossary / stub / wikilinks) can proceed on a normalized baseline:
- All 217 rule files open at H1 with rule number matching file code.
- All See Also trailers at H2.
- PGE12xxx frontmatter `rule:` field is now authoritative for index generation.

## Next

- Commit the 195-file diff on branch `docs/issue-356-audit-compiler-compile-rules`.
- `/paul:unify` then `/paul:plan` to draft Plan 356-02 (directory indexes).
