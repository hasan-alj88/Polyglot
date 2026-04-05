---
audience: developer
rule: "2.4"
code: PGE02004
name: Default Allows Exactly One More Push
severity: error
status: retired
retired: 2026-03-19
superseded_by: PGE02003
---

### Rule 2.4 — Default Allows Exactly One More Push *(retired)*
`PGE02004` → folded into `PGE02003`

**This rule has been retired.** All cases previously covered by PGE02004 are now enforced by [PGE02003 — Final Is Push-Once](PGE02003-final-is-push-once.md):

- **Double-default** (`<~ → <~`): PGE02003 now rejects re-defaulting a Default variable
- **Post-promotion push** (Default → Final → push): Already fired PGE02003

See resolved design issue 008 (git history: `docs/technical/compiler_issues/008-pge203-pge204-overlap.md`) for rationale.

### See Also

- [[concepts/variable-lifecycle|Variable Lifecycle]] — references PGE02004 in Default state description
