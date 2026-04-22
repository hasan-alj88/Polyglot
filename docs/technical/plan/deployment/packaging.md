---
audience: design
type: spec
updated: 2026-04-16
---

# Package Build Specification

<!-- @c:technical/plan/deployment/INDEX -->
<!-- @c:technical/plan/queue-manager/infrastructure -->

This document specifies how to produce the `polyglot` system package (.deb and .rpm) from source. The package bundles all Polyglot binaries, vendor dependencies (Redis, NATS), Queue Handler Lua scripts, systemd unit files, and default configuration into a single installable artifact.

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
cargo build --release --bin polyglot
cargo build --release --bin polyglot-tm
cargo build --release --bin polyglot-runner
cargo build --release --bin polyglot-ctl
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

Hand-written Lua scripts in `lib/qh/`. Loaded into Redis at service startup by `polyglot-ctl setup`.

## Package Contents

```text
/opt/polyglot/
├── bin/
│   ├── polyglot                # Compiler CLI
│   ├── polyglot-tm             # Trigger Monitor daemon
│   ├── polyglot-runner         # Runner daemon
│   ├── polyglot-ctl            # Management CLI
│   ├── redis-server            # Bundled Redis (TLS-enabled build)
│   └── nats-server             # Bundled NATS JetStream
├── lib/
│   └── qh/                    # Queue Handler Lua scripts
│       ├── dispatch.lua
│       ├── constraints.lua
│       └── state.lua
├── etc/
│   ├── polyglot.conf           # Main configuration
│   ├── redis.conf              # Pre-configured for Polyglot
│   └── nats.conf               # Pre-configured for Polyglot
├── share/
│   └── certs/                  # Generated certificates (empty at install)
└── var/
    ├── redis/                  # Redis data directory
    ├── nats/                   # NATS JetStream storage
    └── contracts/              # Deployed Behavior Contracts
```

## nfpm Configuration

```yaml
name: polyglot
arch: amd64
version: "${VERSION}"
maintainer: "Polyglot Team <team@polyglot.dev>"
description: "Polyglot async programming language — compiler and runtime service"
vendor: "Polyglot Project"
homepage: "https://polyglot.dev"
license: "Apache-2.0"

depends:
  - libc6 (>= 2.31)
  - libssl3
  - criu (>= 3.17)

recommends:
  - systemd

contents:
  # Rust binaries
  - src: target/release/polyglot
    dst: /opt/polyglot/bin/polyglot
    file_info:
      mode: 0755
  - src: target/release/polyglot-tm
    dst: /opt/polyglot/bin/polyglot-tm
    file_info:
      mode: 0755
  - src: target/release/polyglot-runner
    dst: /opt/polyglot/bin/polyglot-runner
    file_info:
      mode: 0755
  - src: target/release/polyglot-ctl
    dst: /opt/polyglot/bin/polyglot-ctl
    file_info:
      mode: 0755

  # Vendor binaries
  - src: vendor/redis-server
    dst: /opt/polyglot/bin/redis-server
    file_info:
      mode: 0755
  - src: vendor/nats-server
    dst: /opt/polyglot/bin/nats-server
    file_info:
      mode: 0755

  # QH Lua scripts
  - src: lib/qh/
    dst: /opt/polyglot/lib/qh/

  # Configuration
  - src: etc/polyglot.conf
    dst: /opt/polyglot/etc/polyglot.conf
    type: config|noreplace
  - src: etc/redis.conf
    dst: /opt/polyglot/etc/redis.conf
    type: config|noreplace
  - src: etc/nats.conf
    dst: /opt/polyglot/etc/nats.conf
    type: config|noreplace

  # systemd units
  - src: systemd/polyglot-redis.service
    dst: /etc/systemd/system/polyglot-redis.service
  - src: systemd/polyglot-nats.service
    dst: /etc/systemd/system/polyglot-nats.service
  - src: systemd/polyglot-tm.service
    dst: /etc/systemd/system/polyglot-tm.service
  - src: systemd/polyglot-runner.service
    dst: /etc/systemd/system/polyglot-runner.service

  # PATH symlink
  - src: target/release/polyglot
    dst: /usr/local/bin/polyglot
    type: symlink
  - src: target/release/polyglot-ctl
    dst: /usr/local/bin/polyglot-ctl
    type: symlink

  # Empty directories
  - dst: /opt/polyglot/share/certs/
    type: dir
    file_info:
      mode: 0700
      owner: polyglot
      group: polyglot
  - dst: /opt/polyglot/var/redis/
    type: dir
    file_info:
      owner: polyglot
      group: polyglot
  - dst: /opt/polyglot/var/nats/
    type: dir
    file_info:
      owner: polyglot
      group: polyglot
  - dst: /opt/polyglot/var/contracts/
    type: dir
    file_info:
      owner: polyglot
      group: polyglot

scripts:
  postinstall: scripts/postinstall.sh
  preremove: scripts/preremove.sh
```

