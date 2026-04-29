---
audience: developer
rule: "4.15"
code: PGE04015
name: Conditional Type-Operator Mismatch
severity: error
---

# Rule 4.15 — Conditional Type-Operator Mismatch
`PGE04015`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->

**Statement:** The comparison operator in a `[?]` conditional branch must be compatible with the subject variable's type, and the operand type must be compatible with both the operator and the subject. Numerics (`#int` and `#float`) form one family and freely interoperate. All other types require exact type matching.
**Rationale:** A range operator on a string or an enum match on an integer is always a logic error. Catching type-operator mismatches at compile time prevents nonsensical comparisons that would either always fail or produce undefined results at runtime.
**Detection:** The compiler resolves the subject variable's type, then checks each branch's comparison operator and operand type against the compatibility tables below.

**See also:** PGE04001 (type mismatch — general), PGE06001 (conditional exhaustiveness), PGE06003 (numeric range coverage)

---

## Compatibility Tables

### Equality Operators (`=?`, `=!?`)

| Left (subject) | Operator | Right (operand) | Valid? |
|----------------|----------|-----------------|--------|
| `#int` | `=?` | `#int` | yes |
| `#int` | `=?` | `#float` | yes |
| `#int` | `=?` | `#string` | no — PGE04015 |
| `#int` | `=?` | `#bool` | no — PGE04015 |
| `#int` | `=?` | enum | no — PGE04015 |
| `#float` | `=?` | `#float` | yes |
| `#float` | `=?` | `#int` | yes |
| `#float` | `=?` | `#string` | no — PGE04015 |
| `#string` | `=?` | `#string` | yes |
| `#string` | `=?` | `#int` | no — PGE04015 |
| `#string` | `=?` | `#float` | no — PGE04015 |
| `#bool` | `=?` | `#bool` | yes |
| `#bool` | `=?` | `#int` | no — PGE04015 |
| `#bool` | `=?` | `#string` | no — PGE04015 |
| enum `#S` | `=?` | `.Value` of `#S` | yes |
| enum `#S` | `=?` | `#int` | no — PGE04015 |
| enum `#S` | `=?` | `#string` | no — PGE04015 |
| enum `#S` | `=?` | `.Value` of `#T` | no — PGE04015 |

*(Same table applies to `=!?`)*

### Ordering Operators (`>?`, `<?`, `>!?`, `<!?`)

| Left (subject) | Operator | Right (operand) | Valid? |
|----------------|----------|-----------------|--------|
| `#int` | `>?` | `#int` | yes |
| `#int` | `>?` | `#float` | yes |
| `#int` | `>?` | `#string` | no — PGE04015 |
| `#int` | `>?` | `#bool` | no — PGE04015 |
| `#int` | `>?` | enum | no — PGE04015 |
| `#float` | `>?` | `#float` | yes |
| `#float` | `>?` | `#int` | yes |
| `#float` | `>?` | `#string` | no — PGE04015 |
| `#string` | `>?` | any | no — PGE04015 |
| `#bool` | `>?` | any | no — PGE04015 |
| enum | `>?` | any | no — PGE04015 |

*(Same table applies to `<?`, `>!?`, `<!?`)*

### Range Operators (`?[lo,hi]`, `?(lo,hi)`, mixed)

| Subject | Operator | Lo bound | Hi bound | Valid? |
|---------|----------|----------|----------|--------|
| `#int` | `?[,]` | `#int` | `#int` | yes |
| `#int` | `?[,]` | `#int` | `#float` | yes |
| `#int` | `?[,]` | `#float` | `#int` | yes |
| `#int` | `?[,]` | `#float` | `#float` | yes |
| `#int` | `?[,]` | `#string` | any | no — PGE04015 |
| `#float` | `?[,]` | `#float` | `#float` | yes |
| `#float` | `?[,]` | `#float` | `#int` | yes |
| `#float` | `?[,]` | `#int` | `#float` | yes |
| `#float` | `?[,]` | `#int` | `#int` | yes |
| `#float` | `?[,]` | `#string` | any | no — PGE04015 |
| `#string` | `?[,]` | any | any | no — PGE04015 |
| `#bool` | `?[,]` | any | any | no — PGE04015 |
| enum | `?[,]` | any | any | no — PGE04015 |

*(Same table applies to `?(,)` and mixed `?[,)` / `?(,]`)*

### Enum Match (`.Value`)

| Left (subject) | Operator | Right (operand) | Valid? |
|----------------|----------|-----------------|--------|
| enum `#S` | `.Value` | member of `#S` | yes |
| enum `#S` | `.Value` | member of `#T` | no — PGE04015 |
| `#int` | `.Value` | any | no — PGE04015 |
| `#float` | `.Value` | any | no — PGE04015 |
| `#string` | `.Value` | any | no — PGE04015 |
| `#bool` | `.Value` | any | no — PGE04015 |

### Wildcard (`*?`)

| Left (subject) | Operator | Valid? |
|----------------|----------|--------|
| any type | `*?` | always yes |

---

**VALID:**
```aljam3
[ ] ✓ range operator on int subject
{-} -CheckScore
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) <score#int
   (-) >label#string
   [ ]
   [?] $score
      [?] ?[0,59]
         [-] >label << "fail"
      [?] ?[60,100]
         [-] >label << "pass"
      [?] *?
         [-] >label << "unknown"
```

```aljam3
[ ] ✓ enum match on enum subject
{-} -RouteByStatus
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) <status#Status
   (-) >action#string
   [ ]
   [?] $status
      [?] =? .Active
         [-] >action << "proceed"
      [?] =? .Inactive
         [-] >action << "skip"
      [?] *?
         [-] >action << "error"
```

```aljam3
[ ] ✓ cross-numeric — int subject with float bounds
{-} -CheckTemp
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) <temp#int
   (-) >alert#string
   [ ]
   [?] $temp
      [?] ?[0.0,36.9]
         [-] >alert << "hypothermia"
      [?] ?[37.0,42.0]
         [-] >alert << "normal-to-fever"
      [?] *?
         [-] >alert << "critical"
```

**INVALID:**
```aljam3
[ ] ✗ PGE04015 — range operator on string subject
{-} -BadRange
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) <name#string
   (-) >out#string
   [ ]
   [?] $name
      [?] ?[1,10]                       [ ] ✗ PGE04015 — range requires numeric
         [-] >out << "short"
      [?] *?
         [-] >out << "other"
```

```aljam3
[ ] ✗ PGE04015 — enum match on int subject
{-} -BadEnum
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) <count#int
   (-) >out#string
   [ ]
   [?] $count
      [?] =? .Active                   [ ] ✗ PGE04015 — enum match requires enum type
         [-] >out << "yes"
      [?] *?
         [-] >out << "no"
```

```aljam3
[ ] ✗ PGE04015 — string equality on int subject
{-} -BadEquality
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) <count#int
   (-) >out#string
   [ ]
   [?] $count
      [?] =? "five"                    [ ] ✗ PGE04015 — string operand on int subject
         [-] >out << "matched"
      [?] *?
         [-] >out << "other"
```

**Open point:** None.

## See Also

- [[syntax/operators|Operators]] — type-operator compatibility rules for conditionals
