---
rule: "9.4"
code: PGE-904
name: Unresolved Import Pipeline Reference
severity: error
---

### Rule 9.4 — Unresolved Import Pipeline Reference
`PGE-904`

**Statement:** After an `@alias` resolves to an imported package (PGE-901 passes), the `=Pipeline` name in `@alias=Pipeline` must exist as a `{=}` definition in that package. If the pipeline is not found in the imported package, PGE-904 fires. This is the second-phase resolution check — PGE-901 validates the alias, PGE-904 validates the pipeline name within the resolved package.
**Rationale:** A valid import alias pointing to a real package does not guarantee the pipeline exists there. The pipeline may have been renamed, removed, or never existed. Catching this at compile time prevents runtime lookup failures in cross-package calls.
**Detection:** After PGE-901 resolves the `@alias` to an imported package, the compiler enumerates all `{=}` definitions in that package and checks the `=Pipeline` name against them. If no match is found, PGE-904 fires.

**See also:** PGE-901 (undefined import alias — validates the `@alias` prefix), PGE-903 (unresolved pipeline reference — same concept for local/stdlib pipelines), PGE-110 (pipeline IO name mismatch — validated after the pipeline reference resolves)

**VALID:**
```polyglot
{@} @Local:999.MyApp:v1.0.0
   [@] @utils << @Local:999.Utilities:v1.0.0

[ ] ✓ =Validate exists as {=} =Validate in @Local:999.Utilities:v1.0.0
{=} =Main
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] @utils=Validate                    [ ] ✓ @utils resolves (PGE-901), =Validate found in package (PGE-904)
      [=] <input#string << $data
      [=] >result#string >> $validated
```

**INVALID:**
```polyglot
{@} @Local:999.MyApp:v1.0.0
   [@] @utils << @Local:999.Utilities:v1.0.0

[ ] ✗ PGE-904 — =NonExistent not found in imported package
{=} =Main
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] @utils=NonExistent                 [ ] ✗ PGE-904 — no {=} =NonExistent in @Local:999.Utilities:v1.0.0
      [=] <input#string << $data
```

```polyglot
{@} @Local:999.MyApp:v1.0.0
   [@] @utils << @Local:999.Utilities:v1.0.0

[ ] ✗ PGE-904 — typo in pipeline name
{=} =Main
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] @utils=Validte                     [ ] ✗ PGE-904 — typo: =Validte not found (did you mean =Validate?)
      [=] <input#string << $data
```

```polyglot
{@} @Local:999.MyApp:v1.0.0
   [@] @utils << @Local:999.Utilities:v1.0.0

[ ] ✗ PGE-904 — pipeline was removed in newer version
{=} =Main
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] @utils=OldPipeline                 [ ] ✗ PGE-904 — =OldPipeline no longer exists in @Local:999.Utilities:v1.0.0
      [=] <input#string << $data
```

### See Also

- [[user/syntax/packages|Packages]] — references PGE-904 in import usage rules

**Open point:** None.
