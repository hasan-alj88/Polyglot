---
rule: "9.12"
code: PGE-912
name: Duplicate Import Alias
severity: error
---

### Rule 9.12 — Duplicate Import Alias
`PGE-912`

**Statement:** Each `@alias` name in a file's `{@}` package block must be unique. If two or more `[@]` import declarations use the same alias name, PGE-912 fires. In the serialized path tree, aliases create shorthand references at a given level — duplicate names at the same level make resolution ambiguous.
**Rationale:** When the compiler encounters `@utils=Pipeline`, it must resolve `@utils` to exactly one imported package. Two `[@]` lines declaring `@utils` pointing to different packages make this resolution impossible. Since all objects live in a serialized tree, duplicate alias names at the same level create conflicting paths.
**Detection:** The compiler collects all `[@]` alias names from the `{@}` block. If any alias name appears more than once, PGE-912 fires on the second (and subsequent) declaration(s), reporting the conflicting alias and both target packages.

**See also:** PGE-901 (undefined import alias), PGE-913 (alias shadows stdlib namespace)

**VALID:**
```polyglot
[ ] ✓ each alias is unique
{@} @Local:999.MyApp:v1.0.0
   [@] @utils << @Local:999.Utilities:v1.0.0
   [@] @auth  << @Local:999.AuthLib:v1.0.0

{=} =Main
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] @utils=Validate
      [=] <input#string << $data
   [r] @auth=CheckToken
      [=] <token#string << $jwt
```

**INVALID:**
```polyglot
[ ] ✗ PGE-912 — @utils declared twice with different targets
{@} @Local:999.MyApp:v1.0.0
   [@] @utils << @Local:999.Utilities:v1.0.0
   [@] @utils << @Local:999.HelperLib:v2.0.0   [ ] ✗ PGE-912 — duplicate alias @utils

{=} =Main
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] @utils=Validate                          [ ] ambiguous — which @utils?
      [=] <input#string << $data
```

```polyglot
[ ] ✗ PGE-912 — same alias even if same target package
{@} @Local:999.MyApp:v1.0.0
   [@] @lib << @Local:999.SharedLib:v1.0.0
   [@] @lib << @Local:999.SharedLib:v1.0.0      [ ] ✗ PGE-912 — duplicate alias @lib (even identical target)
```

**Open point:** None.
