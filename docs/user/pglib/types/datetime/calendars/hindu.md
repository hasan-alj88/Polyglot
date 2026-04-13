---
audience: automation-builder
type: reference
updated: 2026-04-09
---

# Hindu Calendar Types

## #HinduDate

```polyglot
{#} #HinduDate
   [%] .description << "Hindu calendar date with era and lunar fortnight"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarValue
   [#] %##Alias << "hindudate"
   [.] .year#int
   [.] .era#HinduEra
   [.] .month#HinduMonth
   [.] .day#int
   [.] .paksha#Paksha
```

## #HinduEra

```polyglot
{#} #HinduEra
   [%] .description << "Hindu calendar era system"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "hinduera"
   [.] .VikramSamvat
   [.] .Saka
```

## #HinduMonth

```polyglot
{#} #HinduMonth
   [%] .description << "Hindu calendar month"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "hindumonth"
   [.] .Chaitra
   [.] .Vaishakha
   [.] .Jyeshtha
   [.] .Ashadha
   [.] .Shravana
   [.] .Bhadrapada
   [.] .Ashvina
   [.] .Kartika
   [.] .Margashirsha
   [.] .Pausha
   [.] .Magha
   [.] .Phalguna
```

## #Paksha

```polyglot
{#} #Paksha
   [%] .description << "Hindu lunar fortnight"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "paksha"
   [.] .Shukla
   [.] .Krishna
```
