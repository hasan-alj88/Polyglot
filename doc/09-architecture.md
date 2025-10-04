# System Architecture

[← Back to README](../README.md)

## Table of Contents
- [Overview](#overview)
- [Microservice Architecture](#microservice-architecture)
- [Component Details](#component-details)
- [Data Flow](#data-flow)
- [Communication Protocols](#communication-protocols)
- [Deployment Architecture](#deployment-architecture)
- [Scalability](#scalability)

## Overview

Polyglot is built as a microservice architecture with three main components that work together to provide event-driven, resource-aware pipeline orchestration.

**Design Principles:**
- **Separation of Concerns**—Each service has a single, well-defined responsibility
- **Loose Coupling**—Services communicate via well-defined interfaces
- **Independent Scaling**—Each service can scale independently based on the load
- **Fault Isolation**—Failures in one service don't cascade to others
- **Observable**—Comprehensive metrics and logging at each layer

## Microservice Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        Polyglot System                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌──────────────┐      ┌──────────────┐      ┌──────────────┐   │
│  │   Trigger    │      │    Queue     │      │ Executioner  │   │
│  │   Monitor    │─────>│   Manager    │─────>│              │   │
│  │              │      │              │      │              │   │
│  └──────────────┘      └──────────────┘      └──────────────┘   │
│         │                     │                      │          │
│         │                     │                      │          │
│         v                     v                      v          │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │              Shared Storage & Metrics                    │   │
│  │  (Database, Queue State, Resource Metrics, Logs)         │   │
│  └──────────────────────────────────────────────────────────┘   │  
└─────────────────────────────────────────────────────────────────┘
```

## Component Details

### 1. Trigger Monitor

**Responsibility:** Detect conditions that should activate pipelines

**Sub-components:**

**File Watcher:**
- Uses OS-level file system events (inotify on Linux, FSEvents on macOS)
- Monitors patterns like `data/*.csv`
- Debouncing to prevent duplicate triggers
- Recursive directory watching

**Scheduler:**
- Cron-based scheduling
- Interval-based triggers
- One-time scheduled execution
- Time zone aware
- Persistent schedule storage for restart recovery

**REST API Server:**
- HTTP endpoints for webhook triggers
- Authentication and authorization
- Rate limiting per endpoint
- Request validation

**Message Queue Listener:**
- RabbitMQ consumer
- Kafka consumer
- Redis Streams consumer
- Dead letter queue handling

**CLI Monitor:**
- Command-line interface for manual triggers
- Interactive pipeline selection
- Argument parsing and validation

**Resource Watcher:**
- Periodic system metrics collection
- CPU, memory, disk, network, GPU monitoring
- Per-process and system-wide metrics
- Time-series database storage
- Configurable sampling rates
- Cached metrics API for other components

**Architecture:**
```
Trigger Monitor
├── Watchers (parallel threads)
│   ├── File System Watcher
│   ├── Scheduler Thread
│   ├── HTTP Server
│   ├── Message Queue Listeners
│   └── Resource Watcher
├── Trigger Aggregator
│   └── Checks all conditions for each pipeline
├── Event Queue (internal)
└── Queue Manager Notifier
    └── Sends activation events
```

**Data Storage:**
- Trigger configurations (database)
- Resource metrics history (time-series DB)
- Activation logs (logging system)

### 2. Queue Manager

**Responsibility:** Manage pipeline queueing, prioritization, and resource-based scheduling

**Sub-components:**

**Queue Registry:**
- Tracks all active queues
- Queue lifecycle management (create, pause, resume, delete)
- Multiple queue types (FIFO, LIFO, Priority, Deadline)
- Queue metadata storage

**Priority Manager:**
- Priority queue algorithms (heap-based)
- Dynamic priority adjustment
- Age-based priority boosting
- Importance weighting

**Admission Controller:**
- Resource availability checking (via Resource Watcher)
- Concurrency limit enforcement
- Rate limiting and throttling
- Backpressure detection and response

**Dead Letter Queue Handler:**
- Captures failed pipelines after retry exhaustion
- Configurable retry strategies (exponential, linear, fixed)
- Context preservation (inputs, errors, retry history)
- DLQ inspection and requesting tools

**Persistence Layer:**
- Durable queue storage (database)
- Replication for high availability
- Write-ahead logging
- Recovery on restart

**Monitoring & Metrics:**
- Real-time queue depth metrics
- Enqueue/dequeue rates
- Wait time distribution
- Processing time percentiles
- Alerting on threshold violations

**Routing Engine:**
- Pipeline routing based on properties
- Load balancing across executioner instances
- Affinity rules (keep related work together)
- Geographic routing

**Kill Condition Manager:**
- Monitors running pipelines continuously
- Checks CPU, memory, execution time against limits
- Kill strategies (Graceful, Immediate, CustomHandler, Degrade)
- Grace period handling
- Integration with error handlers for cleanup

**Architecture:**
```
Queue Manager
├── Queue Registry
│   ├── Queue Type Handlers
│   │   ├── FIFO Queue
│   │   ├── LIFO Queue
│   │   ├── Priority Queue
│   │   └── Deadline Queue
│   └── Queue State Storage
├── Admission Control
│   ├── Resource Checker (reads from Resource Watcher)
│   ├── Concurrency Limiter
│   └── Rate Limiter
├── Retry Logic
│   ├── Retry Strategy Executor
│   └── Dead Letter Queue
├── Kill Condition Manager
│   ├── Resource Monitor
│   ├── Kill Strategy Executor
│   └── Cleanup Coordinator
└── Dispatcher
    └── Executioner Selector
```

**Data Storage:**
- Queue state (database with replication)
- Pipeline context (for retries)
- Metrics history (time-series DB)
- Dead letter queue (persistent storage)

### 3. Executioner

**Responsibility:** Execute pipeline logic across multiple language runtimes

**Sub-components:**

**Language Runtimes:**
- Python 3.10, 3.11, 3.12 interpreters
- Node.js 18, 20 runtimes
- Rust compiler and FFI loader
- C++ compiler and dynamic library loader
- Process pools for reuse (avoid cold starts)

**Bridges:**
- **Standard Bindings:** PyBind11, Node child_process, FFI, etc.
- **Runtime Compilation:** On-the-fly code generation for type conversions
- **Type Converters:** Language-to-language data transformations

**Resource Lifecycle Manager:**
- Connection pools (database, network)
- File handle management
- Memory allocation tracking
- Cleanup on pipeline completion or failure

**Dependency Graph Analyzer:**
- Validates execution order
- Detects race conditions at compile-time
- Enforces data dependencies in parallel execution

**Performance Metrics Collector:**
- CPU time per pipeline
- Memory peak tracking
- Execution duration
- Type conversion overhead
- Per-node granular metrics

**Architecture:**
```
Executioner
├── Execution Orchestrator
│   ├── Sequential Executor ([\], [r], [/], [o])
│   ├── Parallel Executor ([f])
│   ├── Join Coordinator ([j], [Y])
│   └── Background Task Manager ([b])
├── Language Runtimes
│   ├── Python Pool
│   ├── Node.js Pool
│   ├── Rust Loader
│   └── C++ Loader
├── Type Conversion Engine
│   ├── Converters (py↔pg, rust↔pg, etc.)
│   └── Conversion Cache
├── Error Handler
│   ├── Error Type Matcher ([!] !>)
│   ├── Propagation Engine
│   └── Cleanup Coordinator
├── Switch Evaluator
│   ├── Equality Matcher ([?] ?>)
│   └── Boolean Pipeline Executor
└── Metrics Collector
    └── Performance Database Writer
```

**Data Storage:**
- Pipeline execution state (ephemeral, in-memory)
- Performance metrics (time-series DB)
- Error logs (logging system)
- Conversion cache (Redis or in-memory)

## Data Flow

### Pipeline Registration Flow

```
1. User writes .pg file
   ↓
2. Polyglot Compiler parses and validates
   ↓
3. Compiler generates:
   - Trigger configurations → Trigger Monitor
   - Queue configurations → Queue Manager
   - Execution plan → Executioner
   ↓
4. Services register configurations
   ↓
5. System ready to execute
```

### Pipeline Execution Flow

```
1. Trigger Monitor detects condition(s) met
   ↓
2. Sends activation event to Queue Manager
   ↓
3. Queue Manager evaluates queue conditions:
   - Check resource availability (via Resource Watcher)
   - Check concurrency limits
   - Apply priority and routing rules
   ↓
4a. If resources available:
    → Dispatch to Executioner immediately
    ↓
4b. If resources insufficient:
    → Enqueue and wait
    ↓ (when resources free up)
    → Dispatch to Executioner
   ↓
5. Executioner runs pipeline:
   - Setup phase [\]
   - Run phase [r] (with [f], [j], [b])
   - Cleanup phase [/]
   - Output phase [o]
   ↓
6. Kill Condition Manager monitors execution:
   - Check CPU/memory/time limits
   - Apply kill strategy if violated
   ↓
7. Performance Metrics Collector captures data
   ↓
8. Results returned, resources cleaned up
```

### Resource Monitoring Flow

```
Resource Watcher (continuous loop)
   ↓
1. Collect system metrics (CPU, memory, disk, network)
   ↓
2. Store in time-series database
   ↓
3. Update cached metrics (Redis/in-memory)
   ↓
Queue Manager & Kill Condition Manager read cached metrics
   ↓
Make decisions based on current resource state
```

## Communication Protocols

### Inter-Service Communication

**Trigger Monitor → Queue Manager:**
- **Protocol:** gRPC or HTTP/2
- **Message:** Pipeline activation event with trigger context
- **Async:** Non-blocking, fire-and-forget with acknowledgment

**Queue Manager → Executioner:**
- **Protocol:** gRPC for low latency
- **Message:** Pipeline execution request with full context
- **Sync:** Blocking call with timeout

**All Services → Resource Watcher:**
- **Protocol:** HTTP REST API or shared memory
- **Message:** Metrics query
- **Sync:** Quick read from cache

**All Services → Database:**
- **Protocol:** Database-specific (PostgresSQL wire protocol, etc.)
- **Usage:** Configuration, state persistence

**All Services → Time-Series DB:**
- **Protocol:** InfluxDB line protocol, Prometheus remote write, etc.
- **Usage:** Metrics storage and querying

### Message Formats

**Activation Event:**
```json
{
  "pipeline_id": "io.github.alice>ETL#ProcessData",
  "trigger_context": {
    "file_path": "/data/new_file.csv",
    "timestamp": "2025-01-15T10:30:00Z"
  },
  "inputs": {
    "file_path": "/data/new_file.csv"
  }
}
```

**Execution Request:**
```json
{
  "pipeline_id": "io.github.alice>ETL#ProcessData",
  "execution_plan": { /* AST */ },
  "inputs": { /* input values */ },
  "queue_context": {
    "priority": 1,
    "retry_count": 0,
    "enqueued_at": "2025-01-15T10:30:05Z"
  }
}
```

**Metrics Sample:**
```json
{
  "pipeline_id": "io.github.alice>ETL#ProcessData",
  "execution_id": "exec-12345",
  "metrics": {
    "cpu_time_ms": 1234,
    "memory_peak_mb": 512,
    "duration_ms": 5000,
    "conversion_overhead_ms": 45
  },
  "timestamp": "2025-01-15T10:30:10Z"
}
```

## Deployment Architecture

### Development Environment

```
Single machine:
├── Trigger Monitor (process)
├── Queue Manager (process)
├── Executioner (process)
├── PostgreSQL (Docker)
├── Redis (Docker)
└── InfluxDB (Docker)
```

### Production Environment

```
Kubernetes Cluster:
├── Trigger Monitor Deployment (3 replicas)
│   └── Service (LoadBalancer)
├── Queue Manager StatefulSet (3 replicas)
│   └── Service (ClusterIP)
├── Executioner Deployment (10+ replicas, auto-scaling)
│   └── Service (ClusterIP)
├── PostgreSQL StatefulSet (with replication)
├── Redis Cluster (6 nodes)
└── InfluxDB StatefulSet

External:
├── Prometheus (metrics collection)
├── Grafana (dashboards)
└── ELK/Loki (log aggregation)
```

### High Availability

- **Trigger Monitor:** Stateless, multiple replicas with leader election for schedulers
- **Queue Manager:** Stateful, replicated with distributed consensus (Raft/Paxos)
- **Executioner:** Stateless, horizontal scaling
- **Databases:** Replication with automatic failover

## Scalability

### Horizontal Scaling

**Trigger Monitor:**
- Add replicas for different trigger types
- Shard file watchers by directory
- Distribute message queue listeners

**Queue Manager:**
- Shard queues across instances
- Partition by pipeline namespace
- Use consistent hashing for routing

**Executioner:**
- Add replicas based on CPU/memory load
- Auto-scaling based on queue depth
- Affinity for language runtimes (Python-heavy workloads on Python-optimized nodes)

### Performance Targets

| Metric                         | Target                      |
|--------------------------------|-----------------------------|
| Pipeline activation latency    | < 100ms                     |
| Queue dispatch latency         | < 50ms                      |
| Execution startup overhead     | < 500ms                     |
| Type conversion overhead       | < 5% of execution time      |
| Resource monitoring overhead   | < 2% CPU                    |
| Throughput (simple pipelines)  | > 1,000/sec per Executioner |
| Throughput (complex pipelines) | > 100/sec per Executioner   |

### Bottleneck Mitigation

**Common Bottlenecks:**
1. Database writes (metrics, state)
2. Type conversions
3. Runtime cold starts
4. Queue contention

**Mitigation Strategies:**
1. Batch database writes, use time-series DB for metrics
2. Cache conversion results, use zero-copy where possible
3. Keep runtime process pools warm
4. Shard queues, use lock-free data structures

---

[Next: Execution Model →](10-execution-model.md)