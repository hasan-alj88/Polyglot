---
audience: integrator
type: guide
updated: 2026-04-16
---

# Installing the Polyglot Service

<!-- @c:technical/plan/deployment/INDEX -->

This guide covers how to install, configure, and verify the Polyglot Service on Linux. The Polyglot Service is the runtime that executes pipelines compiled from .pg files.

## Prerequisites

### Supported Distributions

| Distribution | Version | Status |
|-------------|---------|--------|
| Ubuntu | 22.04+ (Jammy) | Primary |
| Debian | 12+ (Bookworm) | Primary |
| Fedora | 38+ | Secondary (RPM) |
| RHEL / Rocky / Alma | 9+ | Secondary (RPM) |

### Kernel Requirements

The Runner creates isolated process environments for each job. This requires:

| Requirement | Minimum | How to Check |
|-------------|---------|-------------|
| Linux kernel | 5.10+ | `uname -r` |
| cgroup v2 | Enabled (unified hierarchy) | `mount \| grep cgroup2` |
| PID namespaces | Enabled (default on all modern kernels) | `cat /proc/sys/kernel/unprivileged_userns_clone` returns 1 |
| CRIU | 3.17+ | `criu --version` (installed as package dependency) |

cgroup v2 is the default on Ubuntu 22.04+, Debian 12+, and Fedora 38+. Older distributions using cgroup v1 are not supported.

## Install from APT (Debian/Ubuntu)

```bash
# Add the Polyglot GPG key
curl -fsSL https://polyglot.dev/apt/polyglot.gpg | sudo gpg --dearmor -o /etc/apt/keyrings/polyglot.gpg

# Add the repository
echo "deb [signed-by=/etc/apt/keyrings/polyglot.gpg] https://polyglot.dev/apt stable main" \
  | sudo tee /etc/apt/sources.list.d/polyglot.list

# Install
sudo apt update
sudo apt install polyglot
```

This installs all components: compiler, Trigger Monitor, Runner, management CLI, and bundled Redis + NATS.

## Install from RPM (Fedora/RHEL)

```bash
# Add the repository
sudo dnf config-manager --add-repo https://polyglot.dev/rpm/polyglot.repo

# Install
sudo dnf install polyglot
```

## Install via Script (Other Distributions)

```bash
curl -sSL https://polyglot.dev/install | sudo sh
```

The script detects your distribution, downloads the appropriate package, installs to `/opt/polyglot/`, and creates systemd units.

## Setup

After installation, run `polyglot-ctl setup` to configure and start services. There are two modes.

### All-in-One Mode (Single Machine)

Starts all four services on the local machine. Use for development or small deployments.

```bash
sudo polyglot-ctl setup
```

This command:
1. Generates a local CA and self-signed certificates in `/opt/polyglot/share/certs/`
2. Configures Redis to listen on localhost only
3. Configures NATS to listen on localhost only
4. Loads Queue Handler Lua scripts into Redis
5. Starts and enables all four systemd services
6. Verifies all services are healthy

### Distributed Mode (Multi-Host)

For production deployments with separate infrastructure and worker hosts.

**On the infrastructure host** (runs Redis, NATS, Trigger Monitor):

```bash
sudo polyglot-ctl setup --role infra
```

This starts Redis, NATS, and the Trigger Monitor. It generates a CA certificate and creates a join token for worker hosts.

**Add worker hosts:**

```bash
# On the infra host — generate credentials for a worker
sudo polyglot-ctl add-host worker-01
# Outputs: Join token and instructions
```

**On each worker host** (runs Runner only):

```bash
sudo polyglot-ctl join --token <TOKEN> --infra https://infra-host:9443
```

This downloads TLS certificates from the infra host, configures the Runner to connect to the remote Redis and NATS, and starts the Runner service.

## Configuration

The main configuration file is `/opt/polyglot/etc/polyglot.conf`:

```toml
# Role: "all" (default), "infra", or "worker"
role = "all"

[redis]
# "bundled" uses the included redis-server
# "external" connects to your own Redis instance
mode = "bundled"
# external_url = "rediss://my-redis:6379"    # TLS connection

[nats]
# "bundled" uses the included nats-server
# "external" connects to your own NATS cluster
mode = "bundled"
# external_url = "tls://my-nats:4222"

[tls]
ca_cert   = "/opt/polyglot/share/certs/ca.pem"
cert_file = "/opt/polyglot/share/certs/server.pem"
key_file  = "/opt/polyglot/share/certs/server-key.pem"

[runner]
# Maximum concurrent jobs on this host
max_concurrent = 0    # 0 = auto (based on CPU cores)
```

### Using External Redis or NATS

To use your own Redis or NATS instances instead of the bundled ones:

1. Edit `/opt/polyglot/etc/polyglot.conf` — set `mode = "external"` and provide the connection URL
2. Disable the bundled service: `sudo systemctl disable --now polyglot-redis` (or `polyglot-nats`)
3. Load QH Lua scripts into your external Redis: `sudo polyglot-ctl load-qh-scripts --redis-url <URL>`

External instances must meet these requirements:

| Service | Minimum Version | Required Config |
|---------|----------------|-----------------|
| Redis / Valkey | 7.0+ | TLS enabled, Lua scripting enabled (default) |
| NATS | 2.10+ | JetStream enabled, TLS configured |

## TLS Certificate Management

All inter-service communication uses mTLS (mutual TLS). Certificates are managed by `polyglot-ctl`.

```bash
# Initialize a new CA (done automatically by setup)
sudo polyglot-ctl certs init

# Generate certificate for a new host
sudo polyglot-ctl certs add-host worker-02

# List all certificates
sudo polyglot-ctl certs list

# Rotate certificates (generates new certs, graceful reload)
sudo polyglot-ctl certs rotate
```

In all-in-one mode, certificates are self-signed and generated automatically. In distributed mode, the infra host acts as the CA and issues certificates to workers during `join`.

## Verify Installation

```bash
# Check all service statuses
polyglot-ctl status
```

Expected output (all-in-one mode):

```text
Polyglot Service Status
═══════════════════════════════════════
  Redis          ● running  (bundled, localhost:6379)
  NATS           ● running  (bundled, localhost:4222)
  Trigger Monitor● running  (pid 1234)
  Runner         ● running  (pid 5678, 0 active jobs)
  QH Scripts     ● loaded   (3 scripts)
  TLS            ● active   (self-signed, expires 2027-04-16)
═══════════════════════════════════════
  Role: all-in-one
  Version: 0.1.0
```

### Test with a Sample Pipeline

```bash
# Compile a .pg file
polyglot compile examples/hello.pg --output /opt/polyglot/var/contracts/hello.json

# Deploy the contract
polyglot-ctl deploy hello.json

# Check job status
polyglot-ctl jobs
```

## Firewall Rules

For distributed deployments, worker hosts need access to the infra host:

| Port | Service | Direction |
|------|---------|-----------|
| 6379 | Redis (TLS) | Worker → Infra |
| 4222 | NATS (TLS) | Worker → Infra |
| 9443 | polyglot-ctl join API | Worker → Infra (setup only) |

All connections use TLS. No plaintext ports are exposed.

## Uninstall

```bash
# Debian/Ubuntu
sudo apt remove polyglot

# Fedora/RHEL
sudo dnf remove polyglot
```

Uninstall stops all services and removes binaries, systemd units, and configuration. Data directories (`/opt/polyglot/var/`) are preserved — remove manually if no longer needed:

```bash
sudo rm -rf /opt/polyglot/var/
sudo userdel polyglot
```

---

See also: [[packaging]], [[lifecycle]]
