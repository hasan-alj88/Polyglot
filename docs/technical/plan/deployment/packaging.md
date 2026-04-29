---
audience: design
type: spec
updated: 2026-04-16
---

# Package Build Specification

<!-- @c:technical/plan/deployment/INDEX -->
<!-- @c:technical/plan/queue-manager/infrastructure -->

This document specifies how to produce the `aljam3` system package (.deb and .rpm) from source. The package bundles all Aljam3 binaries, vendor dependencies (Redis, NATS), Queue Handler Lua scripts, systemd unit files, and default configuration into a single installable artifact.

## Build Tool

Use **nfpm** to produce both .deb and .rpm from a single YAML configuration. nfpm eliminates the need for separate Debian packaging infrastructure (debhelper, dpkg-buildpackage) and RPM spec files.

```bash
# Install nfpm
go install github.com/goreleaser/nfpm/v2/cmd/nfpm@latest

# Build packages
nfpm package --packager deb --target dist/
nfpm package --packager rpm --target dist/
```

## Binary Sources

### Rust Binaries (build from source)

```bash
cargo build --release --bin aljam3
cargo build --release --bin aljam3-tm
cargo build --release --bin aljam3-runner
cargo build --release --bin aljam3-ctl
```

All four binaries compile from the same Cargo workspace. Target: `x86_64-unknown-linux-gnu` (primary), `aarch64-unknown-linux-gnu` (secondary).

### Vendor Binaries (download pinned versions)

```bash
# Redis — download static binary
curl -L https://github.com/redis/redis/archive/refs/tags/7.4.2.tar.gz | tar xz
cd redis-7.4.2 && make BUILD_TLS=yes && cp src/redis-server vendor/

# NATS — download official release binary
curl -L https://github.com/nats-io/nats-server/releases/download/v2.11.1/nats-server-v2.11.1-linux-amd64.tar.gz | tar xz
cp nats-server-v2.11.1-linux-amd64/nats-server vendor/
```

Pin exact versions in `vendor/VERSIONS`:

```text
redis=7.4.2
nats=2.11.1
```

### Queue Handler Lua Scripts

Hand-written Lua scripts in `lib/qh/`. Loaded into Redis at service startup by `aljam3-ctl setup`.

## Package Contents

```text
/opt/aljam3/
├── bin/
│   ├── aljam3                # Compiler CLI
│   ├── aljam3-tm             # Trigger Monitor daemon
│   ├── aljam3-runner         # Runner daemon
│   ├── aljam3-ctl            # Management CLI
│   ├── redis-server            # Bundled Redis (TLS-enabled build)
│   └── nats-server             # Bundled NATS JetStream
├── lib/
│   └── qh/                    # Queue Handler Lua scripts
│       ├── dispatch.lua
│       ├── constraints.lua
│       └── state.lua
├── etc/
│   ├── aljam3.conf           # Main configuration
│   ├── redis.conf              # Pre-configured for Aljam3
│   └── nats.conf               # Pre-configured for Aljam3
├── share/
│   └── certs/                  # Generated certificates (empty at install)
└── var/
    ├── redis/                  # Redis data directory
    ├── nats/                   # NATS JetStream storage
    └── contracts/              # Deployed Behavior Contracts
```

## nfpm Configuration

