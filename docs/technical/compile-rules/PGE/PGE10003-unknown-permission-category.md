---
audience: developer
rule: "9.17"
code: PGE10003
name: Unknown Permission Category
severity: error
---

# Rule 9.17 — Unknown Permission Category
`PGE10003`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/packages -->
<!-- @u:syntax/operators -->
<!-- @u:syntax/types -->

**Statement:** A `{_}` permission object's `[.]` field lines must use known field names. The `.category` field must be one of the 8 predefined categories, and the `.capability` field must be a valid capability for that category (see [[permissions#Per-Category Capability Enums]]). If a field references a category or capability not in the predefined set, PGE10003 fires.
**Rationale:** Permission categories are Polyglot-defined, not user-extensible. An unknown category would create a permission that no IO operation can consume — it cannot grant or restrict anything. Catching this at compile time prevents silent misconfiguration where a developer believes they have granted a capability that does not exist.
**Detection:** The compiler parses each `[.] .category` and `[.] .capability` field in `{_}` blocks and checks against the known set: File (Read, Write, Execute, Delete, Create), Web (Request, Socket, Listen), Database (Connect, Read, Write), System (Env, Process, Signal, Shell), Crypto (Key, Sign, Encrypt), IPC (Send, Receive, Subscribe), Device (Camera, Microphone, Location, Bluetooth), Memory (Allocate, Shared). If the value is not in this set, PGE10003 fires.

**See also:** PGE10004 (undeclared permission — using IO without permission), PGE10006 (duplicate permission), [[permissions#Per-Category Capability Enums]]

**VALID:**
```polyglot
[ ] ✓ all permission categories and capabilities are known
{_} _LogAccess
   [.] .intent << #Ceiling
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/var/log/*"
   [.] .path "/var/log/*"

{_} _AppGrant
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/var/log/app/*"
   [.] .path "/var/log/app/current.log"

{@} @Local:999.MyApp:v1.0.0
   (@) _LogAccess

{-} -ReadLogs
   (-) _AppGrant
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] -File.Text.Read
      (-) <path << _AppGrant
      (-) >content >> $content
```

**INVALID:**
```polyglot
[ ] ✗ PGE10003 — unknown category "Network"
{_} _BadCategory
   [.] .intent << #Grant
   [.] .category #Network                         [ ] ✗ PGE10003 — Network is not a known category
   [.] .capability #Send
   [.] .scope "*"
```

```polyglot
[ ] ✗ PGE10003 — unknown capability "Rename" under known category File
{_} _BadCapability
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Rename                        [ ] ✗ PGE10003 — Rename is not a known File capability
   [.] .scope "/tmp/*"
   [.] .path "/tmp/*"
```

```polyglot
[ ] ✗ PGE10003 — unknown capability "Delete" under known category Web
{_} _BadWebCap
   [.] .intent << #Grant
   [.] .category #Web
   [.] .capability #Delete                        [ ] ✗ PGE10003 — Delete is not a known Web capability
   [.] .scope "https://api.example.com/*"
   [.] .host "api.example.com"
```

**Open point:** None.
