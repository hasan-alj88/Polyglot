---
audience: design
type: spec
updated: 2026-04-15
---

# Resource Controls Architecture

<!-- @c:queue-manager/reactive-signals -->
<!-- @c:queue-manager/process-isolation -->

This document specifies the full resource control architecture for Aljam3's `-Q.Job.*` system: pause levels, throttling, the pause reason set, anti-flap mechanisms, default queue behaviors, compiler rules, and snapshot safety.

## Resource Freeing Spectrum

Jobs move through a spectrum of resource states. Each level is a strict superset — `Free.RAM` includes `Free.CPU`, `Free.All` includes `Free.RAM`, `Kill` is `Free.All` without "save to disk."

```
Running ---- full CPU, RAM, FDs, TCP, Locks
  |
  v  -Q.Job.Throttle
Throttled -- reduced CPU/RAM/IO, still running
  |
  v  -Q.Job.Pause.Free.CPU
Paused.CPU - CPU freed, RAM+FDs+TCP+Locks kept
  |
  v  -Q.Job.Pause.Free.RAM.Soft
Paused.RAM.Soft - CPU freed, RAM best-effort swapped, FDs+TCP+Locks kept
  |
  v  -Q.Job.Pause.Free.RAM.Hard
Paused.RAM.Hard - CPU freed, RAM guaranteed freed (OOM-kill risk), FDs+TCP+Locks kept
  |
  v  -Q.Job.Pause.Free.All
Paused.All - ALL freed -> serialized to disk files
  |
  v  -Q.Job.Kill.WithCleanup
Dying -------  cleanup [/] running, then ALL freed -> gone
  |
  v  -Q.Job.Kill.Now
Dead --------  ALL freed -> gone (no cleanup)
```

## The Resource Freeing Levels

| Level | Unix Mechanism | CPU | RAM | FDs | TCP | Locks | State Location |
|-------|---------------|-----|-----|-----|-----|-------|---------------|
| **Free.CPU** | Cgroup freeze | Freed | Kept in memory | Open | Open (peer may timeout) | Held | Process memory |
| **Free.RAM.Soft** | Cgroup freeze + `memory.high` hint | Freed | Best-effort swap (kernel hint, not guaranteed) | Open | Open | Held | Swap partition/file |
| **Free.RAM.Hard** | Cgroup freeze + `memory.max` cap | Freed | Guaranteed freed (OOM-kill if exceeded) | Open | Open | Held | Swap partition/file |
| **Free.All** | CRIU dump | Freed | Freed → image files | Closed → image files | Closed → image files (with TCP\_REPAIR) | Released → image files | CRIU image directory |
| **Kill** | SIGTERM/SIGKILL | Freed | Freed → kernel pool | Closed | Closed (FIN/RST) | Released | **Gone forever** |

## Pause Levels

### Free.CPU

**Mechanism:** Cgroup freezer (`echo 1 > cgroup.freeze`) for `.Now`, `SIGSTOP` for `.Wait`.

- CPU scheduling stopped — process gets zero CPU time
- RAM pages remain resident (not swapped)
- File descriptors remain open
- TCP connections remain open — remote peers may timeout if the pause is long
- Locks remain held — other jobs waiting on shared locks will block
- State location: process memory (unchanged)

**Resume:** Cgroup thaw (`echo 0 > cgroup.freeze`) or `SIGCONT`. Instant — no disk IO.

### Free.RAM.Soft

**Mechanism:** Cgroup freeze + `memory.high` set to a low value.

- CPU stopped (same as Free.CPU)
- `memory.high` is a **kernel hint** — the kernel's memory reclaim attempts to push pages to swap. Not guaranteed; under memory pressure the kernel may not reclaim all pages
- File descriptors, TCP, locks: same as Free.CPU
- State location: swap partition/file (best-effort)

**Resume:** Cgroup thaw + remove `memory.high` limit. Pages faulted back from swap on access.

### Free.RAM.Hard

**Mechanism:** Cgroup freeze + `memory.max` set to a low value.

- CPU stopped (same as Free.CPU)
- `memory.max` is a **hard cap** — the kernel kills processes exceeding this limit (OOM-kill). This guarantees RAM is freed but at the risk of losing the job if it can't compress into the limit
- File descriptors, TCP, locks: same as Free.CPU
- State location: swap partition/file (guaranteed if not OOM-killed)

**Resume:** Cgroup thaw + remove `memory.max` limit. Pages faulted back from swap.

**Risk:** If the job's working set cannot fit within the `memory.max` value, the kernel OOM-kills processes in the cgroup. The compiler emits a warning for `Free.RAM.Hard` usage; if the job has no `[/]` cleanup wrapper, this escalates to a compile error.