```yaml
name: aljam3
arch: amd64
version: "${VERSION}"
maintainer: "Aljam3 Team <team@aljam3.dev>"
description: "Aljam3 async programming language — compiler and runtime service"
vendor: "Aljam3 Project"
homepage: "https://aljam3.dev"
license: "Apache-2.0"

depends:
  - libc6 (>= 2.31)
  - libssl3
  - criu (>= 3.17)

recommends:
  - systemd

contents:
  # Rust binaries
  - src: target/release/aljam3
    dst: /opt/aljam3/bin/aljam3
    file_info:
      mode: 0755
  - src: target/release/aljam3-tm
    dst: /opt/aljam3/bin/aljam3-tm
    file_info:
      mode: 0755
  - src: target/release/aljam3-runner
    dst: /opt/aljam3/bin/aljam3-runner
    file_info:
      mode: 0755
  - src: target/release/aljam3-ctl
    dst: /opt/aljam3/bin/aljam3-ctl
    file_info:
      mode: 0755

  # Vendor binaries
  - src: vendor/redis-server
    dst: /opt/aljam3/bin/redis-server
    file_info:
      mode: 0755
  - src: vendor/nats-server
    dst: /opt/aljam3/bin/nats-server
    file_info:
      mode: 0755

  # QH Lua scripts
  - src: lib/qh/
    dst: /opt/aljam3/lib/qh/

  # Configuration
  - src: etc/aljam3.conf
    dst: /opt/aljam3/etc/aljam3.conf
    type: config|noreplace
  - src: etc/redis.conf
    dst: /opt/aljam3/etc/redis.conf
    type: config|noreplace
  - src: etc/nats.conf
    dst: /opt/aljam3/etc/nats.conf
    type: config|noreplace

  # systemd units
  - src: systemd/aljam3-redis.service
    dst: /etc/systemd/system/aljam3-redis.service
  - src: systemd/aljam3-nats.service
    dst: /etc/systemd/system/aljam3-nats.service
  - src: systemd/aljam3-tm.service
    dst: /etc/systemd/system/aljam3-tm.service
  - src: systemd/aljam3-runner.service
    dst: /etc/systemd/system/aljam3-runner.service

  # PATH symlink
  - src: target/release/aljam3
    dst: /usr/local/bin/aljam3
    type: symlink
  - src: target/release/aljam3-ctl
    dst: /usr/local/bin/aljam3-ctl
    type: symlink

  # Empty directories
  - dst: /opt/aljam3/share/certs/
    type: dir
    file_info:
      mode: 0700
      owner: aljam3
      group: aljam3
  - dst: /opt/aljam3/var/redis/
    type: dir
    file_info:
      owner: aljam3
      group: aljam3
  - dst: /opt/aljam3/var/nats/
    type: dir
    file_info:
      owner: aljam3
      group: aljam3
  - dst: /opt/aljam3/var/contracts/
    type: dir
    file_info:
      owner: aljam3
      group: aljam3

scripts:
  postinstall: scripts/postinstall.sh
  preremove: scripts/preremove.sh
```

## systemd Unit Files

### aljam3-redis.service

```ini
[Unit]
Description=Aljam3 Redis (bundled state store)
After=network.target
ConditionPathExists=/opt/aljam3/etc/redis.conf

[Service]
Type=notify
User=aljam3
Group=aljam3
ExecStart=/opt/aljam3/bin/redis-server /opt/aljam3/etc/redis.conf
ExecStop=/opt/aljam3/bin/redis-server shutdown
Restart=on-failure
RestartSec=5s
LimitNOFILE=65535

[Install]
WantedBy=multi-user.target
```

### aljam3-nats.service

```ini
[Unit]
Description=Aljam3 NATS JetStream (bundled messaging)
After=network.target
ConditionPathExists=/opt/aljam3/etc/nats.conf

[Service]
Type=exec
User=aljam3
Group=aljam3
ExecStart=/opt/aljam3/bin/nats-server -c /opt/aljam3/etc/nats.conf
ExecReload=/bin/kill -HUP $MAINPID
Restart=on-failure
RestartSec=5s

[Install]
WantedBy=multi-user.target
```

### aljam3-tm.service

```ini
[Unit]
Description=Aljam3 Trigger Monitor
After=aljam3-redis.service aljam3-nats.service
Requires=aljam3-redis.service aljam3-nats.service

[Service]
Type=exec
User=aljam3
Group=aljam3
ExecStart=/opt/aljam3/bin/aljam3-tm --config /opt/aljam3/etc/aljam3.conf
Restart=on-failure
RestartSec=5s
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
```

### aljam3-runner.service

```ini
[Unit]
Description=Aljam3 Runner
After=aljam3-nats.service
Requires=aljam3-nats.service

[Service]
Type=exec
User=aljam3
Group=aljam3
ExecStart=/opt/aljam3/bin/aljam3-runner --config /opt/aljam3/etc/aljam3.conf
Restart=on-failure
RestartSec=5s
AmbientCapabilities=CAP_SYS_ADMIN CAP_NET_ADMIN CAP_DAC_OVERRIDE
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
```

