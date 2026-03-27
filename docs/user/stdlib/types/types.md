---
audience: user
type: specification
updated: 2026-03-27
status: complete
---

# Built-in Types (#)

Stdlib structs and enums available in every `.pg` file. No `[@]` import needed.

## Type Hierarchy

```
RawString (compiler intrinsic)
└── #String (foundation — .string + .re)
    ├── #Int (.re = signed integers)
    ├── #UnsignedInt (.re = non-negative integers)
    ├── #Float (.re = decimals)
    ├── #Sci (.re = scientific notation)
    ├── #Eng (.re = engineering notation)
    ├── #Dimension (.re = positive integers — array dimensions)
    └── (user-defined: #emailAddress, #phoneNumber, etc.)

#Boolean (independent enum struct — NOT #String)

#Array<ValueType<Dim (ordered, contiguous, typed elements, N-dimensional)
#Dict<KeyType<ValueType (unordered, sparse, typed K-V pairs)
#Dataframe<KeyType<ValueType (array of dicts — tabular data)
#Serial (schema-free, unlimited depth)
```

---

## Core Types

### #String

```polyglot
{#} #String
   [ ] #String and #string both resolve here
   [#] %Alias << "string"
   [ ] Scalar — no flexible children, no collection nesting
   [#] %Depth.Max << 0
   [ ] The actual string value
   [.] .string#RawString
   [ ] Regex constraint — default accepts all strings
   [ ] <~ allows subtypes to override once to specialize
   [.] .re#RawString <~ ".*"
```

### #Int

```polyglot
{#} #Int
   [ ] Inherits #String schema (.string, .re)
   [#] <~ #String
   [#] %Alias << "int"
   [ ] Matches: 42, -7, 0, 007
   [.] .re#RawString << "^-?[0-9]+$"
```

### #UnsignedInt

```polyglot
{#} #UnsignedInt
   [ ] Non-negative integers only
   [#] <~ #String
   [#] %Alias << "uint"
   [ ] Matches: 0, 1, 42, 007
   [.] .re#RawString << "^[0-9]+$"
```

### #Float

```polyglot
{#} #Float
   [#] <~ #String
   [#] %Alias << "float"
   [ ] Matches: 3.14, -0.5, 007.00
   [.] .re#RawString << "^-?[0-9]+\.[0-9]+$"
```

### #Sci

```polyglot
{#} #Sci
   [#] <~ #String
   [#] %Alias << "sci"
   [ ] Scientific notation with optional decimal
   [ ] Matches: 1e10, 3.14e-2, -5E+3
   [.] .re#RawString << "^-?[0-9]+(\.[0-9]+)?[eE][+-]?[0-9]+$"
```

### #Eng

```polyglot
{#} #Eng
   [#] <~ #String
   [#] %Alias << "eng"
   [ ] Engineering notation: exponents are multiples of 3
   [ ] Matches: 1.5e3, 2.47e-6, 9.99e12
   [.] .re#RawString << "^-?[1-9]\.[0-9]{0,2}[eE][+-]?(0|[369]|[1-9][0-9]*[0369])$"
```

### #Dimension

```polyglot
{#} #Dimension
   [ ] Positive integers only — used for array dimension parameters
   [#] <~ #String
   [#] %Alias << "dim"
   [ ] Matches: 1, 2, 3, 10
   [.] .re#RawString << "^[1-9][0-9]*$"
```

---

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
   [.] .Unix#string
   [.] .Windows#string
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

## #QueueStrategy

```
#QueueStrategy
   [%] .description << "Queue ordering strategy"
   [%] .version << "1.0.0"
   .FIFO
      [%] .alias << #FIFO
   .LIFO
      [%] .alias << #LIFO
   .Priority
      [%] .alias << #Priority
```

## #RetriggerStrategy

```
#RetriggerStrategy
   [%] .description << "Behavior when pipeline is re-triggered while queued or running"
   [%] .version << "1.0.0"
   .Disallow
      [%] .alias << #Disallow
   .Allow
      [%] .alias << #Allow
   .NoDuplicate
      [%] .alias << #NoDuplicate
   .QueueAfter
      [%] .alias << #QueueAfter
```

## #QueueState

```
#QueueState
   [%] .description << "Active queue pipeline state"
   [%] .version << "1.0.0"
   .Running
      [%] .alias << #Running
   .SoftPaused
      [%] .alias << #SoftPaused
   .HardPaused
      [%] .alias << #HardPaused
   .Killed
      [%] .alias << #Killed
```

## #Queue

```
#Queue
   [%] .description << "Queue configuration for pipeline execution"
   [%] .version << "1.0.0"
   [.] .strategy#QueueStrategy
   [.] .retrigger#RetriggerStrategy
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

---

## Collection Types

### #Array

```polyglot
{#} #Array<ValueType<Dim
   [ ] Accepts any type as element type
   [#] <ValueType << #*
      [ ] Constraint: ValueType must be scalar/record (depth 0)
      [<] %Depth.Max << 0
   [ ] Dimension parameter — defaults to 1 if omitted
   [#] <Dim << #Dimension
      [<] %Depth.Max << 0
   [#] %Alias << "array"
   [ ] Keys are unsigned integer indices: :0, :1, :2 ...
   [#] %Key.Type << #UnsignedInt
   [ ] Contiguous — no gaps allowed
   [#] %Key.Gap << #False
   [ ] Insertion order preserved
   [#] %Ordered << #True
   [ ] Depth equals dimension parameter value
   [#] %Depth.Max << Dim
   [ ] All elements share the same type
   [:] :*#ValueType
```

### #Dict

```polyglot
{#} #Dict<KeyType<ValueType
   [ ] Type of dictionary keys
   [#] <KeyType << #*
      [<] %Depth.Max << 0
   [ ] Type of dictionary values
   [#] <ValueType << #*
      [<] %Depth.Max << 0
   [#] %Alias << "dict"
   [ ] Keys typed by first param
   [#] %Key.Type << KeyType
   [ ] Sparse — keys don't need to be contiguous
   [#] %Key.Gap << #True
   [ ] No guaranteed order
   [#] %Ordered << #False
   [ ] Flat only — one level of key-value pairs
   [#] %Depth.Max << 1
   [ ] All values share the same type
   [:] :*#ValueType
```

### #Serial

```polyglot
{#} #Serial
   [#] %Alias << "serial"
   [ ] Sparse keys allowed
   [#] %Key.Gap << #True
   [ ] No guaranteed ordering
   [#] %Ordered << #False
   [ ] Unlimited nesting depth
   [#] %Depth.Max << -1
   [ ] Any key, any value type, any depth
   [:] :*#*
```

### #Dataframe

```polyglot
{#} #Dataframe<KeyType<ValueType
   [ ] Column name type (typically #string)
   [#] <KeyType << #*
      [<] %Depth.Max << 0
   [ ] Cell value type
   [#] <ValueType << #*
      [<] %Depth.Max << 0
   [#] %Alias << "dataframe"
   [ ] Rows indexed by unsigned integers: :0, :1, :2 ...
   [#] %Key.Type << #UnsignedInt
   [ ] Contiguous rows — no gaps
   [#] %Key.Gap << #False
   [ ] Row order preserved
   [#] %Ordered << #True
   [ ] Two levels: row (uint) → column (KeyType) → cell (ValueType)
   [#] %Depth.Max << 2
   [ ] Each row is a flat dict of KeyType → ValueType
   [:] :*#Dict:KeyType:ValueType
```
