---
audience: developer
rule: "6.9"
code: PGE06009
name: Conditional Missing Comparison Operator
severity: error
---

# Rule 6.9 — Conditional Missing Comparison Operator
`PGE06009`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/operators -->
<!-- @u:syntax/types -->

**Statement:** Every `[?]` line must follow the form `[?] $variable <operator> value` or `[?] ?*`. A `[?]` line without a comparison operator is a syntax violation. **Exception:** `[?]` lines in match context (indented under `[-] $source >> $target`) use `[?] value >> result` form without a comparison operator — PGE06009 does not apply to match arms. See [[conditionals#Match Syntax]].

**Rationale:** Conditionals are explicit comparison expressions, not switch/match blocks. There is no "subject" line that introduces a value to match against — each arm is a standalone test. Bare `[?] $variable` lines create ambiguity about what comparison is being performed. Aljam3 requires every condition to be explicit and verifiable — the compiler cannot prove exhaustive coverage if it cannot determine what each branch tests.

**Detection:** The compiler checks every `[?]` line. First, it determines whether the `[?]` is in match context — its parent is a `[-] ... >> ...` match header. If so, PGE06009 is suppressed. Otherwise, if the line does not contain a comparison operator (`?=`, `?!=`, `?>`, `?<`, `?>=`, `?<=`, `?!<=`, `?!>=`, `?!<`, `?!>`, `?*`) or a range operator (`?[`, `?(`, `?]`, `?)`), PGE06009 fires.

---

## Valid Forms

```aljam3
[ ] ✓ Comparison with operator
[?] $status ?= #PipelineStatus.Running
   [-] ...
[?] $status ?= #PipelineStatus.Failed
   [-] ...
[?] ?*
   [-] ...
```

```aljam3
[ ] ✓ Range operator
[?] $count ?[1,10]
   [-] ...
[?] $count ?(10,100]
   [-] ...
[?] ?*
   [-] ...
```

```aljam3
[ ] ✓ Negated comparison
[?] $name ?!= ""
   [-] ...
[?] ?*
   [-] ...
```

```aljam3
[ ] ✓ Match context — no comparison operator needed
[-] $code >> $status#string
   [?] 200 >> "ok"
   [?] 404 >> "not_found"
   [?] ?* >> "unknown"
```

## Invalid — Missing Comparison Operator

```aljam3
[ ] ✗ PGE06009 — bare variable, no comparison operator
[?] $status
   [?] #Running
      [-] ...
   [?] ?*
      [-] ...
```

```aljam3
[ ] ✗ PGE06009 — bare metadata access, no comparison operator
[?] $myVar%state
   [?] #Ready
      [-] ...
```

---

**See also:**
- [[PGE06001-conditional-must-be-exhaustive|PGE06001 — Conditional Must Be Exhaustive]] — parent exhaustiveness rule
- [[operators|operators]] — full comparison operator list

## See Also

- [[user/concepts/conditionals|Conditionals]] — missing comparison operator rule references PGE06009
- [[user/syntax/operators|Operators]] — comparison operator requirement references PGE06009
