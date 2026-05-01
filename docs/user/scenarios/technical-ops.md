---
audience: automation-builder
type: reference
updated: 2026-04-03
---

# Technical Operations Scenarios

<!-- @user/SPEC-INDEX -->
<!-- @user/scenarios/INDEX -->

Automation scenarios for file management, IT operations, DevOps, cloud infrastructure, and database operations. See [[INDEX]] for the full scenario catalog.

## 1. File & Document Management

| # | Scenario | Trigger | Description |
|---|----------|---------|-------------|
| 1 | Invoice PDF extraction | Email | Parse incoming invoice PDFs, extract fields via OCR, save structured data |
| 2 | Document approval workflow | Webhook | Route uploaded docs through multi-level approval chain |
| 3 | Auto-organize downloads | Folder Watch | Sort files into subfolders by type, date, or naming pattern |
| 4 | Google Drive backup sync | Scheduled | Mirror Google Drive folder to S3 on a daily schedule |
| 5 | Contract version control | Webhook | Track document edits, create versioned snapshots, notify stakeholders |
| 6 | Bulk file rename | Folder Watch | Rename batch of files using regex pattern and metadata |
| 7 | Shared folder permission audit | Scheduled | Scan shared drives, report files with overly broad access |
| 8 | Image compression pipeline | Folder Watch | Compress and resize uploaded images, generate thumbnails |
| 9 | PDF merge and watermark | API Poll | Combine multiple PDFs, apply watermark, deliver to requester |
| 10 | Expired document cleanup | Scheduled | Find docs past retention date, archive or delete per policy |
| 11 | Spreadsheet consolidation | Scheduled | Merge weekly spreadsheets from multiple teams into master file |
| 12 | Document template generator | Webhook | Fill templates with form data, produce finalized docs |
| 13 | File deduplication scanner | Scheduled | Hash files across storage, flag and report duplicates |
| 14 | Scan-to-cloud pipeline | Folder Watch | Detect scanned files in local folder, OCR and upload to cloud |
| 15 | Large file transfer handler | Webhook | Split large uploads into chunks, transfer via S3, notify on completion |
| 16 | Attachment archiver | Email | Extract email attachments, tag with metadata, store in DMS |
| 17 | File format converter | Folder Watch | Convert between formats (DOCX to PDF, CSV to XLSX) on upload |

## 4. IT Operations & SysAdmin

| # | Scenario | Trigger | Description |
|---|----------|---------|-------------|
| 52 | User onboarding provisioner | Webhook | Create accounts across AD, email, Slack, GitHub for new hire |
| 53 | User offboarding | Webhook | Disable accounts, revoke tokens, transfer ownership on departure |
| 54 | SSL certificate renewal | Scheduled | Check cert expiry, auto-renew via Let's Encrypt, deploy |
| 55 | Disk space monitor | Scheduled | Poll servers for disk usage, alert and clean temp if threshold hit |
| 56 | DNS record updater | Webhook | Update DNS records via API when infrastructure changes |
| 57 | Patch compliance checker | Scheduled | Scan systems for missing patches, generate compliance report |
| 58 | Service restart handler | Signal | Detect crashed service, attempt restart, escalate if repeated |
| 59 | Firewall rule auditor | Scheduled | Export firewall rules, diff against baseline, flag deviations |
| 60 | Backup verification | Scheduled | Restore latest backup to test environment, run integrity checks |
| 61 | Asset inventory sync | Scheduled | Scan network for devices, update CMDB with discoveries |
| 62 | Log rotation manager | Scheduled | Rotate, compress, and archive logs exceeding size thresholds |
| 63 | VPN access provisioner | Webhook | Grant VPN profiles based on approved access requests |
| 64 | Password expiry notifier | Scheduled | Warn users N days before password expiration via email/Slack |
| 65 | License usage tracker | Scheduled | Poll SaaS admin APIs, report license utilization |
| 66 | Print queue monitor | Scheduled | Check print queues, clear stuck jobs, alert admins on failures |
| 67 | LDAP group sync | Scheduled | Sync AD groups with application role mappings |
| 68 | Stale account disabler | Scheduled | Find accounts inactive for 90+ days, disable and notify |

## 5. DevOps & CI/CD

