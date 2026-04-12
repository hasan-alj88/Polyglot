---
audience: developer
rule: "10.2"
code: PGE12005
name: Inline Format Mismatch
severity: error
---

# Rule 10.2 — Inline Format Mismatch
`PGE12005`

<!-- @u:concepts/pipelines/inline-calls -->

**Statement:** When an inline pipeline call `-Foo"arg"` provides a string that does not match the structural pattern of the pipeline's `%InlineString` template, the call is a compile error. The rendered string must be parseable against the template to extract all required placeholders.
**Rationale:** The `%InlineString` template defines the expected format for inline arguments. If the rendered string cannot be decomposed into the template's fixed parts and placeholder values, the compiler cannot wire inputs correctly. Catching format mismatches at compile time prevents runtime extraction failures.
**Detection:** After string interpolation (`{$var}` resolved), the compiler matches the rendered string against the `%InlineString` template pattern. If the fixed (non-placeholder) portions of the template do not align with the rendered string, PGE12005 is raised.

**See also:** PGE12003 (no template declared), PGE12009 (type coercion failure after extraction)

---

**VALID:**
```polyglot
{-} -DB.Connect
   (-) %InlineString << "{host}:{port}/{db}"
   (-) <host#string
   (-) <port#string <~ "5432"
   (-) <db#string
   (-) >connection#serial
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   [ ] ...

[ ] ✓ matches template pattern: host=localhost, port=3306, db=mydb
[-] $conn << -DB.Connect"localhost:3306/mydb"
```

**INVALID:**
```polyglot
[ ] ✗ PGE12005 — no ":" or "/" found; cannot match "{host}:{port}/{db}"
[-] $conn << -DB.Connect"just-a-hostname"

[ ] ✗ PGE12005 — extra "/" segments; template expects exactly one "/"
[-] $conn << -DB.Connect"host:5432/db/extra"
```

**Open point:** None.
