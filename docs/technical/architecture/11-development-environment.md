## Development Environment

### Prerequisites

**Required:**
- Rust 1.84+ (2024 edition)
- PostgreSQL 14+
- InfluxDB 2.x
- Redis 7+
- SQLx CLI: `cargo install sqlx-cli`
- Python 3.11+ with uv: `pip install uv`

**Optional:**
- Docker & Docker Compose (for containerized development)
- rust-analyzer (LSP for IDE)

### Setup

```bash
# 1. Clone repository
git clone https://github.com/yourusername/polyglot.git
cd polyglot

# 2. Install Rust dependencies
cargo build

# 3. Set up PostgreSQL
createdb polyglot  # or use Docker
export DATABASE_URL="postgresql://localhost/polyglot"

# 4. Set up InfluxDB
influxd  # or Docker
# Create org, bucket, and token via UI (http://localhost:8086)
export INFLUX_URL="http://localhost:8086"
export INFLUX_TOKEN="your-token"

# 5. Set up Redis
redis-server  # or Docker

# 6. Run migrations
sqlx migrate run

# 7. Install Python with uv
pip install uv

# 8. Copy example config
cp polyglot.toml.example polyglot.toml
# Edit polyglot.toml with your database/InfluxDB/Redis URLs

# 9. Run tests
cargo test --workspace

# 10. Run services (in separate terminals)
cargo run --bin polyglot-trigger-monitor
cargo run --bin polyglot-queue-manager
cargo run --bin polyglot-runner

# 11. Use CLI
cargo run --bin polyglot-cli -- compile examples/hello_world.pg  # Validates + converts + registers
cargo run --bin polyglot-cli -- activate hello_world
cargo run --bin polyglot-cli -- trigger hello_world  # Test/debug: bypass trigger logic
cargo run --bin polyglot-cli -- status <instance_id>
cargo run --bin polyglot-cli -- logs <instance_id>
```

### Environment Variables

```bash
# Required
export DATABASE_URL="postgresql://localhost/polyglot"
export INFLUX_URL="http://localhost:8086"
export INFLUX_TOKEN="your-influxdb-token"
export INFLUX_ORG="polyglot"
export INFLUX_BUCKET="metrics"
export REDIS_URL="redis://localhost:6379"

# Optional
export POLYGLOT_CONFIG="./polyglot.toml"
export RUST_LOG="info,polyglot=debug"  # Logging level
export RUST_BACKTRACE=1  # Stack traces on panic
```

### IDE Configuration

**VSCode (.vscode/settings.json):**
```json
{
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.check.command": "clippy",
  "rust-analyzer.rustfmt.extraArgs": ["--edition", "2024"]
}
```

**IntelliJ IDEA / RustRover:**
- Enable Rust plugin
- Set Rust toolchain to 1.84+
- Enable Clippy lints

---

