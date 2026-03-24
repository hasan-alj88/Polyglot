---
audience: user
type: specification
updated: 2026-03-24
status: complete
changes: "[+] OR reassigned to [|] per issue #74"
---

# Conditionals

<!-- @operators -->
<!-- @blocks:Control Flow -->
<!-- @blocks:Logical -->

Conditionals in Polyglot Code use `[?]` block elements to branch execution based on comparisons. Each branch is a standalone test — there is no "subject" line that introduces a value to match against. See [[operators#Comparison Operators]] for the full operator table and [[blocks#Control Flow]] for marker reference.

## Conditional Chains

Sequential `[?]` blocks form a conditional chain. Each branch contains an explicit comparison and indented execution lines:

```polyglot
[?] $status =? #Status.Ok
   [r] >result << "Success"

[?] $status =? #Status.Warn
   [r] >result << "Warning"

[?] $status =? #Status.Fail
   [r] >result << "Failure"
```

Every `[?]` line must include a comparison operator — bare lines like `[?] $variable` are invalid ([[PGE-609|PGE-609]]). Every branch must contain at least one executable statement; use `[r] =DoNothing` for intentionally empty branches ([[PGE-610|PGE-610]]).

## Exhaustiveness

All conditional chains must be exhaustive — every possible value of the branched type must have a defined path ([[PGE-601|PGE-601]]). Exhaustiveness is proven in two ways:

1. **Static proof** — the compiler verifies all values are covered (closed types)
2. **`*?` catch-all** — required for open types where static proof is impossible

### Enum Exhaustiveness

Enums are closed types. When all variants are listed, no `*?` is needed ([[PGE-602|PGE-602]]):

```polyglot
{#} #Direction
   [.] .North
   [.] .South
   [.] .East
   [.] .West

[ ] All variants covered — no *? needed
[?] $dir =? #Direction.North
   [r] $label;string << "N"
[?] $dir =? #Direction.South
   [r] $label;string << "S"
[?] $dir =? #Direction.East
   [r] $label;string << "E"
[?] $dir =? #Direction.West
   [r] $label;string << "W"
```

Partial coverage with `*?` covering the rest is also valid:

```polyglot
[?] $dir =? #Direction.North
   [r] $label;string << "N"
[?] *?
   [r] $label;string << "other"
```

`#Boolean` follows the same rule — list both `#Boolean.True` and `#Boolean.False`, or use `*?`.

### Numeric Exhaustiveness

Numeric types (`;int`, `;float`) are open but rangeable. Ranges must cover the full domain or include `*?` ([[PGE-603|PGE-603]]). Overlapping ranges are flagged as warnings ([[PGE-604|PGE-604]]):

```polyglot
[?] $code =? 200
   [r] $status;string << "ok"
[?] $code =? 404
   [r] $status;string << "not_found"
[?] $code =? 500
   [r] $status;string << "error"
[?] *?
   [r] $status;string << "unknown"
```

### Match Syntax (Conditional Assignment Sugar)

<!-- @EBNF:match_line -->

When every `[?]` arm performs the same operation — mapping one value to another — use match syntax. Match nests `[?]` arms under a `[r] $source >> $target` header:

```polyglot
[ ] Match form — equivalent to the [?] chain above
[r] $code >> $status;string
   [?] 200 >> "ok"
   [?] 404 >> "not_found"
   [?] 500 >> "error"
   [?] * >> "unknown"
```

This desugars to the verbose form shown in the Numeric Exhaustiveness example above. The two forms are equivalent.

**Rules:**

1. The source variable (`$code`) must be in **Final** state — its value is fully resolved
2. The target variable (`$status`) receives the matched result via push
3. Arms are **assignment-only** — no side effects, pipeline calls, or nested logic
4. `[?] *` is the wildcard catch-all (replaces `*?` from the verbose form — no comparison operator in match arms)
5. All exhaustiveness rules ([[PGE-601|PGE-601]] through [[PGE-613|PGE-613]]) apply to the desugared form
6. [[PGE-609|PGE-609]] does not apply to match arms — they use `value >> result` form, not `$var operator value`

**Enum match — exhaustive without wildcard:**

```polyglot
[r] $dir >> $label;string
   [?] #Direction.North >> "N"
   [?] #Direction.South >> "S"
   [?] #Direction.East >> "E"
   [?] #Direction.West >> "W"
```

All variants of `#Direction` are listed, so no `*` is needed — same rule as the verbose form ([[PGE-602|PGE-602]]).

**Not a match:** If `[r] $x >> $y` has no indented `[?]` children, it is a plain assignment — not a match header.

### String and Flexible Field Exhaustiveness

Strings are open sets — `*?` is always required ([[PGE-606|PGE-606]]). Flexible fields (`:`) are also open — `*?` is always required ([[PGE-607|PGE-607]]).

### Exhaustiveness Summary

| Type | Value Set | `*?` Required? |
|------|-----------|----------------|
| Enum (`{#}` with `[.]` fields) | Closed (finite) | No — if all variants listed |
| `#Boolean` | Closed (2 variants) | No — if both listed |
| `;int` / `;float` | Open but rangeable | No — if ranges cover full domain; otherwise yes |
| `;string` | Open (infinite) | Yes — always |
| Flexible field (`:`) | Open | Yes — always |
| Compound (`[&]`/`[\|]`/`[^]`) | Complex | Depends on variable types |

## Logical Operators

Compound conditions combine multiple predicates using block-element logical markers. See [[blocks#Logical]] for marker definitions.

### `[&]` — AND

Both conditions must hold:

```polyglot
[?] $age >=? 18
[&] $verified =? #Boolean.True
   [r] $access << #AccessLevel.Granted
[?] *?
   [r] $access << #AccessLevel.Denied
```

### `[|]` — OR

At least one condition holds:

```polyglot
[?] $role =? #Role.Admin
[|] $role =? #Role.Superuser
   [r] $elevated;bool << #Boolean.True
[?] *?
   [r] $elevated;bool << #Boolean.False
```

### `[^]` — XOR

Exactly one of two conditions holds — not both, not neither:

```polyglot
[?] $isAdmin =? #Boolean.True
[^] $isSudo =? #Boolean.True
   [r] $elevated;bool << #Boolean.True
[?] *?
   [r] $elevated;bool << #Boolean.False
```

### `[-]` — NOT

Negate the preceding condition. For simple negation, prefer negation operators (`=!?`, `<!?`, etc.) over `[-]` — see [[operators#Negation Operators]].

### Compound Exhaustiveness

When logical operators combine conditions, the compiler evaluates whether the compound expression partitions the input space ([[PGE-608|PGE-608]]). If any variable is an open type, `*?` is required. Overlapping compound conditions are flagged ([[PGE-605|PGE-605]]). Tautological branches (always true) and contradictory branches (always false) are compile errors ([[PGE-613|PGE-613]]).

## Nested Conditionals

A `[?]` branch can contain inner `[?]` chains. Each nesting level is independently exhaustive:

```polyglot
[?] $role =? #Role.Admin
   [?] $region =? #Region.EU
      [r] $policy;string << "GDPR"
   [?] $region =? #Region.US
      [r] $policy;string << "CCPA"
   [?] *?
      [r] $policy;string << "Global"
[?] $role =? #Role.User
   [r] $policy;string << "Standard"
[?] *?
   [r] $policy;string << "None"
```

The outer chain branches on `$role`. Inside the Admin branch, a separate chain branches on `$region` — this inner chain has its own `*?` because `#Region` may have more than EU and US variants.

## Switching on Metadata

Conditionals can switch on live metadata fields like pipeline `%status`:

```polyglot
[?] =DataSync%status
   [?] #AwaitTrigger
      [r] $msg;string << "idle"
   [?] #Running
      [r] $msg;string << "in progress"
   [?] #Failed
      [r] $msg;string << "failed"
      [b] =Alert.Send
         [=] <msg << "DataSync failed"
   [?] #Disabled
      [r] $msg;string << "pipeline disabled"
   [?] *?
      [r] $msg;string << "unknown state"
```

See [[types#Live Type Modifier]] and [[pipelines#Querying Pipeline Status]] for metadata access patterns.

## Wildcard Rules

- Only one `*?` per chain ([[PGE-611|PGE-611]])
- `*?` must be the last branch — branches after `*?` are unreachable dead code ([[PGE-612|PGE-612]])
- `*?` catches everything the preceding branches did not

## Compile Rules Reference

| Rule | Name | What it catches |
|------|------|-----------------|
| [[PGE-601\|PGE-601]] | Conditional Must Be Exhaustive | Missing coverage for any possible value |
| [[PGE-602\|PGE-602]] | Enum Exhaustiveness | Missing enum variants without `*?` |
| [[PGE-603\|PGE-603]] | Numeric Range Not Exhaustive | Incomplete numeric range coverage |
| [[PGE-604\|PGE-604]] | Numeric Range Overlap | Overlapping range branches |
| [[PGE-605\|PGE-605]] | Compound Condition Overlap | Overlapping compound expressions |
| [[PGE-606\|PGE-606]] | String Exhaustiveness | Missing `*?` on string conditionals |
| [[PGE-607\|PGE-607]] | Flexible Field Exhaustiveness | Missing `*?` on flexible field conditionals |
| [[PGE-608\|PGE-608]] | Compound Condition Exhaustiveness | Incomplete compound condition coverage |
| [[PGE-609\|PGE-609]] | Conditional Missing Comparison Operator | Bare `[?] $variable` without operator |
| [[PGE-610\|PGE-610]] | Empty Conditional Scope | Branch with no executable statement |
| [[PGE-611\|PGE-611]] | Duplicate Wildcard Catch-All | More than one `*?` in a chain |
| [[PGE-612\|PGE-612]] | Unreachable Branch After Wildcard | Branches placed after `*?` |
| [[PGE-613\|PGE-613]] | Tautological Branch Condition | Always-true or always-false compound expression |
