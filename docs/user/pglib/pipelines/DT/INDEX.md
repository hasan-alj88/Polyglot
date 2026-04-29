---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
---

# -DT.* DateTime Pipelines

<!-- @c:pipelines -->

DateTime pipelines for construction, calendar conversion, arithmetic, comparison, extraction, zone management, formatting, and business day operations. No `[@]` import needed.

All `-DT.*` pipelines are native definitions (`{N}` blocks). They operate on `#dt` (alias for `#DateTime`).

**Three-Context Rule** for DateTime values:

| Context | Mechanism | Syntax |
|---|---|---|
| `[T]`/`[Q]`/`[W]` Infrastructure | Inline pipeline config | `-DT"2026-03-20T12:00:00Z"` |
| Pipeline body — known values | Constructor | `$DT"2026-03-20"` |
| Pipeline body — dynamic values | Pipeline call | `[-] -DT.Parse` |

On infrastructure lines (`[T]`, `[Q]`, `[W]`), `-DT"..."` and `=DateTime"..."` remain valid as sugar for `-DT.From.ISO`. In the execution body, use the `$DT` constructor for known literals (no error handling needed) or `-DT.Parse` for dynamic/untrusted strings (error handling required). See [[constructors/DT|$DT constructor]] and [[DT/Parse|-DT.Parse]].

```aljam3
[ ] infrastructure line — inline notation valid
[T] -T.Cron"0 9 * * *"

[ ] execution body — constructor (compile-time guaranteed)
[-] $deadline << $DT"2026-04-22"

[ ] execution body — dynamic string (error handling required)
[-] $parsed#dt << -DT.Parse
   (<) <raw#string << $userInput
   [!] !Parse.DateTime.InvalidFormat
      [-] $parsed << $DT"Today"
```

## Permissions

<!-- @c:permissions -->

All `-DT.*` pipelines are pure computation and require no `{_}` permission objects, except `-DT.Now` which reads the system clock. See [[permissions]].

| Pipeline | Required Capability | Category |
|----------|-------------------|----------|
| `-DT.Now` | System.Env | System |
| All others | None | --- |

---

## Pipeline Listing

### Construction

- [[DT/Now|-DT.Now]]
- [[DT/From.Epoch|-DT.From.Epoch]]
- [[DT/From.ISO|-DT.From.ISO]]
- [[DT/From.Parts|-DT.From.Parts]]
- [[DT/Parse|-DT.Parse]]
- [[pglib/pipelines/Dur.Parse|-Dur.Parse]] -- duration parsing (separate from DT, at pipelines/ root)

### Calendar Conversion

- [[DT/To.Gregorian|-DT.To.Gregorian]]
- [[DT/To.Hijri|-DT.To.Hijri]]
- [[DT/To.Hebrew|-DT.To.Hebrew]]
- [[DT/To.Chinese|-DT.To.Chinese]]
- [[DT/To.Persian|-DT.To.Persian]]
- [[DT/To.Buddhist|-DT.To.Buddhist]]
- [[DT/To.Hindu|-DT.To.Hindu]]
- [[DT/To.Japanese|-DT.To.Japanese]]
- [[DT/To.Ethiopian|-DT.To.Ethiopian]]
- [[DT/To.Coptic|-DT.To.Coptic]]
- [[DT/To.Custom|-DT.To.Custom]]

### Time Unit Conversion

- [[DT/To.ChineseTime|-DT.To.ChineseTime]]
- [[DT/To.HinduTime|-DT.To.HinduTime]]
- [[DT/To.DecimalTime|-DT.To.DecimalTime]]

### Arithmetic

- [[DT/Add.Duration|-DT.Add.Duration]]
- [[DT/Add.Period|-DT.Add.Period]]
- [[DT/Sub|-DT.Sub]]

### Comparison

- [[DT/Compare|-DT.Compare]]
- [[DT/IsBefore|-DT.IsBefore]]
- [[DT/IsAfter|-DT.IsAfter]]
- [[DT/InInterval|-DT.InInterval]]

### Extraction

- [[DT/Get.Year|-DT.Get.Year]]
- [[DT/Get.Month|-DT.Get.Month]]
- [[DT/Get.Day|-DT.Get.Day]]
- [[DT/Get.Weekday|-DT.Get.Weekday]]
- [[DT/Get.WeekNumber|-DT.Get.WeekNumber]]
- [[DT/Get.Epoch|-DT.Get.Epoch]]
- [[DT/Get.Zone|-DT.Get.Zone]]

### Zone

- [[DT/Zone.Set|-DT.Zone.Set]]
- [[DT/Zone.Convert|-DT.Zone.Convert]]

### Formatting

- [[DT/Format|-DT.Format]]
- [[DT/Format.ISO|-DT.Format.ISO]]
- [[DT/Format.Calendar|-DT.Format.Calendar]]

### Business

- [[DT/Business.IsWorkDay|-DT.Business.IsWorkDay]]
- [[DT/Business.NextWorkDay|-DT.Business.NextWorkDay]]
- [[DT/Business.AddWorkDays|-DT.Business.AddWorkDays]]

---

## Related

- [[pglib/types/datetime|DateTime types]] -- `#dt`, `#Duration`, `#Period`, `#Interval`, calendar date structs
- [[pglib/pipelines/INDEX|Pipelines index]] -- full pglib pipeline listing
- [[permissions]] -- permission system for `{_}` permission objects