## systemd Unit Files

### polyglot-redis.service

```ini
[Unit]
Description=Polyglot Redis (bundled state store)
After=network.target
ConditionPathExists=/opt/polyglot/etc/redis.conf

[Service]
Type=notify
User=polyglot
Group=polyglot
ExecStart=/opt/polyglot/bin/redis-server /opt/polyglot/etc/redis.conf
ExecStop=/opt/polyglot/bin/redis-server shutdown
Restart=on-failure
RestartSec=5s
LimitNOFILE=65535

[Install]
WantedBy=multi-user.target
```

### polyglot-nats.service

```ini
[Unit]
Description=Polyglot NATS JetStream (bundled messaging)
After=network.target
ConditionPathExists=/opt/polyglot/etc/nats.conf

[Service]
Type=exec
User=polyglot
Group=polyglot
ExecStart=/opt/polyglot/bin/nats-server -c /opt/polyglot/etc/nats.conf
ExecReload=/bin/kill -HUP $MAINPID
Restart=on-failure
RestartSec=5s

[Install]
WantedBy=multi-user.target
```

### polyglot-tm.service

```ini
[Unit]
Description=Polyglot Trigger Monitor
After=polyglot-redis.service polyglot-nats.service
Requires=polyglot-redis.service polyglot-nats.service

[Service]
Type=exec
User=polyglot
Group=polyglot
ExecStart=/opt/polyglot/bin/polyglot-tm --config /opt/polyglot/etc/polyglot.conf
Restart=on-failure
RestartSec=5s
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
```

### polyglot-runner.service

```ini
[Unit]
Description=Polyglot Runner
After=polyglot-nats.service
Requires=polyglot-nats.service

[Service]
Type=exec
User=polyglot
Group=polyglot
ExecStart=/opt/polyglot/bin/polyglot-runner --config /opt/polyglot/etc/polyglot.conf
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

# Create polyglot system user (no login shell, no home)
if ! id -u polyglot >/dev/null 2>&1; then
    useradd --system --no-create-home --shell /usr/sbin/nologin polyglot
fi

# Set ownership on data directories
chown -R polyglot:polyglot /opt/polyglot/var/
chown -R polyglot:polyglot /opt/polyglot/share/certs/

# Reload systemd
systemctl daemon-reload

# Load QH Lua scripts into Redis (if Redis is running)
if systemctl is-active --quiet polyglot-redis; then
    /opt/polyglot/bin/polyglot-ctl load-qh-scripts
fi

echo ""
echo "Polyglot installed to /opt/polyglot/"
echo "Run 'polyglot-ctl setup' to configure and start services."
echo ""
```

## preremove Script

```bash
#!/bin/bash
set -e

# Stop and disable all services
for svc in polyglot-runner polyglot-tm polyglot-nats polyglot-redis; do
    systemctl stop "$svc" 2>/dev/null || true
    systemctl disable "$svc" 2>/dev/null || true
done

systemctl daemon-reload

echo ""
echo "Polyglot services stopped."
echo "Data preserved in /opt/polyglot/var/ — remove manually if no longer needed."
echo ""
```

## APT Repository Hosting

### Repository Structure

```text
apt-repo/
├── pool/
│   └── main/
│       └── p/
│           └── polyglot/
│               └── polyglot_0.1.0_amd64.deb
├── dists/
│   └── stable/
│       ├── Release
│       ├── Release.gpg
│       ├── InRelease
│       └── main/
│           └── binary-amd64/
│               ├── Packages
│               └── Packages.gz
└── polyglot.gpg              # Public GPG key
```

### reprepro Setup

```bash
# Initialize repo
mkdir -p apt-repo/conf
cat > apt-repo/conf/distributions <<'EOF'
Origin: Polyglot
Label: Polyglot
Suite: stable
Codename: stable
Architectures: amd64 arm64
Components: main
SignWith: <GPG-KEY-ID>
EOF

# Add package
reprepro -b apt-repo includedeb stable dist/polyglot_0.1.0_amd64.deb
```

### GPG Key Management

```bash
# Generate signing key (one-time)
gpg --batch --gen-key <<'EOF'
Key-Type: RSA
Key-Length: 4096
Name-Real: Polyglot Package Signing
Name-Email: packages@polyglot.dev
Expire-Date: 0
%no-protection
EOF

# Export public key for users
gpg --export --armor packages@polyglot.dev > apt-repo/polyglot.gpg
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
