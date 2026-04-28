---
audience: automation-builder
type: specification
updated: 2026-04-28
---

# Arrays

<!-- @syntax/types/INDEX -->

In Polyglot, arrays are not defined inline. All arrays must be explicitly defined as a structural data tree using the `##Array` schema bundle inside a `{#}` type definition.

## Defining an Array Type

To create an array type, define a `{#}` block, assign the `##Array` schema, and parameterize it using the `(#)` input block:

```polyglot
{#} #StringList
   [#] ##Array
      (#) <#ValueType << #string

{#} #ScoreMatrix
   [#] ##Array
      (#) <#ValueType << #int
      (#) <Dimension#uint << 2
```

### Schema Parameters

The `##Array` schema accepts the following parameters:

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `<#ValueType` | Type Ref | *(Required)* | The type of data contained in the array's leaves. |
| `<Dimension#uint` | `#uint` | `1` | The number of dimensions (e.g., `1` for list, `2` for matrix). |
| `<Range#uint` | `#uint` | `#Inf` | The maximum number of elements allowed per dimension. |

## Initializing Arrays

Once defined, you use your custom array type to type variables and initialize them via **Vertical Block Expansion** using the `(#)` Data IO bracket. 

### Auto-Incrementing Append
Push elements sequentially using `(#) <<`. The compiler automatically assigns the next `#Range` index (`:0`, `:1`, ...).

```polyglot
[-] $names#StringList << {}
   (#) << "Alice"
   (#) << "Bob"
```

### Explicit Indexing
You can specify the `#Range` index explicitly on the `(#)` bracket:

```polyglot
[-] $scores#ScoreMatrix << {}
   (#) :0<0 << 95
   (#) :0<1 << 82
   (#) :1<0 << 77
```

**Important**: `##Array` is structurally defined as a *contiguous* sequence (`%##Gap << #False`). Specifying explicit sparse indices (e.g. jumping from `:1` to `:10`) will cause a compilation error. If you need dynamic or sparse keys, use the `##Map` schema instead.

## Element Access

Element access uses `<` (the tree child accessor) with integer indices. The number of indices must match the declared `<Dimension#uint` count:

```polyglot
[-] $val << $names<0                 [ ] 1 index for 1D Array
[-] $val << $scores<0<1              [ ] 2 indices for 2D Matrix
```

The compiler strictly enforces access depth — providing too many or too few indices compared to the array's declared `<Dimension` triggers PGE04017.

## Deprecation Notice

The legacy inline array syntax (e.g., `#array:string:2D`) has been permanently removed from Polyglot. All sequences must be formally declared as `{#}` data types to ensure consistent structural tree metadata across the project.

## See Also

- [[syntax/types/INDEX|Type System Overview]] — ground truths and type annotation rules
- [[syntax/types/schema-properties|Schema Properties]] — The underlying `%##` properties that `##Array` configures
