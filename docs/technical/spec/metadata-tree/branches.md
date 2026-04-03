---
audience: [architect, designer]
type: spec
updated: 2026-04-03
---

# Branch Specifications

<!-- @source:metadata-tree/INDEX -->

This file specifies the wrapper, queue, trigger, and permission branches of the metadata tree.

## Wrapper Branch

`%W` stores wrapper definitions (`{W}`). Wrappers provide setup/cleanup scope around pipeline execution bodies. Each `[W]` invocation in a pipeline creates a new wrapper instance.

### Structure

```
%W:DB.Connection:0
├── .[{]                     <- inputs from calling pipeline
│   └── .connectionString#string
├── .[}]                     <- outputs exposed to calling pipeline
│   └── .dbConn
├── .setup                   <- [\] setup phase
└── .cleanup                 <- [/] cleanup phase
```

### Key Properties

- **Flexible instances** — each `[W]` invocation creates `%W:Name:N` with sequential numbering, like pipelines.
- **IO via `[{]`/`[}]`** — wrapper inputs (`[{]`) and outputs (`[}]`) are fixed typed data sections, analogous to `.<`/`.>` in pipelines.
- **Composite wrappers** — a `{W}` definition can contain `[W]` references to other wrappers inside `[\]` or `[/]`, creating nested wrapper instances.
- **`live` fields** — wrapper instances report runtime state: `status`, `errors`, `setupDuration`. See [[metadata|user/concepts/metadata]].

## Queue Branch

`%Q` stores queue definitions (`{Q}`). Queues manage pipeline dispatch ordering and concurrency. Each queue dispatches pipelines and tracks active/pending counts.

### Structure

```
%Q:GPUQueue:0
├── .strategy#QueueStrategy        <- FIFO, LIFO, Priority
├── .host#String                   <- target host (1 queue = 1 host)
├── .maxInstances#UnsignedInt      <- max parallel instances per pipeline
├── .maxConcurrent#UnsignedInt     <- max other pipelines alongside
├── .resourceTags#Array:ResourceTag <- resource constraint tags
├── .killPropagation#KillPropagation <- Cascade or Downgrade
├── .maxWaitTime#String            <- max time before escalation
├── .description#String            <- human-readable description
└── .controls                      <- active queue controls
    ├── .pause
    ├── .resume
    └── .kill
```

### Key Properties

- **Flexible instances** — each queue use creates `%Q:Name:N` with sequential numbering.
- **Fields are fixed** — all fields (`.strategy`, `.host`, `.maxInstances`, etc.) are Polyglot-defined fixed fields. `#RetriggerStrategy` is a queue configuration enforced by the Trigger Monitor, not a queue metadata field.
- **Host-based dispatch** — `.host` binds each queue to a specific host. 1 queue = 1 host. Offloading work to another host means switching queues (e.g., via `=Q.Reassign`).
- **Active controls** — nested `[Q]` lines within the definition set default pause/resume/kill behavior.
- **`live` fields** — queue instances report runtime state: `pendingCount`, `activeCount`, `totalProcessed`. See [[metadata|user/concepts/metadata]].

## Trigger Branch

`%T` stores trigger definitions (`{T}`). Triggers are specialized pipeline subtypes that define event sources with IO-only bodies. Each `[T]` invocation in a pipeline creates a new trigger instance.

### Structure

```
%T:Folder.NewFiles:0
├── .<                      <- input ports
│   └── .path#path
└── .>                      <- output ports
    ├── .IsTriggered#bool   <- mandatory
    └── .NewFiles#array:path
```

### Key Properties

- **Flexible instances** — each trigger invocation creates `%T:Name:N` with sequential numbering, like pipelines.
- **IO via `.<`/`.>`** — same as pipelines (not `.[{]`/`.[}]` like wrappers). Inputs are trigger configuration; outputs are trigger results.
- **Mandatory output** — `>IsTriggered#bool` must exist on every trigger definition (compiler enforced).
- **`live` fields** — trigger instances report runtime state: `status`, `lastFired`, `fireCount`. See [[metadata|user/concepts/metadata]].

## Permission Branch

`%_` stores permission declarations. Unlike other branches, `%_` has **no `:{instance}` level** and **no `:` flexible fields** — permissions are compile-time declarations with an entirely fixed schema. All categories and capabilities are Polyglot-defined, not user-extensible. See [[permissions]] for the full permission system.

### Structure

```
%_
├── .File
│   ├── .read               #string  (glob pattern)
│   ├── .write              #string
│   ├── .execute            #string
│   └── .delete             #string
├── .Web
│   ├── .request
│   │   └── .<              (IO inputs)
│   └── .socket
│       └── .<
├── .Database
│   ├── .connect
│   │   └── .<
│   ├── .read               #string
│   └── .write              #string
├── .System
│   ├── .env                #string
│   ├── .process
│   │   └── .<
│   └── .signal             #string
├── .Crypto
│   ├── .key, .sign, .encrypt   #string
├── .IPC
│   ├── .send, .receive
│   │   └── .<
│   └── .subscribe          #string
├── .Device
│   ├── .camera, .microphone, .location, .bluetooth   #bool
└── .Memory
    ├── .allocate, .shared   #string
```

### Key Properties

- **All fixed (`.`)** — every level uses `.` fixed-field navigation. No `:` flexible fields anywhere in `%_`. Permission categories and capabilities are predefined by Polyglot.
- **No instances** — permissions are per-definition, resolved at compile time. No runtime metadata exists.
- **No `live` fields** — all permission data is static. The compiler resolves permissions entirely during compilation.
- **Nested under `%@` and `%=`** — permissions also appear as `._` subsections under package (`%@:<address>._`) and pipeline (`%=:<name>:<instance>._`) branches, representing the package ceiling and pipeline-level declarations respectively.
- **IO-form capabilities** — capabilities like `.request`, `.connect`, `.send` use `.<` for their IO input parameters, mirroring the IO form syntax in `[_]` declarations.

See also: [[io-ports|IO Port Nesting]], [[instance-lifecycle|Instance Lifecycle]], [[object-types|Object Type Branches]]