The Runner service uses `AmbientCapabilities` to grant kernel-level access without running as root. `CAP_SYS_ADMIN` enables PID namespaces and cgroup management. `CAP_NET_ADMIN` enables CRIU TCP connection checkpointing. `CAP_DAC_OVERRIDE` enables CRIU file descriptor access.

## postinstall Script

```bash
#!/bin/bash
set -e

# Create aljam3 system user (no login shell, no home)
if ! id -u aljam3 >/dev/null 2>&1; then
    useradd --system --no-create-home --shell /usr/sbin/nologin aljam3
fi

# Set ownership on data directories
chown -R aljam3:aljam3 /opt/aljam3/var/
chown -R aljam3:aljam3 /opt/aljam3/share/certs/

# Reload systemd
systemctl daemon-reload

# Load QH Lua scripts into Redis (if Redis is running)
if systemctl is-active --quiet aljam3-redis; then
    /opt/aljam3/bin/aljam3-ctl load-qh-scripts
fi

echo ""
echo "Aljam3 installed to /opt/aljam3/"
echo "Run 'aljam3-ctl setup' to configure and start services."
echo ""
```

## preremove Script

```bash
#!/bin/bash
set -e

# Stop and disable all services
for svc in aljam3-runner aljam3-tm aljam3-nats aljam3-redis; do
    systemctl stop "$svc" 2>/dev/null || true
    systemctl disable "$svc" 2>/dev/null || true
done

systemctl daemon-reload

echo ""
echo "Aljam3 services stopped."
echo "Data preserved in /opt/aljam3/var/ — remove manually if no longer needed."
echo ""
```

## APT Repository Hosting

### Repository Structure

```text
apt-repo/
├── pool/
│   └── main/
│       └── p/
│           └── aljam3/
│               └── aljam3_0.1.0_amd64.deb
├── dists/
│   └── stable/
│       ├── Release
│       ├── Release.gpg
│       ├── InRelease
│       └── main/
│           └── binary-amd64/
│               ├── Packages
│               └── Packages.gz
└── aljam3.gpg              # Public GPG key
```

### reprepro Setup

```bash
# Initialize repo
mkdir -p apt-repo/conf
cat > apt-repo/conf/distributions <<'EOF'
Origin: Aljam3
Label: Aljam3
Suite: stable
Codename: stable
Architectures: amd64 arm64
Components: main
SignWith: <GPG-KEY-ID>
EOF

# Add package
reprepro -b apt-repo includedeb stable dist/aljam3_0.1.0_amd64.deb
```

### GPG Key Management

```bash
# Generate signing key (one-time)
gpg --batch --gen-key <<'EOF'
Key-Type: RSA
Key-Length: 4096
Name-Real: Aljam3 Package Signing
Name-Email: packages@aljam3.dev
Expire-Date: 0
%no-protection
EOF

# Export public key for users
gpg --export --armor packages@aljam3.dev > apt-repo/aljam3.gpg
```

### CI/CD Integration

The build pipeline produces packages and publishes to the APT repository on every tagged release:

1. `cargo build --release` — compile all 4 Rust binaries
2. Download pinned vendor binaries (Redis, NATS) from cache or upstream
3. `nfpm package --packager deb` — produce .deb
4. `nfpm package --packager rpm` — produce .rpm
5. `reprepro includedeb stable dist/*.deb` — add to APT repo
6. Upload repo to static hosting (S3, GitHub Pages, or dedicated server)
7. Publish .rpm to RPM repo (or as GitHub Release asset)

## Version Strategy

Packages follow semantic versioning (`MAJOR.MINOR.PATCH`):

| Change Type | Version Bump | Example |
|-------------|-------------|---------|
| Breaking Behavior Contract format change | MAJOR | 1.0.0 → 2.0.0 |
| New feature, backward-compatible | MINOR | 1.0.0 → 1.1.0 |
| Bug fix, no behavior change | PATCH | 1.0.0 → 1.0.1 |
| Vendor dependency update (Redis, NATS) | MINOR or PATCH | Depends on vendor change scope |

---

See also: [[installation]], [[lifecycle]], [[infrastructure]]
