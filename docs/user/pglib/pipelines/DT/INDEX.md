---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =DT.* DateTime Pipelines

<!-- @pipelines -->

DateTime pipelines for construction, calendar conversion, arithmetic, comparison, extraction, zone management, formatting, and business day operations. No `[@]` import needed.

All `=DT.*` pipelines are native definitions (`{N}` blocks). They operate on `#dt` (alias for `#DateTime`).

**Inline notation:** `=DT"..."` and `=DateTime"..."` are sugar for `=DT.From.ISO`. This follows the `=Path"..."` precedent.

```polyglot
[ ] These three are equivalent:
[r] $deadline#dt << =DateTime"2026-03-20T12:00:00Z"
[r] $deadline#dt << =DT"2026-03-20T12:00:00Z"
[r] $deadline#dt
   [r] =DT.From.ISO
      [=] <iso << "2026-03-20T12:00:00Z"
      [=] >dt >> $deadline
```

## Permissions

<!-- @permissions -->

All `=DT.*` pipelines are pure computation and require no `{_}` permission objects, except `=DT.Now` which reads the system clock. See [[permissions]].

| Pipeline | Required Capability | Category |
|----------|-------------------|----------|
| `=DT.Now` | System.Env | System |
| All others | None | --- |

---

## Pipeline Listing

### Construction

- [[DT/Now|=DT.Now]]
- [[DT/From.Epoch|=DT.From.Epoch]]
- [[DT/From.ISO|=DT.From.ISO]]
- [[DT/From.Parts|=DT.From.Parts]]

### Calendar Conversion

- [[DT/To.Gregorian|=DT.To.Gregorian]]
- [[DT/To.Hijri|=DT.To.Hijri]]
- [[DT/To.Hebrew|=DT.To.Hebrew]]
- [[DT/To.Chinese|=DT.To.Chinese]]
- [[DT/To.Persian|=DT.To.Persian]]
- [[DT/To.Buddhist|=DT.To.Buddhist]]
- [[DT/To.Hindu|=DT.To.Hindu]]
- [[DT/To.Japanese|=DT.To.Japanese]]
- [[DT/To.Ethiopian|=DT.To.Ethiopian]]
- [[DT/To.Coptic|=DT.To.Coptic]]
- [[DT/To.Custom|=DT.To.Custom]]

### Time Unit Conversion

- [[DT/To.ChineseTime|=DT.To.ChineseTime]]
- [[DT/To.HinduTime|=DT.To.HinduTime]]
- [[DT/To.DecimalTime|=DT.To.DecimalTime]]

### Arithmetic

- [[DT/Add.Duration|=DT.Add.Duration]]
- [[DT/Add.Period|=DT.Add.Period]]
- [[DT/Sub|=DT.Sub]]

### Comparison

- [[DT/Compare|=DT.Compare]]
- [[DT/IsBefore|=DT.IsBefore]]
- [[DT/IsAfter|=DT.IsAfter]]
- [[DT/InInterval|=DT.InInterval]]

### Extraction

- [[DT/Get.Year|=DT.Get.Year]]
- [[DT/Get.Month|=DT.Get.Month]]
- [[DT/Get.Day|=DT.Get.Day]]
- [[DT/Get.Weekday|=DT.Get.Weekday]]
- [[DT/Get.WeekNumber|=DT.Get.WeekNumber]]
- [[DT/Get.Epoch|=DT.Get.Epoch]]
- [[DT/Get.Zone|=DT.Get.Zone]]

### Zone

- [[DT/Zone.Set|=DT.Zone.Set]]
- [[DT/Zone.Convert|=DT.Zone.Convert]]

### Formatting

- [[DT/Format|=DT.Format]]
- [[DT/Format.ISO|=DT.Format.ISO]]
- [[DT/Format.Calendar|=DT.Format.Calendar]]

### Business

- [[DT/Business.IsWorkDay|=DT.Business.IsWorkDay]]
- [[DT/Business.NextWorkDay|=DT.Business.NextWorkDay]]
- [[DT/Business.AddWorkDays|=DT.Business.AddWorkDays]]

---

## Related

- [[pglib/types/datetime|DateTime types]] -- `#dt`, `#Duration`, `#Period`, `#Interval`, calendar date structs
- [[pglib/pipelines/INDEX|Pipelines index]] -- full pglib pipeline listing
- [[permissions]] -- permission system for `{_}` permission objects
