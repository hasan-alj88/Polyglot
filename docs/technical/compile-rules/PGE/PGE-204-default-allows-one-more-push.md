---
rule: "2.4"
code: PGE-204
name: Default Allows Exactly One More Push
severity: error
status: retired
retired: 2026-03-19
superseded_by: PGE-203
---

### Rule 2.4 — Default Allows Exactly One More Push *(retired)*
`PGE-204` → folded into `PGE-203`

**This rule has been retired.** All cases previously covered by PGE-204 are now enforced by [PGE-203 — Final Is Push-Once](PGE-203-final-is-push-once.md):

- **Double-default** (`<~ → <~`): PGE-203 now rejects re-defaulting a Default variable
- **Post-promotion push** (Default → Final → push): Already fired PGE-203

See resolved design issue 008 (git history: `docs/technical/compiler_issues/008-pge203-pge204-overlap.md`) for rationale.
