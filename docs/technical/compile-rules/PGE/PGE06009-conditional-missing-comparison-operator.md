---
rule: "6.9"
code: PGE06009
name: Conditional Missing Comparison Operator
severity: error
---

# Rule 6.9 — Conditional Missing Comparison Operator
`PGE06009`

**Statement:** Every `[?]` line must follow the form `[?] $variable <operator> value` or `[?] *?`. A `[?]` line without a comparison operator is a syntax violation. **Exception:** `[?]` lines in match context (indented under `[r] $source >> $target`) use `[?] value >> result` form without a comparison operator — PGE06009 does not apply to match arms. See [[conditionals#Match Syntax]].

**Rationale:** Conditionals are explicit comparison expressions, not switch/match blocks. There is no "subject" line that introduces a value to match against — each arm is a standalone test. Bare `[?] $variable` lines create ambiguity about what comparison is being performed.

**Detection:** The compiler checks every `[?]` line. First, it determines whether the `[?]` is in match context — its parent is a `[r] ... >> ...` match header. If so, PGE06009 is suppressed. Otherwise, if the line does not contain a comparison operator (`=?`, `=!?`, `>?`, `<?`, `>=?`, `<=?`, `>!?`, `<!?`, `>=!?`, `<=!?`, `*?`) or a range operator (`?[`, `?(`, `?]`, `?)`), PGE06009 fires.

---

## Valid Forms

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

```polyglot
[ ] ✓ Match context — no comparison operator needed
[r] $code >> $status#string
   [?] 200 >> "ok"
   [?] 404 >> "not_found"
   [?] *? >> "unknown"
```

## Invalid — Missing Comparison Operator

```polyglot
[ ] ✗ PGE06009 — bare variable, no comparison operator
[?] $status
   [?] #Running
      [r] ...
   [?] *?
      [r] ...
```

```polyglot
[ ] ✗ PGE06009 — bare metadata access, no comparison operator
[?] $myVar%state
   [?] #Ready
      [r] ...
```

---

**See also:**
- [[PGE06001-conditional-must-be-exhaustive|PGE06001 — Conditional Must Be Exhaustive]] — parent exhaustiveness rule
- [[operators|operators]] — full comparison operator list

## See Also

- [[user/concepts/conditionals|Conditionals]] — missing comparison operator rule references PGE06009
- [[user/syntax/operators|Operators]] — comparison operator requirement references PGE06009
