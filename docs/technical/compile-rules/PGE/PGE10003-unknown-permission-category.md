---
rule: "9.17"
code: PGE10003
name: Unknown Permission Category
severity: error
---

### Rule 9.17 — Unknown Permission Category
`PGE10003`

**Statement:** A `[_]` permission declaration must use a known category and subfield. Polyglot defines exactly 8 permission categories with fixed subfields (see [[permissions#Permission Categories]]). If a `[_]` declaration references a category or subfield not in the predefined set, PGE10003 fires.
**Rationale:** Permission categories are Polyglot-defined, not user-extensible. An unknown category would create a permission that no IO operation can consume — it cannot grant or restrict anything. Catching this at compile time prevents silent misconfiguration where a developer believes they have granted a capability that does not exist.
**Detection:** The compiler parses each `_Category.subfield` identifier in `[_]` declarations and checks against the known set: File (.read, .write, .execute, .delete), Web (.request, .socket), Database (.connect, .read, .write), System (.env, .process, .signal), Crypto (.key, .sign, .encrypt), IPC (.send, .receive, .subscribe), Device (.camera, .microphone, .location, .bluetooth), Memory (.allocate, .shared). If the category or subfield is not in this set, PGE10003 fires.

**See also:** PGE10004 (undeclared permission — using IO without permission), PGE10006 (duplicate permission), [[permissions#Permission Categories]]

**VALID:**
```polyglot
[ ] ✓ all permission categories and subfields are known
{@} @Local:999.MyApp:v1.0.0
   [_] _File.read"/var/log/*"
   [_] _Web.request
      [_] <url#string << "https://api.example.com/*"
      [_] <method#string << "GET"
   [_] _System.env"LOG_LEVEL"

{=} =ReadLogs
   [_] _File.read"/var/log/app/*"
   [T] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] $content << =File.Text.Read >> "/var/log/app/current.log"
```

**INVALID:**
```polyglot
[ ] ✗ PGE10003 — unknown category "Network"
{=} =SendData
   [_] _Network.send"*"                       [ ] ✗ PGE10003 — _Network is not a known category
   [T] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] $result << =Web.Request >> "https://api.example.com/data"
```

```polyglot
[ ] ✗ PGE10003 — unknown subfield "rename" under known category File
{=} =RenameFile
   [_] _File.rename"/tmp/*"                   [ ] ✗ PGE10003 — _File.rename is not a known subfield
   [T] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] $result << =File.Move >> "/tmp/old.txt"
```

```polyglot
[ ] ✗ PGE10003 — unknown subfield "delete" under known category Web
{=} =CleanupEndpoint
   [_] _Web.delete"https://api.example.com/*" [ ] ✗ PGE10003 — _Web.delete is not a known subfield
   [T] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] $result << =Web.Request >> "https://api.example.com/cleanup"
```

**Open point:** None.
