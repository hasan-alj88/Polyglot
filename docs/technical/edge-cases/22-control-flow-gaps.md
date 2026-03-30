---
audience: developer
type: reference
updated: 2026-03-30
---

<!-- @edge-cases/INDEX -->

## 22. Control Flow — Gaps (S22)

### EC-22.1: Exhaustiveness — `[?] *?` catch-all is mandatory when conditions are non-exhaustive

<!-- @operators -->
**EBNF:** `conditional_chain ::= { conditional_branch } [ wildcard_branch ]` — wildcard required if set is non-exhaustive.

**What it tests:** A conditional on a string/int value (open set) requires `*?`. Missing `*?` is a compile error. See [[operators#Comparison Operators]].

```polyglot
[ ] VALID — open set needs *?
[?] $code =? 200
   [r] $status#string << "ok"
[?] $code =? 404
   [r] $status#string << "not_found"
[?] $code =? 500
   [r] $status#string << "error"
[?] *?
   [r] $status#string << "unknown"

[ ] VALID — exhaustive enum: all variants covered, no *? needed
[?] $dir =? #Direction.North
   [r] $label#string << "N"
[?] $dir =? #Direction.South
   [r] $label#string << "S"
[?] $dir =? #Direction.East
   [r] $label#string << "E"
[?] $dir =? #Direction.West
   [r] $label#string << "W"
```

### EC-22.2: Nested conditionals inside a branch

**What it tests:** A `[?]` block inside another `[?]` branch — each nesting level is independently exhaustive.

```polyglot
[?] $role =? #Role.Admin
   [?] $region =? #Region.EU
      [r] $policy#string << "GDPR"
   [?] $region =? #Region.US
      [r] $policy#string << "CCPA"
   [?] *?
      [r] $policy#string << "Global"
[?] $role =? #Role.User
   [r] $policy#string << "Standard"
[?] *?
   [r] $policy#string << "None"
```

### EC-22.3: Switching on pipeline `%status` — nested enum switch

**What it tests:** `[?]` on a live metadata field; inner `[?]` checks enum variants. All branches plus `*?`. See [[syntax/types/hierarchy#Live Type Modifier]], [[concepts/pipelines/chains#Querying Pipeline Status]].

```polyglot
[?] =DataSync%status
   [?] #AwaitTrigger
      [r] $msg#string << "idle"
   [?] #Running
      [r] $msg#string << "in progress — instances: {$count}"
   [?] #Failed
      [r] $msg#string << "failed — check errors"
      [b] =Alert.Send
         [=] <msg << "DataSync failed"
   [?] #Disabled
      [r] $msg#string << "pipeline disabled"
   [?] *?
      [r] $msg#string << "unknown state"
```

### EC-22.4: `[^]` XOR logical operator

<!-- @blocks:Logical -->
**What it tests:** XOR condition — true when exactly one of two conditions holds. See [[blocks#Logical]].

```polyglot
[ ] Exactly one of $isAdmin or $isSudo — not both, not neither
[?] $isAdmin =? #Boolean.True
[^] $isSudo =? #Boolean.True
   [r] $elevated#bool << #Boolean.True
[?] *?
   [r] $elevated#bool << #Boolean.False
```