### Free.All

**Mechanism:** CRIU checkpoint — full process state serialized to disk.

- Everything freed: CPU, RAM, FDs, TCP (via `TCP_REPAIR`), locks, timers
- Process terminated after checkpoint
- State location: CRIU image directory on disk
- Requires CRIU runtime dependency

**Resume:** `criu restore` — PID namespace recreated from image files. All state restored to exact checkpoint point.

**Limitations:** POSIX named semaphores cannot be checkpointed. GPU VRAM requires vendor plugin. See [[process-isolation]] for the full capability matrix.

## Timing: `.Now` vs `.Wait`

Every pause command has two timing variants:

| Timing | Behavior | Use When |
|--------|----------|----------|
| `.Now` | Immediate — cgroup freeze / CRIU dump happens instantly | Emergency: resource critical, must free immediately |
| `.Wait` | Waits for current work unit boundary, then freezes | Normal: allow job to reach a clean stop point |

**Work unit boundary:** The smallest atomic unit of work in progress — one loop iteration, one sub-job, one IO operation. Not "wait for entire pipeline" or "wait indefinitely."

The Runner tracks work unit boundaries. When a `.Wait` command arrives, the Runner sets a flag and checks it at each boundary. When the boundary is reached, the Runner performs the freeze operation.

## Throttle

Throttling reduces resource allocation without pausing. The job keeps running with constrained resources.

**Mechanism:** Cgroup resource controllers.

| Resource | Cgroup File | Format | Effect |
|----------|------------|--------|--------|
| CPU | `cpu.max` | `{quota} {period}` (e.g., `50000 100000` = 50%) | Limits CPU time per period |
| Memory | `memory.high` | Bytes | Soft memory limit — triggers reclaim pressure |
| IO | `io.max` | `{major}:{minor} rbps={n} wbps={n}` | Limits read/write bytes per second |

**Unthrottle:** Remove cgroup limits — restore full allocation.

**Throttle vs Pause:** Throttled jobs remain in `set:running` — they still hold their dispatch slot. Paused jobs are removed from `set:running` and the slot is freed for other jobs.

## Pause Reason Set

The Trigger Monitor maintains a **pause reason set** per job. Each `#JobRules` rule that wants a job paused adds its reason to the set. Each rule that no longer wants the job paused removes its reason. The job actually resumes only when the set is **empty**.

**How it works (Trigger-Driven):**

1. RAM guard trigger fires (`-QT.Job.Resource.Exceeds.RAM`): rule executes `-Q.Job.Pause.Free.RAM.Hard`. TM adds `{ram.hard}` to set.
2. CPU guard trigger fires: rule executes `-Q.Job.Pause.Free.CPU`. TM adds `{cpu}` to set.
3. CPU recovery trigger fires: rule executes `-Q.Job.Resume`. TM removes `{cpu}` from set. Job stays paused because `{ram.hard}` remains.
4. RAM recovery trigger fires: rule executes `-Q.Job.Resume`. TM removes `{ram.hard}` from set. Set is `{}` — job actually resumes.

**Conflicting pause levels:** When multiple reasons exist at different levels, the **highest level wins**:

```
Free.All > Free.RAM.Hard > Free.RAM.Soft > Free.CPU
```

If the set contains `{cpu, ram.hard}`, the job is paused at `Free.RAM.Hard` level. When `ram.hard` is removed and only `cpu` remains, the job transitions to `Free.CPU` level (cgroup thaw of memory controls, CPU still frozen).

**Implementation:** Redis set `job:{jobId}:pause_reasons`. The Trigger Monitor is fully reactive; it listens for `-Q.*` control signals emitted by triggered rules and applies the delta to the set, sending the actual OS pause/resume commands only when the high-water mark changes or the set empties.

## Anti-Flap Mechanisms

Resource values fluctuate. Without hysteresis, a value oscillating around a threshold causes rapid pause/resume cycles (flapping). Aljam3 provides two anti-flap mechanisms:

### Spatial Hysteresis: `<margin` Parameter

Different thresholds for pausing vs resuming. The `<margin` parameter on getter comparisons creates a gap:

```aljam3
{Q} #JobRules:RAMGuard
   (#) <margin #float <~ 500.0

   [?] -Q.Host.Get.RAM.MB "<? {$margin}"
      [Q] -Q.Job.Pause.Free.RAM.Hard.Wait
   [?] *?
      [Q] -Q.DoNothing
```

If margin = 500 MB and threshold = 2000 MB:
- Pause triggers at RAM < 2000 MB
- Resume triggers at RAM > 2500 MB (threshold + margin)

The 500 MB gap prevents flapping around the threshold.

### Temporal Hysteresis: `.resumeDebounce`

