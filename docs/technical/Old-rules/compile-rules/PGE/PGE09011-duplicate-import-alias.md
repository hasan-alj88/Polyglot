---
audience: developer
rule: "9.12"
code: PGE09011
name: Duplicate Import Alias
severity: error
---

# Rule 9.12 — Duplicate Import Alias
`PGE09011`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->

**Statement:** Each `@alias` name in a file's `{@}` package block must be unique. If two or more `[@]` import declarations use the same alias name, PGE09011 fires. In the serialized path tree, aliases create shorthand references at a given level — duplicate names at the same level make resolution ambiguous.
**Rationale:** When the compiler encounters `@utils-Pipeline`, it must resolve `@utils` to exactly one imported package. Two `[@]` lines declaring `@utils` pointing to different packages make this resolution impossible. Since all objects live in a serialized tree, duplicate alias names at the same level create conflicting paths.
**Detection:** The compiler collects all `[@]` alias names from the `{@}` block. If any alias name appears more than once, PGE09011 fires on the second (and subsequent) declaration(s), reporting the conflicting alias and both target packages.

**See also:** PGE09001 (undefined import alias), PGE09012 (alias shadows jm3lib namespace)

**VALID:**
```aljam3
[ ] ✓ each alias is unique
{@} @Local:999.MyApp:v1.0.0
   [@] @utils << @Local:999.Utilities:v1.0.0
   [@] @auth  << @Local:999.AuthLib:v1.0.0

{-} -Main
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
   [ ]
   [-] @utils-Validate
      (-) <input#string << $data
   [-] @auth-CheckToken
      (-) <token#string << $jwt
```

**INVALID:**
```aljam3
[ ] ✗ PGE09011 — @utils declared twice with different targets
{@} @Local:999.MyApp:v1.0.0
   [@] @utils << @Local:999.Utilities:v1.0.0
   [@] @utils << @Local:999.HelperLib:v2.0.0   [ ] ✗ PGE09011 — duplicate alias @utils

{-} -Main
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
   [ ]
   [-] @utils-Validate                          [ ] ambiguous — which @utils?
      (-) <input#string << $data
```

```aljam3
[ ] ✗ PGE09011 — same alias even if same target package
{@} @Local:999.MyApp:v1.0.0
   [@] @lib << @Local:999.SharedLib:v1.0.0
   [@] @lib << @Local:999.SharedLib:v1.0.0      [ ] ✗ PGE09011 — duplicate alias @lib (even identical target)
```

## See Also

- [[user/syntax/packages|Packages]] — references PGE09011 in import rules

**Open point:** None.
