---
audience: automation-builder
type: reference
updated: 2026-04-09
metadata_definition: "%definition.#:ChineseDate"
metadata_instance: "%#:ChineseDate:N"
---

# Chinese Calendar Types

## #ChineseDate

```aljam3
{#} #ChineseDate
   [%] .description << "Chinese calendar date with stem-branch cycle"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarValue
   [#] %##Alias << "chinesedate"
   [.] .year#int
   [.] .cycle#int
   [.] .month#int
   [.] .leapMonth#bool <~ #Boolean.False
   [.] .day#int
   [.] .stem#HeavenlyStem
   [.] .branch#EarthlyBranch
```

## #HeavenlyStem

```aljam3
{#} #HeavenlyStem
   [%] .description << "Chinese celestial stem in the sexagenary cycle"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "heavenlystem"
   [.] .Jia
   [.] .Yi
   [.] .Bing
   [.] .Ding
   [.] .Wu
   [.] .Ji
   [.] .Geng
   [.] .Xin
   [.] .Ren
   [.] .Gui
```

## #EarthlyBranch

```aljam3
{#} #EarthlyBranch
   [%] .description << "Chinese terrestrial branch in the sexagenary cycle"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "earthlybranch"
   [.] .Zi
   [.] .Chou
   [.] .Yin
   [.] .Mao
   [.] .Chen
   [.] .Si
   [.] .Wu
   [.] .Wei
   [.] .Shen
   [.] .You
   [.] .Xu
   [.] .Hai
```

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:ChineseDate` | Compile-time type template |
| Instance | `%#:ChineseDate:N` | Runtime instance (N = instance number) |
| Definition | `%definition.#:HeavenlyStem` | Compile-time type template |
| Definition | `%definition.#:EarthlyBranch` | Compile-time type template |

See [[metadata-tree/FULL-TREE|Full Metadata Tree]] for the authoritative tree.
