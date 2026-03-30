---
audience: developer
type: reference
updated: 2026-03-30
---

<!-- @edge-cases/INDEX -->

## 17. Negation Operators (S17)

### EC-17.1: All four negation comparison operators

<!-- @operators -->
**EBNF:** negation inserts `!` before `?` in base comparison: `<!?`, `>!?`, `<=!?`, `>=!?`

**What it tests:** Each negated form used correctly in `[?]` conditionals. `<!?` = not-less-than (>=), `>!?` = not-greater-than (<=), `<=!?` = not-less-or-equal (>), `>=!?` = not-greater-or-equal (<). See [[operators#Comparison Operators]].

```polyglot
[ ] Not less than — equivalent to >=
[?] $age <!? 18
   [r] $eligible#bool << #Boolean.True
[?] *?
   [r] $eligible#bool << #Boolean.False

[ ] Not greater than — equivalent to <=
[?] $score >!? 100
   [r] $capped#bool << #Boolean.True
[?] *?
   [r] $capped#bool << #Boolean.False

[ ] Not less-or-equal — equivalent to >
[?] $priority <=!? 3
   [r] $urgent#bool << #Boolean.True
[?] *?
   [r] $urgent#bool << #Boolean.False

[ ] Not greater-or-equal — equivalent to <
[?] $retries >=!? 5
   [r] $giveUp#bool << #Boolean.True
[?] *?
   [r] $giveUp#bool << #Boolean.False
```

### EC-17.2: Negation in compound logical condition

**What it tests:** Negation operators combined with `[&]` / `[|]` logical markers. See [[operators#Comparison Operators]], [[blocks#Logical]].

```polyglot
[ ] Active user who is not banned and age is not less than 13
[?] $active =? #Boolean.True
[&] $banned =!? #Boolean.True
[&] $age <!? 13
   [r] $allowed#bool << #Boolean.True
[?] *?
   [r] $allowed#bool << #Boolean.False
```
