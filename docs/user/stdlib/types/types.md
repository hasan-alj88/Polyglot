---
audience: user
type: specification
updated: 2026-03-20
status: draft
---

# Built-in Types (#)

Stdlib structs and enums available in every `.pg` file. No `[@]` import needed.

## #Boolean

```
#Boolean
   [%] .description << "boolean type Enum"
   [%] .version << "1.0.0"
   .True
      [%] .alias << #True
   .False
      [%] .alias << #False
```

## #None

```
#None
   [ ] Represents the absence of a value.
```

## #OS

```
#OS
   [%] .description << "Operating system enum"
   [%] .version << "1.0.0"
   .Unix
      [%] .alias << #Unix
   .Windows
      [%] .alias << #Windows
```

## #path

```
#path
   [%] .description << "Cross-platform file system path"
   [%] .version << "1.0.0"
   [.] .Unix;string
   [.] .Windows;string
```

## #PipelineStatus

```
#PipelineStatus
   [%] .description << "Pipeline instance status"
   .AwaitTrigger
      [%] .alias << #AwaitTrigger
   .Disabled
      [%] .alias << #Disabled
   .Running
      [%] .alias << #Running
   .Failed
      [%] .alias << #Failed
```

## #FileAccess

```
#FileAccess
   [%] .description << "File access state"
   [%] .version << "1.0.0"
   .Available
      [%] .alias << #Available
   .Locked
      [%] .alias << #Locked
   .NotFound
      [%] .alias << #NotFound
```

## #VarState

```
#VarState
   [%] .description << "Variable lifecycle state"
   .Declared
      [%] .alias << #Declared
   .Default
      [%] .alias << #Default
   .Final
      [%] .alias << #Final
   .Failed
      [%] .alias << #Failed
   .Released
      [%] .alias << #Released
```
