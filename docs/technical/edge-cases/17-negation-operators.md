---
audience: design
type: reference
updated: 2026-03-30
---

<!-- @edge-cases/INDEX -->

## 17. Negation Operators (S17)

### EC-17.1: All four negation comparison operators

<!-- @u:operators -->
**EBNF:** negation inserts `!` before `?` in base comparison: `<!?`, `>!?`, `<=!?`, `>=!?`

**What it tests:** Each negated form used correctly in `[?]` conditionals. `<!?` = not-less-than (>=), `>!?` = not-greater-than (<=), `<=!?` = not-less-or-equal (>), `>=!?` = not-greater-or-equal (<). See [[operators#Comparison Operators]].

```aljam3
[ ] Not less than — equivalent to >=
[?] $age <!? 18
   [-] $eligible#bool << #Boolean.True
[?] *?
   [-] $eligible#bool << #Boolean.False

[ ] Not greater than — equivalent to <=
[?] $score >!? 100
   [-] $capped#bool << #Boolean.True
[?] *?
   [-] $capped#bool << #Boolean.False

[ ] Not less-or-equal — equivalent to >
[?] $priority <=!? 3
   [-] $urgent#bool << #Boolean.True
[?] *?
   [-] $urgent#bool << #Boolean.False

[ ] Not greater-or-equal — equivalent to <
[?] $retries >=!? 5
   [-] $giveUp#bool << #Boolean.True
[?] *?
   [-] $giveUp#bool << #Boolean.False
```

### EC-17.2: Negation in compound logical condition

**What it tests:** Negation operators combined with `[&]` / `[+]` logical markers. See [[operators#Comparison Operators]], [[blocks#Logical]].

```aljam3
[ ] Active user who is not banned and age is not less than 13
[?] $active =? #Boolean.True
[&] $banned =!? #Boolean.True
[&] $age <!? 13
   [-] $allowed#bool << #Boolean.True
[?] *?
   [-] $allowed#bool << #Boolean.False
```
