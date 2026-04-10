---
audience: developer
rule: "3.6"
code: PGE03006
name: Race Collector Type Homogeneity
severity: error
---

### Rule 3.6 — Race Collector Type Homogeneity
`PGE03006`

<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->
<!-- @u:syntax/types -->

**Statement:** All `(*) <<` candidate inputs to `*First`, `*Second`, or `*Nth` race collectors must match the target variable's schema (per [TYPE-IDENTITY](../TYPE-IDENTITY.md)). Mixed-schema candidates are a compile error. The `(*) >>` output inherits the target schema. This rule does not apply to `*All` (collect-all) — `*All` has no type constraint since it does not merge values into a single output.
**Rationale:** Race collectors pick a winner from competing parallels. The caller receives one value via `(*) >>` — if candidates have different schemas, the output type is ambiguous. Schema matching per PGE04001 eliminates runtime type confusion.
**Detection:** The compiler resolves the target variable's schema, then compares each `(*) <<` input's schema against it per PGE04001. If any input's schema differs from the target, PGE03006 fires.

**VALID:**
```polyglot
[ ] ✓ all candidates are #string
[=] -Search.EngineA
   (-) >result#string >> $resultA

[=] -Search.EngineB
   (-) >result#string >> $resultB

(*) *First
   (*) << $resultA
   (*) << $resultB
   (*) >> $fastest                   [ ] ✓ type is #string
```

```polyglot
[ ] ✓ *All has no type constraint — mixed types allowed
[=] -Fetch.Profile
   (-) >profile#UserProfile >> $profile

[=] -Fetch.Count
   (-) >count#int >> $count

(*) *All
   (*) << $profile
   (*) << $count                     [ ] ✓ *All does not merge — no type constraint
```

**INVALID:**
```polyglot
[ ] ✗ PGE03006 — mixed types in *First
[=] -Search.Fast
   (-) >result#string >> $fast

[=] -Search.Slow
   (-) >result#int >> $slow

(*) *First
   (*) << $fast                      [ ] #string
   (*) << $slow                      [ ] #int — ✗ PGE03006
   (*) >> $winner
```

```polyglot
[ ] ✗ PGE03006 — array.string ≠ array.int
[=] -Fetch.Names
   (-) >list#array:string >> $names

[=] -Fetch.Counts
   (-) >list#array:int >> $counts

(*) *First
   (*) << $names                     [ ] #array:string
   (*) << $counts                    [ ] #array:int — ✗ PGE03006
   (*) >> $winner
```

**Note:** Type matching is schema-based, not name-based — per [TYPE-IDENTITY.md](../TYPE-IDENTITY.md). Two differently-named `{#}` types with identical field structures are the same type. See resolved design issue 005 (git history: `docs/technical/compiler_issues/005-race-type-matching-semantics.md`) for the design decision.

### See Also

- [[concepts/collections/collect|Collect]] — documents race collector type constraint (references PGE03006)
- [[concepts/collections/examples|Collection Examples]] — error table mapping PGE03006 to example scenarios
- [[technical/spec/type-identity|Type Identity]] — defines schema-based type matching used by PGE03006
