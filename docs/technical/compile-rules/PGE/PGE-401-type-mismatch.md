---
rule: "4.1"
code: PGE-401
name: Type Mismatch
severity: error
---

### Rule 4.1 — Type Mismatch
`PGE-401`

**Statement:** Pushing a value into a variable or parameter whose declared type differs from the value's type is a compile error. This applies to all assignment operators (`<<`, `>>`, `<~`, `~>`), IO wiring (`[=]`), collector output (`[*] >>`), and race collector inputs (`[*] <<`). There are no implicit coercions — `int` does not auto-promote to `float`, `string` does not coerce to `path`, etc.

Type identity is defined in [TYPE-IDENTITY.md](../TYPE-IDENTITY.md) — "same type" means "same schema" (structural matching, not nominal).

**Rationale:** Polyglot is type-safe first. Catching type mismatches at compile time eliminates an entire class of runtime errors. Explicit coercion (via a pipeline call) makes type conversions visible and intentional. Schema-based matching (rather than name-based) reflects the reality that all data is serialized strings — the structure is the type.
**Detection:** The compiler compares the resolved schema of the source expression against the declared schema of the target. If the schemas differ in structure or field types, PGE-401 fires at the assignment site.

**Boundary with PGE-402:** PGE-401 fires when the source and target have **structurally different schemas** (different fields or field types). If the schemas match structurally but the source is **missing values** for required fields, see PGE-402.

**VALID:**
```polyglot
[ ] ✓ matching basic types
[=] <name#string
[r] =Greet
   [=] <input#string << <name       [ ] string → string ✓
   [=] >output#string >> $greeting
```

```polyglot
[ ] ✓ matching element-typed arrays
[r] =Fetch.Names
   [=] >list#array:string >> $names
[r] =Process.Names
   [=] <items#array:string << $names [ ] array.string → array.string ✓
```

```polyglot
[ ] ✓ matching user-defined types
[r] =Fetch.User
   [=] >user#UserRecord >> $user
[r] =Save.User
   [=] <record#UserRecord << $user   [ ] UserRecord → UserRecord ✓
```

**INVALID:**
```polyglot
[ ] ✗ PGE-401 — int pushed into string
[r] =Compute
   [=] >count#int >> $count
[r] =Label
   [=] <text#string << $count        [ ] ✗ PGE-401 — int ≠ string
```

```polyglot
[ ] ✗ PGE-401 — array.int pushed into array.string
[r] =Fetch.Scores
   [=] >scores#array:int >> $scores
[r] =Display
   [=] <names#array:string << $scores [ ] ✗ PGE-401 — array.int ≠ array.string
```

```polyglot
[ ] ✗ PGE-401 — different schema (different field structure)
{#} #UserRecord
   [.] .name#string
   [.] .age#int

{#} #OrderRecord
   [.] .orderId#string
   [.] .total#float

[r] =Fetch.User
   [=] >user;#UserRecord >> $user
[r] =Process.Order
   [=] <order;#OrderRecord << $user  [ ] ✗ PGE-401 — schemas differ (name+age ≠ orderId+total)
```

```polyglot
[ ] ✓ different names, same schema — this is valid
{#} #UserProfile
   [.] .name#string
   [.] .email#string

{#} #ContactInfo
   [.] .name#string
   [.] .email#string

[r] =Fetch.Profile
   [=] >profile;#UserProfile >> $profile
[r] =Send.Email
   [=] <contact;#ContactInfo << $profile [ ] ✓ same schema — name#string + email#string
```

```polyglot
[ ] ✗ PGE-401 — no implicit int → float coercion
[r] =Count.Items
   [=] >total#int >> $total
[r] =Divide
   [=] <numerator#float << $total    [ ] ✗ PGE-401 — int ≠ float
```
