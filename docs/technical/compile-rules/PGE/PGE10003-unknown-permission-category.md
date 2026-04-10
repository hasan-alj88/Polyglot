---
audience: developer
rule: "9.17"
code: PGE10003
name: Unknown Permission Category
severity: error
---

### Rule 9.17 — Unknown Permission Category
`PGE10003`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/packages -->
<!-- @u:syntax/operators -->
<!-- @u:syntax/types -->

**Statement:** A `{_}` permission object's `[.]` field lines must use a known `category_name.capability_name` pair. Polyglot defines exactly 8 permission categories with fixed capabilities (see [[permissions#Per-Category Capability Enums]]). If a `[.]` field line references a category or capability not in the predefined set, PGE10003 fires.
**Rationale:** Permission categories are Polyglot-defined, not user-extensible. An unknown category would create a permission that no IO operation can consume — it cannot grant or restrict anything. Catching this at compile time prevents silent misconfiguration where a developer believes they have granted a capability that does not exist.
**Detection:** The compiler parses each `[.] .Category.Capability` field line in `{_}` blocks and checks against the known set: File (Read, Write, Execute, Delete), Web (Request, Socket), Database (Connect, Read, Write), System (Env, Process, Signal), Crypto (Key, Sign, Encrypt), IPC (Send, Receive, Subscribe), Device (Camera, Microphone, Location, Bluetooth), Memory (Allocate, Shared). If the category or capability is not in this set, PGE10003 fires.

**See also:** PGE10004 (undeclared permission — using IO without permission), PGE10006 (duplicate permission), [[permissions#Per-Category Capability Enums]]

**VALID:**
```polyglot
[ ] ✓ all permission categories and capabilities are known
{_} _LogAccess
   [.] .intent << #Ceiling
   [.] .File.Read "/var/log/*"
   [.] .Web.Request "https://api.example.com/*"
   [.] .System.Env "LOG_LEVEL"

{_} _AppGrant
   [.] .intent << #Grant
   [.] .File.Read "/var/log/app/*"

{@} @Local:999.MyApp:v1.0.0
   [_] _LogAccess

{-} -ReadLogs
   [_] _AppGrant
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] $content << -File.Text.Read >> "/var/log/app/current.log"
```

**INVALID:**
```polyglot
[ ] ✗ PGE10003 — unknown category "Network"
{_} _BadCategory
   [.] .intent << #Grant
   [.] .Network.Send "*"                       [ ] ✗ PGE10003 — Network is not a known category
```

```polyglot
[ ] ✗ PGE10003 — unknown capability "Rename" under known category File
{_} _BadCapability
   [.] .intent << #Grant
   [.] .File.Rename "/tmp/*"                   [ ] ✗ PGE10003 — File.Rename is not a known capability
```

```polyglot
[ ] ✗ PGE10003 — unknown capability "Delete" under known category Web
{_} _BadWebCap
   [.] .intent << #Grant
   [.] .Web.Delete "https://api.example.com/*" [ ] ✗ PGE10003 — Web.Delete is not a known capability
```

**Open point:** None.
