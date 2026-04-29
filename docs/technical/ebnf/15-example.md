---
audience: design
type: spec
updated: 2026-04-09
---

<!-- @ebnf/INDEX -->

## 15. Complete File Example (Informative)

```aljam3
file
  └─ package_block          {@ } @Local:999.MyPkg:v1.0.0
  │    └─ import_line          [@] @utils << @Community:user.Utils:v2.0.0
  │
  ├─ data_def               {#} #Status
  │    ├─ metadata              [%] .description << "entity status"
  │    ├─ enum_field            [.] .Active
  │    │    └─ metadata          [%] %alias
  │    │         └─ alias_entry    [:] "Active"
  │    └─ enum_field            [.] .Inactive
  │         └─ metadata          [%] %alias
  │              └─ alias_entry    [:] "Inactive"
  │
  ├─ data_def               {#} #Record
  │    ├─ value_field           [.] .name#string <~ ""
  │    └─ value_field           [.] .count#int <~ 0
  │
  ├─ error_def              {!} !Processing
  │    └─ leaf                 [.] .InvalidRecord#Error
  │
  └─ pipeline_def           {-} -ProcessItems
       ├─ metadata              [%] .version << "1.0.0"
       ├─ trigger               [T] -T.Call
       ├─ io                    (-) <items#array:Record
       │                        (-) >total#int ~> 0
       ├─ error_decl            (-) !Processing.InvalidRecord
       ├─ queue                 [Q] -Q.Default
       ├─ wrapper               [W] -W.Aljam3
       └─ execution
            ├─ expand            [=] =ForEach.Array
            │   ├─ io            (=) <Array << $items
            │   └─ io            (=) >item >> $rec
            │      └─ collect    [-] *Agg.Sum
            │          ├─ io     (*) <number << $rec.count
            │          └─ io     (*) >sum >> >total
            └─ run               [-] @utils-Report.Generate
                ├─ io            (-) <total << $total
                └─ error         [!] !Report.Failed
                                    [-] >total << -1
```
