---
rule: "9.1"
code: PGE-901
name: Undefined Import Alias
severity: error
---

### Rule 9.1 — Undefined Import Alias
`PGE-901`

**Statement:** Every `@alias` reference in a pipeline must resolve to a `[@]` import declared in the file's `{@}` package block. Using an `@alias` that was never declared is a compile error.
**Rationale:** Unresolved aliases would cause runtime lookup failures. Catching them at compile time ensures all cross-package references are valid before execution.
**Detection:** The compiler collects all `[@]` import declarations from the `{@}` block, then checks every `@alias=Pipeline` and `@alias#DataType` reference in the file. If any alias is not in the declared set, PGE-901 fires.

**See also:** PGE-103 (one package per file), PGE-902 (circular dependency)

**VALID:**
```polyglot
[ ] ✓ alias declared before use
{@} @Local:999.MyApp:v1.0.0
   [@] @utils << @Local:999.Utilities:v1.0.0

{=} =Process
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] @utils=Validate                [ ] ✓ @utils declared in [@]
      [=] <input << $data
```

**INVALID:**
```polyglot
[ ] ✗ PGE-901 — @unknown never declared
{@} @Local:999.MyApp:v1.0.0
   [@] @utils << @Local:999.Utilities:v1.0.0

{=} =Process
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] @unknown=Validate              [ ] ✗ PGE-901 — @unknown not in any [@] declaration
      [=] <input << $data
```

### See Also

- [[user/syntax/packages|Packages]] — references PGE-901 in import usage rules

**Open point:** None.
