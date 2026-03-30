---
audience: developer
type: spec
updated: 2026-03-30
---

<!-- @ebnf/INDEX -->

## 15. Complete File Example (Informative)

```
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
  └─ pipeline_def           {=} =ProcessItems
       ├─ metadata              [%] .version << "1.0.0"
       ├─ trigger               [t] =T.Call
       ├─ io                    [=] <items#array:Record
       │                        [=] >total#int ~> 0
       ├─ error_decl            [=] !Processing.InvalidRecord
       ├─ queue                 [Q] =Q.Default
       ├─ wrapper               [W] =W.Polyglot
       └─ execution
            ├─ expand            [p] ~ForEach.Array
            │   ├─ io            [~] <Array << $items
            │   └─ io            [~] >item >> $rec
            │      └─ collect    [r] *Agg.Sum
            │          ├─ io     [*] <number << $rec.count
            │          └─ io     [*] >sum >> >total
            └─ run               [r] @utils=Report.Generate
                ├─ io            [=] <total << $total
                └─ error         [!] !Report.Failed
                                    [r] >total << -1
```
