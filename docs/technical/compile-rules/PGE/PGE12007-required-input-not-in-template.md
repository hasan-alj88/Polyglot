---
audience: developer
rule: "10.2"
code: PGE12007
name: Required Input Not In Template
severity: error
---

# Rule 10.2 — Required Input Not In Template
`PGE12007`

<!-- @u:concepts/pipelines/inline-calls -->

**Statement:** When a pipeline declares `%InlineString` and has a required input (`<name` with no `<~` default) that is not covered by any placeholder in the template, the definition is a compile error. Inline callers would have no way to satisfy that input.
**Rationale:** Inline calls wire inputs exclusively through template extraction. A required input not represented in the template cannot be filled by inline callers, making the pipeline impossible to call inline without a compile error at every call site. This catches the design flaw at the definition, not at each caller.
**Detection:** The compiler collects all required inputs (no `<~` default) and checks that each appears as a `{name}` placeholder in the `%InlineString` template. If any required input is missing from the template, PGE12007 is raised.

**See also:** PGE12006 (placeholder not in IO — the reverse direction), PGE08008 (missing required input at call site)

---

**VALID:**
```polyglot
[ ] ✓ all required inputs appear in template
{-} -DB.Connect
   (-) %InlineString << "{host}:{port?}/{db}"
   (-) <host#string
   (-) <port#string <~ "5432"
   (-) <db#string
   (-) >connection#serial
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   [ ] ...
```

```polyglot
[ ] ✓ <port has default — optional placeholder {port?} is fine
{-} -DB.Connect
   (-) %InlineString << "{host}/{db}"
   (-) <host#string
   (-) <port#string <~ "5432"
   (-) <db#string
   (-) >connection#serial
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   [ ] ...
```

**INVALID:**
```polyglot
[ ] ✗ PGE12007 — <db is required but not in template
{-} -DB.Connect
   (-) %InlineString << "{host}:{port}"
   (-) <host#string
   (-) <port#string
   (-) <db#string
   (-) >connection#serial
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   [ ] ...
```

**Open point:** None.