Many DevOps scenarios below are natural fits for `-T.Git.*` triggers (`-T.Git.Push`, `-T.Git.PR`, `-T.Git.Tag`). These semantic triggers abstract over local git hooks and remote webhooks — see [[jm3lib/pipelines/T/INDEX|-T.* Trigger Pipelines]] for the Git trigger family and [[jm3lib/types/git|#Git Type Tree]] for typed event payloads.

| # | Scenario | Trigger | Description |
|---|----------|---------|-------------|
| 69 | PR auto-labeler | Webhook | Label PRs by file paths changed and diff size |
| 70 | Build failure notifier | Webhook | Alert Slack channel with failure details on CI build failure |
| 71 | Dependency update PR | Scheduled | Check for outdated deps, create PRs with changelogs |
| 72 | Container image scanner | Webhook | Scan newly pushed images for CVEs, block deployment if critical |
| 73 | Blue-green deploy orchestrator | Webhook | Switch traffic between environments, verify health, rollback |
| 74 | Feature flag sync | Webhook | Propagate flag changes to all environments on update |
| 75 | Release notes generator | Webhook | Compile commit messages between tags into release notes |
| 76 | Hotfix cherry-picker | Webhook | Auto cherry-pick tagged commits to release branches |
| 77 | Staging environment provisioner | Webhook | Spin up ephemeral staging per PR, tear down on merge |
| 78 | Code coverage gate | Webhook | Block merge if coverage drops below threshold |
| 79 | Secrets rotation | Scheduled | Rotate API keys and passwords, update vault and services |
| 80 | Deployment approval gate | Webhook | Hold production deploy until approvals collected via Slack |
| 81 | Changelog updater | Webhook | Auto-update CHANGELOG.md from conventional commit messages |
| 82 | Performance regression detector | Webhook | Compare benchmark results against baseline, flag regressions |
| 83 | Terraform plan reviewer | Webhook | Run terraform plan on PR, post diff as comment |
| 84 | Artifact cleanup | Scheduled | Delete old build artifacts exceeding retention policy |
| 85 | Canary deploy monitor | Signal | Watch error rate during canary rollout, auto-rollback if spike |

## 16. Cloud Infrastructure

| # | Scenario | Trigger | Description |
|---|----------|---------|-------------|
| 252 | Auto-scaling trigger | Signal | Scale instances based on CPU/memory threshold alerts |
| 253 | Unused resource finder | Scheduled | Identify unattached EBS volumes, idle instances, stale IPs |
| 254 | Cloud cost optimizer | Scheduled | Recommend reserved instances and savings plans |
| 255 | Tag compliance enforcer | Scheduled | Find untagged resources, apply defaults, notify owners |
| 256 | Multi-cloud inventory sync | Scheduled | Aggregate resource inventories across AWS, GCP, Azure |
| 257 | Spot instance manager | Signal | Handle spot interruptions, migrate workloads, relaunch |
| 258 | S3 lifecycle policy auditor | Scheduled | Verify lifecycle rules match retention requirements |
| 259 | CloudFormation drift detector | Scheduled | Check stacks for drift, alert on manual changes |
| 260 | IAM access reviewer | Scheduled | Report overly permissive IAM policies, suggest tightening |
| 261 | Reserved instance expiry alerter | Scheduled | Warn before reserved instances expire for renewal planning |
| 262 | VPC peering auditor | Scheduled | Map VPC connections, flag unauthorized peering |
| 263 | Serverless cold start monitor | Scheduled | Track Lambda cold starts, optimize memory and concurrency |
| 264 | Cloud backup scheduler | Scheduled | Trigger snapshots of volumes and databases per policy |
| 265 | CDN cache invalidator | Webhook | Purge CDN cache on content deployment |
| 266 | Cross-region replication checker | Scheduled | Verify replication lag and consistency across regions |
| 267 | Kubernetes node scaler | Signal | Add or remove nodes based on pod scheduling pressure |
| 268 | Cloud network latency monitor | Scheduled | Measure inter-region latency, alert on degradation |

## 17. Database Operations

| # | Scenario | Trigger | Description |
|---|----------|---------|-------------|
| 269 | Automated backup tester | Scheduled | Restore backup to test instance, run validation queries |
| 270 | Slow query detector | Scheduled | Parse slow query logs, alert on new patterns |
| 271 | Index usage analyzer | Scheduled | Report unused indexes, suggest new ones from query patterns |
| 272 | Schema migration runner | Webhook | Execute pending migrations on deploy, verify rollback plan |
| 273 | Connection pool monitor | Scheduled | Track connection counts, alert on pool exhaustion |
| 274 | Data archival pipeline | Scheduled | Move records older than retention period to archive tables |
| 275 | Replication lag alerter | Scheduled | Monitor replica lag, alert if exceeding threshold |
| 276 | Table size growth tracker | Scheduled | Record table sizes daily, project growth, alert on anomalies |
| 277 | Query plan change detector | Scheduled | Capture explain plans, alert when execution plans change |
| 278 | Database user auditor | Scheduled | Review DB users and privileges, flag dormant accounts |
| 279 | Partition maintenance | Scheduled | Create new partitions, drop expired ones per schedule |
| 280 | Cross-database sync | Scheduled | Replicate selected tables between different database engines |
| 281 | Deadlock detector | Stream | Parse logs for deadlocks, alert with query details |
| 282 | Read replica promoter | Signal | Promote read replica to primary on primary failure |
| 283 | Statistics updater | Scheduled | Run ANALYZE on tables with significant data changes |
| 284 | Temp table cleanup | Scheduled | Drop orphaned temporary tables from long-running sessions |
| 285 | Parameter tuning advisor | Scheduled | Analyze workload, recommend config parameter changes |
