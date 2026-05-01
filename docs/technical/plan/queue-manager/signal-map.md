---
audience: design
type: spec
updated: 2026-04-16
---

# Signal Map — `-Q.*` Cross-Reference

<!-- @c:queue-manager/reactive-signals -->
<!-- @c:queue-manager/signal-payloads -->
<!-- @c:queue-manager/nats-namespace -->
<!-- @c:queue-manager/process-isolation -->
<!-- @c:queue-manager/resource-controls -->

Consolidated cross-reference mapping every `-Q.*` pipeline to its full signal chain: NATS subject, payload, QH Redis operation, control signal to Runner, and Unix mechanism. See [[reactive-signals]] for full state write details.

## Action Commands

| `-Q.*` Command | NATS Command Subject | Payload | QH Redis Write | Control Signal to Runner | Unix Mechanism |
|---|---|---|---|---|---|
| `-Q.Job.Pause.Free.CPU` (.Now/.Wait) | `aljam3.command.job.pause.free.cpu.{jobId}` | `{jobId, timing}` | SREM set:running, HSET set:suspended "cpu", decrement counters, HSET job status "suspended.cpu" | `aljam3.queue.control.{jobId}.job.pause.free.cpu` | .Now: `echo 1 > cgroup.freeze`; .Wait: `SIGSTOP` after work unit |
| `-Q.Job.Pause.Free.RAM.Soft` (.Now/.Wait) | `aljam3.command.job.pause.free.ram.soft.{jobId}` | `{jobId, timing}` | SREM set:running, HSET set:suspended "ram.soft", decrement counters, HSET job status "suspended.ram.soft" | `aljam3.queue.control.{jobId}.job.pause.free.ram.soft` | `cgroup.freeze` + `echo {limit} > memory.high` (best-effort swap) |
| `-Q.Job.Pause.Free.RAM.Hard` (.Now/.Wait) | `aljam3.command.job.pause.free.ram.hard.{jobId}` | `{jobId, timing}` | SREM set:running, HSET set:suspended "ram.hard", decrement counters, HSET job status "suspended.ram.hard" | `aljam3.queue.control.{jobId}.job.pause.free.ram.hard` | `cgroup.freeze` + `echo {limit} > memory.max` (hard cap, OOM-kill risk) |
| `-Q.Job.Pause.Free.All` (.Now/.Wait) | `aljam3.command.job.pause.free.all.{jobId}` | `{jobId, timing}` | SREM set:running, HSET set:suspended "all", decrement counters, HSET job status "suspended.all" | `aljam3.queue.control.{jobId}.job.pause.free.all` | .Now: `criu dump --tree {pid} --images-dir {path}`; .Wait: work unit boundary then `criu dump` |
| `-Q.Job.Resume` | `aljam3.command.job.resume.{jobId}` | `{jobId}` | HDEL set:suspended, RPUSH queue:resume, HSET job status "resuming" | `aljam3.queue.control.{jobId}.job.resume` | cpu/ram: `echo 0 > cgroup.freeze`; all: `criu restore --images-dir {path}` |
| `-Q.Job.Kill.WithCleanup` | `aljam3.command.job.kill.with-cleanup.{jobId}` | `{jobId}` | Status-aware cleanup (SREM/HDEL/LREM by state), RPUSH queue:teardown, HSET job status "teardown.pending" | None immediately; Runner gets `aljam3.queue.control.{jobId}.job.kill.with-cleanup` when dispatched from teardown queue | `SIGTERM` → `[/]` cleanup runs → `SIGKILL` on timeout |
| `-Q.Job.Kill.Now` | `aljam3.command.job.kill.now.{jobId}` | `{jobId}` | Status-aware cleanup (SREM/HDEL/LREM by state), DEL job:{jobId} | `aljam3.queue.control.{jobId}.job.kill.now` (if was executing) | `SIGKILL` — immediate termination |
| `-Q.Job.Throttle` | `aljam3.command.job.throttle.{jobId}` | `{jobId, cpu?, memory?, io?}` | HSET job status "executing.throttled", throttled true, throttle_config | `aljam3.queue.control.{jobId}.job.throttle` | `echo {quota} {period} > cpu.max`, `echo {bytes} > memory.high`, `echo {limits} > io.max` |
| `-Q.Job.Unthrottle` | `aljam3.command.job.unthrottle.{jobId}` | `{jobId}` | HSET job status "executing", throttled false, HDEL throttle_config | `aljam3.queue.control.{jobId}.job.unthrottle` | Remove cgroup limits (restore `cpu.max`, `memory.high`, `io.max` to defaults) |
| `-Q.Job.Snapshot` | `aljam3.command.job.snapshot.{jobId}` | `{jobId, targetQueue?}` | Generate forkId, enqueue fork in targetQueue, HSET fork job hash with forked_from | `aljam3.queue.control.{jobId}.job.snapshot` | `criu dump --leave-running` (original continues, fork enqueued) |
| `-Q.Job.Inspect` | `aljam3.command.job.inspect.{jobId}` | `{jobId}` | None (read-only): HGETALL job:{jobId} | None | None — Redis read only |
| `-Q.Job.Reassign` | `aljam3.command.reassign.{jobId}` | `{jobId, fromQueue, toQueue, priority?}` | Lua script: LREM/ZREM from source, RPUSH/ZADD to target, HSET job queue | None (pending job moves queues) | Same-host: bookkeeping only; Cross-host: `criu dump` → image transfer → `criu restore` on target |
| `-Q.Job.Priority.Update` | `aljam3.command.priority.update.{jobId}` | `{jobId, score}` | ZADD queue:dispatch:{queue} {score} {jobId} | None | None — Redis write only |
| `-Q.Queue.Drain` | `aljam3.command.drain.{queue}` | `{queue}` | SADD queues:draining {queue} | None | None — subsequent enqueues rejected |
| `-Q.Queue.Flush` | `aljam3.command.flush.{queue}` | `{queue}` | FOR each job in queue: DEL job, remove from all sets/queues, DEL queue, SREM queues:registered | `aljam3.queue.control.{jobId}.job.kill.now` per executing job | `SIGKILL` per executing job |
| `-Q.Dispatch.Wait.TimeOut` (escalate) | `aljam3.command.dispatch.escalate.{jobId}` | `{jobId, queue}` | Strategy-aware: ZADD MAX_SCORE (Priority) or LREM+LPUSH (FIFO) or LREM+RPUSH (LIFO) | None | None — queue position change only |
| `-Q.Dispatch.Wait.TimeOut.Kill.Graceful` | `aljam3.command.job.kill.with-cleanup.{jobId}` | `{jobId}` | Same as `-Q.Job.Kill.WithCleanup` | Same as `-Q.Job.Kill.WithCleanup` | Same as `-Q.Job.Kill.WithCleanup` |
| `-Q.Dispatch.Wait.TimeOut.Kill.Hard` | `aljam3.command.job.kill.now.{jobId}` | `{jobId}` | Same as `-Q.Job.Kill.Now` | Same as `-Q.Job.Kill.Now` | Same as `-Q.Job.Kill.Now` |
| `-Q.Dispatch.Wait.TimeOut.Reassign` | `aljam3.command.reassign.{jobId}` | `{jobId, fromQueue, toQueue, priority?}` | Same as `-Q.Job.Reassign` | None | Same as `-Q.Job.Reassign` |
| `-Q.DoNothing` | None | None | None | None | No runtime signal. Compiler no-op satisfying exhaustiveness. |

