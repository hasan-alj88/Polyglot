# Lesson 023: DataTrees, Expanders, and Collectors Syntax Evolution

**Date**: 2026-05-02
**Context**: Re-architecting Aljam3 DataTrees handling, expander usage, and collection semantics for better consistency and capability.

## Lesson Summary

Aljam3 has evolved its handling of DataTrees, specifically arrays and records, to use unified, universal iteration and collection constructs instead of data-structure-specific operations. Additionally, the instantiation syntax for collections now utilizes formal Constructor Input (`($)`) and Schema Shaping parameters (`#<`), while making a strong distinction between Data Types (`#`) and Schemas (`##`).

### 1. Schemas (`##`) vs Data Types (`#`)

Data structures like Arrays or Uniform Records are not simple data types; they are **Schemas**. Therefore, they use the `##` prefix (e.g., `##Array`, `##Uniform`, `##UniformRecord`).

### 2. Schema Shaping Input (`#<`) and Constructor Input (`($)`)

When instantiating schemas like `##Array` or `##UniformRecord`, you must define the shaping input parameters using the `#<` parameter identifier inside Constructor Input lines (`($)`). This explicitly separates schema parameters from the actual data payload.

**Verbose Constructors**:
Verbose constructors use `($)` IO lines to explicitly define properties.
```aljam3
   [-] $dirArray##Array
      ($) #<ValueType << #Folder
      ($) #<Dimensions << $$"1D"
      ($) << $reportsPath
      ($) << $logsPath
```

**Common Values Constructor**: For common standard values like numbers, booleans, or dimensions, the overloaded constructor `$$"value"` is used directly. For example: `$$"1D"` or `$$"False"`.

**Literal Primitives**: Numeric and string literals do not require constructors. You can pass them directly (e.g., `150.0` or `1.1`). Keys in any data tree are Enums, denoted by a dot (`.KEY`).

```aljam3
   [-] $prices##UniformRecord
      ($) #<ValueType << #Float
      ($) .AAPL << 150.0
      ($) .GOOGL << 2800.0
```

### 3. File and Folder Validation

To ensure safety, you must use `$Folder"..."` or `$File"..."` constructors instead of generic `$Path"..."`. The compiler will verify at compile-time that the string is actually a folder or file, and that it exists on the system. Compile error otherwise.

### 4. IO Declaration Ordering

The `(-)` output parameter declarations MUST come before the Pipeline configuration blocks (`[T]`, `[Q]`, `[W]`).

### 5. Unified Expander (`=ForEach`)

Data-structure-specific expanders (like `=ForEach.Array` or `=ForEach.Map`) are retired. Use the universal `=ForEach` expander instead. This expander iterates through all the leaves of the DataTree regardless of shape.

**Inputs & Outputs**:
- `(<) <Data`: The data tree collection to iterate.
- `(>) >item`: The current iteration value.
- `(>) >key`: The current iteration index or map key.

**Correct Usage**:
```aljam3
   [=] =ForEach
      (=) <Data << $dirArray
      (=) >item >> $currentDir
      (=) >key >> $index
```

### 6. Unified Collector (`*Collect`)

Similarly, specific collectors like `*Into.Array` or `*Into.Map` are retired. Use the universal `*Collect` operator. Output can be piped directly to an output parameter using `>> >parameter_name`.

**Inputs**:
- `(*) <item`: The data item to collect.
- `(*) <key`: The specific index or key for the collection placement.
- `(*) >Array` (or `>Map`, `>Data`): The target array or collection to pipe the collected result into.

**Correct Usage**:
```aljam3
      [*] *Collect
         (*) <item << $currentDir
         (*) <key << $index
         (*) >Array >> >processedDirs
```

### 7. Simple Error Fallbacks (`>!`)

Instead of verbose `[!] *!` error handler blocks for simple action fallbacks, use the inline fallback syntax `>!`.

**Correct Usage**:
```aljam3
      [-] @FS-API.Directory.Create
         (-) <path << $currentDir
         (-) >status
            (>) >! $$"False"
```