The pause reason set must be empty for a configured duration before the job actually resumes:

```aljam3
{Q} #Queue:WorkerQueue
   [.] .resumeDebounce << #DT"10s"
```

Even if the resource recovers and the pause reason set empties, the TM waits 10 seconds of sustained empty-set before issuing `command.job.resume`. If a reason reappears within that window, the debounce resets.

### Reactive Event Evaluation

Because Aljam3 uses a reactive trigger architecture (`-QT.*`), there is no generic "tick period" that evaluates every rule sequentially. Rules are compiled into signal maps and evaluate only when the underlying resource monitor emits a state change or edge trigger.

Smallest possible resolution depends on the resource watcher's polling interval, which is configurable per-queue:

```aljam3
{Q} #Queue:WorkerQueue
   [.] .pollPeriod << #DT"5s"
```

## Default Queue Behaviors

Every queue includes these built-in rules unless explicitly overridden. They are visible in the compiled behavior contract.

| Default | Condition | Action | Why |
|---------|-----------|--------|-----|
| **RAM overflow prevention** | Job RAM approaching host available | `-Q.Job.Pause.Free.RAM.Wait` | Prevent OOM-kill |
| **Disk space guard** | Host disk < 1GB | `-Q.Job.Pause.Free.All.Wait` | Prevent disk-full crash |
| **Runaway CPU** | Job CPU > 100% for > 5m (single-threaded job) | `-Q.Job.Throttle` | Likely infinite loop |
| **Zombie cleanup** | Job status = finished but process alive | `-Q.Job.Kill.Now` | Leaked process |
| **Orphan detection** | Job's parent pipeline terminated | `-Q.Job.Kill.WithCleanup` | No one waiting for result |
| **CRIU image cleanup** | Paused.All job not resumed in 24h | Notify + optional Kill | Stale checkpoint consuming disk |

Override by setting the default to false:

```aljam3
{Q} #Queue:MyQueue
   [.] .defaults.ramOverflow << false
```

## Compiler Rules

### Implicit Exhaustiveness via Triggers

Because rules are trigger-driven (`[T]`), you do not need `*?` wildcards or `-Q.DoNothing` blocks to handle states you don't care about. If a trigger doesn't fire, the rule simply doesn't wake up.

This removes the need for the compiler to warn about "Uncovered States" in Queue logic, as non-firing triggers naturally represent the absence of an action.

### Free.RAM.Hard OOM-Kill Risk

If `Free.RAM.Hard` is used, the compiler emits a **warning** noting OOM-kill risk. If the job has no `[/]` cleanup wrapper, the compiler escalates to an **error** — a hard memory cap without cleanup could lose data.

```
PGW: Free.RAM.Hard may OOM-kill processes in job cgroup.
PGE: Free.RAM.Hard used without [/] cleanup wrapper — potential data loss.
```

### GPU Level Capping

The compiler verifies pause level compatibility at compile time. Using an action beyond the job's maximum level is a compile error:

| Job Uses | Max Pause Level | Max Spatial | Compile Error If Exceeds |
|----------|----------------|-------------|--------------------------|
| CPU only | Free.All | Cross-host Reassign, Snapshot | — |
| CPU + TCP (not repairable) | Free.RAM | Snapshot only | Free.All, Reassign |
| CPU + TCP (repairable) | Free.All | Cross-host Reassign, Snapshot | — |
| GPU (with vendor plugin) | Free.All | Cross-host Reassign, Snapshot | — |
| GPU (no plugin) | Free.RAM | None | Free.All, Reassign, Snapshot |
| POSIX named semaphores | Free.RAM | None | Free.All, Reassign, Snapshot |

## Snapshot Safety

`-Q.Job.Snapshot` creates a point-in-time state fork. The original job keeps running. The snapshot can spawn a second instance (fork). The compiler enforces safety rules to prevent concurrency pitfalls.

### Compiler-Enforced Rules

The compiler rejects snapshot usage when:

- **Held locks at snapshot point** — compile error. A forked instance with held locks creates inconsistent state (two holders of the same lock)
- **Open non-repairable TCP** — compile error. Connection state cannot be serialized without `TCP_REPAIR`
- **Non-idempotent side effects** (DB writes, network messages) — compile error unless wrapped in an idempotency guard. A forked instance replaying side effects could cause duplicates

### Fork Semantics

- Forked instance gets a **new Job ID** (original keeps its ID)
- Forked instance starts with **zero locks** — must re-acquire
- Process identity is distinct — no shared mutable state between original and fork
- The forked instance goes through normal dispatch (enqueued as a new job)

---

See also: [[process-isolation]], [[reactive-signals]], [[signal-payloads]]
