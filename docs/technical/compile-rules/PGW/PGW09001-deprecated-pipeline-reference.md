---
audience: developer
rule: "9.1"
code: PGW09001
name: Deprecated Pipeline Reference
severity: warning
---

### Rule 9.1 — Deprecated Pipeline Reference
`PGW09001`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->

**Statement:** Referencing a pipeline whose `%deprecated` metadata is `true` emits a compile warning. If the pipeline's `%deprecatedMessage` field is set, the warning includes it. This applies to all call forms: `[-]`, `[=]`, and chain (`->`) references, whether local or imported via `@alias`.
**Rationale:** Deprecated pipelines still compile and run, but should be phased out. Surfacing deprecation at compile time — with an actionable message when available — guides developers toward replacements without breaking builds.
**Detection:** After the compiler resolves a pipeline reference (PGE09003/PGE09004), it checks the target pipeline's `%deprecated` metadata. If `true`, PGW09001 fires. The warning message includes `%deprecatedMessage` if present, otherwise a generic "pipeline is deprecated" note.

**See also:** PGE09003 (unresolved pipeline reference), PGE09004 (unresolved import pipeline reference)

**VALID:**
```polyglot
{-} -Transform
   [%] .deprecated << false
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <input#string
   (-) >output#string
   [-] >output << $input

[ ] ✓ no warning — -Transform is not deprecated
[-] -Transform
   (-) <input#string << "hello"
   (-) >output#string >> $result
```

**WARNING:**
```polyglot
{-} -OldProcess
   [%] .deprecated << true
   [%] .deprecatedMessage << "Use -NewProcess instead"
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <data#string
   (-) >result#string
   [-] >result << $data

[ ] ⚠ PGW09001 — -OldProcess is deprecated: "Use -NewProcess instead"
[-] -OldProcess
   (-) <data#string << $input
   (-) >result#string >> $output
```

```polyglot
{-} -LegacyFetch
   [%] .deprecated << true
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <url#string
   (-) >content#string
   [-] >content << "stub"

[ ] ⚠ PGW09001 — -LegacyFetch is deprecated (no message provided)
[-] -LegacyFetch
   (-) <url#string << "https://example.com"
   (-) >content#string >> $page
```

```polyglot
{@} @Local:999.MyApp:v1.0.0
   [@] @legacy << @Local:999.OldUtils:v1.0.0

[ ] ⚠ PGW09001 — imported pipeline is deprecated
[-] @legacy-Format
   (-) <text#string << $raw               [ ] ⚠ PGW09001 — -Format in @Local:999.OldUtils:v1.0.0 is deprecated
   (-) >formatted#string >> $output
```

**Open point:** None.
