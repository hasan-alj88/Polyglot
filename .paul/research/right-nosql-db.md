# Research: The Right NoSQL DB for Polyglot

## Context & Objectives
Polyglot requires a NoSQL database to store:
- Queue definitions (`{Q}` schema fields)
- Job hierarchy (parent→children runtime tree)

This data is heavily read by the Trigger Monitor (TM) for kill propagation and collector logic. According to the `core-philosophy.md` and `infrastructure.md` docs, the solution must support both lightweight local execution and distributed deployments. 

## Candidates Examined
1. **SurrealDB**
2. **RedisJSON (via existing Redis/Valkey)**
3. **MongoDB**
4. **PoloDB / redb**

## Key Findings & Comparisons

### 1. SurrealDB (Recommended if BSL is acceptable)
- **Architecture Fit**: Built natively in Rust.
- **Data Model**: Multi-model (Document + Graph). The Polyglot job hierarchy is inherently a graph structure. SurrealDB treats relationships as first-class citizens, allowing native, multi-depth traversal of the job tree without complex application-level logic.
- **Deployment**: Can run embedded (satisfying Polyglot's local execution requirement) or distributed (for scalable service deployment) using the exact same API.
- **Licensing & Cost**: **Free ($0) but BSL 1.1**. It is completely free for development and production self-hosting. However, it uses the Business Source License (converts to Apache 2.0 after 4 years), meaning it is not strictly OSI "Open Source".

### 2. Valkey + ValkeyJSON (Best Strict FOSS Alternative)
- **Architecture Fit**: Polyglot already uses Redis/Valkey for queue ordering and state. Adding the ValkeyJSON module simplifies the stack to just NATS + Valkey.
- **Data Model**: Key-Value + Document. While it handles JSON well, it lacks native graph capabilities. Managing the `job hierarchy` would require manual orchestration in the Rust application layer (e.g., maintaining adjacency lists).
- **Deployment**: Excellent for distributed setups, but requires running a separate Valkey process locally.
- **Licensing & Cost**: **100% Free and Open Source (BSD 3-Clause)**. Valkey is the Linux Foundation's fully open-source fork of Redis. 

### 3. MongoDB
- **Architecture Fit**: Industry standard, excellent document support, good Rust driver.
- **Data Model**: Document store. Can handle hierarchical data via nested documents or references, but lacks native graph traversal.
- **Deployment**: Heavyweight. Requires a separate daemon process, making it less ideal for the lightweight "local execution" philosophy.
- **Licensing & Cost**: **Free ($0) but SSPL**. Server Side Public License is not OSI-approved. Free to self-host, but carries restrictions if providing managed services.

### 4. PoloDB / redb
- **Architecture Fit**: Pure Rust embedded databases.
- **Data Model**: PoloDB is document-oriented (MongoDB-like API), redb is Key-Value. Neither provides native graph traversal for job trees.
- **Deployment**: Excellent for local, but lacks out-of-the-box distributed clustering.
- **Licensing & Cost**: **100% Free and Open Source (MIT/Apache 2.0)**. 

## Conclusion & Recommendation
If by "free" you mean **$0 cost**, **SurrealDB** remains the strongest candidate. It is completely free to self-host and embed, and its graph capabilities perfectly solve the `job hierarchy` requirement.

If by "free" you mean **Strictly Open Source (OSI-approved)**, then **Valkey + ValkeyJSON** is the best choice. It is fully open-source (BSD), already aligns with your existing infrastructure choices, and avoids the need to introduce a new BSL/SSPL dependency, though you will have to manually manage the graph traversal for job hierarchies in Rust.

---

## What about the `%` Metadata Data Tree?

Given your decision to use Redis (Valkey) for Queues and Job Hierarchy, you need a solution for the mega `%` Data Tree (which contains `%definition`, `%@` packages, `%$` variables, and `%-` pipeline instances). 

### Analysis of the `%` Tree Requirements:
1. **Tree Structure**: Polyglot accesses data via paths (e.g., `%-:ProcessData:0.<.filepath`). This maps perfectly to JSON document paths.
2. **Access Pattern**: Distributed Queue Handlers (runners across different hosts) must be able to read definitions and read/write variable instances concurrently. Thus, the database **must be distributed/shared**.
3. **Data Lifecycle**:
   - *Static*: Definitions and Packages (compiled once, read-heavy).
   - *Ephemeral*: Instances and Variables (created at runtime, mutated, and eventually garbage-collected when they reach the `Released` state).

### Option A: Valkey + ValkeyJSON (Unified Stack) - *Recommended for Speed*
Since you are already running Valkey, adding the **ValkeyJSON** module allows you to store the entire `%` tree in the same infrastructure. 
- **How it works**: The `%` tree is stored as JSON documents. A variable lookup translates directly to a `JSON.GET` or `JSON.SET` command using JSONPath (e.g., `JSON.GET %-:ProcessData:0 .status`).
- **Pros**: Zero additional infrastructure. 100% Free (BSD 3-Clause). Extremely fast (sub-millisecond) in-memory lookups, which is ideal for runtime variable resolution.
- **Cons**: RAM constraints. Because Valkey is an in-memory store, if you have millions of large, long-living variables (like massive `#Dataframe` instances), you could exhaust your server's RAM. You will rely heavily on Polyglot's `Released` state to garbage-collect variables.

### Option B: PostgreSQL with JSONB - *Recommended for FOSS & Massive Data*
While technically a relational database, PostgreSQL's `JSONB` datatype makes it an elite document store, and it happens to solve almost all of Polyglot's problems.
- **How it works**: The `%` tree is stored as JSONB data. You use PostgreSQL's JSONPath operators to read and write variables. 
- **The Graph Superpower**: For your Job Hierarchy, Postgres has a feature called **Recursive CTEs** (`WITH RECURSIVE`). This allows you to write a *single* SQL query that traverses the entire `parent→children` job tree down to the very bottom. The database engine does all the traversing natively and returns the whole list of descendants instantly—giving you the benefits of a Graph DB without actually needing one.
- **Pros**: 100% Free and Open Source (OSI-approved PostgreSQL License). Persists to disk, bypassing the Valkey RAM limits for massive datasets. Solves the Job Hierarchy traversal problem natively. Unbeatable Rust integration (via `sqlx`).
- **Cons**: Requires running a PostgreSQL server, which is slightly heavier than Valkey but extremely standard. 

### Option C: SurrealDB - *The Native Graph Approach*
You run Valkey for Queues, but use SurrealDB specifically for the `%` Data Tree and Job Hierarchy.
- **How it works**: SurrealDB acts as the persistent document store for all packages, definitions, and instances, while handling the job tree natively.
- **Pros**: Bypasses RAM limitations. Native graph traversal for job hierarchies without needing to write SQL recursive CTEs.
- **Cons**: Adds a second database technology to your stack. Free ($0), but uses the BSL license (not strictly OSI Open Source).

**Verdict**: If you are strictly committed to OSI-approved Open Source, **PostgreSQL** is an incredible "silver bullet" here. It gives you the disk-backed JSON storage of MongoDB, the tree-traversal capabilities of SurrealDB (via `WITH RECURSIVE`), and the 100% Free OSI licensing of Valkey.

---

## Final Architecture Decision
Based on the research and project constraints (100% Free Open Source, massive concurrency, memory safety), the following architecture has been selected:

1. **Redis / Valkey**: Will handle **Queues and the Job Hierarchy**. It provides the raw, lock-free, sub-millisecond in-memory speed required for orchestrating thousands of concurrent dispatch operations.
2. **PostgreSQL**: Will act as the canonical persistence layer for everything else (**The `%` Metadata Data Tree**). Using its `JSONB` document storage, it will store all definitions, variable instances, packages, and imports. This bypasses the RAM-exhaustion risks of an in-memory database, handles concurrent writes natively, provides 100% Free OSI licensing, and integrates flawlessly with Rust via `sqlx`.
