---
rule: "6.2"
code: PGE-602
name: Enum Exhaustiveness
severity: error
---

### Rule 6.2 — Enum Exhaustiveness
`PGE-602`

**Statement:** When a `[?]` conditional branches on a `{#}` enum type, all declared enum variants must either appear as explicit branches or be covered by a `*?` catch-all. When **all** variants are explicitly listed, `*?` is optional — exhaustiveness is proven statically. When some variants are missing and no `*?` is present, PGE-602 fires.
**Rationale:** Enum types have a closed set of variants defined at compile time. Failing to handle a variant is a likely bug — the compiler enforces that every variant has a path, either explicitly or via `*?`.
**Detection:** The compiler resolves the `{#}` enum definition, enumerates all declared variants, and compares them against the explicit `[?]` branches. If any variant is missing and no `*?` branch exists, PGE-602 fires.

---

#### Enum Types Subject to PGE-602

| Type | Example | Variants |
|------|---------|----------|
| User-defined enum | `{#} #Severity` with `[.] .Critical`, `[.] .Error`, etc. | All declared `[.]` fields |
| `#Boolean` (system enum) | Built-in | `#Boolean.True`, `#Boolean.False` |
| Nested enum with sub-fields | `{#} #Result` with `[.] .Success` containing value sub-fields | Top-level `[.]` fields only |

---

#### User-Defined Enum

**VALID:**
```polyglot
{#} #Severity
   [.] .Critical
   [.] .Error
   [.] .Warning
   [.] .Info

[ ] ✓ all variants listed — exhaustive, no *? needed
[?] $level#Severity
   [?] #Severity.Critical
      [r] =Alert.Page
   [?] #Severity.Error
      [r] =Alert.Email
   [?] #Severity.Warning
      [r] =Log.Warning
   [?] #Severity.Info
      [r] =Log.Info
```

```polyglot
[ ] ✓ partial coverage + *? covers the rest
[?] $level#Severity
   [?] #Severity.Critical
      [r] =Alert.Page
   [?] *?
      [r] =Log.Default           [ ] ✓ *? covers Error, Warning, Info
```

**INVALID:**
```polyglot
{#} #Severity
   [.] .Critical
   [.] .Error
   [.] .Warning
   [.] .Info

[ ] ✗ PGE-602 — .Warning and .Info not covered, no *?
[?] $level#Severity
   [?] #Severity.Critical
      [r] =Alert.Page
   [?] #Severity.Error
      [r] =Alert.Email
   [ ] ✗ PGE-602 — .Warning, .Info unhandled
   [ ] also ✗ PGE-601 — not exhaustive, no *?
```

---

#### `#Boolean` — System Enum

`#Boolean` is a system-provided enum with exactly two variants. PGE-602 treats it identically to any user-defined enum. When both variants are listed, `*?` is optional.

**VALID:**
```polyglot
[ ] ✓ both variants listed — no *? needed
[?] $isActive#bool
   [?] #Boolean.True
      [r] =Handle.Active
   [?] #Boolean.False
      [r] =Handle.Inactive
```

```polyglot
[ ] ✓ one variant + *? covers the other
[?] $isActive#bool
   [?] #Boolean.True
      [r] =Handle.Active
   [?] *?
      [r] =Handle.NotActive    [ ] ✓ *? covers #Boolean.False
```

**INVALID:**
```polyglot
[ ] ✗ PGE-602 — #Boolean.False not covered, no *?
[?] $isActive#bool
   [?] #Boolean.True
      [r] =Handle.Active
   [ ] ✗ PGE-602 — #Boolean.False unhandled
   [ ] also ✗ PGE-601 — not exhaustive, no *?
```

---

#### Nested Enum — Variants With Value Sub-Fields

When an enum variant contains value sub-fields, PGE-602 checks only the **top-level variants**. Sub-field values are accessed after matching the variant, not as part of the exhaustiveness check.

**VALID:**
```polyglot
{#} #Result
   [.] .Success
      [.] .message#string <~ ""
   [.] .Failed
      [.] .reason#string <~ "unknown"
      [.] .retries#int <~ 0

[ ] ✓ both top-level variants — exhaustive, no *? needed
[?] $outcome#Result
   [?] #Result.Success
      [r] =Log.Success
         [=] <msg << $outcome.message
   [?] #Result.Failed
      [r] =Log.Failure
         [=] <reason << $outcome.reason
```

---

#### Cross-Package Enum

When branching on an enum imported from another package, use the `@alias#EnumName.Variant` form. PGE-602 resolves the enum definition across packages.

**VALID:**
```polyglot
[ ] ✓ imported enum — all variants covered, no *? needed
[?] $status;@models#OrderStatus
   [?] @models#OrderStatus.Pending
      [r] =Process.Queue
   [?] @models#OrderStatus.Shipped
      [r] =Process.Track
   [?] @models#OrderStatus.Delivered
      [r] =Process.Archive
```

---

#### PGE-602 vs PGE-601 Interaction

| Scenario | PGE-601 | PGE-602 | Result |
|----------|---------|---------|--------|
| All enum variants listed, no `*?` | ✓ (exhaustive) | ✓ | Valid |
| All enum variants listed, `*?` present | ✓ | ✓ | Valid |
| Partial variants, `*?` present | ✓ | ✓ (`*?` covers rest) | Valid |
| Partial variants, no `*?` | ✗ | ✗ | Error (both) |

---

**Open point:** None.
