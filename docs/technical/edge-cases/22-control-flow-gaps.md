---
audience: design
type: reference
updated: 2026-04-17
---

<!-- @edge-cases/INDEX -->

## 22. Control Flow — Gaps (S22)

### EC-22.1: Exhaustiveness — `[?] *?` catch-all is mandatory when conditions are non-exhaustive

<!-- @u:operators -->
**EBNF:** `conditional_chain ::= { conditional_branch } [ wildcard_branch ]` — wildcard required if set is non-exhaustive.

**What it tests:** A conditional on a string/int value (open set) requires `*?`. Missing `*?` is a compile error. See [[operators#Comparison Operators]].

```polyglot
[ ] VALID — open set needs *?
[?] $code =? 200
   [-] $status#string << "ok"
[?] $code =? 404
   [-] $status#string << "not_found"
[?] $code =? 500
   [-] $status#string << "error"
[?] *?
   [-] $status#string << "unknown"

[ ] VALID — exhaustive enum: all variants covered, no *? needed
[?] $dir =? #Direction.North
   [-] $label#string << "N"
[?] $dir =? #Direction.South
   [-] $label#string << "S"
[?] $dir =? #Direction.East
   [-] $label#string << "E"
[?] $dir =? #Direction.West
   [-] $label#string << "W"
```

### EC-22.2: Nested conditionals inside a branch

**What it tests:** A `[?]` block inside another `[?]` branch — each nesting level is independently exhaustive.

```polyglot
[?] $role =? #Role.Admin
   [?] $region =? #Region.EU
      [-] $policy#string << "GDPR"
   [?] $region =? #Region.US
      [-] $policy#string << "CCPA"
   [?] *?
      [-] $policy#string << "Global"
[?] $role =? #Role.User
   [-] $policy#string << "Standard"
[?] *?
   [-] $policy#string << "None"
```

### EC-22.3: Switching on pipeline `%status` — nested enum switch

**What it tests:** `[?]` on a live metadata field; inner `[?]` checks enum variants. All branches plus `*?`. See [[syntax/types/hierarchy#Live Type Modifier]], [[concepts/pipelines/chains#Querying Pipeline Status]].

```polyglot
[?] -DataSync%status
   [?] #AwaitTrigger
      [-] $msg#string << "idle"
   [?] #Running
      [-] $msg#string << "in progress — instances: {$count}"
   [?] #Failed
      [-] $msg#string << "failed — check errors"
      [b] -Alert.Send
         (-) <msg << "DataSync failed"
   [?] #Disabled
      [-] $msg#string << "pipeline disabled"
   [?] *?
      [-] $msg#string << "unknown state"
```

### EC-22.4: `[^]` XOR logical operator

<!-- @u:blocks:Logical -->
**What it tests:** XOR condition — true when exactly one of two conditions holds. See [[blocks#Logical]].

```polyglot
[ ] Exactly one of $isAdmin or $isSudo — not both, not neither
[?] $isAdmin =? #Boolean.True
[^] $isSudo =? #Boolean.True
   [-] $elevated#bool << #Boolean.True
[?] *?
   [-] $elevated#bool << #Boolean.False
```

### EC-22.5: [C] foreign code lines valid outside -RT.* scope — PGW01004 (X.44 PGW)

**EBNF ref:** `foreign_code_line ::= "[C]" any_text` (§11.6)
**What it tests:** `[C]` lines are syntactically valid in any execution scope because `foreign_code_elem` is a general block element (§5). However, the spec says they are only meaningful "passed to `-RT.*` runtime pipelines." Nothing in the grammar restricts `[C]` to appear only as children of `-RT.*` calls. Orphaned `[C]` lines parse fine but serve no purpose.

**Decision:** PGW01004 warns on `[C]` lines not scoped under a `-RT.*` pipeline call. Tightening the EBNF would require the grammar to understand pipeline semantics (which `-` prefixed names are `-RT.*`), so a semantic warning is more practical.

```polyglot
[ ] ✓ VALID — [C] under -RT.Python.Script (intended use)
[-] -RT.Python.Script
   (-) <env << $env
   (-) <script <<
      [C] import os
      [C] print(os.getcwd())
   (-) >stdout >> $output

[ ] ⚠ PGW01004 — [C] at pipeline top level, not under -RT.* call
[-] -File.Text.Read
   (-) <path << "/tmp/data.txt"
[C] print("orphaned code")

[ ] ⚠ PGW01004 — [C] inside conditional without -RT.* parent
[?] $mode =? "debug"
   [C] console.log("debug")
[?] *?
   [-] -DoNothing
```

**See also:** [[compile-rules/PGW/PGW01004-orphaned-foreign-code|PGW01004 — Orphaned Foreign Code]], [[compile-rules/PGE/PGE01027-empty-foreign-code|PGE01027 — Empty Foreign Code Block]], [[ebnf/11-control-flow#11.6 Foreign Code Injection]]
