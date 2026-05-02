---
audience: automation-builder
type: spec
updated: 2026-05-02
---

<!-- @concepts/collections/INDEX -->
<!-- @u:data-is-trees -->

## `##Set:V` -- Unique Value Collection

A `##Set` is a structural collection where the fundamental condition is that **ALL leaves are unique**. 

Unlike `##Map` or `##Array` where duplicate values can exist under different keys, a `##Set` structurally guarantees that no two identical values exist within its leaf nodes.

### Schema Composition

In Aljam3's tree architecture, a `##Set` is essentially a sparse tree where the **key and the value are inherently the same**, preventing duplicate branches. 

If you attempt to insert a value into a `##Set` that already exists, the tree does not grow (the new branch simply merges into the existing one).

### Usage

```aljam3
[ ] Initialize a Set of Strings
[-] $activeUsers##Set:String <<
   ($) <<"Hasan"
   ($) <<"Paul"
   ($) <<"Hasan"  [ ] Safely ignored; leaf "Hasan" already exists.
```

### Collecting into Sets

You can use the universal `*Collect` operator to automatically deduplicate a stream of data into a `##Set`.

```aljam3
[-] =ForEach
   (=) <Data << $rawTrafficLogs
   (=) >item >> $ipAddress
   
   [*] *Collect
      (*) <item << $ipAddress
      (*) >Data >> >uniqueIPs##Set:String
```

## See Also

- [[concepts/collections/map|##Map]] -- Key-Value collections
- [[concepts/collections/array|##Array]] -- Incremental numeric key collections
- [[concepts/collections/collect|Collect Operators]] -- Collection techniques
