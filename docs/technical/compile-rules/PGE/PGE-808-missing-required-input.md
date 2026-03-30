---
rule: "8.8"
code: PGE-808
name: Missing Required Input at Call Site
severity: error
---

### Rule 8.8 — Missing Required Input at Call Site
`PGE-808`

**Statement:** Every declared `<input` on the called pipeline or operator that has no default value (`<~`) must be wired by the caller. If the caller omits a required input, PGE-808 fires. This applies to sequential pipeline (`[r]`), parallel pipeline (`[p]`), expand operator (`[~]`), collect operator (`[*]`), and wrapper (`[W]`) calls.
**Rationale:** The compiler knows the full IO contract of every pipeline and operator from its definition. An unwired required input leaves the parameter in Declared state — the called pipeline cannot read it (PGE-202), so execution would always fail. Catching this at compile time prevents silent failures.
**Detection:** The compiler resolves the called pipeline's `[=]` declarations, enumerates all `<input` parameters, and checks each against the caller's wiring lines. Any `<input` that has no default (`<~`) and no corresponding wiring line from the caller triggers PGE-808.

**See also:**
- PGE-110 (pipeline IO name mismatch — wrong name, not missing name)
- PGW-808 (unaddressed input with default — the warning counterpart)
- PGE-109 (wrapper IO mismatch — covers wrapper-to-macro completeness)
- PGE-803 (auto-wire unmatched parameter — covers chain auto-wiring)

**VALID:**
```polyglot
[ ] ✓ all required inputs wired — sequential pipeline call
{=} =Greet
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <name#string
   [=] <title#string
   [=] >greeting#string
   [r] >greeting << "Hello, {$title} {$name}!"

{=} =UseGreet
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >out#string
   [r] =Greet
      [=] <name << "Alice"                 [ ] ✓ required input wired
      [=] <title << "Dr."                  [ ] ✓ required input wired
      [=] >greeting >> >out
```

```polyglot
[ ] ✓ all required inputs wired — parallel pipeline call
{=} =BatchProcess
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <items#array:string
   [=] >results#array:string
   [p] ~ForEach.Array
      [~] <Array << $items
      [~] >item >> $item
   [p] =Transform
      [=] <data << $item                   [ ] ✓ required input wired
      [=] >result >> $result
   [p] *Into.Array
      [*] <item << $result
      [*] >Array >> >results
```

**INVALID:**
```polyglot
[ ] ✗ PGE-808 — missing required input on sequential pipeline call
{=} =Greet
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <name#string
   [=] <title#string
   [=] >greeting#string
   [r] >greeting << "Hello, {$title} {$name}!"

{=} =UseGreet
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >out#string
   [r] =Greet
      [=] <name << "Alice"
                                            [ ] ✗ PGE-808 — <title not wired, has no default
      [=] >greeting >> >out
```

```polyglot
[ ] ✗ PGE-808 — missing required input on parallel pipeline call
{=} =Transform
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <data#string
   [=] <mode#string
   [=] >result#string
   [r] >result << $data

{=} =BatchTransform
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >out#string
   [p] =Transform
      [=] <data << "hello"
                                            [ ] ✗ PGE-808 — <mode not wired, has no default
      [=] >result >> $out
   [r] >out << $out
```

**Diagnostic:** "Missing required input `<{name}` on {operator type} call to `={PipelineName}` at line {N} — input has no default and must be wired"

### See Also

- [[user/concepts/pipelines/inline-calls|Inline Calls]] — references PGE-808 in call site rules

**Open point:** None.
