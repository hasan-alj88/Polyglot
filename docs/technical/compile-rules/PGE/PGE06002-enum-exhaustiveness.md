---
audience: developer
rule: "6.2"
code: PGE06002
name: Enum Exhaustiveness
severity: error
---

# Rule 6.2 — Enum Exhaustiveness
`PGE06002`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->
<!-- @u:syntax/types -->

**Statement:** When a `[?]` conditional branches on a `{#}` enum type, all declared enum variants must either appear as explicit branches or be covered by a `*?` catch-all. When **all** variants are explicitly listed, `*?` is optional — exhaustiveness is proven statically. When some variants are missing and no `*?` is present, PGE06002 fires.
**Rationale:** Enum types have a closed set of variants defined at compile time. Failing to handle a variant is a likely bug — the compiler enforces that every variant has a path, either explicitly or via `*?`.
**Detection:** The compiler resolves the `{#}` enum definition, enumerates all declared variants, and compares them against the explicit `[?]` branches. If any variant is missing and no `*?` branch exists, PGE06002 fires.

---

## Enum Types Subject to PGE06002

| Type | Example | Variants |
|------|---------|----------|
| User-defined enum | `{#} #Severity` with `[.] .Critical`, `[.] .Error`, etc. | All declared `[.]` fields |
| `#Boolean` (system enum) | Built-in | `#Boolean.True`, `#Boolean.False` |
| Nested enum with sub-fields | `{#} #Result` with `[.] .Success` containing value sub-fields | Top-level `[.]` fields only |

---

## User-Defined Enum

**VALID:**
```aljam3
{#} #Severity
   [.] .Critical
   [.] .Error
   [.] .Warning
   [.] .Info

[ ] ✓ all variants listed — exhaustive, no *? needed
[?] $level#Severity
   [?] #Severity.Critical
      [-] -Alert.Page
   [?] #Severity.Error
      [-] -Alert.Email
   [?] #Severity.Warning
      [-] -Log.Warning
   [?] #Severity.Info
      [-] -Log.Info
```

```aljam3
[ ] ✓ partial coverage + *? covers the rest
[?] $level#Severity
   [?] #Severity.Critical
      [-] -Alert.Page
   [?] *?
      [-] -Log.Default           [ ] ✓ *? covers Error, Warning, Info
```

**INVALID:**
```aljam3
{#} #Severity
   [.] .Critical
   [.] .Error
   [.] .Warning
   [.] .Info

[ ] ✗ PGE06002 — .Warning and .Info not covered, no *?
[?] $level#Severity
   [?] #Severity.Critical
      [-] -Alert.Page
   [?] #Severity.Error
      [-] -Alert.Email
   [ ] ✗ PGE06002 — .Warning, .Info unhandled
   [ ] also ✗ PGE06001 — not exhaustive, no *?
```

---

## `#Boolean` — System Enum

`#Boolean` is a system-provided enum with exactly two variants. PGE06002 treats it identically to any user-defined enum. When both variants are listed, `*?` is optional.

**VALID:**
```aljam3
[ ] ✓ both variants listed — no *? needed
[?] $isActive#bool
   [?] #Boolean.True
      [-] -Handle.Active
   [?] #Boolean.False
      [-] -Handle.Inactive
```

```aljam3
[ ] ✓ one variant + *? covers the other
[?] $isActive#bool
   [?] #Boolean.True
      [-] -Handle.Active
   [?] *?
      [-] -Handle.NotActive    [ ] ✓ *? covers #Boolean.False
```

**INVALID:**
```aljam3
[ ] ✗ PGE06002 — #Boolean.False not covered, no *?
[?] $isActive#bool
   [?] #Boolean.True
      [-] -Handle.Active
   [ ] ✗ PGE06002 — #Boolean.False unhandled
   [ ] also ✗ PGE06001 — not exhaustive, no *?
```

---

## Nested Enum — Variants With Value Sub-Fields

When an enum variant contains value sub-fields, PGE06002 checks only the **top-level variants**. Sub-field values are accessed after matching the variant, not as part of the exhaustiveness check.

**VALID:**
```aljam3
{#} #Result
   [.] .Success
      [.] .message#string <~ ""
   [.] .Failed
      [.] .reason#string <~ "unknown"
      [.] .retries#int <~ 0

[ ] ✓ both top-level variants — exhaustive, no *? needed
[?] $outcome#Result
   [?] #Result.Success
      [-] -Log.Success
         (-) <msg << $outcome.message
   [?] #Result.Failed
      [-] -Log.Failure
         (-) <reason << $outcome.reason
```

---

## Cross-Package Enum

When branching on an enum imported from another package, use the `@alias#EnumName.Variant` form. PGE06002 resolves the enum definition across packages.

**VALID:**
```aljam3
[ ] ✓ imported enum — all variants covered, no *? needed
[?] $status;@models#OrderStatus
   [?] @models#OrderStatus.Enqueued
      [-] -Process.Queue
   [?] @models#OrderStatus.Shipped
      [-] -Process.Track
   [?] @models#OrderStatus.Delivered
      [-] -Process.Archive
```

---

## PGE06002 vs PGE06001 Interaction

| Scenario | PGE06001 | PGE06002 | Result |
|----------|---------|---------|--------|
| All enum variants listed, no `*?` | ✓ (exhaustive) | ✓ | Valid |
| All enum variants listed, `*?` present | ✓ | ✓ | Valid |
| Partial variants, `*?` present | ✓ | ✓ (`*?` covers rest) | Valid |
| Partial variants, no `*?` | ✗ | ✗ | Error (both) |

---

**Open point:** None.

## See Also

- [[user/concepts/conditionals|Conditionals]] — enum exhaustiveness examples and PGE06002 error table
- [[technical/edge-cases/11-control-flow|EC: Control Flow]] — references PGE06002 in enum match exhaustiveness test
