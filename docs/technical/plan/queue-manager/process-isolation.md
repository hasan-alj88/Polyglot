---
audience: design
type: spec
updated: 2026-04-16
---

# Process Isolation Architecture

<!-- @c:queue-manager/reactive-signals -->
<!-- @c:queue-manager/resource-controls -->

Polyglot enforces per-job process isolation using Linux PID namespaces, pidfd-based signal delivery, cgroup resource control, and CRIU checkpoint/restore. This document covers the kernel mechanisms backing the `-Q.Job.*` control signals.

## PID Namespaces

Each job runs in its own Linux PID namespace. The job process sees itself as PID 1 — it cannot see, address, or signal any process outside its namespace.

**Per-job isolation guarantees:**

- A job cannot enumerate other jobs' processes
- A job cannot send signals to other jobs
- A job's child processes are contained within the same namespace
- The Queue Handler holds external references (pidfds) from the host namespace

**Namespace lifecycle:**

1. Job dispatch: Runner creates a new PID namespace for the job via `clone(CLONE_NEWPID)`
2. Job execution: all processes spawned by the job inherit the namespace
3. Job completion/kill: namespace destroyed, all processes terminated

The PID namespace is the isolation boundary for all control operations. Cgroup freezer, CRIU checkpoint, and signal delivery all operate on the namespace scope.

## pidfd Operations

All signal operations use `pidfd_send_signal()` instead of `kill()`. This eliminates PID reuse races — a pidfd refers to a specific process instance, not a recycled PID number.

**pidfd lifecycle:**

1. Runner creates job process via `clone()` → receives pidfd
2. pidfd stored in job metadata (QH Redis: `job:{jobId}` pid field holds pidfd reference)
3. All pause/kill/inspect commands use `pidfd_send_signal()` or `pidfd_getfd()`
4. pidfd automatically invalidated when process exits

**Key operations:**

| Operation | Mechanism | Used By |
|-----------|-----------|---------|
| Pause (SIGSTOP) | `pidfd_send_signal(pidfd, SIGSTOP, ...)` | `command.job.pause.free.cpu.wait` (after work unit) |
| Kill (SIGTERM) | `pidfd_send_signal(pidfd, SIGTERM, ...)` | `command.job.kill.withcleanup` |
| Kill (SIGKILL) | `pidfd_send_signal(pidfd, SIGKILL, ...)` | `command.job.kill.now` |
| Inspect | `pidfd_getfd(pidfd, targetfd, 0)` | `command.job.inspect` |
| Wait | `waitid(P_PIDFD, pidfd, ...)` | Runner waits for job completion |

**Why pidfd over PID:** Between sending a kill signal and the process exiting, the PID could be recycled by the kernel. `pidfd_send_signal()` is immune to this — the file descriptor refers to the original process regardless of PID recycling.

## Cgroup Integration

Each job gets a dedicated cgroup under the Polyglot cgroup hierarchy. The cgroup provides resource measurement, resource limiting, and atomic freeze/thaw.

**Cgroup hierarchy:**

```
/sys/fs/cgroup/polyglot/
├── queue:{queueName}/
│   ├── job:{jobId}/          ← per-job cgroup
│   │   ├── cgroup.freeze     ← atomic pause (Free.CPU)
│   │   ├── cpu.max           ← throttle / priority
│   │   ├── cpu.weight        ← scheduling weight
│   │   ├── memory.high       ← soft RAM limit (Free.RAM.Soft)
│   │   ├── memory.max        ← hard RAM limit (Free.RAM.Hard)
│   │   ├── io.max            ← IO throttle
│   │   └── memory.current    ← measurement (getters)
│   └── job:{jobId2}/
└── queue:{queueName2}/
```

**Cgroup operations by pause level:**

| Pause Level | Cgroup Operation | Kernel Behavior |
|-------------|------------------|-----------------|
| Free.CPU.Now | `echo 1 > cgroup.freeze` | Atomic freeze — all threads stopped instantly, no SIGSTOP delay |
| Free.CPU.Wait | Wait for work unit → `SIGSTOP` | Process-level stop after clean boundary |
| Free.RAM.Soft | `cgroup.freeze` + `echo {limit} > memory.high` | Kernel hint to reclaim pages to swap (best-effort, not guaranteed) |
| Free.RAM.Hard | `cgroup.freeze` + `echo {limit} > memory.max` | Hard cap — kernel OOM-kills processes exceeding limit |
| Free.All | CRIU checkpoint (uses cgroup for process enumeration) | Full state serialized to disk |

**Cgroup freezer vs SIGSTOP:** The cgroup freezer (`cgroup.freeze`) is preferred over `SIGSTOP` for `.Now` timing because it is atomic across all threads in the cgroup. `SIGSTOP` is per-process and has race windows with multi-threaded jobs. `.Wait` uses `SIGSTOP` because the Runner controls the work unit boundary timing.

**Throttle operations:**

