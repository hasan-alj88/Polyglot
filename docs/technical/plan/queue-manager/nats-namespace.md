---
audience: design
type: spec
updated: 2026-04-19
---

# NATS Subject Namespace

<!-- @u:queue-manager/signal-payloads -->

All inter-service communication flows through NATS subjects:

## Trigger Events (Trigger Sources → Trigger Monitor)

```text
aljam3.trigger.fire.{pipeline}         — trigger condition met, TM evaluates and enqueues
```

## SDK Call Signals (SDK ↔ Trigger Monitor)

<!-- @c:spec/aljam3-sdk -->
<!-- @c:spec/native-dispatch -->
Related: [[aljam3-sdk#call]], [[native-dispatch#-T.Call Signal Path]]

These subjects implement the SDK `call()` function's NATS request-reply protocol. The SDK publishes a call request; the TM processes it, enqueues the job, and publishes the result back after execution completes.

```text
aljam3.call.{pipeline}                 — SDK publishes call request, TM subscribes and processes
aljam3.result.{correlation_id}         — TM publishes result, SDK subscribes (request-reply)
```

See [[aljam3-sdk#call]] for the request/response payload schema and [[native-dispatch#-T.Call Signal Path]] for the TM-side processing logic including pipeline matching, Redis binding storage, and result collection.

## Lifecycle Control (Trigger Monitor → Queue Handler)

```text
aljam3.command.enqueue                              — enqueue a job
aljam3.command.job.pause.free.cpu.{jobId}           — suspend (CPU freed, timing in payload)
aljam3.command.job.pause.free.ram.soft.{jobId}      — suspend (RAM best-effort swap)
aljam3.command.job.pause.free.ram.hard.{jobId}      — suspend (RAM guaranteed freed)
aljam3.command.job.pause.free.all.{jobId}           — suspend (CRIU checkpoint to disk)
aljam3.command.job.resume.{jobId}                   — resume from any suspended type
aljam3.command.job.kill.with-cleanup.{jobId}        — queue for [/] cleanup, then terminate
aljam3.command.job.kill.now.{jobId}                 — terminate immediately
aljam3.command.job.throttle.{jobId}                 — reduce resource allocation (stays executing)
aljam3.command.job.unthrottle.{jobId}               — restore full resource allocation
aljam3.command.job.snapshot.{jobId}                 — CRIU fork, auto-enqueue new job
aljam3.command.job.inspect.{jobId}                  — read-only state query
aljam3.command.reassign.{jobId}                     — move job between queues
aljam3.command.dispatch.escalate.{jobId}            — escalate dispatch priority
aljam3.command.priority.update.{jobId}              — change priority score
aljam3.command.drain.{queue}                        — stop accepting new enqueues
aljam3.command.flush.{queue}                        — hard kill entire queue
aljam3.command.queue.register                       — register new queue
aljam3.command.queue.update                         — update queue properties
```

## Control Signals (Queue Handler → Runner)

```text
aljam3.queue.control.{jobId}.start                    — begin execution
aljam3.queue.control.{jobId}.job.resume               — resume (includes images_dir for Free.All)
aljam3.queue.control.{jobId}.job.pause.free.cpu       — suspend CPU (timing in payload)
aljam3.queue.control.{jobId}.job.pause.free.ram.soft  — suspend RAM soft (timing in payload)
aljam3.queue.control.{jobId}.job.pause.free.ram.hard  — suspend RAM hard (timing in payload)
aljam3.queue.control.{jobId}.job.pause.free.all       — CRIU checkpoint (timing in payload)
aljam3.queue.control.{jobId}.job.throttle             — reduce resource allocation
aljam3.queue.control.{jobId}.job.unthrottle           — restore full allocation
aljam3.queue.control.{jobId}.job.snapshot             — CRIU fork (includes forkId)
aljam3.queue.control.{jobId}.job.kill.with-cleanup    — run [/] cleanup, then terminate
aljam3.queue.control.{jobId}.job.kill.now             — terminate immediately
```

## Runner Acknowledgments (Runner → Queue Handler + Trigger Monitor)

```text
aljam3.runner.started.{jobId}              — job is now executing
aljam3.runner.completed.{jobId}            — job finished successfully
aljam3.runner.teardown_completed.{jobId}   — job finished [/] cleanup
aljam3.runner.failed.{jobId}               — job failed
aljam3.runner.paused.{jobId}               — job suspended (ACK, includes type + images_dir?)
aljam3.runner.snapshot_completed.{jobId}   — CRIU fork completed (includes forkId, images_dir)
```

## Collector Signals (Trigger Monitor → Runner)

```text
aljam3.collector.{jobId}.collected     — sub-job results collected, parent continues
```

## Resource Monitoring (Resource Monitor → Trigger Monitor)

```text
aljam3.resource.cpu                    — CPU usage metrics
aljam3.resource.ram                    — RAM availability metrics
```

## Pipeline State (Queue Handler → any subscriber)

```text
aljam3.state.{pipeline}.{event}        — state changes for cross-pipeline conditions
```

---

See also: [[signal-payloads]], [[reactive-signals]], [[infrastructure]]
