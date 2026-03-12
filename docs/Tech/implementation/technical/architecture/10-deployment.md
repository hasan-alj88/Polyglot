## Deployment Architecture

### MVP Deployment (Local/Single-Node)

**All services on one machine:**
```
┌─────────────────────────────────────┐
│         Local Machine               │
│                                     │
│  ┌──────────────┐  ┌──────────────┐│
│  │ PostgreSQL   │  │    Redis     ││
│  └──────────────┘  └──────────────┘│
│                                     │
│  ┌──────────────────────────────┐  │
│  │  Polyglot Services           │  │
│  │  - Trigger Monitor           │  │
│  │  - Queue Manager             │  │
│  │  - Runner                    │  │
│  └──────────────────────────────┘  │
│                                     │
│  ┌──────────────────────────────┐  │
│  │  polyglot CLI                │  │
│  └──────────────────────────────┘  │
└─────────────────────────────────────┘
```

**Setup:**
```bash
# Install dependencies
brew install postgresql redis  # macOS
sudo apt install postgresql redis  # Linux

# Start services
brew services start postgresql redis  # macOS
systemctl start postgresql redis  # Linux

# Build Polyglot
cargo build --release

# Run migrations
sqlx migrate run

# Start services (3 terminals)
./target/release/polyglot-trigger-monitor
./target/release/polyglot-queue-manager
./target/release/polyglot-runner
```

### Production Deployment (Future: Docker + Orchestration)

**Docker Compose:**
```yaml
services:
  postgres:
    image: postgres:16
    environment:
      POSTGRES_DB: polyglot
      POSTGRES_USER: polyglot
      POSTGRES_PASSWORD: ${DB_PASSWORD}
    volumes:
      - postgres_data:/var/lib/postgresql/data

  redis:
    image: redis:7-alpine
    volumes:
      - redis_data:/data

  trigger-monitor:
    build: .
    command: /usr/local/bin/polyglot-trigger-monitor
    depends_on:
      - postgres
      - redis
    environment:
      DATABASE_URL: postgresql://polyglot:${DB_PASSWORD}@postgres/polyglot
      REDIS_URL: redis://redis:6379

  queue-manager:
    build: .
    command: /usr/local/bin/polyglot-queue-manager
    depends_on:
      - postgres
      - redis

  runner:
    build: .
    command: /usr/local/bin/polyglot-runner
    depends_on:
      - postgres
      - redis
```

### Scalability (Post-MVP)

**Horizontal Scaling:**
- Multiple Runner instances (stateless, can run in parallel)
- Queue Manager can be sharded by queue
- Trigger Monitor: Single instance (LISTEN/NOTIFY limitation), future: leader election

**Managed Services:**
- AWS RDS PostgreSQL
- AWS ElastiCache Redis
- Deploy services to ECS/EKS or Railway

---

