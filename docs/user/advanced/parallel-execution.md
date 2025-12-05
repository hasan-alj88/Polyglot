# Polyglot Parallel Execution

**Version:** 0.0.2  
**Last Updated:** 2025-12-02

---

## Overview

Polyglot's **parallel execution** uses the `[p]` marker with **copy-in/copy-out semantics** for thread safety.

---

## Parallel Block `[p]`

Execute operations in parallel (mini-pipelines run concurrently:

```polyglot
[p] ~ForEach
[<] .items                         // Copy-in (automatic
[>] .item                          // Each parallel instance
[~][r] |ProcessItem                // Runs in parallel
[~][<] .data << .item
[~][>] .result >> .processed
[~]
[~][Y] ~Y.IntoArray                // Join results
[~][<] .processed
[~][>] .all_results
[~]
```

---

## Copy-In Semantics (Automatic

Variables passed to `[<]` are **automatically copied** into parallel context:

```polyglot
[r] .shared_config:pg.serial << ...
[r] .items:pg.array.pg.string << {"a", "b", "c"

[p] ~ForEach
[<] .items                         // Copy array into parallel context
[>] .item
[~][r] |ProcessItem
[~][<] .config << .shared_config   // Each instance gets copy
[~][<] .data << .item
[~]
```

**Key Point:** No shared mutable state - each parallel instance has its own copy.

---

## Copy-Out Semantics (Explicit with `[>]` and `>>`

Results must be **explicitly pulled out** with `[>]` and joined:

```polyglot
[p] ~ForEach
[<] .items
[>] .item
[~][r] |ProcessItem
[~][>] >result:pg.string >> .processed  // Copy-out (explicit
[~]
[~][Y] ~Y.IntoArray
[~][<] .processed                  // Collect results
[~][>] .all_results                // Final joined output
[~]
```

---

## Join Operations `[Y]`

Synchronize and collect parallel results:

### `~Y.IntoArray`

Collects results into array:

```polyglot
[Y] ~Y.IntoArray
[<] .results
[>] >combined:pg.array{T
```

### `~Y.IntoSet`

Collects unique results:

```polyglot
[Y] ~Y.IntoSet
[<] .results
[>] >unique:pg.set{T
```

### `~Y.IntoSerial`

Merges results into serial structure:

```polyglot
[Y] ~Y.IntoSerial
[<] .results
[>] >merged:pg.serial
```

---

## Unpack Operators

### `~ForEach`

Iterate over collection (parallel or sequential based on `[p]` or `[r]`:

```polyglot
[p] ~ForEach
[<] .items
[>] .item
[~]// Process each item
[~]
```

### `~Enumerate`

Iterate with index:

```polyglot
[p] ~Enumerate
[<] .items
[>] .index
[>] .item
[~]// Process with index
[~]
```

### `~Zip`

Combine two collections:

```polyglot
[p] ~Zip
[<] .list1
[<] .list2
[>] .item1
[>] .item2
[~]// Process paired items
[~]
```

---

## Thread Safety by Design

Polyglot's parallel execution is **safe by default**:

1. **Copy-in** - No shared mutable state
2. **Isolated execution** - Each parallel instance independent
3. **Explicit join** - Results collected at join point
4. **No race conditions** - No shared memory

---

## Complete Example

### Parallel Image Processing

```polyglot
[|] ProcessImages
[i] .image_paths:pg.array.pg.path
[t] |T.Call
[W] |W.Polyglot.Scope

[p] ~ForEach
[<] .image_paths
[>] .path
[~][r] |LoadImage
[~][<] .path << .path
[~][>] .image >> .loaded
[~]
[~][r] |ResizeImage
[~][<] .image << .loaded
[~][<] .width << 800
[~][>] .resized >> .result
[~]
[~][r] |SaveImage
[~][<] .image << .result
[~][>] .saved_path >> .output_path
[~]
[~]
[~][Y] ~Y.IntoArray
[~][<] .output_path
[~][>] .processed_paths
[~]

[o] .processed_paths:pg.array.pg.path
[X]
```

---

## Parallel vs Sequential

```polyglot
// Sequential (waits for each
[r] ~ForEach
[<] .items
[>] .item
[~][r] |ProcessItem
[~]

// Parallel (all at once
[p] ~ForEach
[<] .items
[>] .item
[~][r] |ProcessItem
[~]
```

---

## Background Execution `[b]`

Fire-and-forget (no join needed:

```polyglot
[b] |LogEvent                      // Doesn't block
[<] .event << .user_action

[r] |ContinueExecution             // Runs immediately
```

---

**Next:** [Macro System →](macro-system.md
