---
audience: developer
type: guide
updated: 2026-04-28
---

# Local Development Infrastructure Setup

<!-- @c:technical/plan/deployment/installation -->
<!-- @c:technical/plan/queue-manager/infrastructure -->

To develop Aljam3 locally, your machine needs the three core infrastructure services running: **PostgreSQL**, **Redis**, and **NATS**.

This guide provides a single bash script that you can run on Ubuntu/Debian to automatically check if these services are installed, and install them if they are missing.

## Prerequisites
- Ubuntu 22.04+ or Debian 12+
- `sudo` privileges

## The Setup Script

Create a file named `setup-dev-infrastructure.sh` in your project's `scripts/` directory, make it executable (`chmod +x scripts/setup-dev-infrastructure.sh`), and run it:

```bash
#!/bin/bash
set -e

echo "============================================="
echo " Aljam3 Dev Infrastructure Setup (Ubuntu/Debian) "
echo "============================================="

# Ensure sudo privileges upfront
echo "Requesting root privileges for setup..."
if ! sudo -v; then
    echo "❌ This script requires sudo privileges to install packages and configure users."
    exit 1
fi

# Keep sudo session alive
while true; do sudo -n true; sleep 60; kill -0 "$$" || exit; done 2>/dev/null &

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# 0. Create Aljam3 UNIX User
echo -e "\n[0/4] Checking Aljam3 UNIX user..."
if id "aljam3" &>/dev/null; then
    echo "✅ UNIX user 'aljam3' already exists."
else
    echo "❌ UNIX user 'aljam3' not found. Creating..."
    sudo useradd -r -s /bin/false -d /opt/aljam3 -m aljam3
    echo "✅ UNIX user 'aljam3' created."
fi

# 1. Check & Install PostgreSQL
echo -e "\n[1/4] Checking PostgreSQL..."
if command_exists psql && id "postgres" &>/dev/null && psql -V >/dev/null 2>&1; then
    echo "✅ PostgreSQL is already installed ($(psql -V | head -n 1))."
else
    echo "❌ PostgreSQL not fully installed. Installing..."
    sudo apt update
    sudo apt install -y postgresql postgresql-contrib
    sudo systemctl enable --now postgresql
    echo "✅ PostgreSQL installed and started."
fi

# Configure PostgreSQL Database & User
echo "Checking PostgreSQL 'aljam3' role and database..."
if sudo -u postgres psql -tAc "SELECT 1 FROM pg_roles WHERE rolname='aljam3'" | grep -q 1; then
    echo "✅ PostgreSQL user 'aljam3' already exists."
else
    echo "Creating PostgreSQL user 'aljam3'..."
    sudo -u postgres psql -c "CREATE USER aljam3 WITH PASSWORD 'aljam3';"
fi

if sudo -u postgres psql -lqt | cut -d \| -f 1 | grep -qw aljam3; then
    echo "✅ PostgreSQL database 'aljam3' already exists."
else
    echo "Creating PostgreSQL database 'aljam3'..."
    sudo -u postgres psql -c "CREATE DATABASE aljam3 OWNER aljam3;"
fi

# 2. Check & Install Redis
echo -e "\n[2/4] Checking Redis..."
if command_exists redis-cli; then
    echo "✅ Redis is already installed ($(redis-cli -v))."
else
    echo "❌ Redis not found. Installing..."
    sudo apt update
    sudo apt install -y redis-server
    sudo systemctl enable --now redis-server
    echo "✅ Redis installed and started."
fi

# 3. Check & Install NATS
echo -e "\n[3/4] Checking NATS JetStream..."
if command_exists nats-server; then
    echo "✅ NATS is already installed ($(nats-server -v))."
else
    echo "❌ NATS not found. Installing..."
    # NATS is not in default apt repos, downloading latest binary
    NATS_VERSION="2.10.14"
    echo "Downloading NATS v${NATS_VERSION}..."
    curl -L https://github.com/nats-io/nats-server/releases/download/v${NATS_VERSION}/nats-server-v${NATS_VERSION}-linux-amd64.zip -o /tmp/nats.zip
    
    # Unzip and install
    sudo apt install -y unzip
    unzip -q /tmp/nats.zip -d /tmp/
    sudo mv /tmp/nats-server-v${NATS_VERSION}-linux-amd64/nats-server /usr/local/bin/
    
    # Create basic systemd service for NATS
    echo "Creating systemd service for NATS..."
    cat <<EOF | sudo tee /etc/systemd/system/nats.service
[Unit]
Description=NATS Server
After=network.target

[Service]
ExecStart=/usr/local/bin/nats-server -js -sd /opt/aljam3/nats
Restart=always
User=aljam3
Group=aljam3
LimitNOFILE=65536

[Install]
WantedBy=multi-user.target
EOF

    # Create NATS storage directory
    sudo mkdir -p /opt/aljam3/nats
    sudo chown -R aljam3:aljam3 /opt/aljam3/nats
    
    sudo systemctl daemon-reload
    sudo systemctl enable --now nats
    
    # Clean up
    rm -rf /tmp/nats.zip /tmp/nats-server-*
    echo "✅ NATS installed and started with JetStream enabled."
fi

echo -e "\n============================================="
echo "🎉 Setup Complete!"
echo "Your local Aljam3 dev environment is ready."
echo "- PostgreSQL: localhost:5432 (User: aljam3, DB: aljam3)"
echo "- Redis: localhost:6379"
echo "- NATS: localhost:4222"
echo "============================================="
```

## Manual Verification

If you prefer to check manually without running the script:

**PostgreSQL**:
```bash
psql -V                   # Check if installed
systemctl status postgresql # Check if running
```

**Redis**:
```bash
redis-cli ping            # Should return PONG
systemctl status redis-server
```

**NATS**:
```bash
curl http://localhost:8222/varz # Check NATS health (if monitoring port is open)
systemctl status nats
```
