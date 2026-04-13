---
audience: automation-builder
type: reference
updated: 2026-04-09
---

# Chinese Calendar Types

## #ChineseDate

```polyglot
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

```polyglot
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

```polyglot
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
