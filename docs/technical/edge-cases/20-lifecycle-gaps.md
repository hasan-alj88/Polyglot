---
audience: designer
type: reference
updated: 2026-03-30
---

<!-- @edge-cases/INDEX -->

## 20. Variable Lifecycle — Gaps (S20)

### EC-20.1: Declared state — value field without assignment cannot be pulled

<!-- @c:variable-lifecycle -->
<!-- @u:identifiers:Serialization Rules -->
**What it tests:** A value field with no assignment is in **Declared** state. Pulling from it before assignment is a compile error. Assignment within value siblings is individually optional. See [[identifiers#Serialization Rules]].

```polyglot
[ ] VALID — declared field, pushed to later
{#} #Request
   [.] .id#string
   [.] .method#string <~ "GET"

[ ] In execution: .id is Declared, must be pushed before use
[-] $req#Request
   [.] .id << $incomingId
[ ] .method uses default; .id is now Final
[-] >requestOut << $req

[ ] INVALID — pulling from Declared variable is a compile error
[ ] [-] -Pipeline.Call
[ ]    (-) <x << $req.id   <- compile error if .id never pushed
```

### EC-20.2: Released state — variable in mini-pipeline cannot be used outside expand scope

**What it tests:** Variables declared inside `=ForEach` body are Released when the mini-pipeline ends. Accessing them outside is a compile error. See [[concepts/collections/expand#Expand Operators]].

```polyglot
[=] =ForEach.Array
   (=) <Array << $items
   (=) >item >> $item

   [-] $doubled#int << $item * 2

   [-] *Agg.Sum
      (*) <number << $doubled
      (*) >sum >> >total

[ ] VALID — $total was written to output port, accessible here
[-] -Log.Value
   (-) <n << >total

[ ] INVALID — $doubled is Released after expand scope ends
[ ] [-] -Log.Value
[ ]    (-) <n << $doubled   <- compile error: variable released
```

### EC-20.3: `~>` default operator on output parameters

<!-- @u:operators -->
**What it tests:** `~>` sets a default on an **output** parameter — if execution does not push a value, the default is used. See [[operators#Assignment Operators]].

```polyglot
{-} -Safe.Lookup
   (-) <key#string
   (-) >result#string ~> "not_found"
   (-) >found#bool ~> #Boolean.False
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Polyglot

   [-] -Cache.Get
      (-) <key << $key
      (-) >value >> $value
      [!] !Cache.Miss
         [ ] No push to >result — default "not_found" used
         [ ] No push to >found — default #Boolean.False used

   [ ] Cache hit path
   [-] >result << $value
   [-] >found << #Boolean.True
```
