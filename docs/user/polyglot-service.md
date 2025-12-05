# Polyglot Service

The Polyglot Service is the runtime environment that executes pipelines. It consists of three coordinated services that handle triggers, queue management, and execution.

## Three-Service Architecture

Polyglot runs as three independent but coordinated services:

```
┌──────────────────────────────┐
│   TRIGGER MONITOR            │
│   Watches for events:        │
│   - Time-based (cron        │
│   - File system events       │
│   - HTTP webhooks            │
│   - Message queue events     │
└──────────┬───────────────────┘
           │ Enqueues job
           ▼
┌──────────────────────────────┐
│   QUEUE MANAGER              │
│   Manages execution queue:   │
│   - Priority handling        │
│   - Retry logic              │
│   - Concurrency limits       │
│   - Job persistence          │
└──────────┬───────────────────┘
           │ Dispatches to worker
           ▼
┌──────────────────────────────┐
│   RUNNER SERVICE             │
│   Executes pipelines:        │
│   - Worker pool              │
│   - FFI integration          │
│   - State tracking           │
│   - Result storage           │
└──────────────────────────────┘
```

## Trigger Monitor

The **Trigger Monitor** watches for events that should start pipeline execution.

### Supported Trigger Types

#### Time-Based Triggers

```polyglot
// Daily at specific time
[t] .daily: #T.Daily(.hour: 9, .minute: 0

// Every hour
[t] .hourly: #T.Hourly(.minute: 30

// Weekly
[t] .weekly: #T.Weekly(
    .day: #DayOfWeek.Monday,
    .hour: 9,
    .minute: 0


// Cron expression
[t] .cron: #T.Cron(.expression: "0 */6 * * *"
```

#### File System Triggers

```polyglot
// File created
[t] .on_upload: #T.FileCreated(.path: "/uploads/*.csv"

// File modified
[t] .on_change: #T.FileModified(.path: "/config/app.yaml"

// File deleted
[t] .on_delete: #T.FileDeleted(.path: "/temp/*"
```

#### HTTP Triggers

```polyglot
// HTTP endpoint
[t] .webhook: #T.HTTP(
    .port: 8080,
    .path: "/webhook/process"


// POST to http://localhost:8080/webhook/process triggers pipeline
```

#### Message Queue Triggers

```polyglot
// Subscribe to queue topic
[t] .queue: #T.Queue(.topic: "events.user.created"

// Redis Streams
[t] .redis: #T.RedisStream(
    .url: "redis://localhost:6379",
    .stream: "events"


// RabbitMQ
[t] .rabbitmq: #T.RabbitMQ(
    .url: "amqp://localhost",
    .queue: "tasks"

```

### Trigger Monitor Configuration

Located at `/etc/polyglot/trigger-monitor.yaml`:

```yaml
trigger_monitor:
  # Polling intervals
  time_precision: 1s          # Check time triggers every second
  file_poll_interval: 100ms   # Check file system every 100ms

  # File watching
  file_watcher:
    use_inotify: true         # Use inotify on Linux (more efficient
    max_watches: 10000        # Maximum file watches

  # HTTP server
  http_server:
    bind: "0.0.0.0:8080"
    timeout: 30s
    max_body_size: 10MB

  # Queue connections
  message_queues:
    redis:
      urls:
        - "redis://localhost:6379"
    rabbitmq:
      urls:
        - "amqp://localhost"
```

## Queue Manager

The **Queue Manager** manages the execution queue, handling priorities, retries, and concurrency.

### Queue Features

**1. Priority Handling**

```polyglot
[|Pipeline] .urgent_task
[Q] .priority: 10  // Higher priority (default: 5
[Q] .max_retries: 3
```

**2. Concurrency Limits**

```polyglot
[|Pipeline] .resource_intensive
[Q] .concurrency: 2  // Max 2 concurrent executions
```

**3. Retry Logic**

