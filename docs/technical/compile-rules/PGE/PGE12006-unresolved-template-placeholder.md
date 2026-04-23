---
audience: developer
rule: "12.6"
code: PGE12006
name: Unresolved Template Placeholder
severity: error
---

# Rule 12.6 — Unresolved Template Placeholder
`PGE12006`

<!-- @u:concepts/pipelines/inline-calls -->

**Statement:** When a pipeline's `%InlineString` template contains a placeholder `{name}` or `{name?}` and no `(-) <name` input is declared, the definition is a compile error.
**Rationale:** Each template placeholder must map to a declared input so the compiler can wire extracted values. A placeholder with no matching input means the extracted value has nowhere to go — the pipeline author likely forgot to declare the input or misspelled the placeholder name.
**Detection:** The compiler extracts all placeholder names from the `%InlineString` template and verifies each has a corresponding `<name` input in the pipeline's `(-)` IO declarations. If any placeholder name is unmatched, PGE12006 is raised at the pipeline definition site.

**See also:** PGE12007 (required input not in template — the reverse direction)

---

**VALID:**
```polyglot
[ ] ✓ all placeholders match declared inputs
{-} -Greeting
   (-) %InlineString << "{name}"
   (-) <name#string <~ "World"
   (-) >message#string
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] >message << "Hello {$name}"
```

**INVALID:**
```polyglot
[ ] ✗ PGE12006 — placeholder {user} has no matching <user input
{-} -Greeting
   (-) %InlineString << "{user}"
   (-) <name#string <~ "World"
   (-) >message#string
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] >message << "Hello {$name}"
```

**Open point:** None.