| Resource | Cgroup File | Format |
|----------|------------|--------|
| CPU | `cpu.max` | `{quota} {period}` (e.g., `50000 100000` = 50%) |
| Memory | `memory.high` | Bytes (soft limit, triggers reclaim) |
| IO | `io.max` | `{major}:{minor} rbps={n} wbps={n}` |

## CRIU Integration

CRIU (Checkpoint/Restore In Userspace) provides the mechanism for `Free.All`, cross-host `Reassign`, and `Snapshot` operations. It serializes an entire PID namespace to disk image files and can restore it later — potentially on a different host.

### CRIU as Runtime Dependency

CRIU is a **required runtime dependency** for full functionality. The Polyglot runtime checks at startup:

- **CRIU present:** `Free.All`, cross-host `Reassign`, `Snapshot` all available
- **CRIU missing:** These operations disabled; using them produces a compile/runtime error. `Free.CPU` and `Free.RAM` still work (they use cgroup freezer, not CRIU)

### What CRIU Can and Cannot Checkpoint

| Resource | CRIU Support | Notes |
|----------|-------------|-------|
| Process memory | Yes | Full address space serialized |
| Open file descriptors | Yes | Paths + offsets saved |
| TCP connections | Yes (with TCP_REPAIR) | Socket state serialized via `TCP_REPAIR` option |
| TCP connections (no repair) | No | Connection lost on restore — see `.tcpRepairable` queue property |
| SysV semaphores (SEM_UNDO) | Yes | Runtime uses these internally |
| POSIX named semaphores | **No** | Cannot checkpoint — disables `Free.All` and cross-host `Reassign` |
| GPU VRAM | Plugin-dependent | Vendor plugin required; without it, max level = `Free.RAM` |
| Unix domain sockets | Yes | If both endpoints in same namespace |
| Pipes | Yes | Pipe buffer contents preserved |
| Timers | Yes | Timer state + remaining time preserved |

### CRIU Image Transfer

For cross-host `Reassign`:

1. **Control signals:** via NATS (existing messaging infrastructure)
2. **Image transfer:** direct TCP between hosts or shared storage (NFS/distributed filesystem)
3. **Job ID preservation:** the job keeps its ID across hosts — the TM updates the host mapping in NoSQL

Image transfer is handled by the Runner, not the Queue Handler. The QH only knows about queue membership changes.

### CRIU Operations by Signal

| Signal | CRIU Action |
|--------|------------|
| `command.job.pause.free.all` (timing: "now") | `criu dump --tree {pid} --images-dir {path}` — immediate checkpoint |
| `command.job.pause.free.all` (timing: "wait") | Wait for work unit boundary → `criu dump` |
| `command.job.resume` (type was "all") | `criu restore --images-dir {path}` — restore PID namespace |
| `command.reassign` (cross-host) | `criu dump` → transfer images → `criu restore` on target host |
| `command.job.snapshot` | `criu dump --leave-running` (fork semantics — original continues) |

## TCP Repairability

The `TCP_REPAIR` socket option allows CRIU to serialize TCP connection state (sequence numbers, window sizes, buffers). This is declared per-queue:

```polyglot
{Q} #Queue:MigrateableQueue
   [.] .tcpRepairable << true
```

**Behavior by declaration:**

| `.tcpRepairable` | `Free.All` | Cross-host Reassign | Implication |
|------------------|-----------|---------------------|-------------|
| `true` | Full CRIU (TCP preserved) | Available | Queue guarantees all jobs use repairable TCP |
| `false` (default) | Downgrade to `Free.RAM` | Compile error | TCP connections would be lost |

The compiler enforces this at compile time — a `Free.All` action on a queue with `tcpRepairable: false` is a compile error.

## GPU Considerations

GPU state (VRAM contents, shader programs, compute contexts) cannot be serialized by CRIU alone. Polyglot supports two modes:

**With vendor plugin:**
- Plugin provides VRAM serialize/deserialize hooks
- Full CRIU support — `Free.All`, cross-host `Reassign`, `Snapshot` available
- Plugin registered at runtime startup

**Without vendor plugin (default):**
- Maximum pause level: `Free.RAM` (GPU state stays on device)
- Cross-host `Reassign`: not available (GPU state not portable)
- `Snapshot`: not available (incomplete state capture)
- Compiler enforces these limits — using `Free.All` on a GPU job without plugin = compile error

**Level capping table (compile-time verification):**

| Job Uses | Max Pause Level | Max Spatial | Reason |
|----------|----------------|-------------|--------|
| CPU only | Free.All | Cross-host Reassign, Snapshot | Full CRIU support |
| CPU + TCP (not repairable) | Free.RAM | Snapshot only | TCP can't survive Free.All |
| CPU + TCP (repairable) | Free.All | Cross-host Reassign, Snapshot | TCP_REPAIR enabled |
| GPU (with vendor plugin) | Free.All | Cross-host Reassign, Snapshot | Plugin handles VRAM |
| GPU (no plugin) | Free.RAM | None | CRIU can't serialize GPU state |
| POSIX named semaphores | Free.RAM | None | CRIU can't checkpoint these |

---

See also: [[reactive-signals]], [[resource-controls]], [[signal-payloads]]
