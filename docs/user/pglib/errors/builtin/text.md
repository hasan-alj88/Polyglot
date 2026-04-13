---
audience: automation-builder
type: specification
updated: 2026-04-10
status: complete
---

# `!Text`

No `[@]` import needed — pglib built-in.

```polyglot
{!} !Text
   [:] :Diff
      [.] .EmptyInput#Error
         (-) .MessageTemplate << "Diff input is empty: {side}"
         (-) .Info
            [:] :side#string
   [:] :Lines
      [.] .Empty#Error
         (-) .MessageTemplate << "Text has no lines"
   [:] :Append
      [.] .EmptyResult#Error
         (-) .MessageTemplate << "Append produced empty result"
   [:] :Merge
      [.] .InvalidLineNumber#Error
         (-) .MessageTemplate << "Invalid line number: {lineNumber}"
         (-) .Info
            [:] :lineNumber#int
      [.] .EmptyBase#Error
         (-) .MessageTemplate << "Merge base text is empty"
```
