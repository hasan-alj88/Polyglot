---
audience: automation-builder
type: specification
updated: 2026-05-04
status: draft
metadata_definition: "%definition.#:Serial"
metadata_instance: "%#:Serial:N"
---

# `#Serial` — Unconstrained Data Tree Schema

<!-- @c:schemas -->

The `#Serial` schema represents **non-uniform**, heterogeneous Data Trees. It is the default, unconstrained schema for tree structures where branches can have arbitrary depth and arbitrary terminal datatypes.

## Structural Definition

Unlike `##Uniform` trees which guarantee symmetry, a `#Serial` tree makes no guarantees about the sibling branches at any depth level.

```aljam3
[#] #Serial
```

### JSON / Document Equivalency
`#Serial` natively models NoSQL documents or arbitrary JSON blobs where nested objects can have vastly different shapes.

```aljam3
[$] $ApiResponse#Serial
   << .user
      << .name << "Alice"
      << .age << 30
   << .metadata
      << .latency << 12ms
      << .tags
         << "prod" | "v1" | "fast"
```

## Interaction with Expanders and Collectors

Because the structure is entirely unconstrained:
1. **`=*PermuteLevels` is forbidden**: The compiler will throw an error if you attempt to transpose a `#Serial` tree, as there is no guarantee that Level 2 exists symmetrically across all Level 1 branches.
2. **`=ForEach.Leaves` Exhaustive Search**: The most common way to parse a `#Serial` tree is an exhaustive depth-first search (`=ForEach.Leaves`) which ignores structural branches and yields only the `##Scalar` terminals and their absolute paths.
3. **`*Collect` Default**: When piping data into `*Collect` without specifying an explicit `(*) <schema`, the output tree defaults to `#Serial`.

## File and DB Streaming
When reading from nested/unconstrained data sources, the resulting Expanders yield `#Serial` trees:
* `=File.JSON.Nodes` $\rightarrow$ Yields a `#Serial` Data Tree per top-level node.
* `=DB.Collection.Documents` $\rightarrow$ Yields a `#Serial` Data Tree per NoSQL document.

## Related

- [[jm3lib/types/schemas/Uniform|##Uniform]]
- [[jm3lib/types/schemas/Record|##Record]]
