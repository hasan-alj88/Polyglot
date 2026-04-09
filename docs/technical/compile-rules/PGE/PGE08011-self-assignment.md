---
audience: developer
rule: "8.11"
code: PGE08011
name: Self-Assignment
severity: error
---

### Rule 8.11 — Self-Assignment
`PGE08011`

**Statement:** An assignment expression where the target and source resolve to the same identifier is a no-op and a compile error. This applies to: (a) output params — `>x << >x` where the same output parameter appears on both sides within the same operation scope, and (b) variables — `$x << $x` where the same variable name appears on both sides. The rule applies to all assignment operators (`<<`, `>>`, `<~`, `~>`). Cross-port wiring where the names differ (`>out1 << >out2`) is valid behavior.
**Rationale:** Writing a value to itself produces no state change. In output params, the port already holds the value being assigned. In variables, the assignment is redundant. This is always a wiring mistake — the developer intended to wire two different identifiers.
**Detection:** The compiler compares the resolved identifier on both sides of an assignment expression. If they are the same name and same kind (both `>` output params or both `$` variables), PGE08011 fires. For output params, both must be within the same operation scope (same pipeline body).

**VALID:**
```polyglot
[ ] ✓ Different output ports — cross-port wiring
(-) >out1#string
(-) >out2#string
[-] >out1 << >out2                      [ ] ✓ different ports, valid wiring

[ ] ✓ Different variables
[-] $a#string << $b                     [ ] ✓ different variables, valid assignment

[ ] ✓ Output port from called pipeline to local output port
[-] -Compute
   (-) >result >> >localOut             [ ] ✓ different scopes — called pipeline output to local
```

**INVALID:**
```polyglot
[ ] ✗ PGE08011 — same output param on both sides
(-) >result#string
[-] >result << >result                  [ ] ✗ PGE08011 — self-assignment, no state change

[ ] ✗ PGE08011 — same variable on both sides
[-] $name#string << $name               [ ] ✗ PGE08011 — self-assignment, no-op

[ ] ✗ PGE08011 — self-assignment with default operator
[-] $count#int <~ $count                [ ] ✗ PGE08011 — still a no-op regardless of operator
```

**Diagnostic:** "Self-assignment of `{identifier}` — target and source are the same identifier, producing no state change"

### See Also

- [[technical/compile-rules/PGE/PGE02010-discard-default-assignment|PGE02010]] — Discard Default Assignment (related no-op pattern)
- [[technical/compile-rules/PGE/PGE02003-final-is-push-once|PGE02003]] — Final Is Push-Once (lifecycle context)
