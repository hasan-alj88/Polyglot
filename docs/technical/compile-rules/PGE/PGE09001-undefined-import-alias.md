---
audience: developer
rule: "9.1"
code: PGE09001
name: Undefined Import Alias
severity: error
---

# Rule 9.1 — Undefined Import Alias
`PGE09001`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->
<!-- @u:syntax/types -->

**Statement:** Every `@alias` reference in a pipeline must resolve to a `[@]` import declared in the file's `{@}` package block. Using an `@alias` that was never declared is a compile error.
**Rationale:** Unresolved aliases would cause runtime lookup failures. Catching them at compile time ensures all cross-package references are valid before execution.
**Detection:** The compiler collects all `[@]` import declarations from the `{@}` block, then checks every `@alias-Pipeline` and `@alias#DataType` reference in the file. If any alias is not in the declared set, PGE09001 fires.

**See also:** PGE01003 (one package per file), PGE09002 (circular dependency)

**VALID:**
```polyglot
[ ] ✓ alias declared before use
{@} @Local:999.MyApp:v1.0.0
   [@] @utils << @Local:999.Utilities:v1.0.0

{-} -Process
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] @utils-Validate                [ ] ✓ @utils declared in [@]
      (-) <input << $data
```

**INVALID:**
```polyglot
[ ] ✗ PGE09001 — @unknown never declared
{@} @Local:999.MyApp:v1.0.0
   [@] @utils << @Local:999.Utilities:v1.0.0

{-} -Process
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] @unknown-Validate              [ ] ✗ PGE09001 — @unknown not in any [@] declaration
      (-) <input << $data
```

## See Also

- [[user/syntax/packages|Packages]] — references PGE09001 in import usage rules

**Open point:** None.