```polyglot
[|Pipeline] .with_retries
[Q] .max_retries: 5
[Q] .retry_delay: 30s
[Q] .backoff_multiplier: 2.0  // Exponential backoff
```

**4. Timeouts**

```polyglot
[|Pipeline] .long_running
[Q] .timeout: 1h  // Fail if exceeds 1 hour
```

### Queue Manager Configuration

Located at `/etc/polyglot/queue-manager.yaml`:

```yaml
queue_manager:
  # Queue backend (sqlite, postgres, redis
  backend: postgres
  connection: "postgresql://localhost/polyglot_queue"

  # Worker pool
  max_workers: 10
  worker_idle_timeout: 5m

  # Retry configuration
  default_max_retries: 3
  default_retry_delay: 10s
  max_retry_delay: 1h

  # Priorities
  priority_levels: 10

  # Persistence
  job_retention: 30d          # Keep completed jobs for 30 days
  failed_job_retention: 90d   # Keep failed jobs for 90 days
```

### Queue CLI Commands

```bash
# View queue status
polyglot queue status

# List pending jobs
polyglot queue list

# List failed jobs
polyglot queue failed

# Retry specific job
polyglot queue retry <job-id>

# Retry all failed jobs for pipeline
polyglot queue retry-all <pipeline-name>

# Cancel job
polyglot queue cancel <job-id>

# Purge completed jobs older than 30 days
polyglot queue purge --older-than 30d

# View job details
polyglot queue info <job-id>
```

## Runner Service

The **Runner Service** executes pipeline code using a pool of workers.

### Worker Pool

The runner maintains a pool of workers that execute pipelines:

```
┌────────────────────┐
│  Runner Service    │
│                    │
│  ┌──────────────┐  │
│  │  Worker 1    │  │ ← Executing pipeline A
│  ├──────────────┤  │
│  │  Worker 2    │  │ ← Executing pipeline B
│  ├──────────────┤  │
│  │  Worker 3    │  │ ← Idle
│  ├──────────────┤  │
│  │  Worker 4    │  │ ← Idle
│  └──────────────┘  │
└────────────────────┘
```

### FFI Integration

Each worker can call foreign functions in different languages:

```
Worker → Python Interpreter (via PyO3
      → Rust Functions (native calls
      → Go Runtime (via CGO
      → Node.js (via N-API
      → Julia (via julia-sys
```

### Runner Configuration

Located at `/etc/polyglot/runner.yaml`:

```yaml
runner:
  # Worker configuration
  num_workers: 4
  worker_stack_size: 8MB

  # Language runtimes
  python:
    interpreter: "/usr/bin/python3"
    virtualenv: "/opt/polyglot/venv"
    sys_path_additions:
      - "/opt/polyglot/python-libs"

  rust:
    library_paths:
      - "/opt/polyglot/rust-libs"

  go:
    go_path: "/opt/polyglot/go"

  nodejs:
    node_path: "/usr/bin/node"
    npm_modules: "/opt/polyglot/node_modules"

  julia:
    julia_home: "/opt/julia"
    depot_path: "/opt/polyglot/julia-depot"

  # Execution
  max_execution_time: 1h
  memory_limit: 2GB

  # State tracking
  state_database: "postgresql://localhost/polyglot_state"
```

## Installation

### Docker Deployment (Recommended

