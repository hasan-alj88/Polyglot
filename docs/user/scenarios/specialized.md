---
audience: automation-builder
type: reference
updated: 2026-04-03
---

# Specialized Domain Scenarios

<!-- @user/SPEC-INDEX -->
<!-- @user/scenarios/INDEX -->

Automation scenarios for monitoring, API management, security, healthcare, supply chain, and agriculture. See [[INDEX]] for the full scenario catalog.

## 6. Monitoring & Alerting

| # | Scenario | Trigger | Description |
|---|----------|---------|-------------|
| 86 | Multi-endpoint health check | Scheduled | Ping endpoints in parallel, alert on failures |
| 87 | Error rate spike detector | Stream | Monitor error counts, alert if rate exceeds rolling average |
| 88 | Uptime report generator | Scheduled | Calculate monthly uptime per service, email report |
| 89 | PagerDuty escalation | Webhook | Route alerts by severity, escalate if unacknowledged |
| 90 | Anomaly detection pipeline | Scheduled | Run statistical analysis on metrics, flag anomalies |
| 91 | SLA breach warning | Scheduled | Check response times against SLA thresholds, alert before breach |
| 92 | Custom dashboard updater | Scheduled | Aggregate metrics from multiple sources into dashboard |
| 93 | Alert deduplication | Stream | Group related alerts, suppress duplicates, send consolidated |
| 94 | Synthetic transaction monitor | Scheduled | Run scripted user journeys, measure latency, alert on failure |
| 95 | Cost anomaly alerter | Scheduled | Compare cloud spend against forecast, alert on spikes |
| 96 | Third-party status watcher | API Poll | Poll vendor status pages, notify team of incidents |
| 97 | Log-based alerting | Stream | Match log patterns for critical events, trigger alerts |
| 98 | Capacity planning reporter | Scheduled | Track resource usage trends, project exhaustion dates |
| 99 | Incident auto-creator | Webhook | Create incident tickets from monitoring alerts with runbook links |
| 100 | Dead letter queue monitor | Scheduled | Check DLQ depth, alert and retry failed messages |
| 101 | Browser performance tracker | Scheduled | Collect Core Web Vitals from RUM data, alert on degradation |
| 102 | Alert fatigue analyzer | Scheduled | Report on alert frequency and resolution times, flag noisy alerts |

## 18. API Management & Integration

| # | Scenario | Trigger | Description |
|---|----------|---------|-------------|
| 286 | API key rotator | Scheduled | Generate new keys, update consumers, deprecate old keys |
| 287 | Rate limit monitor | Stream | Track API usage per consumer, alert on approaching limits |
| 288 | Webhook retry manager | Scheduled | Retry failed webhook deliveries with exponential backoff |
| 289 | API deprecation notifier | Scheduled | Warn consumers of upcoming API version deprecations |
| 290 | Integration health checker | Scheduled | Test all third-party integrations, report failures |
| 291 | API response validator | Scheduled | Run contract tests against APIs, flag breaking changes |
| 292 | OAuth token refresher | Scheduled | Pre-refresh expiring OAuth tokens to avoid downtime |
| 293 | API usage analytics | Scheduled | Aggregate endpoint usage, latency, error rates into dashboard |
| 294 | Swagger doc generator | Webhook | Regenerate API documentation on schema changes |
| 295 | Third-party API failover | Signal | Switch to backup provider when primary API is down |
| 296 | API gateway config sync | Webhook | Propagate routing rules across gateway instances |
| 297 | Payload transformation proxy | Webhook | Translate between API versions for backward compatibility |
| 298 | API changelog publisher | Webhook | Document API changes and notify subscribed consumers |
| 299 | GraphQL schema stitcher | Webhook | Merge sub-schemas into federated graph on service deploy |
| 300 | API mock server updater | Webhook | Update mock responses when API contracts change |
| 301 | Consumer onboarding provisioner | Webhook | Create API credentials, set quotas, send welcome docs |
| 302 | Circuit breaker monitor | Stream | Track circuit breaker states, alert on repeated opens |

## 19. Security & Compliance

| # | Scenario | Trigger | Description |
|---|----------|---------|-------------|
| 303 | Vulnerability scan aggregator | Scheduled | Collect scan results from tools, deduplicate, prioritize |
| 304 | Access review automator | Scheduled | Generate quarterly access reviews, collect approvals |
| 305 | Incident response orchestrator | Webhook | Execute IR playbook steps on security alert trigger |
| 306 | Compliance evidence collector | Scheduled | Gather screenshots, configs, logs for audit evidence |
| 307 | Secret scanner | Webhook | Scan code commits for leaked credentials, alert and revoke |
| 308 | GDPR data request handler | Webhook | Process data subject requests, export/delete data |
| 309 | Security training tracker | Scheduled | Track completion of mandatory security training, chase |
| 310 | Threat intel feed processor | Scheduled | Ingest threat feeds, correlate with internal assets |
| 311 | SOC alert enricher | Webhook | Enrich SIEM alerts with context from CMDB and threat intel |
| 312 | Pentest finding tracker | Webhook | Import pentest findings, create remediation tickets |
| 313 | MFA enrollment checker | Scheduled | Identify users without MFA, send enrollment reminders |
| 314 | Security policy publisher | Webhook | Distribute updated policies, track acknowledgments |
| 315 | Data classification scanner | Scheduled | Scan storage for unclassified sensitive data |
| 316 | Privilege escalation detector | Stream | Alert on unusual privilege changes or sudo usage |
| 317 | Third-party risk assessor | Scheduled | Request and track vendor security questionnaires |
| 318 | Encryption compliance checker | Scheduled | Verify all storage and transit encryption meets policy |
| 319 | WAF rule updater | Webhook | Update WAF rules based on new threat patterns |

