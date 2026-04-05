---
audience: architect
type: spec
updated: 2026-04-03
---

# NATS Subject Namespace

<!-- @queue-manager/signal-payloads -->

All inter-service communication flows through NATS subjects:

## Trigger Events (Trigger Sources → Trigger Monitor)

```text
polyglot.trigger.fire.{pipeline}         — trigger condition met, TM evaluates and enqueues
```

## Lifecycle Control (Trigger Monitor → Queue Handler)

```text
polyglot.command.enqueue                 — enqueue a job
polyglot.command.pause.soft.{jobId}      — finish current work, then suspend
polyglot.command.pause.hard.{jobId}      — suspend immediately
polyglot.command.resume.{jobId}          — resume from suspended
polyglot.command.kill.graceful.{jobId}   — queue for [/] cleanup, then terminate
polyglot.command.kill.hard.{jobId}       — terminate immediately
polyglot.command.reassign.{jobId}        — move job between queues
polyglot.command.dispatch.escalate.{jobId} — escalate dispatch priority
polyglot.command.drain.{queue}           — stop accepting new enqueues
polyglot.command.flush.{queue}           — hard kill entire queue
polyglot.command.queue.register          — register new queue
polyglot.command.queue.update            — update queue properties
```

## Control Signals (Queue Handler → Runner)

```text
polyglot.queue.control.{jobId}.start         — begin execution
polyglot.queue.control.{jobId}.resume        — resume from suspended state
polyglot.queue.control.{jobId}.pause.soft    — finish current work, then suspend
polyglot.queue.control.{jobId}.pause.hard    — suspend immediately
polyglot.queue.control.{jobId}.kill.graceful — run [/] cleanup, then terminate
polyglot.queue.control.{jobId}.kill.hard     — terminate immediately
```

## Runner Acknowledgments (Runner → Queue Handler + Trigger Monitor)

```text
polyglot.runner.started.{jobId}              — job is now executing
polyglot.runner.completed.{jobId}            — job finished successfully
polyglot.runner.teardown_completed.{jobId}   — job finished [/] cleanup
polyglot.runner.failed.{jobId}               — job failed
polyglot.runner.paused.{jobId}               — job suspended (ACK)
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