**Single Container (Development:**

```bash
docker run -d \
  --name polyglot-service \
  -p 8080:8080 \
  -v /var/lib/polyglot:/data \
  -v /var/log/polyglot:/logs \
  polyglot/service:latest
```

**Docker Compose (Production:**

```yaml
version: '3.8'

services:
  trigger-monitor:
    image: polyglot/trigger-monitor:latest
    ports:
      - "8080:8080"
    volumes:
      - /var/lib/polyglot:/data
      - ./config/trigger-monitor.yaml:/etc/polyglot/trigger-monitor.yaml
    environment:
      - POLYGLOT_QUEUE_URL=postgresql://queue-db/polyglot

  queue-manager:
    image: polyglot/queue-manager:latest
    volumes:
      - /var/lib/polyglot:/data
      - ./config/queue-manager.yaml:/etc/polyglot/queue-manager.yaml
    depends_on:
      - queue-db
    environment:
      - POLYGLOT_QUEUE_URL=postgresql://queue-db/polyglot

  runner:
    image: polyglot/runner:latest
    deploy:
      replicas: 4  # 4 runner instances
    volumes:
      - /var/lib/polyglot:/data
      - ./config/runner.yaml:/etc/polyglot/runner.yaml
    depends_on:
      - queue-manager
    environment:
      - POLYGLOT_STATE_URL=postgresql://state-db/polyglot

  queue-db:
    image: postgres:15
    volumes:
      - queue-data:/var/lib/postgresql/data
    environment:
      - POSTGRES_DB=polyglot
      - POSTGRES_PASSWORD=secure_password

  state-db:
    image: postgres:15
    volumes:
      - state-data:/var/lib/postgresql/data
    environment:
      - POSTGRES_DB=polyglot
      - POSTGRES_PASSWORD=secure_password

volumes:
  queue-data:
  state-data:
```

### Manual Installation

**1. Build from source:**

```bash
git clone https://github.com/polyglot-lang/polyglot.git
cd polyglot

# Build all services
cargo build --release

# Install binaries
sudo cp target/release/polyglot-trigger-monitor /usr/local/bin/
sudo cp target/release/polyglot-queue-manager /usr/local/bin/
sudo cp target/release/polyglot-runner /usr/local/bin/
```

**2. Create configuration directory:**

```bash
sudo mkdir -p /etc/polyglot
sudo cp config/*.yaml /etc/polyglot/
```

**3. Create systemd services:**

`/etc/systemd/system/polyglot-trigger-monitor.service`:

```ini
[Unit]
Description=Polyglot Trigger Monitor
After=network.target

[Service]
Type=simple
User=polyglot
ExecStart=/usr/local/bin/polyglot-trigger-monitor
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

Similar files for `polyglot-queue-manager.service` and `polyglot-runner.service`.

**4. Enable and start services:**

```bash
sudo systemctl enable polyglot-trigger-monitor
sudo systemctl enable polyglot-queue-manager
sudo systemctl enable polyglot-runner

sudo systemctl start polyglot-trigger-monitor
sudo systemctl start polyglot-queue-manager
sudo systemctl start polyglot-runner
```

## Service Management

### Starting/Stopping Services

**All services:**

```bash
# Using polyglot-service wrapper
polyglot-service start
polyglot-service stop
polyglot-service restart
polyglot-service status
```

**Individual services:**

```bash
# systemd
sudo systemctl start polyglot-trigger-monitor
sudo systemctl stop polyglot-queue-manager
sudo systemctl restart polyglot-runner

# Docker
docker start polyglot-service
docker stop polyglot-service
docker restart polyglot-service
```

### Health Monitoring

**CLI:**

```bash
# Overall service status
polyglot status

# Individual service health
polyglot status --service trigger-monitor
polyglot status --service queue-manager
polyglot status --service runner
```

**HTTP Health Endpoints:**

```bash
# Trigger Monitor
curl http://localhost:8080/health

# Queue Manager
curl http://localhost:8081/health

# Runner
curl http://localhost:8082/health
```

**Response:**

```json
{
  "status": "healthy",
  "service": "trigger-monitor",
  "uptime": "5d 3h 22m",
  "active_triggers": 15,
  "events_processed": 12453

```

### Logging

**Service logs:**

```bash
# All services
polyglot logs --service

# Specific service
polyglot logs --service trigger-monitor
polyglot logs --service queue-manager
polyglot logs --service runner

# Follow logs
polyglot logs --service --follow

# Filter by level
polyglot logs --service --level error
```

**Pipeline execution logs:**

```bash
# Specific pipeline
polyglot logs my_pipeline

# Failed executions only
polyglot logs my_pipeline --failed

# Specific execution
polyglot logs my_pipeline --execution <id>

# JSON output
polyglot logs my_pipeline --format json
```

## CLI Commands

### Service Status

```bash
polyglot status                    # Overall status
polyglot status --verbose          # Detailed status
polyglot status --service runner   # Specific service
```

### Pipeline Management

```bash
# Register pipeline with service
polyglot register workflow.pg pipeline_name

# List registered pipelines
polyglot list

# Show pipeline details
polyglot info pipeline_name

# Activate triggers
polyglot activate pipeline_name

# Deactivate triggers
polyglot deactivate pipeline_name

# Unregister pipeline
polyglot unregister pipeline_name
```

### Execution

```bash
# Run pipeline manually
polyglot run workflow.pg pipeline_name --input '{...'

# Schedule one-time execution
polyglot schedule pipeline_name --at "2025-12-03 15:00"

# List running executions
polyglot executions --running

# Cancel execution
polyglot cancel <execution-id>
```

### Queue Management

```bash
# Queue status
polyglot queue status

# List jobs
polyglot queue list
polyglot queue list --pending
polyglot queue list --running
polyglot queue list --failed

# Job details
polyglot queue info <job-id>

# Retry job
polyglot queue retry <job-id>
polyglot queue retry-all pipeline_name

# Cancel job
polyglot queue cancel <job-id>

# Purge old jobs
polyglot queue purge --older-than 30d
```

## Performance Tuning

### Worker Pool Sizing

```yaml
# runner.yaml
num_workers: 8  # 2x CPU cores for I/O-bound workloads
```

**Guidelines:**
- **CPU-bound:** `num_workers = CPU cores`
- **I/O-bound:** `num_workers = 2-4 × CPU cores`
- **Mixed:** `num_workers = 1.5 × CPU cores`

### Database Configuration

**PostgreSQL (recommended for production:**

```yaml
# queue-manager.yaml
backend: postgres
connection: "postgresql://localhost/polyglot?pool_size=20"
```

**SQLite (good for development:**

```yaml
backend: sqlite
connection: "/var/lib/polyglot/queue.db"
```

**Redis (best for ephemeral queues:**

```yaml
backend: redis
connection: "redis://localhost:6379/0"
```

### Memory Limits

```yaml
# runner.yaml
worker_stack_size: 8MB
memory_limit: 2GB  # Per worker
```

## Troubleshooting

### Service Won't Start

```bash
# Check logs
journalctl -u polyglot-trigger-monitor -n 50

# Check configuration
polyglot-trigger-monitor --validate-config

# Check port conflicts
sudo lsof -i :8080
```

### High Memory Usage

```bash
# Check running executions
polyglot executions --running

# Reduce worker pool size
# Edit /etc/polyglot/runner.yaml
num_workers: 2  # Reduce from 4

# Restart runner
sudo systemctl restart polyglot-runner
```

### Jobs Not Processing

```bash
# Check queue status
polyglot queue status

# Check runner health
polyglot status --service runner

# Check worker logs
polyglot logs --service runner --level error

# Restart queue manager
sudo systemctl restart polyglot-queue-manager
```

### Triggers Not Firing

```bash
# List active triggers
polyglot list --triggers

# Check trigger monitor logs
polyglot logs --service trigger-monitor

# Verify trigger is activated
polyglot info pipeline_name

# Restart trigger monitor
sudo systemctl restart polyglot-trigger-monitor
```

## Next Steps

- **Write Pipelines:** See [Getting Started](getting-started.md
- **Configure Triggers:** Learn [Trigger Types](syntax/block-markers.md#triggers
- **Monitor Performance:** Read [Advanced Topics](advanced/parallel-execution.md
- **Scale Deployment:** Study [Architecture](../technical/architecture.md
