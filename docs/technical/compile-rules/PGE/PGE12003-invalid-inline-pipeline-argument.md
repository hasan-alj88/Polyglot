---
audience: developer
rule: "12.3"
code: PGE12003
name: Undefined Inline Template
severity: error
---

# Rule 12.3 — Undefined Inline Template
`PGE12003`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:concepts/pipelines/inline-calls -->

**Statement:** When an inline pipeline call `-Foo"arg"` on an infrastructure line (`[T]`, `[Q]`, `[W]`) targets a pipeline that does not declare `(-) %InlineString << "{template}"` in its IO section, the call is a compile error.
**Rationale:** Inline calls require a `%InlineString` template to extract named values from the string argument and wire them to declared inputs. Without a template, the compiler cannot determine how to parse the argument. This catches calls to pipelines that were never designed for inline usage.
**Detection:** The compiler checks the target pipeline's `(-)` IO declarations for a `%InlineString` line. If absent and the pipeline is called with an inline argument (`-Foo"..."`), PGE12003 is raised.

> **Scope:** This rule applies to infrastructure inline calls only. For constructor errors in execution body, see PGE14xxx ([[syntax/constructors|Constructors]] category).

**See also:** PGE12005 (inline format mismatch), PGW12001 (template with no placeholders), PGE14012 (undefined constructor — analogous for `{$}` constructors)

---

**VALID:**
```aljam3
[ ] ✓ infrastructure inline call — pipeline declares %InlineString
{-} -MyWrapper
   (-) %InlineString << "{envName}"
   (-) <envName#string
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Aljam3
   [ ] ...

{-} -MyPipeline
   [T] -T.Daily"9AM"
   [Q] -Q.Default
   [W] -MyWrapper"production"
```

```aljam3
[ ] ✓ normal call to pipeline without %InlineString — no error (not inline)
{-} -NormalPipeline
   (-) <input#string
   (-) >output#string
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Aljam3
   [ ]
   [-] >output << $input

[-] -NormalPipeline
   (-) <input << "test"
   (-) >output >> $result
```

**INVALID:**
```aljam3
[ ] ✗ PGE12003 — -MyWrapper has no %InlineString declaration
{-} -MyWrapper
   (-) <envName#string
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Aljam3
   [ ] ...

{-} -MyPipeline
   [T] -T.Daily"9AM"
   [Q] -Q.Default
   [W] -MyWrapper"production"
```

**Open point:** None.
