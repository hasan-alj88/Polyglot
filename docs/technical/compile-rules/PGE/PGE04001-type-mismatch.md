---
audience: developer
rule: "4.1"
code: PGE04001
name: Type Mismatch
severity: error
---

# Rule 4.1 — Type Mismatch
`PGE04001`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->
<!-- @u:syntax/types -->

**Statement:** Pushing a value into a variable or parameter whose declared type differs from the value's type is a compile error. This applies to all assignment operators (`<<`, `>>`, `<~`, `~>`), IO wiring (`(-)`), collector output (`(*) >>`), and race collector inputs (`(*) <<`). There are no implicit coercions — `int` does not auto-promote to `float`, `string` does not coerce to `path`, etc.

Type identity is defined in [TYPE-IDENTITY.md](../TYPE-IDENTITY.md) — "same type" means "same schema" (structural matching, not nominal).

**Rationale:** Polyglot is type-safe first. Catching type mismatches at compile time eliminates an entire class of runtime errors. Explicit coercion (via a pipeline call) makes type conversions visible and intentional. Schema-based matching (rather than name-based) reflects the reality that all data is serialized strings — the structure is the type.
**Detection:** The compiler compares the resolved schema of the source expression against the declared schema of the target. If the schemas differ in structure or field types, PGE04001 fires at the assignment site.

**Boundary with PGE04002:** PGE04001 fires when the source and target have **structurally different schemas** (different fields or field types). If the schemas match structurally but the source is **missing values** for required fields, see PGE04002.

**VALID:**
```polyglot
[ ] ✓ matching basic types
(-) <name#string
[-] -Greet
   (-) <input#string << <name       [ ] string → string ✓
   (-) >output#string >> $greeting
```

```polyglot
[ ] ✓ matching element-typed arrays
[-] -Fetch.Names
   (-) >list#array:string >> $names
[-] -Process.Names
   (-) <items#array:string << $names [ ] array.string → array.string ✓
```

```polyglot
[ ] ✓ matching user-defined types
[-] -Fetch.User
   (-) >user#UserRecord >> $user
[-] -Save.User
   (-) <record#UserRecord << $user   [ ] UserRecord → UserRecord ✓
```

**INVALID:**
```polyglot
[ ] ✗ PGE04001 — int pushed into string
[-] -Compute
   (-) >count#int >> $count
[-] -Label
   (-) <text#string << $count        [ ] ✗ PGE04001 — int ≠ string
```

```polyglot
[ ] ✗ PGE04001 — array.int pushed into array.string
[-] -Fetch.Scores
   (-) >scores#array:int >> $scores
[-] -Display
   (-) <names#array:string << $scores [ ] ✗ PGE04001 — array.int ≠ array.string
```

```polyglot
[ ] ✗ PGE04001 — different schema (different field structure)
{#} #UserRecord
   [.] .name#string
   [.] .age#int

{#} #OrderRecord
   [.] .orderId#string
   [.] .total#float

[-] -Fetch.User
   (-) >user#UserRecord >> $user
[-] -Process.Order
   (-) <order#OrderRecord << $user  [ ] ✗ PGE04001 — schemas differ (name+age ≠ orderId+total)
```

```polyglot
[ ] ✓ different names, same schema — this is valid
{#} #UserProfile
   [.] .name#string
   [.] .email#string

{#} #ContactInfo
   [.] .name#string
   [.] .email#string

[-] -Fetch.Profile
   (-) >profile#UserProfile >> $profile
[-] -Send.Email
   (-) <contact#ContactInfo << $profile [ ] ✓ same schema — name#string + email#string
```

```polyglot
[ ] ✗ PGE04001 — no implicit int → float coercion
[-] -Count.Items
   (-) >total#int >> $total
[-] -Divide
   (-) <numerator#float << $total    [ ] ✗ PGE04001 — int ≠ float
```

## See Also

- [[syntax/types/strings|String Types]] — plain string to `#path` type mismatch
- [[pglib/types/structs|Struct Types]] — `#path` type mismatch examples
- [[technical/edge-cases/04-type-system|EC-4.7: Plain string to #path type mismatch]] — references PGE04001
- [[technical/edge-cases/04-type-system|EC-4.14: Typed flexible wildcard — contradicting annotation]] — references PGE04001