## Getters and State Guards

| `-Q.*` Getter/State | Data Source | Mechanism |
|---|---|---|
| `-Q.Job.Get.RAM.GB` / `.MB` | cgroup `memory.current` | Read bytes from per-job cgroup file, convert to GB/MB |
| `-Q.Job.Get.CPU.Percent` | cgroup `cpu.stat` | Compute usage from `usage_usec` delta over sample interval |
| `-Q.Job.Get.IO.MBps` | cgroup `io.stat` | Compute throughput from `rbytes`/`wbytes` delta over sample interval |
| `-Q.Job.Get.Time` | Redis `job:{jobId}` `started_at` | `now - started_at` wall-clock duration |
| `-Q.Job.Get.Status` | Redis `job:{jobId}` `status` | HGET status field, return as `#QueueState` enum |
| `-Q.Job.Get.Disk.GB` | `du` on job data directory | Disk usage of job's working directory |
| `-Q.Job.Is.Active` | Redis `job:{jobId}` `status` | Check status == "executing" \|\| "executing.throttled" |
| `-Q.Job.Is.Paused` | Redis `job:{jobId}` `status` | Check status starts with "suspended." |
| `-Q.Job.Is.Throttled` | Redis `job:{jobId}` `throttled` | HGET throttled field |
| `-Q.Job.Get.Idle.CPU` | cgroup `cpu.stat` delta | Time since last `usage_usec` change |
| `-Q.Job.Get.Idle.Network` | cgroup `io.stat` network delta | Time since last network bytes change |
| `-Q.Job.Get.Idle.IO` | cgroup `io.stat` disk delta | Time since last disk bytes change |
| `-Q.Job.Get.Idle.All` | All cgroup stat deltas | Minimum of CPU, Network, IO idle durations |
| `-Q.Job.Active.CPU` | cgroup `cpu.stat` delta | `usage_usec` changed since last sample |
| `-Q.Job.Active.Network` | cgroup `io.stat` network delta | Network bytes changed since last sample |
| `-Q.Job.Active.IO` | cgroup `io.stat` disk delta | Disk bytes changed since last sample |
| `-Q.Job.Active.All` | All cgroup stat deltas | Any resource activity detected |
| `-Q.Host.Get.RAM.GB` | `/proc/meminfo` | `MemAvailable` field, convert to GB |
| `-Q.Host.Get.RAM.Used.GB` | `/proc/meminfo` | `MemTotal - MemAvailable`, convert to GB |
| `-Q.Host.Get.CPU.Percent` | `/proc/stat` | Compute from `cpu` line delta over sample interval |
| `-Q.Host.Get.Disk.GB` | `df` / `statvfs()` | Available bytes on data partition, convert to GB |
| `-Q.Host.Get.Status` | Resource Monitor aggregate | Online/Offline/Draining from host health checks |
| `-Q.Host.Get.GPU.Status` | `nvidia-smi` / vendor tool | InUse/Free from GPU utilization query |
| `-Q.Queue.Get.Length` | Redis `LLEN`/`ZCARD queue:dispatch:{queue}` | Container size of dispatch queue |
| `-Q.Queue.Get.Running` | Redis `SCARD set:running` filtered by queue | Count members where `job.queue == {queue}` |
| `-Q.Queue.Get.Suspended` | Redis `HLEN set:suspended` filtered by queue | Count members where `job.queue == {queue}` |
| `-Q.Queue.Jobs.Get.RAM.GB` | Per-job cgroup `memory.current` | SMEMBERS set:running (queue filter) → read each job's cgroup |
| `-Q.Queue.Jobs.Get.CPU.Percent` | Per-job cgroup `cpu.stat` | SMEMBERS set:running (queue filter) → compute each job's CPU |
| `-Q.Queue.Jobs.Get.Idle.All` | Per-job cgroup stat deltas | SMEMBERS set:running (queue filter) → compute each job's idle |

---

See also: [[reactive-signals]], [[signal-payloads]], [[nats-namespace]], [[process-isolation]], [[resource-controls]]
