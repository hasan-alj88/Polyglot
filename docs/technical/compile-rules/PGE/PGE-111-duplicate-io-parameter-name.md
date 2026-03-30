---
rule: "1.11"
code: PGE-111
name: Duplicate IO Parameter Name
severity: error
---

### Rule 1.11 — Duplicate IO Parameter Name
`PGE-111`

**Statement:** A pipeline's `[t]`/`[=]` IO section must not declare the same parameter name more than once in the same direction. Two `<name` inputs or two `>name` outputs with identical serialized identifiers are a compile error. An input `<name` and output `>name` sharing the same base name is allowed — direction disambiguates them.
**Rationale:** Duplicate IO names create ambiguous wiring targets. When a caller writes `[=] <person << $data`, the compiler cannot determine which of two `<person` declarations to fill. Catching this at definition time prevents silent wiring bugs.
**Detection:** The compiler collects all `<name` and `>name` declarations in a pipeline's `[t]`/`[=]` IO section into two sets (inputs and outputs). If any serialized identifier appears more than once within a set, the compiler emits PGE-111.

**See also:** PGE-110 (pipeline IO name mismatch — checks call-site names against declaration), PGE-109 (wrapper IO mismatch)

**VALID:**
```polyglot
{=} =Transfer
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <data#string
   [=] >data#string
   [r] >data << $data

[ ] ✓ <data and >data share a base name but differ in direction — allowed
```

```polyglot
{=} =ProcessPerson
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <person.name#string
   [=] <person.age#int
   [=] >result#string
   [r] >result << "{$person.name} is {$person.age}"

[ ] ✓ <person.name and <person.age are distinct serialized identifiers — allowed
```

**INVALID:**
```polyglot
{=} =Broken
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <data#string
   [=] <data#int                               [ ] ✗ PGE-111 — <data declared twice
   [=] >result#string
   [r] >result << $data
```

```polyglot
{=} =AlsoBroken
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input#string
   [=] >output#string
   [=] >output#int                             [ ] ✗ PGE-111 — >output declared twice
   [r] >output << $input
```

**Open point:** Parent-child overlap (`<person` + `<person.age` in the same IO section) is a separate concern — not covered by this rule.

### See Also

- [[concepts/pipelines/inline-calls|Inline Calls]] — documents duplicate IO wiring constraint, references PGE-111
