---
audience: developer
rule: "8.8"
code: PGE08008
name: Missing Required Input at Call Site
severity: error
---

### Rule 8.8 — Missing Required Input at Call Site
`PGE08008`

**Statement:** Every declared `<input` on the called pipeline or operator that has no default value (`<~`) must be wired by the caller. If the caller omits a required input, PGE08008 fires. This applies to sequential pipeline (`[-]`), parallel pipeline (`[=]`), expand operator (`(=)`), collect operator (`(*)`), and wrapper (`[W]`) calls.
**Rationale:** The compiler knows the full IO contract of every pipeline and operator from its definition. An unwired required input leaves the parameter in Declared state — the called pipeline cannot read it (PGE02002), so execution would always fail. Catching this at compile time prevents silent failures.
**Detection:** The compiler resolves the called pipeline's `(-)` declarations, enumerates all `<input` parameters, and checks each against the caller's wiring lines. Any `<input` that has no default (`<~`) and no corresponding wiring line from the caller triggers PGE08008.

**See also:**
- PGE01010 (pipeline IO name mismatch — wrong name, not missing name)
- PGW08002 (unaddressed input with default — the warning counterpart)
- PGE01009 (wrapper IO mismatch — covers wrapper IO completeness)
- PGE08003 (auto-wire unmatched parameter — covers chain auto-wiring)

**VALID:**
```polyglot
[ ] ✓ all required inputs wired — sequential pipeline call
{-} -Greet
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <name#string
   (-) <title#string
   (-) >greeting#string
   [-] >greeting << "Hello, {$title} {$name}!"

{-} -UseGreet
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) >out#string
   [-] -Greet
      (-) <name << "Alice"                 [ ] ✓ required input wired
      (-) <title << "Dr."                  [ ] ✓ required input wired
      (-) >greeting >> >out
```

```polyglot
[ ] ✓ all required inputs wired — parallel pipeline call
{-} -BatchProcess
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <items#array:string
   (-) >results#array:string
   [=] =ForEach.Array
      (=) <Array << $items
      (=) >item >> $item
   [=] -Transform
      (-) <data << $item                   [ ] ✓ required input wired
      (-) >result >> $result
   [=] *Into.Array
      (*) <item << $result
      (*) >Array >> >results
```

**INVALID:**
```polyglot
[ ] ✗ PGE08008 — missing required input on sequential pipeline call
{-} -Greet
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <name#string
   (-) <title#string
   (-) >greeting#string
   [-] >greeting << "Hello, {$title} {$name}!"

{-} -UseGreet
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) >out#string
   [-] -Greet
      (-) <name << "Alice"
                                            [ ] ✗ PGE08008 — <title not wired, has no default
      (-) >greeting >> >out
```

```polyglot
[ ] ✗ PGE08008 — missing required input on parallel pipeline call
{-} -Transform
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <data#string
   (-) <mode#string
   (-) >result#string
   [-] >result << $data

{-} -BatchTransform
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) >out#string
   [=] -Transform
      (-) <data << "hello"
                                            [ ] ✗ PGE08008 — <mode not wired, has no default
      (-) >result >> $out
   [-] >out << $out
```

**Diagnostic:** "Missing required input `<{name}` on {operator type} call to `={PipelineName}` at line {N} — input has no default and must be wired"

### See Also

- [[user/concepts/pipelines/inline-calls|Inline Calls]] — references PGE08008 in call site rules

**Open point:** None.
