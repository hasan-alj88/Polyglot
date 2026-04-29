---
audience: developer
rule: "12.10"
code: PGE12010
name: Optional Placeholder Without Default
severity: error
---

# Rule 12.10 — Optional Placeholder Without Default
`PGE12010`

<!-- @u:concepts/pipelines/inline-calls -->

**Statement:** When a pipeline's `%InlineString` template contains an optional placeholder `{name?}` but the corresponding `<name` input does not have a `<~` default, the definition is a compile error.
**Rationale:** Optional placeholders may match empty during inline calls. When the placeholder matches empty, the compiler does not wire a value to the input. If the input has no default, it remains unsatisfied — violating the pipeline's contract. Every optional placeholder must have a default to fall back on.
**Detection:** The compiler identifies all `{name?}` placeholders in the `%InlineString` template and checks that each corresponding `<name` input has a `<~` default assignment. If any optional placeholder's input lacks a default, PGE12010 is raised at the pipeline definition site.

**See also:** PGE12006 (placeholder not in IO), PGE12007 (required input not in template)

---

**VALID:**
```aljam3
[ ] ✓ {port?} is optional and <port has <~ default
{-} -DB.Connect
   (-) %InlineString << "{host}:{port?}/{db}"
   (-) <host#string
   (-) <port#string <~ "5432"
   (-) <db#string
   (-) >connection#serial
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Aljam3
   [ ] ...
```

**INVALID:**
```aljam3
[ ] ✗ PGE12010 — {port?} is optional but <port has no default
{-} -DB.Connect
   (-) %InlineString << "{host}:{port?}/{db}"
   (-) <host#string
   (-) <port#string
   (-) <db#string
   (-) >connection#serial
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Aljam3
   [ ] ...
```

**Open point:** None.
