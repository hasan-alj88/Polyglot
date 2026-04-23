---
audience: developer
rule: "4.11"
code: PGE04012
name: Division by Literal Zero
severity: error
---

# Rule 4.11 — Division by Literal Zero
`PGE04011`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->

**Statement:** A call to `-Math.Divide` or `-Math.Modulo` where the divisor input is the literal value `0` is a compile error.
**Rationale:** Division by zero is always a runtime failure. When the divisor is a literal `0`, the compiler can detect this statically and reject the program. This catches obvious mistakes before deployment. This exemplifies Polyglot's core principle of offloading runtime crashes to compile time — if a failure is provable before the pipeline runs, the compiler must reject it rather than let it surface in production.
**Detection:** The compiler inspects all `[-] -Math.Divide` and `[-] -Math.Modulo` calls. If the second positional input (`<< 0`) is the integer literal `0` or float literal `0.0`, the call is rejected.

**See also:** PGE04010 (invalid arithmetic operator — rejects non-math operators), PGE04001 (type mismatch — general type validation)

**VALID:**
```polyglot
[ ] ✓ variable divisor — cannot be checked statically
[-] -Math.Divide
   (-) << $numerator
   (-) << $divisor
   (-) >> $result
```

```polyglot
[ ] ✓ literal non-zero divisor
[-] -Math.Divide
   (-) << $total
   (-) << 3
   (-) >> $average
```

**INVALID:**
```polyglot
[ ] ✗ PGE04011 — literal zero divisor
[-] -Math.Divide
   (-) << $value
   (-) << 0                                     [ ] ✗ PGE04011 — division by literal zero
   (-) >> $result
```

```polyglot
[ ] ✗ PGE04011 — literal zero modulo
[-] -Math.Modulo
   (-) << $value
   (-) << 0                                     [ ] ✗ PGE04011 — modulo by literal zero
   (-) >> $remainder
```

**Open point:** None.

## See Also

- [[syntax/operators|Operators]] — division and modulo by literal zero compile error
