---
name: Pipeline Call Cycle Detection
type: algorithm
consumes: PGE09013
audience: designer
updated: 2026-03-24
---

# Pipeline Call Cycle Detection Algorithm

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/types -->

Detects circular call chains among `{-}` pipelines within the same package. Polyglot has no recursion — no base case construct, no call stack, no termination mechanism. A circular call graph executes forever and is always a compile error ([[PGE09013-circular-pipeline-call|PGE09013]]).

Cross-package cycles are excluded — those are caught by [[PGE09002-circular-package-dependency|PGE09002]] at the import level.

## Inputs

- All `{-}` pipeline definitions in a single package
- All `[-]`/`[=]`/`[b]` call sites within those pipelines that reference same-package pipelines

## Graph Construction

Build a directed adjacency list representing the intra-package call graph.

```text
function buildCallGraph(package):
    nodes = {}
    edges = {}

    for each pipeline P in package:
        nodes[P.name] = P
        edges[P.name] = []

    for each pipeline P in package:
        for each call site C in P (where C is [-], [=], or [b]):
            target = C.referencedPipeline
            if target in nodes:              // same-package only
                edges[P.name].append(target)

    return (nodes, edges)
```

**Filtering rules:**
- Include `[-]` (serial call), `[=]` (parallel call), and `[b]` (fire-and-forget call) references
- Exclude calls to pglib pipelines (`-T.*`, `-Q.*`, `-W.*`, `-Math.*`, etc.) — these are not user-defined
- Exclude calls via `@alias-PipelineName` — these are cross-package, covered by [[PGE09002-circular-package-dependency|PGE09002]]

## Cycle Detection — DFS Three-Color Marking

The primary algorithm uses depth-first search with three-color marking. When a gray (in-progress) node is revisited, a back edge exists and a cycle is found.

```text
function detectCycles(nodes, edges):
    color = {node: WHITE for node in nodes}
    parent = {}
    cycles = []

    for each node in nodes:
        if color[node] == WHITE:
            dfs(node, color, parent, edges, cycles)

    return cycles

function dfs(node, color, parent, edges, cycles):
    color[node] = GRAY

    for each neighbor in edges[node]:
        if color[neighbor] == GRAY:
            // Back edge found — extract cycle path
            cycle = extractCycle(node, neighbor, parent)
            cycles.append(cycle)
        else if color[neighbor] == WHITE:
            parent[neighbor] = node
            dfs(neighbor, color, parent, edges, cycles)

    color[node] = BLACK

function extractCycle(from, to, parent):
    // Walk parent chain from 'from' back to 'to'
    path = [from]
    current = from
    while current != to:
        current = parent[current]
        path.prepend(current)
    path.append(to)     // close the cycle
    return path
```

**Color semantics:**

| Color | Meaning |
|---|---|
| WHITE | Not yet visited |
| GRAY | Currently on the DFS stack (ancestors of the current node) |
| BLACK | Fully explored — all descendants visited |

A back edge (GRAY → GRAY) proves a cycle. The cycle path is recovered by walking the parent chain from the source of the back edge back to the target.

### Worked Example — Transitive Cycle

Given pipelines: `-StepA → -StepB → -StepC → -StepA`

```polyglot
DFS starts at -StepA (WHITE → GRAY)
  Visit -StepB (WHITE → GRAY)
    Visit -StepC (WHITE → GRAY)
      Visit -StepA — already GRAY → back edge found
      Extract: -StepA → -StepB → -StepC → -StepA
    -StepC → BLACK
  -StepB → BLACK
-StepA → BLACK

Result: cycle [-StepA, -StepB, -StepC, -StepA]
```

### Worked Example — Self-Call

Given pipeline: `-Recurse` calls `-Recurse`

```polyglot
DFS starts at -Recurse (WHITE → GRAY)
  Visit -Recurse — already GRAY → back edge (self-edge)
  Extract: -Recurse → -Recurse
-Recurse → BLACK

Result: cycle [-Recurse, -Recurse]
```

## Alternative — Kahn's Topological Sort

Kahn's algorithm provides an alternative approach. If the topological sort cannot consume all nodes, remaining nodes are part of at least one cycle.

```text
function kahnsSort(nodes, edges):
    inDegree = {node: 0 for node in nodes}
    for each node in nodes:
        for each neighbor in edges[node]:
            inDegree[neighbor] += 1

    queue = [node for node in nodes if inDegree[node] == 0]
    sorted = []

    while queue is not empty:
        node = queue.dequeue()
        sorted.append(node)
        for each neighbor in edges[node]:
            inDegree[neighbor] -= 1
            if inDegree[neighbor] == 0:
                queue.enqueue(neighbor)

    if len(sorted) < len(nodes):
        // Nodes not in sorted list are involved in cycles
        cycleNodes = nodes - sorted
        return CYCLES_FOUND(cycleNodes)
    else:
        return NO_CYCLES(sorted)
```

**Trade-off:** Kahn's detects cycle *existence* and identifies involved nodes, but does not directly produce the cycle path needed for the diagnostic message. An additional DFS on the remaining subgraph is needed to extract exact paths. The DFS three-color approach finds paths directly, making it the preferred algorithm.

## Edge Cases

| Case | Graph Shape | Expected Result |
|---|---|---|
| Self-call | `-A → -A` (self-edge) | PGE09013: `-A → -A` |
| Mutual recursion | `-A → -B`, `-B → -A` | PGE09013: `-A → -B → -A` |
| Transitive cycle | `-A → -B → -C → -A` | PGE09013: `-A → -B → -C → -A` |
| Diamond (no cycle) | `-A → -B`, `-A → -C`, `-B → -D`, `-C → -D` | Valid — no cycle, `-D` reached by two acyclic paths |
| Multiple independent cycles | `-A → -B → -A` and `-X → -Y → -X` | PGE09013 fires twice — report all cycles, not just first |
| Single node, no edges | `-A` (no calls) | Valid — trivially acyclic |
| Linear chain | `-A → -B → -C` | Valid — DAG, no back edges |

**Multiple cycles:** The DFS traversal visits all nodes. Each back edge discovery produces one cycle. The algorithm naturally reports all cycles in a single pass — it does not stop at the first cycle found.

## Complexity

| Metric | Value |
|---|---|
| Time | O(V + E) — each node and edge visited once |
| Space | O(V) — color array, parent map, DFS stack |

Where V = number of `{-}` pipelines in the package and E = number of intra-package `[-]`/`[=]`/`[b]` call edges.

In practice, V and E are small (packages typically contain fewer than 50 pipelines), so performance is not a concern.

## Diagnostic Output

When a cycle is detected, the compiler emits:

```polyglot
PGE09013: Circular pipeline call detected: -A → -B → -C → -A — Polyglot does not support recursion
```

Format: the full cycle path with `→` separators, starting and ending at the same node.

If multiple cycles exist, each produces a separate diagnostic.

## See Also

- [[PGE09013-circular-pipeline-call|PGE09013 — Circular Pipeline Call]] — the compile rule this algorithm implements
- [[PGE09002-circular-package-dependency|PGE09002 — Circular Package Dependency]] — analogous cycle detection for cross-package import graphs
- [[PGE05004-recursive-data-definition|PGE05004 — Recursive Data Definition]] — analogous cycle detection for `{#}` type-reference graphs
