---
audience: developer
rule: "10.2"
code: PGE12003
name: Undefined Inline Template
severity: error
---

# Rule 10.2 — Undefined Inline Template
`PGE12003`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:concepts/pipelines/inline-calls -->

**Statement:** When an inline pipeline call `-Foo"arg"` targets a pipeline that does not declare `(-) %InlineString << "{template}"` in its IO section, the call is a compile error.
**Rationale:** Inline calls require a `%InlineString` template to extract named values from the string argument and wire them to declared inputs. Without a template, the compiler cannot determine how to parse the argument. This catches calls to pipelines that were never designed for inline usage.
**Detection:** The compiler checks the target pipeline's `(-)` IO declarations for a `%InlineString` line. If absent and the pipeline is called with an inline argument (`-Foo"..."`), PGE12003 is raised.

**See also:** PGE12005 (inline format mismatch), PGW12001 (template with no placeholders)

---

**VALID:**
```polyglot
[ ] ✓ pipeline declares %InlineString — inline call accepted
{-} -Greeting
   (-) %InlineString << "{name}"
   (-) <name#string <~ "World"
   (-) >message#string
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] >message << "Hello {$name}"

[-] $msg#string << -Greeting"Alice"
```

```polyglot
[ ] ✓ normal call to pipeline without %InlineString — no error
{-} -NormalPipeline
   (-) <input#string
   (-) >output#string
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] >output << $input

[-] -NormalPipeline
   (-) <input << "test"
   (-) >output >> $result
```

**INVALID:**
```polyglot
[ ] ✗ PGE12003 — -NormalPipeline has no %InlineString declaration
{-} -NormalPipeline
   (-) <input#string
   (-) >output#string
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] >output << $input

[-] $result#string << -NormalPipeline"test"
```

**Open point:** None.
