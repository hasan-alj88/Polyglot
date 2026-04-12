---
audience: developer
rule: "4.16"
code: PGE04016
name: Invalid Pipeline Input Literal
severity: error
---

### Rule 4.16 — Invalid Pipeline Input Literal
`PGE04016`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:concepts/errors -->
<!-- @u:syntax/operators -->

**Statement:** When a pipeline declares `%constraint#array:RawString` on an input parameter and a caller passes a literal value that matches any entry in the constraint set, it is a compile error. Variable inputs cannot be checked statically — use fallback assignment (`!<`) for runtime safety.
**Rationale:** Some inputs have values that are provably invalid regardless of context (e.g., division by zero). When the pipeline author declares these constraints and the caller passes a matching literal, the compiler can catch the bug at compile time rather than at runtime.
**Detection:** The compiler reads the `%constraint` metadata on each input of the called pipeline. For each literal value passed at the call site, if the literal matches any entry in the constraint array, PGE04016 fires.

**See also:** PGE04001 (type mismatch), PGE08008 (missing required input)

**Declaring constraints (pipeline definition):**
```polyglot
[ ] ✓ pipeline author declares invalid values via %constraint
{-} -Math.Divide
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <numerator#int
   (-) <denominator#int
      (<) %constraint#array:RawString << {"0"}
   (-) >result#float
   [-] >result << $numerator / $denominator
```

**VALID:**
```polyglot
[ ] ✓ literal does not match any constraint
[-] -Math.Divide
   (-) <numerator << $total
   (-) <denominator << 2
   (-) >result >> $half
```

```polyglot
[ ] ✓ variable input — cannot be checked at compile time, use fallback for safety
[-] -Math.Divide
   (-) <numerator << $total
   (-) <denominator << $divisor
   (-) >result >> $half
   [!] !DivisionByZero
      (>) !> 0
```

**INVALID:**
```polyglot
[ ] ✗ PGE04016 — literal matches constraint
[-] -Math.Divide
   (-) <numerator << $total
   (-) <denominator << 0                      [ ] ✗ PGE04016 — 0 is in constraint set {"0"}
   (-) >result >> $result
```

```polyglot
[ ] ✗ PGE04016 — literal matches constraint
[-] -Math.Modulo
   (-) <numerator << $total
   (-) <denominator << 0                      [ ] ✗ PGE04016 — modulo by zero
   (-) >result >> $remainder
```

**Diagnostic:** `"Invalid literal value {value} for input <{name} on call to ={PipelineName} at line {N} — value is in the constraint set {constraints}"`
