---
rule: "4.15"
code: PGE-415
name: Conditional Type-Operator Mismatch
severity: error
---

### Rule 4.15 — Conditional Type-Operator Mismatch
`PGE-415`

**Statement:** The comparison operator in a `[?]` conditional branch must be compatible with the subject variable's type, and the operand type must be compatible with both the operator and the subject. Numerics (`;int` and `;float`) form one family and freely interoperate. All other types require exact type matching.
**Rationale:** A range operator on a string or an enum match on an integer is always a logic error. Catching type-operator mismatches at compile time prevents nonsensical comparisons that would either always fail or produce undefined results at runtime.
**Detection:** The compiler resolves the subject variable's type, then checks each branch's comparison operator and operand type against the compatibility tables below.

**See also:** PGE-401 (type mismatch — general), PGE-601 (conditional exhaustiveness), PGE-603 (numeric range coverage)

---

#### Compatibility Tables

##### Equality Operators (`=?`, `=!?`)

| Left (subject) | Operator | Right (operand) | Valid? |
|----------------|----------|-----------------|--------|
| `;int` | `=?` | `;int` | yes |
| `;int` | `=?` | `;float` | yes |
| `;int` | `=?` | `;string` | no — PGE-415 |
| `;int` | `=?` | `;bool` | no — PGE-415 |
| `;int` | `=?` | enum | no — PGE-415 |
| `;float` | `=?` | `;float` | yes |
| `;float` | `=?` | `;int` | yes |
| `;float` | `=?` | `;string` | no — PGE-415 |
| `;string` | `=?` | `;string` | yes |
| `;string` | `=?` | `;int` | no — PGE-415 |
| `;string` | `=?` | `;float` | no — PGE-415 |
| `;bool` | `=?` | `;bool` | yes |
| `;bool` | `=?` | `;int` | no — PGE-415 |
| `;bool` | `=?` | `;string` | no — PGE-415 |
| enum `#S` | `=?` | `.Value` of `#S` | yes |
| enum `#S` | `=?` | `;int` | no — PGE-415 |
| enum `#S` | `=?` | `;string` | no — PGE-415 |
| enum `#S` | `=?` | `.Value` of `#T` | no — PGE-415 |

*(Same table applies to `=!?`)*

##### Ordering Operators (`>?`, `<?`, `>!?`, `<!?`)

| Left (subject) | Operator | Right (operand) | Valid? |
|----------------|----------|-----------------|--------|
| `;int` | `>?` | `;int` | yes |
| `;int` | `>?` | `;float` | yes |
| `;int` | `>?` | `;string` | no — PGE-415 |
| `;int` | `>?` | `;bool` | no — PGE-415 |
| `;int` | `>?` | enum | no — PGE-415 |
| `;float` | `>?` | `;float` | yes |
| `;float` | `>?` | `;int` | yes |
| `;float` | `>?` | `;string` | no — PGE-415 |
| `;string` | `>?` | any | no — PGE-415 |
| `;bool` | `>?` | any | no — PGE-415 |
| enum | `>?` | any | no — PGE-415 |

*(Same table applies to `<?`, `>!?`, `<!?`)*

##### Range Operators (`?[lo,hi]`, `?(lo,hi)`, mixed)

| Subject | Operator | Lo bound | Hi bound | Valid? |
|---------|----------|----------|----------|--------|
| `;int` | `?[,]` | `;int` | `;int` | yes |
| `;int` | `?[,]` | `;int` | `;float` | yes |
| `;int` | `?[,]` | `;float` | `;int` | yes |
| `;int` | `?[,]` | `;float` | `;float` | yes |
| `;int` | `?[,]` | `;string` | any | no — PGE-415 |
| `;float` | `?[,]` | `;float` | `;float` | yes |
| `;float` | `?[,]` | `;float` | `;int` | yes |
| `;float` | `?[,]` | `;int` | `;float` | yes |
| `;float` | `?[,]` | `;int` | `;int` | yes |
| `;float` | `?[,]` | `;string` | any | no — PGE-415 |
| `;string` | `?[,]` | any | any | no — PGE-415 |
| `;bool` | `?[,]` | any | any | no — PGE-415 |
| enum | `?[,]` | any | any | no — PGE-415 |

*(Same table applies to `?(,)` and mixed `?[,)` / `?(,]`)*

##### Enum Match (`.Value`)

| Left (subject) | Operator | Right (operand) | Valid? |
|----------------|----------|-----------------|--------|
| enum `#S` | `.Value` | member of `#S` | yes |
| enum `#S` | `.Value` | member of `#T` | no — PGE-415 |
| `;int` | `.Value` | any | no — PGE-415 |
| `;float` | `.Value` | any | no — PGE-415 |
| `;string` | `.Value` | any | no — PGE-415 |
| `;bool` | `.Value` | any | no — PGE-415 |

##### Wildcard (`*?`)

| Left (subject) | Operator | Valid? |
|----------------|----------|--------|
| any type | `*?` | always yes |

---

**VALID:**
```polyglot
[ ] ✓ range operator on int subject
{=} =CheckScore
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <score;int
   [=] >label;string
   [?] $score
      [?] ?[0,59]
         [r] >label << "fail"
      [?] ?[60,100]
         [r] >label << "pass"
      [?] *?
         [r] >label << "unknown"
```

```polyglot
[ ] ✓ enum match on enum subject
{=} =RouteByStatus
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <status;Status
   [=] >action;string
   [?] $status
      [?] =? .Active
         [r] >action << "proceed"
      [?] =? .Inactive
         [r] >action << "skip"
      [?] *?
         [r] >action << "error"
```

```polyglot
[ ] ✓ cross-numeric — int subject with float bounds
{=} =CheckTemp
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <temp;int
   [=] >alert;string
   [?] $temp
      [?] ?[0.0,36.9]
         [r] >alert << "hypothermia"
      [?] ?[37.0,42.0]
         [r] >alert << "normal-to-fever"
      [?] *?
         [r] >alert << "critical"
```

**INVALID:**
```polyglot
[ ] ✗ PGE-415 — range operator on string subject
{=} =BadRange
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <name;string
   [=] >out;string
   [?] $name
      [?] ?[1,10]                       [ ] ✗ PGE-415 — range requires numeric
         [r] >out << "short"
      [?] *?
         [r] >out << "other"
```

```polyglot
[ ] ✗ PGE-415 — enum match on int subject
{=} =BadEnum
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <count;int
   [=] >out;string
   [?] $count
      [?] =? .Active                   [ ] ✗ PGE-415 — enum match requires enum type
         [r] >out << "yes"
      [?] *?
         [r] >out << "no"
```

```polyglot
[ ] ✗ PGE-415 — string equality on int subject
{=} =BadEquality
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <count;int
   [=] >out;string
   [?] $count
      [?] =? "five"                    [ ] ✗ PGE-415 — string operand on int subject
         [r] >out << "matched"
      [?] *?
         [r] >out << "other"
```

**Open point:** None.
