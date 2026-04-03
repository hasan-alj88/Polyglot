---
audience: pg-coder
type: reference
updated: 2026-04-03
---

# Data Processing Scenarios

<!-- @user/SPEC-INDEX -->
<!-- @user/scenarios/INDEX -->

Automation scenarios for ETL pipelines, AI/ML operations, IoT sensor data, reporting, and education workflows. See [[INDEX]] for the full scenario catalog.

## 2. Data Processing & ETL

| # | Scenario | Trigger | Description |
|---|----------|---------|-------------|
| 18 | CSV to database import | Folder Watch | Parse CSV rows, validate schema, bulk insert into Postgres |
| 19 | API response transformer | Scheduled | Fetch JSON from REST API, flatten nested structures, remap keys |
| 20 | XML to JSON converter | Webhook | Accept XML payloads, transform to JSON, forward downstream |
| 21 | Data deduplication | Scheduled | Scan tables for duplicate records, merge or flag for review |
| 22 | Incremental data sync | Scheduled | Sync changed records between two databases using CDC |
| 23 | Data quality validator | Scheduled | Run validation rules on new records, quarantine failures |
| 24 | Multi-source data merge | Scheduled | Combine data from API, database, and spreadsheet into unified view |
| 25 | Real-time stream processor | Stream | Consume Kafka messages, enrich with lookups, write to data lake |
| 26 | Legacy system data migration | Manual | Extract from legacy DB, transform schema, load into modern system |
| 27 | Geolocation enrichment | Webhook | Append lat/long and address data to records with IP or zip |
| 28 | Currency conversion pipeline | Scheduled | Fetch exchange rates, convert monetary fields across datasets |
| 29 | Log parsing and structuring | Stream | Parse unstructured logs into structured events with regex |
| 30 | Data anonymization | Scheduled | Mask PII fields in datasets before sharing with analytics |
| 31 | Webhook payload normalizer | Webhook | Accept varied webhook formats, normalize to canonical schema |
| 32 | Time series aggregation | Scheduled | Roll up minute-level data into hourly/daily summaries |
| 33 | Data catalog updater | Scheduled | Scan databases, update metadata catalog with schema changes |
| 34 | Batch geocoding | Scheduled | Geocode address lists via Google Maps API in rate-limited batches |

## 20. AI/ML Operations

| # | Scenario | Trigger | Description |
|---|----------|---------|-------------|
| 320 | Model training pipeline | Scheduled | Fetch data, preprocess, train, evaluate, version model |
| 321 | Model performance monitor | Scheduled | Track prediction accuracy, alert on drift |
| 322 | Feature store updater | Scheduled | Compute and refresh feature values from raw data |
| 323 | A/B model experiment | Event | Route traffic between model versions, collect metrics |
| 324 | Data labeling dispatcher | Scheduled | Queue unlabeled data, distribute to annotators |
| 325 | GPU cluster scaler | Event | Scale GPU nodes based on training job queue depth |
| 326 | Model registry updater | Webhook | Register trained models, update metadata and lineage |
| 327 | Inference endpoint deployer | Webhook | Deploy approved models to serving infrastructure |
| 328 | Training data validator | Scheduled | Check for label quality, class imbalance, data leakage |
| 329 | Cost tracker for ML workloads | Scheduled | Track GPU hours and storage per experiment |
| 330 | Prompt template manager | Webhook | Version and deploy prompt templates for LLM applications |
| 331 | LLM output evaluator | Scheduled | Run evaluation suites on LLM outputs, track quality scores |
| 332 | Embedding index rebuilder | Scheduled | Rebuild vector indexes when source documents change |
| 333 | Model explainability reporter | Scheduled | Generate SHAP/LIME explanations for model decisions |
| 334 | Bias detection scanner | Scheduled | Run fairness metrics on model predictions across groups |
| 335 | RAG pipeline refresher | Scheduled | Re-index documents, update chunk embeddings for retrieval |
| 336 | Annotation consensus checker | Scheduled | Compare multi-annotator labels, resolve disagreements |

## 21. IoT & Sensor Data

