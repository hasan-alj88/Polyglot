---
audience: developer
rule: "12.8"
code: PGE12008
name: Duplicate Template Placeholder
severity: error
---

# Rule 12.8 — Duplicate Template Placeholder
`PGE12008`

<!-- @u:concepts/pipelines/inline-calls -->

**Statement:** When a pipeline's `%InlineString` template contains the same placeholder name more than once (e.g., `"{name}-{name}"`), the definition is a compile error.
**Rationale:** Each placeholder extracts a value and wires it to a single input. Duplicate placeholders create ambiguity — the compiler cannot determine which occurrence provides the authoritative value for the input. The pipeline author should use a single placeholder and reference `$name` multiple times in the execution body instead.
**Detection:** The compiler extracts all placeholder names from the `%InlineString` template and checks for duplicates. If any name appears more than once, PGE12008 is raised.

**See also:** PGE01011 (duplicate IO parameter name)

---

**VALID:**
```polyglot
[ ] ✓ each placeholder name appears once
{-} -FormatName
   (-) %InlineString << "{first} {last}"
   (-) <first#string
   (-) <last#string
   (-) >full#string
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] >full << "{$first} {$last}"
```

**INVALID:**
```polyglot
[ ] ✗ PGE12008 — placeholder {name} appears twice
{-} -EchoTwice
   (-) %InlineString << "{name}-{name}"
   (-) <name#string
   (-) >out#string
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] >out << "{$name}-{$name}"
```

**Open point:** None.
