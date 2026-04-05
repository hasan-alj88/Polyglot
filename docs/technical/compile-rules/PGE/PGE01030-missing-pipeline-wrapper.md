---
audience: developer
rule: "1.30"
code: PGE01030
name: Missing Pipeline Wrapper
severity: error
---

### Rule 1.30 — Missing Pipeline Wrapper
`PGE01030`

<!-- @brainstorming:marker-declarations §4 Rule B -->

**Statement:** Every `{=}[exe]` pipeline must contain a `[W]` wrapper invocation. A pipeline without a wrapper has no lifecycle management and is a compile error.
**Rationale:** The wrapper provides setup/cleanup lifecycle around pipeline execution. Even pipelines with no special requirements must declare `[W] =W.Polyglot` (the identity wrapper). This completes the required trio: `[T]` (PGE01005), `[Q]` (PGE01006), and `[W]` (PGE01030).
**Detection:** The compiler checks that every `{=}[exe]` block contains exactly one `[W]` section.

**VALID:**
```polyglot
[ ] ✓ — all three required elements present
{=}[exe] =Good.Pipeline
   [T] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input#string
   [r] =DoWork
```

**INVALID:**
```polyglot
[ ] ✗ PGE01030 — [W] missing
{=}[exe] =Bad.NoWrapper
   [T] =T.Call
   [Q] =Q.Default
   [=] <input#string
   [r] =DoWork
```

**Diagnostic:** "Pipeline `=Bad.NoWrapper` has no wrapper — add `[W]` (use `=W.Polyglot` if no setup/cleanup needed)"

### See Also

- [[PGE01005-missing-trigger|PGE01005]] — missing `[T]` (same pattern)
- [[PGE01006-missing-queue|PGE01006]] — missing `[Q]` (same pattern)
- [[concepts/pipelines/INDEX|Pipeline Structure]] — documents mandatory `[T]`/`[Q]`/`[W]` requirement
