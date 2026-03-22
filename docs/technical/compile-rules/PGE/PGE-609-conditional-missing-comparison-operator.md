---
rule: "6.9"
code: PGE-609
name: Conditional Missing Comparison Operator
severity: error
---

### Rule 6.9 — Conditional Missing Comparison Operator
`PGE-609`

**Statement:** Every `[?]` line must follow the form `[?] $variable <operator> value` or `[?] *?`. A `[?]` line without a comparison operator is a syntax violation.

**Rationale:** Conditionals are explicit comparison expressions, not switch/match blocks. There is no "subject" line that introduces a value to match against — each arm is a standalone test. Bare `[?] $variable` lines create ambiguity about what comparison is being performed.

**Detection:** The compiler checks every `[?]` line. If the line does not contain a comparison operator (`=?`, `=!?`, `>?`, `<?`, `>=?`, `<=?`, `>!?`, `<!?`, `>=!?`, `<=!?`, `*?`) or a range operator (`?[`, `?(`, `?]`, `?)`), PGE-609 fires.

---

#### Valid Forms

```polyglot
[ ] ✓ Comparison with operator
[?] $status =? #PipelineStatus.Running
   [r] ...
[?] $status =? #PipelineStatus.Failed
   [r] ...
[?] *?
   [r] ...
```

```polyglot
[ ] ✓ Range operator
[?] $count ?[1,10]
   [r] ...
[?] $count ?(10,100]
   [r] ...
[?] *?
   [r] ...
```

```polyglot
[ ] ✓ Negated comparison
[?] $name =!? ""
   [r] ...
[?] *?
   [r] ...
```

#### Invalid — Missing Comparison Operator

```polyglot
[ ] ✗ PGE-609 — bare variable, no comparison operator
[?] $status
   [?] #Running
      [r] ...
   [?] *?
      [r] ...
```

```polyglot
[ ] ✗ PGE-609 — bare metadata access, no comparison operator
[?] $myVar%state
   [?] #Ready
      [r] ...
```

---

**See also:**
- [[PGE-601-conditional-must-be-exhaustive|PGE-601 — Conditional Must Be Exhaustive]] — parent exhaustiveness rule
- [[operators|operators]] — full comparison operator list
