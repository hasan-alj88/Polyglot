---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.#:Array"
metadata_instance: "%#:Array:N"
---

# #Array Collection

<!-- @types -->
<!-- @macros -->

Contiguous, rectangular collection with typed elements and N-dimensional support. `#Array` inherits from `#Map` with `#UnsignedInt` keys.

---

## Definition

```polyglot
{M} #Array
   [ ] First input: a DataType — <# means type input (all definitions are data trees)
   [#] <#ValueType
      [<] << ##Scalar
   [ ] Optional second input: a Dimension (defaults to 1D)
   [#] <Dim##Dimension <~ "1D"
      [<] << ##Scalar

   [ ] Direct substitution via {} inside double quotes — implicit inline pipeline
   [ ] ##DataTypeString: new pglib schema for valid {x} definition names
   [r] $ArrayName##DataTypeString << "Array{$Dim}:{$ValueType%name}"
   [r] $dim#RawString << =String.Lower"{$Dim}"

   {#} #{$ArrayName}
      [#] <~ #Map:#UnsignedInt:$ValueType
      [#] %##Alias
         [:] << "array:{$ValueType%name}:{$dim}"
         [:] << "array{$dim}:{$ValueType%name}"
         [:] << "Array{$Dim}:{$ValueType%name}"
      [#] %##Children.Type << #UnsignedInt
      [#] %##Children.Ordered << #True
      [#] %##Children.Uniform << #True
      [#] << ##Contiguous
      [#] << ##Rectangular
      [#] %##Depth.Max << $Dim
      [:] :*#$ValueType
```

---

## Inheritance from #Map

`#Array` inherits from `#Map:#UnsignedInt:$ValueType` via the `<~` operator. This means arrays are maps with integer keys (`:0`, `:1`, `:2`, ...) and additional constraints for contiguity, ordering, and rectangular shape.

---

## Schema Properties

| Property | Value | Meaning |
|----------|-------|---------|
| `%##Children.Type` | `#UnsignedInt` | Integer indices `:0`, `:1`, `:2` ... |
| `##Contiguous` | `%##Children.Gap << #False`, `%##Children.Ordered << #True` | No gaps, insertion order preserved |
| `##Rectangular` | `%##Children.Regular << #True`, `%##Children.Uniform << #True` | Regular shape, uniform types |
| `%##Depth.Max` | `$Dim` | Equals dimension parameter value |

---

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:Array` | Compile-time type template |
| Instance | `%#:Array:N` | Runtime instance (N = instance number) |

---

## Related

- [[collections]] — collection type overview
- [[Map]] — parent type (#Array inherits from #Map)
- [[scalars]] — scalar schema classifications
- [[syntax/types/INDEX|types]] — full type system specification
- [[macros]] — macro system
