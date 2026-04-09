---
audience: developer
rule: "9.4"
code: PGE09004
name: Unresolved Import Pipeline Reference
severity: error
---

### Rule 9.4 — Unresolved Import Pipeline Reference
`PGE09004`

**Statement:** After an `@alias` resolves to an imported package (PGE09001 passes), the `-Pipeline` name in `@alias-Pipeline` must exist as a `{-}` definition in that package. If the pipeline is not found in the imported package, PGE09004 fires. This is the second-phase resolution check — PGE09001 validates the alias, PGE09004 validates the pipeline name within the resolved package.
**Rationale:** A valid import alias pointing to a real package does not guarantee the pipeline exists there. The pipeline may have been renamed, removed, or never existed. Catching this at compile time prevents runtime lookup failures in cross-package calls.
**Detection:** After PGE09001 resolves the `@alias` to an imported package, the compiler enumerates all `{-}` definitions in that package and checks the `-Pipeline` name against them. If no match is found, PGE09004 fires.

**See also:** PGE09001 (undefined import alias — validates the `@alias` prefix), PGE09003 (unresolved pipeline reference — same concept for local/pglib pipelines), PGE01010 (pipeline IO name mismatch — validated after the pipeline reference resolves)

**VALID:**
```polyglot
{@} @Local:999.MyApp:v1.0.0
   [@] @utils << @Local:999.Utilities:v1.0.0

[ ] ✓ -Validate exists as {-} -Validate in @Local:999.Utilities:v1.0.0
{-} -Main
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] @utils-Validate                    [ ] ✓ @utils resolves (PGE09001), -Validate found in package (PGE09004)
      (-) <input#string << $data
      (-) >result#string >> $validated
```

**INVALID:**
```polyglot
{@} @Local:999.MyApp:v1.0.0
   [@] @utils << @Local:999.Utilities:v1.0.0

[ ] ✗ PGE09004 — -NonExistent not found in imported package
{-} -Main
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] @utils-NonExistent                 [ ] ✗ PGE09004 — no {-} -NonExistent in @Local:999.Utilities:v1.0.0
      (-) <input#string << $data
```

```polyglot
{@} @Local:999.MyApp:v1.0.0
   [@] @utils << @Local:999.Utilities:v1.0.0

[ ] ✗ PGE09004 — typo in pipeline name
{-} -Main
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] @utils-Validte                     [ ] ✗ PGE09004 — typo: -Validte not found (did you mean -Validate?)
      (-) <input#string << $data
```

```polyglot
{@} @Local:999.MyApp:v1.0.0
   [@] @utils << @Local:999.Utilities:v1.0.0

[ ] ✗ PGE09004 — pipeline was removed in newer version
{-} -Main
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] @utils-OldPipeline                 [ ] ✗ PGE09004 — -OldPipeline no longer exists in @Local:999.Utilities:v1.0.0
      (-) <input#string << $data
```

### See Also

- [[user/syntax/packages|Packages]] — references PGE09004 in import usage rules

**Open point:** None.
