---
audience: developer
rule: "10.2"
code: PGW12001
name: Template With No Placeholders
severity: warning
---

# Rule 10.2 — Template With No Placeholders
`PGW12001`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:concepts/pipelines/inline-calls -->

**Statement:** When a pipeline declares `(-) %InlineString << "{template}"` but the template string contains no `{name}` or `{name?}` placeholders, the compiler emits a warning. The inline call accepts the string but extracts no values — inputs must still be wired normally.
**Rationale:** A `%InlineString` template with no placeholders provides no compile-time extraction benefit. The template string is a constant pattern that only serves as a documentation hint. This is likely an authoring oversight — the pipeline author probably intended to add placeholders for named extraction.
**Detection:** The compiler scans the `%InlineString` template for `{name}` or `{name?}` patterns. If none are found, PGW12001 is emitted.

**See also:** PGE12003 (no %InlineString at all), PGE12006 (unresolved placeholder — name not in IO)

**WARNING:**
```polyglot
[ ] ⚠ PGW12001 — template has no placeholders
{-} -StaticCall
   (-) %InlineString << "fixed-string"
   (-) <input#string
   (-) >out#string
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] >out << $input

[-] $result#string << -StaticCall"anything"     [ ] ⚠ PGW12001 — no extraction occurs
```

**Open point:** None.
