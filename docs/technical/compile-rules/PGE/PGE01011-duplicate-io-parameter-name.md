---
rule: "1.11"
code: PGE01011
name: Duplicate IO Parameter Name
severity: error
---

### Rule 1.11 — Duplicate IO Parameter Name
`PGE01011`

**Statement:** A pipeline's `[T]`/`[=]` IO section must not declare the same parameter name more than once in the same direction. Two `<name` inputs or two `>name` outputs with identical serialized identifiers are a compile error. An input `<name` and output `>name` sharing the same base name is allowed — direction disambiguates them.
**Rationale:** Duplicate IO names create ambiguous wiring targets. When a caller writes `[=] <person << $data`, the compiler cannot determine which of two `<person` declarations to fill. Catching this at definition time prevents silent wiring bugs.
**Detection:** The compiler collects all `<name` and `>name` declarations in a pipeline's `[T]`/`[=]` IO section into two sets (inputs and outputs). If any serialized identifier appears more than once within a set, the compiler emits PGE01011.

**See also:** PGE01010 (pipeline IO name mismatch — checks call-site names against declaration), PGE01009 (wrapper IO mismatch)

**VALID:**
```polyglot
{=} =Transfer
   [T] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <data#string
   [=] >data#string
   [r] >data << $data

[ ] ✓ <data and >data share a base name but differ in direction — allowed
```

```polyglot
{=} =ProcessPerson
   [T] =T.Manual
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
   [T] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <data#string
   [=] <data#int                               [ ] ✗ PGE01011 — <data declared twice
   [=] >result#string
   [r] >result << $data
```

```polyglot
{=} =AlsoBroken
   [T] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input#string
   [=] >output#string
   [=] >output#int                             [ ] ✗ PGE01011 — >output declared twice
   [r] >output << $input
```

**Open point:** Parent-child overlap (`<person` + `<person.age` in the same IO section) is a separate concern — not covered by this rule.

### See Also

- [[concepts/pipelines/inline-calls|Inline Calls]] — documents duplicate IO wiring constraint, references PGE01011