| # | Scenario | Trigger | Description |
|---|----------|---------|-------------|
| 337 | Sensor threshold alerter | Stream | Alert when sensor readings exceed defined thresholds |
| 338 | Device fleet OTA updater | Scheduled | Push firmware updates to device fleet in rolling batches |
| 339 | Telemetry aggregator | Stream | Aggregate high-frequency sensor data into time-series DB |
| 340 | Predictive maintenance trigger | Scheduled | Analyze vibration/temperature patterns, predict failures |
| 341 | Device heartbeat monitor | Scheduled | Detect devices that stopped reporting, alert operations |
| 342 | Geofence event processor | Stream | Trigger actions when tracked assets enter/exit zones |
| 343 | Energy consumption optimizer | Scheduled | Analyze usage patterns, adjust HVAC/lighting schedules |
| 344 | Environmental compliance logger | Stream | Record environmental readings for regulatory reporting |
| 345 | Smart lock access logger | Event | Log access events, detect anomalous entry patterns |
| 346 | Fleet tracking dashboard | Stream | Aggregate vehicle GPS data, calculate routes and ETA |
| 347 | Water leak detector | Stream | Monitor flow sensors, alert on anomalous usage patterns |
| 348 | Air quality monitor | Stream | Track AQI sensors, alert building management on poor readings |
| 349 | Cold chain monitor | Stream | Track temperature in transit, alert on deviations |
| 350 | Device provisioning pipeline | Webhook | Register new devices, assign configs, activate connectivity |
| 351 | Occupancy tracker | Stream | Count people from sensors, optimize space utilization |
| 352 | Battery level monitor | Scheduled | Track device battery levels, schedule replacements |

## 22. Reporting & Analytics

| # | Scenario | Trigger | Description |
|---|----------|---------|-------------|
| 353 | Executive dashboard builder | Scheduled | Aggregate KPIs from multiple sources into C-suite dashboard |
| 354 | Weekly metrics email | Scheduled | Compile key metrics, render charts, email stakeholders |
| 355 | Ad-hoc report runner | Webhook | Execute parameterized queries, format and deliver results |
| 356 | Data freshness checker | Scheduled | Verify dashboards show recent data, alert on stale sources |
| 357 | Cohort analysis pipeline | Scheduled | Segment users by signup week, compute retention curves |
| 358 | Funnel conversion tracker | Scheduled | Calculate conversion rates between funnel stages |
| 359 | Custom report scheduler | Scheduled | Run user-configured reports on their chosen schedule |
| 360 | Benchmark comparison report | Scheduled | Compare metrics against industry benchmarks |
| 361 | Data export fulfiller | Webhook | Generate data exports on request, deliver via secure link |
| 362 | Report access auditor | Scheduled | Track who accessed which reports, flag unusual patterns |
| 363 | Predictive analytics runner | Scheduled | Run forecasting models, update prediction dashboards |
| 364 | Survey results analyzer | Webhook | Process survey responses, generate summary with charts |
| 365 | Usage analytics pipeline | Scheduled | Process event logs into user behavior analytics |
| 366 | Churn prediction reporter | Scheduled | Score customers for churn risk, alert account managers |
| 367 | A/B test results reporter | Scheduled | Calculate test metrics, determine significance, publish results |
| 368 | OKR progress tracker | Scheduled | Pull metrics for each OKR, calculate progress, report |

## 25. Education & Training

| # | Scenario | Trigger | Description |
|---|----------|---------|-------------|
| 401 | Course enrollment processor | Webhook | Enroll students, provision LMS access, send welcome |
| 402 | Assignment deadline reminder | Scheduled | Notify students of upcoming deadlines |
| 403 | Grade book sync | Webhook | Sync grades between LMS and student information system |
| 404 | Certificate generator | Webhook | Generate completion certificates with unique IDs |
| 405 | Plagiarism check submitter | Webhook | Submit assignments to plagiarism service, log results |
| 406 | Attendance tracker | Event | Record attendance from check-in events, flag absences |
| 407 | Tutor matching engine | Webhook | Match tutoring requests to available tutors by subject |
| 408 | Learning path recommender | Scheduled | Suggest next courses based on completion history |
| 409 | Student progress reporter | Scheduled | Generate progress reports for parents/advisors |
| 410 | Library hold notifier | Webhook | Alert patrons when reserved materials become available |
| 411 | Scholarship deadline tracker | Scheduled | Track application deadlines, remind eligible students |
| 412 | Course feedback analyzer | Webhook | Aggregate course evaluations, generate summary reports |
| 413 | Academic calendar publisher | Scheduled | Sync academic calendar events to student calendars |
| 414 | Continuing education tracker | Scheduled | Track CE credit hours, alert before renewal deadlines |
| 415 | Classroom resource scheduler | Webhook | Book rooms and equipment for classes, handle conflicts |
| 416 | Financial aid processor | Webhook | Validate applications, calculate awards, notify students |
