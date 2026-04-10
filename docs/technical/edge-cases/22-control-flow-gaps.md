---
audience: designer
type: reference
updated: 2026-03-30
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