## 24. Healthcare & Medical

| # | Scenario | Trigger | Description |
|---|----------|---------|-------------|
| 385 | Appointment reminder | Scheduled | Send SMS/email reminders 48h and 2h before appointments |
| 386 | Lab result notifier | Webhook | Alert patients and providers when lab results are ready |
| 387 | Prescription refill handler | Scheduled | Process auto-refill requests, verify eligibility, notify pharmacy |
| 388 | Patient intake form processor | Webhook | Digitize intake forms, populate EHR fields |
| 389 | Insurance verification | Webhook | Verify patient coverage before appointments via payer API |
| 390 | Clinical trial enrollment | Webhook | Match patients to eligible trials, notify coordinators |
| 391 | HIPAA audit logger | Signal | Log all PHI access events for compliance reporting |
| 392 | Medication interaction checker | Webhook | Check new prescriptions against current meds for interactions |
| 393 | Referral management | Webhook | Process referrals, schedule with specialist, update PCP |
| 394 | Waitlist manager | Signal | Notify waitlisted patients when slots open |
| 395 | Telehealth session setup | Scheduled | Send video links and prep instructions before virtual visits |
| 396 | Medical device alert handler | Webhook | Process FDA alerts, identify affected devices in inventory |
| 397 | Patient satisfaction surveyor | Webhook | Send surveys post-visit, aggregate scores |
| 398 | Prior authorization tracker | Webhook | Submit prior auths to payers, track status, follow up |
| 399 | Care plan compliance monitor | Scheduled | Track patient adherence to care plans, alert providers on gaps |
| 400 | Credentialing tracker | Scheduled | Monitor provider credential expirations, initiate renewals |

## 27. Supply Chain & Logistics

| # | Scenario | Trigger | Description |
|---|----------|---------|-------------|
| 433 | Shipment tracking updater | API Poll | Poll carrier APIs, update tracking status, notify customers |
| 434 | Purchase order generator | Scheduled | Generate POs when inventory hits reorder point |
| 435 | Supplier lead time monitor | Scheduled | Track actual vs quoted lead times, flag delays |
| 436 | Warehouse receiving processor | Webhook | Log received goods, update inventory, trigger put-away |
| 437 | Demand forecasting pipeline | Scheduled | Run forecasting models from sales data, update procurement |
| 438 | Freight rate comparator | Webhook | Compare carrier rates for shipment, select optimal |
| 439 | Customs documentation builder | Webhook | Generate customs forms from shipment and product data |
| 440 | Delivery exception handler | Webhook | Process delivery failures, schedule redelivery or return |
| 441 | Lot traceability logger | Webhook | Track product lots through supply chain for recall readiness |
| 442 | Vendor scorecard generator | Scheduled | Rate vendors on delivery, quality, price; generate reports |
| 443 | Route optimization | Scheduled | Calculate optimal delivery routes from daily orders |
| 444 | Container tracking | API Poll | Monitor ocean container positions, alert on delays |
| 445 | Return merchandise processor | Webhook | Process RMAs, schedule pickup, update inventory on receipt |
| 446 | Safety stock calculator | Scheduled | Recalculate safety stock levels from demand variability |
| 447 | Supplier onboarding workflow | Webhook | Collect documents, verify compliance, activate in ERP |
| 448 | Warehouse slotting optimizer | Scheduled | Analyze pick frequency, recommend optimal bin assignments |

## 30. Agriculture & Environment

| # | Scenario | Trigger | Description |
|---|----------|---------|-------------|
| 481 | Irrigation scheduler | Stream | Adjust watering based on soil moisture sensors and weather |
| 482 | Crop health monitor | Scheduled | Analyze drone/satellite imagery for disease detection |
| 483 | Weather alert responder | Stream | Trigger protective actions on severe weather warnings |
| 484 | Harvest yield tracker | Webhook | Log harvest data, compare against projections |
| 485 | Soil nutrient analyzer | Scheduled | Process soil test results, recommend fertilizer adjustments |
| 486 | Pest detection alerter | Stream | Analyze trap sensor data, alert on pest threshold breach |
| 487 | Livestock health tracker | Stream | Monitor animal sensors, flag abnormal vital patterns |
| 488 | Compliance reporting for emissions | Scheduled | Aggregate emissions data, generate regulatory reports |
| 489 | Water quality monitor | Stream | Track water quality parameters, alert on violations |
| 490 | Equipment maintenance scheduler | Scheduled | Track equipment hours, schedule preventive maintenance |
| 491 | Market price tracker | Scheduled | Monitor commodity prices, recommend optimal sell timing |
| 492 | Supply chain traceability | Webhook | Track produce from farm to shelf for food safety |
| 493 | Carbon credit calculator | Scheduled | Estimate carbon sequestration from land use data |
| 494 | Frost warning responder | Stream | Activate frost protection systems on temperature drop |
| 495 | Feed inventory manager | Scheduled | Track feed consumption, auto-generate purchase orders |
| 496 | Satellite imagery processor | Scheduled | Download and process NDVI imagery for field health maps |
| 497 | Pollinator activity tracker | Stream | Monitor hive sensors, alert on colony health issues |
| 498 | Greenhouse climate controller | Stream | Adjust ventilation and heating based on sensor readings |
| 499 | Organic certification tracker | Scheduled | Track input usage, verify organic compliance requirements |
| 500 | Seasonal labor coordinator | Scheduled | Match labor demand forecasts to worker availability |
