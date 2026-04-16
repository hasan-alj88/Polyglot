---
audience: architect
type: spec
updated: 2026-04-15
---

# NATS Subject Namespace

<!-- @u:queue-manager/signal-payloads -->

All inter-service communication flows through NATS subjects:

## Trigger Events (Trigger Sources → Trigger Monitor)

```text
polyglot.trigger.fire.{pipeline}         — trigger condition met, TM evaluates and enqueues
```

## Lifecycle Control (Trigger Monitor → Queue Handler)

```text
polyglot.command.enqueue                              — enqueue a job
polyglot.command.job.pause.free.cpu.{jobId}           — suspend (CPU freed, timing in payload)
polyglot.command.job.pause.free.ram.soft.{jobId}      — suspend (RAM best-effort swap)
polyglot.command.job.pause.free.ram.hard.{jobId}      — suspend (RAM guaranteed freed)
polyglot.command.job.pause.free.all.{jobId}           — suspend (CRIU checkpoint to disk)
polyglot.command.job.resume.{jobId}                   — resume from any suspended type
polyglot.command.job.kill.with-cleanup.{jobId}        — queue for [/] cleanup, then terminate
polyglot.command.job.kill.now.{jobId}                 — terminate immediately
polyglot.command.job.throttle.{jobId}                 — reduce resource allocation (stays executing)
polyglot.command.job.unthrottle.{jobId}               — restore full resource allocation
polyglot.command.job.snapshot.{jobId}                 — CRIU fork, auto-enqueue new job
polyglot.command.job.inspect.{jobId}                  — read-only state query
polyglot.command.reassign.{jobId}                     — move job between queues
polyglot.command.dispatch.escalate.{jobId}            — escalate dispatch priority
polyglot.command.priority.update.{jobId}              — change priority score
polyglot.command.drain.{queue}                        — stop accepting new enqueues
polyglot.command.flush.{queue}                        — hard kill entire queue
polyglot.command.queue.register                       — register new queue
polyglot.command.queue.update                         — update queue properties
```

## Control Signals (Queue Handler → Runner)

```text
polyglot.queue.control.{jobId}.start                    — begin execution
polyglot.queue.control.{jobId}.job.resume               — resume (includes images_dir for Free.All)
polyglot.queue.control.{jobId}.job.pause.free.cpu       — suspend CPU (timing in payload)
polyglot.queue.control.{jobId}.job.pause.free.ram.soft  — suspend RAM soft (timing in payload)
polyglot.queue.control.{jobId}.job.pause.free.ram.hard  — suspend RAM hard (timing in payload)
polyglot.queue.control.{jobId}.job.pause.free.all       — CRIU checkpoint (timing in payload)
polyglot.queue.control.{jobId}.job.throttle             — reduce resource allocation
polyglot.queue.control.{jobId}.job.unthrottle           — restore full allocation
polyglot.queue.control.{jobId}.job.snapshot             — CRIU fork (includes forkId)
polyglot.queue.control.{jobId}.job.kill.with-cleanup    — run [/] cleanup, then terminate
polyglot.queue.control.{jobId}.job.kill.now             — terminate immediately
```

## Runner Acknowledgments (Runner → Queue Handler + Trigger Monitor)

```text
polyglot.runner.started.{jobId}              — job is now executing
polyglot.runner.completed.{jobId}            — job finished successfully
polyglot.runner.teardown_completed.{jobId}   — job finished [/] cleanup
polyglot.runner.failed.{jobId}               — job failed
polyglot.runner.paused.{jobId}               — job suspended (ACK, includes type + images_dir?)
polyglot.runner.snapshot_completed.{jobId}   — CRIU fork completed (includes forkId, images_dir)
```

## Collector Signals (Trigger Monitor → Runner)

```text
polyglot.collector.{jobId}.collected     — sub-job results collected, parent continues
```

## Resource Monitoring (Resource Monitor → Trigger Monitor)

```text
polyglot.resource.cpu                    — CPU usage metrics
polyglot.resource.ram                    — RAM availability metrics
```

## Pipeline State (Queue Handler → any subscriber)

```text
polyglot.state.{pipeline}.{event}        — state changes for cross-pipeline conditions
```

---

See also: [[signal-payloads]], [[reactive-signals]], [[infrastructure]]
