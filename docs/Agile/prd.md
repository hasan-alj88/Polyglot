# Polyglot - Product Requirements Document

**Author:** hhj
**Date:** 2025-11-15
**Version:** 1.0

**Syntax Version Note:** Code examples in this document use **v0.0.4 syntax** (finalized December 2025).

**Current Implementation Status (as of December 29, 2025):**
- ✅ **v0.0.3 Complete** - Epic 1 (Stories 1.1-1.8): Lexer & Parser fully functional
- ✅ **v0.0.4 Architecture Complete** - Epic 13: Full lexer & parser architecture designed
  - **Lexer Architecture:** 124 token types, state machine design, 3-char lookahead, indentation tracking
  - **Parser Architecture:** Complete AST design, recursive descent, three-phase resolution
  - **Implementation Stories:** Story 13.1 (Lexer) and 13.2 (Parser) ready for development
- 🔄 **v0.0.4 Implementation Ready** - Estimated 5-7 weeks for Stories 13.1-13.2
- 📅 **Target:** Full v0.0.4 implementation Q1 2026 (architecture complete, ahead of schedule)

**Breaking Changes Alert:** v0.0.4 introduces breaking syntax changes (indentation-based nesting, `$` variable prefix, `;` reserved indication). Existing v0.0.3 code will require migration. See `/docs/User/specifications/v0.0.4/` for complete specification.

---

## Executive Summary

Polyglot is an asynchronous automation language designed to eliminate two fundamental pain points in modern software development: the complexity of cross-language integration and the inability to manage task priorities descriptively in automated workflows.

**The Vision:** Enable developers to use the "right tool for the right job" without the integration tax. Call C++ from Python, Rust from Node, or any language combination—without manual FFI setup, without pybind boilerplate, without the hours of integration work that currently discourages polyglot development.

**Core Architecture:** Built on three backend services (Trigger Monitor, Queue Manager, Runner) that orchestrate multi-runtime workflows through a unified language syntax. Developers define pipelines in `.pg` files, and Polyglot handles all the cross-language complexity under the hood.

**Target Impact:** Transform what currently takes hours of FFI setup into a single line of descriptive code. Enable production automation systems to declaratively manage resource priorities, preventing low-priority batch jobs from starving high-priority operations.

### What Makes This Special

**Runtime vs. Compile-Time Type Crossing** - The Breakthrough Innovation

Polyglot's async architecture enables type resolution at **runtime** instead of compile-time, unlocking cross-language interoperability that was previously impossible.

**The Magic Moment:**
```python
import polyglot as pg

# This would normally require hours of pybind setup
pg.load_cpp_header("fast_math.h")
result = pg.cpp.fast_math.matrix_multiply(matrix_a, matrix_b)
# It just works - Polyglot handles all FFI complexity
```

**Why This Is Different:**
- Traditional FFI tools (pybind, PyO3, GraalVM) require compile-time type knowledge
- Dynamic languages pass runtime-determined types (e.g., list sizes unknown until execution)
- Polyglot bridges this gap: Python's dynamic list → runtime size detection → Rust's fixed-size array
- This enables conversions that compile-time solutions cannot achieve

**The Dual-Strategy Advantage:**
1. **Universal Translator:** All data serialized through intermediate representation (Strategy 1)
2. **Direct Conversion:** Leverage existing FFI tools when optimal (Strategy 2)
3. **Best of Both:** Flexibility + performance, with automatic fallback

---

## Problem Statement

### Current State Pain Points

Modern software development faces two critical friction points that force developers into suboptimal solutions:

**Pain Point #1: The FFI Integration Tax**

**The Problem:**
Developers waste 2-8 hours setting up Foreign Function Interface (FFI) bindings for simple cross-language function calls. What should be a 15-minute task becomes an afternoon of wrestling with build systems, type annotations, and cryptic compilation errors.

**Concrete Examples:**
- **Python calling Rust:** Requires PyO3 setup with `Cargo.toml` configuration, manual type annotations (`#[pyfunction]`), build scripts, and Python package distribution setup
- **Python calling C++:** Requires pybind11 with CMake configuration, template metaprogramming knowledge, manual memory management, and deep C++ expertise
- **Node calling Rust:** Requires napi-rs with node-bindgen macros, complex async bridging, and N-API versioning management

**Root Causes:**
1. **Compile-time type mismatch:** Dynamic languages (Python, Node) have runtime types, static languages (Rust, C++) require compile-time types—existing FFI tools bridge this at compile-time, requiring manual type declarations
2. **Manual marshalling:** Developers must explicitly convert data structures between language representations (Python list → Rust Vec, Python dict → Rust HashMap)
3. **Build complexity:** Each FFI tool adds build dependencies (CMake, setuptools-rust, cargo-build-python) with their own configuration files and failure modes
4. **Deep language expertise:** Effective FFI requires understanding memory models of BOTH languages (Python's reference counting + Rust's ownership, or C++'s manual memory + Python's GC)

**User Impact:**
- **Lost Productivity:** 20-30% of polyglot development time spent on integration vs. feature work
- **Monolingual Codebases:** Teams avoid cross-language integration entirely, forcing "rewrite in our primary language" decisions that sacrifice performance or ecosystem advantages
- **Developer Frustration:** Stack Overflow is littered with "PyO3 compilation failed" and "pybind11 segfault" questions with no clear answers
- **Innovation Barrier:** Promising libraries in other languages remain inaccessible to developers who can't afford the FFI tax

**Quantified Pain:**
- Average PyO3/pybind11 setup: **4-6 hours** for first integration
- Developer salary cost (at $150k/year ≈ $75/hour): **$300-450 per integration**
- For teams doing 10+ integrations/year: **$3000-4500 in lost productivity**

---

**Pain Point #2: No Declarative Priority Management in Automation**

**The Problem:**
Production automation systems treat all tasks as equal priority, leading to resource contention where low-priority batch jobs starve high-priority operations. Infrastructure teams resort to manual queue management scripts that are brittle and hard to maintain.

**Concrete Examples:**
- **Nightly Backups Blocking API Calls:** A 3-hour data backup job starts at 2 AM, consuming all database connections, causing customer-facing API timeouts from 5-8 AM during business hours
- **Log Aggregation Starving Deployments:** Log processing pipeline consumes all available workers, delaying critical security patch deployment by 45 minutes
- **Test Suites Blocking Releases:** 2000-test regression suite runs on same queue as production deployments, forcing manual cancellation before urgent hotfixes

**Root Causes:**
1. **Cron's equality:** All cron jobs have equal priority—first-in-first-out execution with no preemption
2. **Airflow's homogeneous queues:** Airflow pools treat all DAGs equally; priority exists but requires custom operators and manual queue management
3. **No declarative syntax:** Priority must be configured in separate YAML/JSON files or infrastructure code, not in workflow definition itself
4. **Manual intervention required:** Pausing low-priority jobs requires SSH into servers, editing configs, restarting services

**User Impact:**
- **Operational Incidents:** Low-priority jobs cause high-priority outages (backup jobs causing API timeouts = customer-visible incidents)
- **Manual Toil:** DevOps engineers wake up at 3 AM to manually pause batch jobs when high-priority work arrives
- **Infrastructure Complexity:** Custom queue management scripts in production (brittle, undocumented, bus-factor risk)
- **Lost Revenue:** API downtime during business hours due to resource starvation from batch jobs

**Quantified Pain:**
- Average operational incident (API timeout): **1-3 hours** downtime
- Revenue impact (e-commerce, $10k/hour): **$10,000-30,000 per incident**
- DevOps on-call burden: **5-10 manual interventions/month** @ 30 min each = **2.5-5 hours/month**
- Infrastructure complexity cost: **40-80 hours/year** maintaining custom queue scripts

---

### Pain Point Severity Matrix

| Pain Point | Frequency | Severity | User Segment | Quantified Impact |
|------------|-----------|----------|--------------|-------------------|
| **FFI integration takes 2-8 hours** | Daily (for polyglot teams) | **CRITICAL** | Backend Engineers, Data Scientists | $300-450 per integration, 20-30% productivity loss |
| **Manual type marshalling** | Weekly | **HIGH** | Python/Node developers | 1-2 hours per integration, frequent segfaults |
| **No priority control in automation** | Daily (for DevOps) | **HIGH** | DevOps Engineers, Infrastructure Teams | 5-10 incidents/month, $10k-30k revenue impact per incident |
| **Low-priority jobs starve high-priority** | Weekly | **MEDIUM** | Infrastructure Teams | 2.5-5 hours/month manual intervention |
| **Monolingual codebases (avoid FFI)** | Chronic | **HIGH** | All Developers | Can't use "best tool for the job", performance sacrifices |
| **Custom queue scripts (brittle)** | Chronic | **MEDIUM** | DevOps Teams | 40-80 hours/year maintenance burden |

---

### The "Do Nothing" Alternative (Current Workarounds)

**For FFI Pain:**
1. **Rewrite in primary language:** Sacrifice performance/ecosystem for simplicity (common choice)
2. **Use manual FFI tools:** Accept 4-6 hour setup tax (painful but works)
3. **Use subprocess + JSON:** Call other language as subprocess, serialize via JSON (slow, no type safety)
4. **Abandon cross-language integration:** Stay monolingual (limits "best tool for job")

**For Priority Pain:**
1. **Manual queue management:** SSH, pause jobs, restart (operational toil)
2. **Over-provision resources:** Throw hardware at problem (expensive, wasteful)
3. **Schedule around conflicts:** Run batch jobs only when API is idle (inflexible, brittle)
4. **Accept incidents:** Let low-priority jobs occasionally starve high-priority (risky)

**Why Workarounds Fail:**
- Rewrites sacrifice ecosystem advantages (can't use optimized C++ ML libraries from Python)
- Manual FFI requires expertise most developers lack (segfault debugging is black magic)
- Subprocess + JSON is 10-100x slower than native FFI (unacceptable for hot paths)
- Manual queue management doesn't scale (human bottleneck, on-call burden)
- Over-provisioning is expensive (2-10x resource cost for 95th percentile spikes)

---

### User Stories (Pain-Driven)

**Story 1: Backend Engineer's FFI Nightmare**

> "I spent 6 hours yesterday trying to call a Rust function from Python using PyO3. The Rust code was 10 lines. The PyO3 bindings were 50 lines. The build configuration was another 30 lines. I got 12 different compilation errors, half of which made no sense. After finally getting it to compile, it segfaulted on the first call. I gave up and rewrote the Rust code in Python—it's 10x slower but at least it works. I lost a full day of productivity, and we're stuck with a slow solution forever."

**Story 2: DevOps Engineer's Queue Crisis**

> "Our nightly data backup started at 2 AM and usually finishes by 5 AM. Last week it ran long due to data growth, and by 7 AM it was still running, consuming all our database connections. Customer-facing API calls started timing out. I got paged at 7:15 AM, had to SSH into the server, manually kill the backup job, and restart it with a connection limit. Then I spent 2 hours writing a custom script to pause backups if high-priority API traffic is detected. That script broke last month when we upgraded PostgreSQL. I'm tired of babysitting batch jobs."

**Story 3: Data Scientist's Blocked Innovation**

> "There's an incredible C++ library for fast matrix operations that would speed up our ML inference by 50x. But I'm a Python data scientist, not a C++ engineer. I asked our backend team to write PyO3 bindings—they said it would take 2 weeks because they're backed up. So I'm stuck with slow NumPy operations while a perfect solution exists in C++, just out of reach. If only I could load that C++ library directly from Python without needing a full FFI integration project."

---

### Opportunity: What Polyglot Unlocks

**If we solve these pain points:**

1. **FFI goes from 4 hours to 15 minutes:** Developer writes `.pg` pipeline, Polyglot handles all FFI complexity → 95% time savings
2. **Priority is declarative:** Add `[Q] |Q.Queue.HighPriority` to pipeline definition → No manual queue scripts needed
3. **"Best tool for the job" becomes reality:** Python devs use Rust libraries freely, C++ devs use Python ML ecosystems, polyglot teams thrive
4. **Operations become reliable:** No more 3 AM pages to pause batch jobs, automated priority management prevents incidents
5. **Ecosystem unlocked:** Every language's ecosystem becomes accessible to every developer

**The Vision:**
A world where developers choose languages based on merit ("Rust for performance, Python for ML, Node for async I/O") without the integration tax that currently forces monolingual compromises.

---

## Project Classification

**Technical Type:** Developer Tool (Programming Language + Framework)
**Domain:** General Software Development
**Complexity:** Medium

Polyglot is classified as a **developer tool** because it provides a new programming language with runtime framework for orchestrating multi-language workflows. Like other developer tools (compilers, interpreters, build systems), it targets developers as the primary users and solves development workflow problems.

**Technical Complexity:** Medium - Sophisticated architecture (async services, IR compilation, database-driven communication) but no domain-specific regulations or compliance requirements. The complexity comes from technical execution, not domain constraints.

{{#if domain_context_summary}}

### Domain Context

{{domain_context_summary}}
{{/if}}

---

## Success Criteria

**The Ultimate Success Metric:** Developers choose Polyglot over manual FFI setup

Success means a developer facing a cross-language integration problem thinks "I'll use Polyglot" instead of "I'll just rewrite it in my primary language" or "I'll spend the afternoon setting up pybind."

**MVP Success (Proof of Concept):**
- ✅ End-to-end pipeline execution works: Define → Compile → Register → Activate → Trigger → Queue → Execute
- ✅ At least one runtime wrapper functional (Python proof-of-concept)
- ✅ Developer can write a `.pg` pipeline that orchestrates async operations across language boundaries
- ✅ 3+ example pipelines demonstrate real automation value
- ✅ 5-10 early adopters (colleagues, online community) try it and provide meaningful feedback
- ✅ No fundamental architectural blockers discovered during MVP development

**v1.0 Success (Community Launch):**
- **Adoption Signal:** 100+ GitHub stars within first month of announcement
- **Community Health:** 10+ community-contributed examples, utilities, or runtime wrappers
- **Technical Completeness:** 3+ runtime wrappers fully functional (Python, Node, Rust minimum)
- **Self-Service:** Documentation complete enough for developers to onboard independently
- **Production Validation:** At least 1 documented production use case (personal or commercial)

**Long-Term Success (Ecosystem Establishment):**
- **Developer Adoption:** Trending GitHub stars, active forks, developers blogging about using Polyglot
- **Production Usage:** Companies using Polyglot for production automation workflows; case studies of Polyglot replacing manual FFI or proprietary automation tools
- **Ecosystem Growth:** Third-party packages published to Polyglot registry; community-contributed runtime wrappers for additional languages; IDE plugins and tooling (syntax highlighting, debuggers, LSP servers)
- **Community Health:** Active Discord/Slack helping each other; regular contributor meetings and RFCs; diverse contributor base beyond original author

**The Winning Moment:**
A developer tweets: "Spent 15 minutes with Polyglot to call Rust from Python. Would have taken me a full day with PyO3. Never going back."

### Business Metrics

**Open-Source Adoption Metrics:**

Since Polyglot is an open-source developer tool, traditional business metrics (revenue, users) are replaced by community health indicators:

- **GitHub Activity:** Stars, forks, watchers, contributor count, PR velocity
- **Community Engagement:** Discord/Slack member count, discussion activity, Stack Overflow questions
- **Package Ecosystem:** Third-party packages published, downloads from package registry
- **Production Adoption:** Case studies, testimonials, company logos on "who's using Polyglot" page
- **Tool Integration:** IDE plugins, CI/CD integrations, framework adapters

**Leading Indicators (Early Signals):**
- Documentation page views and search traffic
- Example pipeline copy-paste rate (developers trying examples)
- GitHub issue quality (thoughtful feature requests vs. "how do I install" confusion)
- Community contributions beyond bug reports (RFCs, design discussions)

---

## User Personas

Polyglot targets experienced developers who face cross-language integration challenges or automation priority management needs. Below are the three primary personas driving product decisions.

### Primary Persona: Alex - Backend Polyglot Engineer

**Demographics:**
- **Role:** Backend Engineer at Series B Startup (50-200 employees)
- **Experience:** 5+ years professional development
- **Primary Languages:** Python (daily), C++ (performance-critical code)
- **Team Size:** 5-10 person engineering team
- **Tech Stack:** Python microservices + C++ performance libraries

**Background:**
Alex joined a fast-growing startup that chose Python for rapid development but now faces performance bottlenecks in data processing pipelines. The team identified C++ libraries that could speed up operations 10-50x, but integrating them requires PyO3/pybind11 setup that Alex finds intimidating and time-consuming.

**Pain Points:**
- **FFI Tax:** Spends 4+ hours setting up pybind11 for each new C++ integration, only does it when absolutely necessary
- **Compilation Errors:** Gets cryptic template metaprogramming errors from pybind11 that require deep C++ knowledge to debug
- **Maintenance Burden:** FFI bindings break on Python version upgrades, requiring hours of debugging
- **Avoided Optimizations:** Avoids Rust libraries entirely despite performance benefits because PyO3 complexity seems even worse than pybind11
- **Manual FFI Bugs:** Production segfaults from incorrect memory management in FFI layer (nightmares to debug)

**Current Workarounds:**
- Rewrites performance-critical code in Python with NumPy/Cython (10x slower than C++ but no FFI hassle)
- Uses subprocess + JSON for rare C++ calls (100x slower, no type safety, but "it just works")
- Sticks to pure Python ecosystem, sacrificing performance for developer velocity

**Goals:**
- **Primary:** Call Rust/C++ libraries from Python without spending afternoons on FFI setup
- **Secondary:** Maintain single Python codebase without C++ wrapper directories
- **Tertiary:** Avoid segfaults and memory management bugs in FFI layer

**Success Metric for Polyglot:**
Alex chooses Polyglot over manual FFI: "I can integrate a Rust library in 15 minutes vs. 4 hours with PyO3"

**Polyglot Adoption Journey:**
1. Discovers Polyglot via Hacker News post: "Call Rust from Python without PyO3"
2. Tries hello-world example: compiles in 2 minutes, works first try
3. Integrates first production Rust library: 20 minutes total (vs. 6 hours with PyO3)
4. "Magic moment": No build scripts, no type annotations, no segfaults—it just works
5. Champions Polyglot to team, migrates 3+ integrations from PyO3 to Polyglot
6. Writes blog post: "How Polyglot eliminated our FFI tax"

---

### Secondary Persona: Jordan - DevOps Automation Engineer

**Demographics:**
- **Role:** Infrastructure Team Lead at Mid-Size Company (500-1000 employees)
- **Experience:** 8+ years in DevOps/SRE, heavy automation focus
- **Primary Tools:** Bash, Python, cron, Airflow, Terraform
- **Team Size:** 3-person infrastructure team supporting 50+ engineers
- **Responsibilities:** CI/CD, automation workflows, observability, incident response

**Background:**
Jordan manages all automation for a growing engineering org. The team has 50+ cron jobs and 20+ Airflow DAGs handling everything from data backups to log aggregation to deployment pipelines. As the company scaled, resource contention became a major operational issue—low-priority batch jobs frequently starve high-priority deployments and API operations.

**Pain Points:**
- **No Priority Control:** Cron treats all jobs equally; low-priority backups block urgent deployments
- **Operational Incidents:** Nightly backups consume all database connections, causing API timeouts during business hours (5-10 incidents/month)
- **Manual Intervention:** Gets paged at 3 AM to manually pause batch jobs when high-priority work arrives (2.5-5 hours/month on-call burden)
- **Brittle Scripts:** Wrote custom queue management scripts in Python that break on infrastructure changes, undocumented and bus-factor risk (40-80 hours/year maintenance)
- **No Declarative Priority:** Priority exists in Airflow but requires separate config files and custom operators, not in workflow definition itself

**Current Workarounds:**
- Over-provisions database connections and workers (2-10x cost increase)
- Schedules batch jobs in narrow time windows when API traffic is low (inflexible, fails when data volume grows)
- Manually pauses low-priority jobs during incidents (operational toil, doesn't scale)
- Runs high-priority jobs on dedicated workers (resource waste, high complexity)

**Goals:**
- **Primary:** Declaratively define job priorities in workflow code, not separate config
- **Secondary:** Automatically pause low-priority queues when high-priority work arrives
- **Tertiary:** Eliminate manual queue management scripts

**Success Metric for Polyglot:**
Jordan replaces custom queue scripts with Polyglot: "Priority management is now in my workflow definition, not a separate ops playbook"

**Polyglot Adoption Journey:**
1. Hears about Polyglot from colleague: "It has built-in priority queue management"
2. Migrates one problematic cron job (backup pipeline) to Polyglot with `[Q] |Q.Queue.LowPriority`
3. Configures automatic pause on resource contention
4. First incident where Polyglot automatically pauses backup, high-priority deployment succeeds
5. "Magic moment": No manual intervention, system handles priority automatically
6. Migrates 10+ batch jobs to Polyglot, retires custom queue management scripts
7. Writes internal case study: "How Polyglot reduced our on-call burden by 80%"

---

### Tertiary Persona: Morgan - Data Scientist / ML Engineer

**Demographics:**
- **Role:** ML Engineer at AI Startup or Large Tech Company
- **Experience:** 3+ years in data science/ML engineering
- **Primary Languages:** Python (daily), R (statistical analysis)
- **Tools:** Jupyter notebooks, scikit-learn, PyTorch, pandas
- **Needs:** Fast C++ inference engines, optimized linear algebra libraries

**Background:**
Morgan builds ML models in Python but frequently encounters performance bottlenecks in inference. There are excellent C++ libraries for fast matrix operations and model serving, but Morgan lacks the C++ expertise to integrate them. Backend engineers are backlogged for weeks, so Morgan is stuck with slow NumPy/pure-Python implementations.

**Pain Points:**
- **Can't Use Optimized Libraries:** C++ inference engines would speed up inference 50x, but integration requires backend engineer support (2+ week wait)
- **Dependency on Backend Team:** Every FFI integration requires opening a ticket, waiting in backlog, explaining requirements to non-ML engineer
- **Jupyter Workflow Broken:** Can't load C++ libraries directly in notebooks, must use pre-built Python wrappers (limited flexibility)
- **Performance Sacrifices:** Uses slower Python libraries because FFI integration isn't worth the hassle

**Current Workarounds:**
- Sticks to pure Python/NumPy (10-50x slower than C++)
- Uses pre-built Python bindings when available (limited to popular libraries, can't use niche/internal C++ code)
- Escalates to backend team for critical performance needs (weeks of delay)
- Rewrites C++ algorithms in Python (slow, error-prone)

**Goals:**
- **Primary:** Load C++ models and libraries directly from Jupyter notebooks without backend support
- **Secondary:** Independently integrate performance-critical C++ code without waiting weeks
- **Tertiary:** Maintain Python-first workflow while accessing C++ performance

**Success Metric for Polyglot:**
Morgan independently integrates C++ inference in <30 minutes: "I can use that optimized C++ library without filing a backend ticket"

**Polyglot Adoption Journey:**
1. Hears about Polyglot from ML Slack channel: "Load C++ from Python without PyO3"
2. Tries in Jupyter notebook, discovers C++ header loading is v1.0+ feature (not MVP)
3. Uses MVP's subprocess approach as workaround (better than nothing, but not "magic moment")
4. Waits for v1.0 with C++ direct integration
5. When v1.0 ships: Loads C++ inference engine in 10 minutes, sees 50x speedup
6. "Magic moment": No backend engineer needed, works in notebook, production-ready
7. Champions Polyglot to ML team, becomes community contributor

---

### Target User Segmentation

**By Experience Level:**
- **Primary Target:** Intermediate developers (3-7 years) - experienced enough to recognize FFI pain, junior enough to appreciate simplicity
- **Secondary:** Senior developers (8+ years) - may contribute to Polyglot ecosystem, early adopters willing to try new tools
- **Non-Target:** Junior developers (<3 years) - may struggle with Polyglot setup (PostgreSQL/Redis/InfluxDB), not yet facing FFI pain

**By Company Size:**
- **Primary Target:** Startups (10-100 employees) - fast iteration, willing to adopt new tools, polyglot tech stacks common
- **Secondary:** Mid-size (100-1000 employees) - cautious but open to OSS, have FFI and automation pain points
- **Non-Target (MVP):** Enterprise (1000+ employees) - require commercial support, compliance, SLAs (not available in MVP)

**By Programming Background:**
- **Primary Target:** Python/Node developers needing performance (Rust/C++ integration pain)
- **Secondary:** Rust developers wanting easy Python integration (PyO3 alternative)
- **Tertiary:** Polyglot teams (multiple languages in production, frequent FFI needs)

**By Use Case:**
- **Primary Target:** Cross-language library integration (FFI pain point)
- **Secondary:** Automation workflows with priority control (DevOps pain point)
- **Tertiary:** Data pipelines (data scientists, ML engineers)

**Anti-Personas (Who Polyglot Is NOT For):**
- **Real-time Systems Developers:** Serialization overhead (5-10ms) too high for microsecond-latency requirements
- **Enterprise Architects:** Need commercial support, SLAs, compliance certifications (not available in OSS v1.0)
- **Non-Technical Users:** CLI-only interface, no GUI (requires command-line comfort)
- **Monolingual Teams:** No cross-language needs, no FFI pain (cron is sufficient for automation)
- **Hobbyists/Students:** Polyglot setup (PostgreSQL, Redis, InfluxDB) may be overkill for learning projects

---

## User Journeys

### Journey 1: Alex's First Cross-Language Call (Primary Value Prop - FFI Magic)

**Persona:** Alex (Backend Polyglot Engineer)

**Scenario:** Alex needs to integrate a Rust library for fast JSON parsing into a Python microservice.

**Steps:**

1. **Problem Recognition** (Day 1, 9 AM)
   - Python JSON parsing is bottleneck (1000 req/sec limit)
   - Discovers `simd-json` Rust library (5x faster than Python's `json` module)
   - Googles "use Rust from Python" → finds PyO3 docs, sees 50-line example
   - Estimates 4-6 hours to integrate with PyO3 → puts in backlog for "later"

2. **Discovery** (Day 3, 2 PM)
   - Sees Hacker News post: "Polyglot: Call Rust from Python in 15 minutes (no FFI setup)"
   - Clicks link, reads landing page: "No build scripts, no type annotations, no segfaults"
   - Skeptical but intrigued: "This sounds too good to be true"
   - Clicks "Getting Started" guide

3. **Installation** (Day 3, 2:10 PM - 2:25 PM)
   - Follows installation guide: `cargo install polyglot`
   - Polyglot checks dependencies: PostgreSQL, Redis, InfluxDB
   - **Friction Point #1:** Doesn't have InfluxDB installed locally
   - Follows doc link to quick setup: `brew install influxdb` (5 minutes)
   - Runs `polyglot init` → creates `polyglot.toml` config with database URLs
   - **Friction Point #2:** PostgreSQL connection fails (wrong port in config)
   - Checks troubleshooting guide: updates `DATABASE_URL` to correct port
   - Runs `polyglot services status` → all services green ✓
   - **Time:** 15 minutes (would be 5 minutes if InfluxDB pre-installed)

4. **First Pipeline** (Day 3, 2:25 PM - 2:35 PM)
   - Copies `hello_world.pg` example from docs
   - Modifies to call Rust `simd-json` library:
     ```polyglot
     {|} FastJsonParse
     [|] <json_string :pg.string
     [W] |W.Rust
     [r] |parse_json
        [|] <input :pg.string << $json_string
        [|] >parsed :pg.serial >> $result
     [|] >parsed :pg.serial
     {x}
     ```
   - **Time:** 10 minutes (most time spent reading docs on syntax)

5. **Compilation & Registration** (Day 3, 2:35 PM - 2:36 PM)
   - Runs `polyglot compile fast_json.pg`
   - Compiles successfully! IR generated
   - Runs `polyglot register fast_json`
   - Registered to PostgreSQL registry
   - **Time:** 1 minute

6. **Execution & Validation** (Day 3, 2:36 PM - 2:40 PM)
   - Runs `polyglot activate fast_json`
   - Runs `polyglot trigger fast_json --input '{"key": "value"}'`
   - **Friction Point #3:** Error: "Rust runtime wrapper not found"
   - Checks docs: Realizes Rust wrapper is v1.0+, not MVP
   - **Pivot:** Uses Python wrapper with subprocess call to Rust (Strategy 1)
   - Modifies pipeline to use Python wrapper calling Rust binary
   - Re-compiles, triggers → Success! Parsed JSON returned
   - **Time:** 4 minutes (includes learning about runtime wrapper limitation)

7. **Success Moment** (Day 3, 2:40 PM)
   - **Total time:** 20 minutes (installation + pipeline + execution)
   - **Alternative (PyO3):** Would have taken 4-6 hours
   - **Time savings:** 95% reduction (3.5-5.5 hours saved)
   - **Reaction:** "This is incredible. I just integrated Rust in 20 minutes. With PyO3 this would have taken me the rest of the day."
   - **Adoption decision:** "I'm using Polyglot for all future FFI integrations"

8. **Production Integration** (Day 4)
   - Migrates production Python service to use Polyglot pipeline
   - Benchmarks: 5x JSON parsing speedup (as expected)
   - Deploys to staging, then production
   - **Result:** 1000 req/sec → 5000 req/sec throughput improvement

9. **Community Engagement** (Day 7)
   - Writes blog post: "How I replaced PyO3 with Polyglot"
   - Posts to Hacker News, Reddit r/Python, Twitter
   - Becomes early adopter, contributes Python wrapper improvements

**Friction Points Identified:**
- **Installation:** InfluxDB dependency not obvious (needs better onboarding docs)
- **Database Config:** PostgreSQL connection errors confusing for first-timers (needs better error messages)
- **Runtime Wrapper Limitations:** MVP only has Python wrapper, Rust/Node in v1.0+ (needs clear documentation)

**Success Factors:**
- 95% time savings vs. PyO3 (15 minutes vs. 4 hours)
- No build scripts, type annotations, or segfaults
- Clear error messages when things go wrong
- Fast path to success (20 minutes total)

---

### Journey 2: Jordan's Queue Priority Setup (Secondary Value Prop - Priority Management)

**Persona:** Jordan (DevOps Automation Engineer)

**Scenario:** Nightly backup job is blocking morning API traffic, causing customer incidents.

**Steps:**

1. **Incident Trigger** (Monday, 7:15 AM)
   - PagerDuty alert: API response time > 5 seconds
   - Logs show database connection pool exhausted
   - Backup job started at 2 AM, still running at 7 AM (normally finishes by 5 AM)
   - Customer complaints rolling in, revenue impact mounting

2. **Manual Intervention** (Monday, 7:20 AM)
   - SSH into server, finds backup job consuming 90% of database connections
   - Manually kills backup job to restore API service
   - API recovers, incident resolved
   - **Time cost:** 30 minutes on-call response + 1 hour post-incident cleanup

3. **Root Cause Analysis** (Monday, 10 AM)
   - Backup job data volume grew 3x (didn't anticipate growth)
   - Cron has no priority control (backup = API calls = equal priority)
   - Current workaround: manual queue management script (breaks frequently)
   - **Decision:** Need declarative priority management

4. **Research** (Monday, 11 AM)
   - Googles "cron job priority management" → finds Airflow, but setup is heavy
   - Asks in DevOps Slack: "How do you handle priority in automation?"
   - Colleague mentions Polyglot: "Has built-in priority queue management"
   - Reads docs, sees `[Q] |Q.Queue.Assign` syntax

5. **Installation & Setup** (Monday, 2 PM - 2:30 PM)
   - Installs Polyglot: `cargo install polyglot`
   - Already has PostgreSQL/Redis for other services (no friction)
   - Installs InfluxDB for metrics: `apt install influxdb2`
   - Runs `polyglot init`, configures database URLs
   - **Time:** 30 minutes

6. **Migrate Backup Job** (Monday, 2:30 PM - 3:00 PM)
   - Converts backup cron job to Polyglot pipeline:
     ```polyglot
     {|} NightlyBackup
     [t] |T.Cron
        [|] <schedule :pg.string << "0 2 * * *"  // 2 AM daily
     [Q] |Q.Queue.Assign
        [|] <queue :pg.string << "low_priority"
        [|] <pause_on_resource_threshold :pg.bool << #;Boolean;True
     [r] |RunBackup
        [|] <database :pg.string << "production_db"
     {x}
     ```
   - Compiles, registers, activates
   - Tests manually: `polyglot trigger NightlyBackup`
   - Backup runs successfully
   - **Time:** 30 minutes (mostly learning Polyglot syntax)

7. **Configure Priority Logic** (Monday, 3:00 PM - 3:30 PM)
   - Creates high-priority queue for API operations
   - Configures automatic pause: low-priority queue pauses when CPU > 80% or DB connections > 80%
   - Migrates one API operation to high-priority queue for testing
   - **Time:** 30 minutes

8. **Testing** (Monday, 4:00 PM)
   - Simulates load: triggers backup + high-priority API call simultaneously
   - Monitors: backup pauses when API work arrives, resumes when API completes
   - **Success!** Priority system works as expected

9. **Production Rollout** (Tuesday - Friday)
   - Migrates 10 batch jobs to Polyglot low-priority queue
   - Migrates 5 critical deployments to high-priority queue
   - Monitors for 1 week: zero incidents of low-priority work blocking high-priority

10. **Success Moment** (Following Monday, 7:15 AM)
    - Same scenario: backup runs long into morning hours
    - But this time: API traffic triggers automatic pause of backup
    - **No PagerDuty alert**
    - **No manual intervention**
    - Jordan wakes up, checks logs: "Polyglot handled it automatically"
    - **Reaction:** "This just paid for itself. No more 3 AM pages to pause batch jobs."

11. **Ecosystem Contribution** (Week 2)
    - Writes internal case study: "How Polyglot reduced on-call burden by 80%"
    - Shares on company blog, HackerNews
    - Retires 200-line custom queue management script
    - **ROI:** 5-10 hours/month saved on manual intervention

**Friction Points Identified:**
- **Queue Configuration:** Not obvious how to define custom queues (needs better docs)
- **Resource Threshold Tuning:** Took trial-and-error to find right pause thresholds (needs monitoring dashboard)

**Success Factors:**
- Declarative priority in workflow definition (not separate config)
- Automatic pause/resume (no manual intervention)
- Eliminated custom queue scripts (reduced maintenance burden)

---

### Journey 3: Morgan's ML Inference Speedup (Tertiary Value Prop - Blocked on v1.0)

**Persona:** Morgan (Data Scientist / ML Engineer)

**Scenario:** Needs to integrate C++ inference engine for 50x speedup.

**Steps:**

1. **Problem Recognition** (Week 1)
   - Python inference is too slow (10 inferences/sec, need 500/sec)
   - Finds `fast_inference.cpp` C++ library (50x faster than scikit-learn)
   - Googles "use C++ from Python" → finds pybind11, overwhelmed by complexity
   - Files ticket with backend team: "Need FFI wrapper for fast_inference.cpp"
   - Backend team response: "Backlogged for 2 weeks"

2. **Discovery** (Week 2)
   - Hears about Polyglot from ML Slack: "Load C++ from Python without PyO3"
   - Reads docs, sees "C++ header loading" feature
   - **Critical Discovery:** C++ direct integration is v1.0+ (not MVP)
   - MVP only supports Python/Node wrappers with subprocess approach

3. **Workaround Attempt** (Week 2)
   - Tries Polyglot MVP with Python wrapper calling C++ subprocess
   - Works, but subprocess overhead negates 50x speedup (only 5x improvement)
   - **Decision:** Wait for v1.0 with C++ direct integration

4. **Outcome:**
   - Morgan doesn't adopt Polyglot MVP (blocked on missing feature)
   - Waits 6 months for v1.0 release
   - **When v1.0 ships:** Becomes immediate adopter, sees 50x speedup
   - **Lesson:** Tertiary persona (data scientists) needs v1.0 for full value

**Friction Points:**
- C++ integration is post-MVP (blocks key use case for data scientists)
- Subprocess workaround insufficient for performance needs

**Recommendation:**
- Prioritize C++ direct integration in v1.0 roadmap
- This persona can wait 6 months if FFI value prop is strong enough

---

## Assumptions & Constraints

### Technical Assumptions

**User Environment:**
- Users have local machine with 4GB+ RAM, 2+ CPU cores (modern laptop/desktop)
- Users can install and configure PostgreSQL 14+, Redis 7+, InfluxDB 2.x locally
- Users have Rust toolchain installed (`rustup`, `cargo`) or can follow installation guide
- Users have Python 3.11+ with `pip`/`uv` installed for MVP runtime wrapper
- Users are comfortable with command-line tools (no GUI requirement)
- Local development environment assumed (not cloud-first deployment)

**Network & Infrastructure:**
- Local network sufficient (no distributed deployment in MVP)
- Database connections on localhost (PostgreSQL, Redis, InfluxDB)
- No firewall/VPN complications for local services
- Internet connection for `cargo install` and package downloads

**User Technical Proficiency:**
- Target users are experienced developers (3+ years professional experience)
- Users understand async programming concepts (callbacks, promises, event loops)
- Users familiar with basic database operations (connection strings, migrations)
- Users can read and write basic YAML/TOML configuration files
- Users understand FFI concepts conceptually (even if they haven't implemented FFI before)
- Users comfortable debugging compilation errors and reading logs

---

### Project Constraints

**Team & Resources:**
- **Team Size:** Solo developer (hhj) + potential community contributions (post-v1.0)
- **Budget:** $0 (pure open-source project, no paid services or infrastructure)
- **Timeline:** MVP target 3-6 months based on solo development pace (10-20 hours/week)
- **Maintenance:** hhj commits 10-20 hours/week long-term for maintenance, community management
- **Infrastructure:** Self-hosted only in MVP (no managed cloud services, no SaaS offering)
- **Support:** Community-driven only (GitHub issues, Discord), no commercial support, no SLAs

**Technical Constraints:**
- **Performance:** Acceptable overhead for automation workflows (5-10ms serialization OK), not suitable for real-time systems (<1ms latency requirements)
- **Scalability:** MVP targets 100-1000 concurrent pipeline instances, not millions (horizontal scaling in v1.0+)
- **Compatibility:** Linux/macOS first (Windows deferred to v1.0 due to InfluxDB/Redis compatibility)
- **Dependencies:** Must use battle-tested, stable crates (no experimental dependencies, minimum 1.0 releases preferred)
- **Observability:** Basic logging sufficient for MVP, advanced telemetry (OpenTelemetry, dashboards) in v1.0+

**Development Constraints:**
- **Solo Developer Velocity:** 10-20 hours/week, ~40-80 hours/month, ~200-400 hours for 6-month MVP
- **Rust Learning Curve:** hhj is experienced Rustacean, but some async patterns may require research
- **Community Contributions:** Cannot rely on community for MVP (post-v1.0 feature), hhj must implement all core features
- **Testing Burden:** Comprehensive testing required but time-consuming (aiming for 80% code coverage minimum)

---

### Regulatory & Compliance Constraints

**None for MVP:**
- Polyglot is a developer tool, not in a regulated domain (no HIPAA, SOC2, GDPR requirements)
- No PII handling (pipelines process user-defined data, Polyglot doesn't store/transmit sensitive data)
- Open-source license: MIT (permissive, no copyleft restrictions)
- No export control restrictions (general-purpose developer tool)

---

### Market & Adoption Constraints

**Competition:**
- Must compete with free, established alternatives (pybind11, PyO3, cron, Airflow)
- Success depends on developer adoption (GitHub stars, community contributions, word-of-mouth)
- No revenue model in v1.0 (pure OSS, potential for sponsorships/consulting post-v1.0 if successful)
- Developer tool market is crowded (differentiation via runtime type resolution innovation)

**Ecosystem Dependency:**
- Success requires community runtime wrappers (Node, Rust, Go post-MVP)
- If no community contributions, hhj must implement all wrappers (limits language support)
- Package registry depends on adoption (empty registry = low value, chicken-egg problem)

---

### Scope Constraints (Explicit Boundaries)

**MVP Will NOT Include:**
- Multi-tenancy (single-user/team deployment only)
- Authentication/authorization (local-only deployment, no multi-user access control)
- Web UI dashboard (CLI-first, observability dashboards in v1.0+)
- Distributed deployment (all 3 services run on single machine in MVP)
- Windows support (Linux/macOS only, Windows in v1.0+)
- Real-time streaming (batch/event-driven only, not Kafka/Flink competitor)
- Built-in secrets vault (users must use environment variables or external tools like HashiCorp Vault)

**Technical Limitations:**
- **Serialization Overhead:** 5-10ms per cross-language call (acceptable for automation, not for hot loops)
- **Single Queue:** MVP has one dispatch queue (multi-queue priority management in v0.2+)
- **Python Runtime Only:** MVP proves concept with Python wrapper, Node/Rust/Go in v1.0+

---

## Product Scope

### MVP - Minimum Viable Product

**Philosophy:** Foundation-first approach - language integration is built on top of the automated pipeline system, not alongside it.

**Phase 1: Core Pipeline System (MVP Foundation)**

The MVP proves that Polyglot's async architecture works end-to-end. Success = a developer can write a `.pg` pipeline, compile it, and execute it through the 3-backend service orchestration.

**Must-Have Components:**

1. **Language Syntax & Compilation**
   - Complete Polyglot syntax specification (v0.0.2 already exists)
   - Lexer/Parser that transforms `.pg` files into Intermediate Representation (IR)
   - IR validation and storage

2. **3 Backend Services**
   - **Trigger Monitor:** Continuously monitors pipeline triggers (synchronous component)
   - **Queue Manager:** Manages dispatch queue (single queue for MVP)
   - **Runner:** Executes pipeline instances

3. **Database Integration**
   - PostgreSQL for IR storage, pipeline registry, activation state
   - Schema for pipelines, instances, execution logs

4. **Pipeline Lifecycle**
   - Define pipelines using Polyglot syntax (`{|}` definition, `{x}` end, `[r]` return, `[p]` parallel, etc.)
   - Compile `.pg` files to IR
   - Register pipelines to registry
   - Activate/deactivate pipelines via CLI
   - Pipeline instance states: Created → Queued → Running → Exited

5. **Basic Trigger System**
   - At least one trigger type functional (manual CLI trigger or time-based)
   - Demonstrates trigger monitoring → queue → execution flow

6. **Basic Runtime Wrapper**
   - Python runtime wrapper functional (proof-of-concept)
   - Demonstrates pipeline can execute code in another language
   - Foundation for future FFI abstraction
   - Basic type conversion (serialization-based, Strategy 1)

7. **CLI Interface**
   - Commands to compile, register, activate, trigger pipelines
   - View pipeline status and execution logs
   - Basic debugging capabilities

**MVP Success Gate:**
- ✅ Developer writes `.pg` pipeline that orchestrates async operations
- ✅ Pipeline compiles to IR and registers successfully
- ✅ Trigger monitoring detects when pipeline should run
- ✅ Queue Manager queues instance and Runner executes it
- ✅ Pipeline executes Python code via runtime wrapper
- ✅ End-to-end flow works: Define → Compile → Register → Activate → Trigger → Queue → Execute

### Growth Features (Post-MVP)

**v0.2-v0.3: Advanced Queue Management & Priority Control**

This phase delivers the priority management value proposition.

- **Multiple Dispatch Queues:** User-defined queues with custom priority algorithms
- **Queue Routing:** `[Q] |Q.Queue.Assign` syntax for routing pipelines to specific queues
- **Pause/Resume Functionality:** Pause low-priority queues when high-priority work arrives
- **Queue Characteristics:** Rate limiting, cost optimization, resource isolation (GPU vs CPU)
- **Resource Monitoring:** Track CPU, RAM, database connection pool usage per pipeline

**v0.5-v1.0: Full Language Integration Layer (Phase 2)**

This phase delivers the cross-language FFI magic.

- **Advanced Runtime Wrappers:** Node, Rust, Go runtime wrappers fully functional
- **Dual-Strategy Type Conversion:**
  - Strategy 1: Universal translator (serialization) - already in MVP
  - Strategy 2: Direct language-to-language conversion using existing FFI tools
  - Automatic fallback logic
- **FFI Abstraction:** Auto-generate language bridges without manual setup
- **Type Bridge:** Handle dynamic-to-static type conversions at runtime
- **"Python calls C++" magic moment:** `pg.load_cpp_header("fast_math.h")` works seamlessly

**v0.5-v1.0: Observability Stack**

- **OpenTelemetry Integration:** Built-in tracing and metrics from day one
- **InfluxDB Time-Series Logging:** Replace PostgreSQL logging with proper time-series DB
- **Dashboard:** Visualize pipeline execution, queue states, resource usage
- **Alerting:** Notify on pipeline failures, resource saturation, queue backlogs

---

### Out of Scope (Explicitly Excluded)

To maintain focus and ship MVP in 6 months, the following capabilities are explicitly OUT OF SCOPE. This section prevents scope creep and sets clear expectations.

**Polyglot Is NOT:**
- **General-Purpose Programming Language:** Polyglot is for automation workflows and cross-language orchestration, not application development (use Python, Rust, Node for apps)
- **Real-Time Systems Framework:** Serialization overhead (5-10ms) makes Polyglot unsuitable for sub-millisecond latency requirements (use native FFI for real-time)
- **Stream Processing Engine:** Polyglot is batch/event-driven, not a Kafka/Flink competitor (integrate Polyglot WITH streaming platforms via triggers)
- **Container Orchestration:** Polyglot is not a Kubernetes replacement (Polyglot can RUN in Kubernetes, doesn't replace it)
- **ETL Platform:** While Polyglot can do ETL, it's not specialized for it like Airflow/dbt (use those if ETL is primary need)

---

**MVP Will NOT Include:**

**User Interface:**
- ❌ **Web UI Dashboard** - CLI-first in MVP, dashboards in v1.0+ (reduced scope, faster MVP)
- ❌ **Visual Workflow Designer** - No drag-and-drop GUI for `.pg` files (code-first approach)
- ❌ **Mobile Apps/Clients** - Desktop/server only (not on roadmap)

**Security & Access Control:**
- ❌ **Authentication/Authorization** - Local-only deployment, no multi-user access control in MVP
- ❌ **Multi-Tenancy** - Single-user/team deployment only, no tenant isolation
- ❌ **Built-in Secrets Vault** - Users integrate with existing tools (HashiCorp Vault, AWS Secrets Manager, env variables)
- ❌ **RBAC (Role-Based Access Control)** - No role/permission system in MVP
- ❌ **Audit Logging** - Basic execution logs only, no compliance-level audit trails

**Deployment & Infrastructure:**
- ❌ **Distributed Deployment** - All 3 services run on single machine in MVP (horizontal scaling in v1.0+)
- ❌ **Windows Support** - Linux/macOS only in MVP due to InfluxDB/Redis compatibility (Windows in v1.0+)
- ❌ **Cloud Provider SDKs** - No built-in AWS/GCP/Azure integrations (users add via runtime wrappers)
- ❌ **Kubernetes Operator** - Deploy Polyglot in K8s manually, no operator automation (v1.0+)
- ❌ **Docker Compose Development Environment** - Users install PostgreSQL/Redis/InfluxDB manually (compose file in v1.0+)
- ❌ **Managed/SaaS Offering** - Self-hosted only, no cloud-hosted Polyglot service

**Advanced Features:**
- ❌ **Compensation Pipelines** - Undo/rollback system for failed workflows (v1.0+, complex to implement)
- ❌ **Live Code Migration** - Move running pipelines between nodes without restart (v1.0+, advanced feature)
- ❌ **AI-Powered Queue Optimization** - ML-based priority tuning (v1.0+, experimental)
- ❌ **Multiple Dispatch Queues** - MVP has single queue, multi-queue in v0.2+ (priority management deferred)
- ❌ **Resource-Based Triggers** - Trigger on CPU/RAM thresholds (v0.2+, requires Resource Monitor subservice)
- ❌ **File Watch Triggers** - Trigger on file system changes (v0.2+, MVP has time-based + manual only)
- ❌ **Webhook Triggers** - HTTP webhook support (v0.2+, adds HTTP server complexity)

**Language Support:**
- ❌ **JVM Languages (Java, Kotlin, Scala)** - Community contribution only, not core supported
- ❌ **NET Languages (C#, F#)** - Community contribution only
- ❌ **Compiled Languages without Runtime (C/C++ Direct Execution)** - C++ header loading in v1.0+, not MVP
- ❌ **Rust Runtime Wrapper** - MVP proves concept with Python only, Rust wrapper in v1.0+
- ❌ **Node Runtime Wrapper** - MVP is Python-first, Node in v1.0+
- ❌ **Go Runtime Wrapper** - v1.0+
- ❌ **Dual-Strategy Type Conversion (Strategy 2)** - MVP uses serialization (Strategy 1), direct FFI in v1.0+

**Observability & Monitoring:**
- ❌ **OpenTelemetry Integration** - Basic logging only in MVP, distributed tracing in v1.0+
- ❌ **Monitoring Platform Plugins (Datadog, New Relic)** - No native integrations, users export logs manually
- ❌ **Real-Time Dashboard** - No live visualization of pipeline execution in MVP
- ❌ **Alerting System** - No PagerDuty/Slack/email alerts on failures in MVP (users poll logs)
- ❌ **Metrics Export (Prometheus, InfluxDB full integration)** - Basic InfluxDB writes in MVP, full metrics pipeline in v1.0+

**Development Tooling:**
- ❌ **IDE Plugins (VSCode, IntelliJ, Vim)** - No syntax highlighting, autocomplete, or debuggers in MVP (v1.0+)
- ❌ **LSP Server** - No Language Server Protocol implementation for `.pg` files (v1.0+)
- ❌ **Debugger** - No step-through debugging for pipelines in MVP (use logs)
- ❌ **CI/CD Platform Plugins (GitHub Actions, GitLab CI)** - Manual integration only in MVP (plugins in v1.0+)
- ❌ **Test Framework** - No built-in testing DSL for `.pg` files (users write tests in Python/Rust)

**Package Management:**
- ❌ **Package Registry** - No central registry for third-party `.pg` packages in MVP (v1.0+)
- ❌ **Package Versioning** - No semantic versioning or dependency resolution for packages in MVP
- ❌ **Package Discovery** - No search/browse interface for community packages

**Data & Storage:**
- ❌ **Caching System** - No built-in caching layer for pipeline outputs (users implement manually)
- ❌ **Object Storage Integration (S3, GCS, Azure Blob)** - No native cloud storage connectors (users use runtime wrappers)
- ❌ **Database Connectors** - No built-in PostgreSQL/MySQL/MongoDB query syntax (users write in Python/Rust)

---

**Rationale for Exclusions:**

**MVP Focus:** The MVP proves Polyglot's core value proposition (FFI abstraction + async orchestration). Advanced features risk delaying launch by 6-12 months, missing market window.

**Solo Developer Constraint:** hhj has 10-20 hours/week. Scope must be ruthlessly prioritized. Community contributions can add features post-v1.0.

**Market Validation First:** Launch minimal product, gather user feedback, iterate. Building features users don't want wastes effort. Let real usage drive v1.0+ roadmap.

**Avoid "Nice to Have" Trap:** Every excluded feature was tempting ("wouldn't a dashboard be great?"). But each adds 20-40 hours of dev time. MVP ships in 6 months by saying "no" to 80% of ideas.

---

**Post-MVP Roadmap:**
These features are deferred, not abandoned. Based on user feedback and adoption, v0.2-v1.0 will prioritize:
1. **v0.2-v0.3:** Priority queue management (multiple queues, resource triggers) - if DevOps adoption is strong
2. **v0.5:** Additional runtime wrappers (Node, Rust, Go) - if FFI adoption is strong
3. **v0.8:** Observability stack (OpenTelemetry, dashboards) - if production usage demands it
4. **v1.0:** Community-requested features based on GitHub issues, usage patterns

---

### Vision (Future)

**v1.0+ Advanced Features:**

- **Compensation Pipelines:** Undo system for failed workflows
- **Irreversibility Warnings:** Warn before executing irreversible operations
- **Code-Level Checkpoints:** Pause pipelines mid-execution (not just process-level)
- **AI-Powered Queue Optimization:** ML-based priority tuning and resource allocation
- **Live Code Migration:** Move running pipelines between nodes without restart
- **Distributed Execution:** Horizontal scaling across multiple Runner instances
- **Kafka Queue Backend:** Replace Redis for massive scale
- **Package Ecosystem:** Third-party runtime wrappers, standard library utilities, example pipelines
- **IDE Integration:** Syntax highlighting, debuggers, LSP server for `.pg` files
- **Visual Pipeline Designer:** GUI for non-developers to create workflows
- **Cloud-Native Deployment:** Kubernetes operators, Helm charts, SaaS offering

---

{{#if domain_considerations}}

## Domain-Specific Requirements

{{domain_considerations}}

This section shapes all functional and non-functional requirements below.
{{/if}}

---

## Innovation & Novel Patterns

**Core Innovation: Runtime vs. Compile-Time Type Crossing**

Polyglot challenges a fundamental assumption in cross-language integration: that type information must be known at compile time.

**The Paradigm Shift:**

Traditional FFI tools (pybind11, PyO3, GraalVM) operate at compile time:
- C++ library compiled → pybind generates bridge → Python bindings compiled
- All type information must be known upfront
- Dynamic types (Python list with runtime-determined size) cannot bridge to static types (Rust fixed-size array)

**Polyglot's Innovation:**
- **Async architecture enables runtime type resolution**
- Python passes dynamic data → Polyglot inspects at runtime → Determines actual types/sizes → Converts to target language requirements
- Example: Python list `[1, 2, 3]` → Runtime inspection reveals 3 elements → Rust receives `[i32; 3]`
- This unlocks conversions that were **previously impossible** with compile-time tools

**Novel Architecture Pattern: Dual-Strategy Type Conversion**

Instead of choosing between "universal intermediate format" OR "direct language-to-language conversion," Polyglot does **both**:

1. **Strategy 1 (Universal Translator):** Serialize all data through Polyglot's intermediate representation
   - Works for any language pair
   - `py\str` → `pg\string` → `rust\String`
   - Guaranteed to work, may have serialization overhead

2. **Strategy 2 (Direct Conversion):** Leverage existing FFI tools when optimal
   - Python → C++ via pybind (if installed and beneficial)
   - Rust → Python via PyO3 (if available)
   - Falls back to Strategy 1 if direct path unavailable

3. **Automatic Selection:** Polyglot chooses the best strategy at runtime based on:
   - Available FFI tools on the system
   - Performance characteristics of the conversion
   - Type compatibility between source and target

This "best of both worlds" approach is novel in the FFI space.

**DSL Innovation: Pipeline-Centric Language Design**

Polyglot introduces a new programming paradigm:
- Pipelines as first-class citizens (`[|]` syntax)
- Declarative queue routing (`[Q] |Q.Queue.Assign`)
- Runtime wrappers as language construct (`[W] |W.Python3.11`)
- Priority and resource management as **descriptive code**, not infrastructure config

This is a new DSL category: **async multi-runtime workflow orchestration language**.

**Rethinking the Problem Space:**

Current thinking: "Cross-language integration is an infrastructure problem - set it up once, use FFI tools"

Polyglot's challenge: "Cross-language integration should be a **language feature** - write one line of code, integration happens automatically"

This shifts FFI from infrastructure layer to language layer.

### Validation Approach

**Innovation Validation Strategy:**

**1. Technical Validation: Prove Runtime Type Resolution Works**

- **Prototype Test:** Build proof-of-concept that demonstrates:
  - Python function returns list of unknown size
  - Polyglot captures at runtime, determines size is 5
  - Rust function receives `[i32; 5]` correctly
  - No compile-time type information required
- **Success Criteria:** Type conversion succeeds for dynamic → static language direction
- **Timeline:** During MVP development (critical architectural validation)

**2. Performance Validation: Serialization Overhead Acceptable**

- **Benchmark:** Compare Strategy 1 (serialization) vs. traditional FFI for common operations
- **Questions:**
  - Is serialization overhead <10ms for typical data sizes?
  - At what data size does overhead become prohibitive?
  - Does Strategy 2 (direct conversion) provide measurable performance benefit?
- **Success Criteria:** Overhead acceptable for automation workflows (not real-time systems)
- **Fallback:** If overhead too high, prioritize Strategy 2 implementation in v0.5

**3. Developer Experience Validation: "Magic Moment" Test**

- **User Testing:** 5-10 developers (early adopters) try the "Python calls C++" example
- **Measure:**
  - Time to get working: Polyglot vs. manual pybind setup
  - Friction points and confusion
  - "Would you use this in production?" feedback
- **Success Criteria:** Average time <15 minutes with Polyglot vs. >2 hours with manual FFI
- **Timeline:** After MVP completion, before v1.0 announcement

**4. Architectural Validation: No Fundamental Blockers**

- **Risk Areas:**
  - Can we reliably detect types at runtime for all supported languages?
  - Are there edge cases where runtime type crossing fails?
  - Does async architecture introduce unacceptable latency?
- **Validation:** Build MVP, test against diverse use cases, document limitations
- **Success Criteria:** No showstopper discovered that invalidates core architecture
- **Contingency:** If runtime crossing fails for certain type combinations, document as "not supported" and rely on Strategy 1 serialization

**5. Community Validation: Check for Prior Art**

- **Web Research:** Has anyone attempted runtime type crossing before? (Search: "runtime FFI type resolution", "dynamic static type bridge", "async cross-language integration")
- **Learning:** If similar approaches exist, study their successes and failures
- **Differentiation:** Document how Polyglot differs from prior art
- **Timeline:** Before v1.0 announcement (inform positioning)

---

## Developer Tool Specific Requirements

As a programming language and framework, Polyglot must meet developer tool standards for installation, documentation, examples, and ecosystem integration.

### Language & Runtime Support

**Polyglot Language Syntax:**
- `.pg` file format for pipeline definitions
- Complete syntax specification (v0.0.2 already exists in `docs/user/`)
- Support for all documented operators: `{|}` (pipeline definition), `{x}` (block end), `[r]` (return), `[p]` (parallel), `[Q]` (queue), `[W]` (wrapper), `[|]` (IO marker), etc.
- Type system: Polyglot types with runtime conversion to target languages
- Error handling constructs
- Comments and documentation syntax

**Runtime Wrapper Support Matrix:**

| Runtime | MVP (v0.1) | Growth (v0.5-v1.0) | Vision (v1.0+) |
|---------|-----------|-------------------|----------------|
| Python 3.11+ | ✅ Full support | ✅ Enhanced | ✅ Complete FFI |
| Node.js | ❌ Not supported | ✅ Full support | ✅ Complete FFI |
| Rust | ❌ Not supported | ✅ Full support | ✅ Complete FFI |
| Go | ❌ Not supported | ✅ Full support | ✅ Complete FFI |
| Ruby | ❌ Not supported | ❌ Not supported | ✅ Community-contributed |
| Deno | ❌ Not supported | ❌ Not supported | ✅ Community-contributed |
| C/C++ | ❌ Not supported | ❌ Not supported | ✅ Direct header loading |

**Implementation Language:**
- Polyglot implemented in **Rust** for performance, async capabilities, and serialization excellence
- Leverages Tokio for async runtime
- Uses `serde` for IR serialization

### Installation Methods

**MVP Installation:**
- **Source Build:** Clone repository, `cargo build --release`
- **Prerequisites:** Rust toolchain, PostgreSQL, Redis
- **Installation script:** Setup database schema, initialize configuration
- **Binary output:** `polyglot` CLI executable

**v1.0 Installation Goals:**
- **Package Managers:**
  - Cargo: `cargo install polyglot`
  - Homebrew (macOS): `brew install polyglot`
  - apt/yum (Linux): Platform-specific packages
  - Chocolatey (Windows): `choco install polyglot`
- **Docker Image:** `docker pull polyglot/polyglot:latest`
- **Cloud Deploy:** One-click deploy to AWS/GCP/Azure with managed PostgreSQL/Redis

**Configuration:**
- YAML or TOML config file (`polyglot.config.yaml`)
- Environment variables for PostgreSQL/Redis connection strings
- Default config generated on first run

### CLI API Surface

**MVP Commands:**

```bash
# Compilation
polyglot compile <file.pg>           # Compile .pg to IR
polyglot validate <file.pg>          # Syntax validation only

# Registry Management
polyglot register <file.pg>          # Register pipeline to database
polyglot list                        # List all registered pipelines
polyglot show <pipeline-name>        # Show pipeline details

# Activation
polyglot activate <pipeline-name>    # Activate pipeline for triggering
polyglot deactivate <pipeline-name>  # Deactivate pipeline

# Execution
polyglot trigger <pipeline-name>     # Manual trigger
polyglot status <pipeline-name>      # View execution status
polyglot logs <instance-id>          # View instance logs

# Services (for development)
polyglot services start              # Start all 3 backend services
polyglot services stop               # Stop all services
polyglot services status             # Check service health
```

**v1.0 Additional Commands:**

```bash
# Package Management
polyglot package init                # Initialize new package
polyglot package publish             # Publish to registry
polyglot package install <name>      # Install from registry

# Development Tools
polyglot debug <instance-id>         # Interactive debugger
polyglot test <file.pg>              # Run pipeline tests
polyglot format <file.pg>            # Format .pg file

# Queue Management (post-MVP)
polyglot queue create <queue-name>   # Create custom queue
polyglot queue pause <queue-name>    # Pause queue
polyglot queue resume <queue-name>   # Resume queue
```

### Code Examples & Documentation

**MVP Example Requirements (Minimum 3 Pipelines):**

1. **Hello World Pipeline:** Simplest possible `.pg` file demonstrating basic syntax
2. **Cross-Language Example:** Python calling Rust or C++ (demonstrates FFI value)
3. **Automation Workflow:** Real-world scenario (e.g., file processing, data pipeline, scheduled task)

**v1.0 Example Library:**

- **By Use Case:**
  - Data processing pipelines
  - API automation
  - File monitoring and transformation
  - Scheduled jobs with priority management
  - Multi-language integration examples

- **By Runtime:**
  - Python wrapper examples
  - Node wrapper examples
  - Rust wrapper examples
  - Mixed runtime examples

- **By Feature:**
  - Queue routing and priority
  - Error handling and compensation
  - Parallel execution
  - Trigger types (manual, time-based, file-watch, webhook)

**Documentation Structure:**

**MVP Docs:**
- **Getting Started:** Installation, first pipeline, basic concepts
- **Syntax Reference:** Complete `.pg` language spec (leverage v0.0.2 docs)
- **CLI Reference:** All command documentation
- **Architecture Overview:** 3 backend services, IR design, data flow
- **Examples:** 3+ working pipelines with explanations

**v1.0 Docs:**
- **Tutorials:** Step-by-step guides for common scenarios
- **API Reference:** Complete CLI and library API
- **Runtime Wrapper Guides:** How to use each supported language
- **Advanced Topics:** Queue management, FFI internals, performance tuning
- **Contributing Guide:** How to add runtime wrappers, submit examples
- **Migration Guides:** Migrating from traditional automation tools

### IDE Integration

**Vision (v1.0+):**

- **Syntax Highlighting:** VSCode, JetBrains, Vim, Emacs support for `.pg` files
- **LSP Server:** Language Server Protocol implementation for:
  - Autocomplete
  - Syntax errors and warnings
  - Go-to-definition
  - Hover documentation
  - Refactoring support
- **Debugger Integration:** Step through pipeline execution, inspect variables
- **Plugin Ecosystem:** Community-contributed IDE extensions

**MVP (Out of Scope):**
- Basic `.pg` files edited as text
- No IDE integration required for MVP

---

{{#if ux_principles}}

## User Experience Principles

{{ux_principles}}

### Key Interactions

{{key_interactions}}
{{/if}}

---

## Functional Requirements

These functional requirements define the complete capability set for Polyglot. Every capability listed here must be implemented to deliver the product vision. Requirements are organized by capability area and numbered sequentially.

### Pipeline Development & Compilation

**FR1:** Developers can write pipeline definitions in `.pg` files using Polyglot syntax

**FR2:** System validates `.pg` file syntax and reports errors with line numbers and descriptions

**FR3:** System compiles valid `.pg` files into Intermediate Representation (IR)

**FR4:** System stores IR in PostgreSQL database with versioning metadata

**FR5:** System validates IR structure before storage to prevent malformed pipelines

**FR6:** Developers can reference the complete v0.0.2 syntax specification to write pipelines

**FR7:** Pipelines support all documented operators: `{|}` (pipeline definition), `{x}` (block end), `[r]` (return), `[p]` (parallel), `[Q]` (queue), `[W]` (wrapper), `[|]` (IO marker), and others

**FR8:** System supports Polyglot type system with runtime type conversion

**FR9:** Pipelines can include comments and documentation

### Pipeline Registry & Lifecycle Management

**FR10:** Developers can register compiled pipelines to the central registry

**FR11:** System maintains registry of all pipelines with metadata (name, version, IR reference, creation date)

**FR12:** Developers can list all registered pipelines

**FR13:** Developers can view detailed information for a specific pipeline

**FR14:** Developers can activate registered pipelines to make them eligible for triggering

**FR15:** Developers can deactivate pipelines to prevent triggering without deletion

**FR16:** System tracks pipeline activation state in database

**FR17:** System prevents duplicate pipeline names in registry

**FR18:** Developers can update existing pipeline definitions (creating new versions)

### Trigger System

**FR19:** System continuously monitors for pipeline trigger conditions

**FR20:** System supports manual triggers via CLI command

**FR21:** System supports time-based triggers (scheduled execution) (MVP: at least one trigger type functional)

**FR22:** Trigger Monitor service operates synchronously and continuously

**FR23:** System creates pipeline instance when trigger condition is met

**FR24:** System associates each instance with source pipeline and trigger metadata

**FR25:** System supports multiple trigger types per pipeline (post-MVP: file-watch, webhook)

**FR26:** Developers can view trigger history and next scheduled execution

### Queue Management & Execution

**FR27:** System maintains dispatch queue for pending pipeline instances

**FR28:** Queue Manager service transitions instances from pending to dispatch state

**FR29:** Queue Manager assigns instances to Runner service for execution

**FR30:** Runner service executes pipeline instances according to IR instructions

**FR31:** System tracks pipeline instance states: Created → Queued → Running → Exited

**FR32:** System persists instance execution state to database

**FR33:** Developers can view current queue status

**FR34:** System supports single dispatch queue (MVP baseline)

**FR35:** Developers can create multiple custom dispatch queues (post-MVP)

**FR36:** Developers can assign pipelines to specific queues using `[Q] |Q.Queue.Assign` syntax (post-MVP)

**FR37:** System supports queue-specific priority algorithms (post-MVP)

**FR38:** Developers can pause and resume queues (post-MVP)

**FR39:** System supports queue characteristics: rate limiting, cost optimization, resource isolation (post-MVP)

**FR40:** System monitors resource usage (CPU, RAM, database connections) per pipeline instance (post-MVP)

### Runtime Integration & Cross-Language FFI

**FR41:** System executes code in Python runtime via wrapper (MVP baseline)

**FR42:** Pipelines can invoke Python functions using `[W] |W.Python3.11` syntax

**FR43:** System performs type conversion between Polyglot types and Python types

**FR44:** System uses serialization-based type conversion (Strategy 1) for MVP

**FR45:** System executes code in Node.js runtime via wrapper (post-MVP)

**FR46:** System executes code in Rust runtime via wrapper (post-MVP)

**FR47:** System executes code in Go runtime via wrapper (post-MVP)

**FR48:** System supports dual-strategy type conversion: serialization (Strategy 1) and direct FFI (Strategy 2) (post-MVP)

**FR49:** System automatically selects optimal type conversion strategy based on available tools and performance (post-MVP)

**FR50:** System performs runtime type resolution for dynamic-to-static language conversions (post-MVP)

**FR51:** Developers can load C++ headers and call functions without manual FFI setup (vision: `pg.load_cpp_header()`)

**FR52:** System auto-generates language bridges for cross-language calls (post-MVP)

**FR53:** System handles errors across language boundaries and reports to Polyglot context

### CLI & Developer Tools

**FR54:** Developers can compile `.pg` files via `polyglot compile` command

**FR55:** Developers can validate syntax without compilation via `polyglot validate` command

**FR56:** Developers can register pipelines via `polyglot register` command

**FR57:** Developers can list registered pipelines via `polyglot list` command

**FR58:** Developers can view pipeline details via `polyglot show` command

**FR59:** Developers can activate pipelines via `polyglot activate` command

**FR60:** Developers can deactivate pipelines via `polyglot deactivate` command

**FR61:** Developers can manually trigger pipelines via `polyglot trigger` command

**FR62:** Developers can view pipeline execution status via `polyglot status` command

**FR63:** Developers can view instance logs via `polyglot logs` command

**FR64:** Developers can start all backend services via `polyglot services start` command

**FR65:** Developers can stop all backend services via `polyglot services stop` command

**FR66:** Developers can check service health via `polyglot services status` command

**FR67:** System provides helpful error messages for CLI commands

**FR68:** Developers can initialize new packages via `polyglot package init` (v1.0)

**FR69:** Developers can publish packages to registry via `polyglot package publish` (v1.0)

**FR70:** Developers can install packages via `polyglot package install` (v1.0)

**FR71:** Developers can debug pipeline instances interactively via `polyglot debug` (v1.0)

**FR72:** Developers can run pipeline tests via `polyglot test` (v1.0)

**FR73:** Developers can format `.pg` files via `polyglot format` (v1.0)

**FR74:** Developers can manage custom queues via CLI (create, pause, resume) (post-MVP)

### Installation & Configuration

**FR75:** Developers can build Polyglot from source using Rust toolchain

**FR76:** Installation script sets up PostgreSQL database schema

**FR77:** Installation script initializes default configuration

**FR78:** System generates default config file on first run

**FR79:** System reads configuration from YAML/TOML file

**FR80:** System supports environment variables for PostgreSQL and Redis connection strings

**FR81:** Developers can install Polyglot via package managers (cargo, brew, apt, choco) (v1.0)

**FR82:** Developers can deploy Polyglot via Docker container (v1.0)

**FR83:** Developers can deploy to cloud platforms with managed PostgreSQL/Redis (v1.0)

### Documentation & Examples

**FR84:** Developers can access Getting Started guide for installation and first pipeline

**FR85:** Developers can reference complete Polyglot syntax documentation

**FR86:** Developers can reference CLI command documentation

**FR87:** Developers can review architecture overview (3 services, IR design, data flow)

**FR88:** Developers can access at least 3 example pipelines (Hello World, cross-language, automation workflow)

**FR89:** Developers can access comprehensive example library organized by use case, runtime, and feature (v1.0)

**FR90:** Developers can access tutorials for common scenarios (v1.0)

**FR91:** Developers can reference runtime wrapper guides for each supported language (v1.0)

**FR92:** Developers can access advanced topics documentation (queue management, FFI internals, performance) (v1.0)

**FR93:** Contributors can access contributing guide for adding runtime wrappers and examples (v1.0)

**FR94:** Developers can access migration guides from traditional automation tools (v1.0)

### Observability & Monitoring

**FR95:** System logs all pipeline execution events to PostgreSQL

**FR96:** Developers can view execution logs via CLI

**FR97:** System logs errors with stack traces and context

**FR98:** System integrates with OpenTelemetry for tracing and metrics (post-MVP)

**FR99:** System stores time-series logs in InfluxDB (post-MVP)

**FR100:** Developers can visualize pipeline execution, queue states, and resource usage via dashboard (post-MVP)

**FR101:** System sends alerts on pipeline failures, resource saturation, and queue backlogs (post-MVP)

**FR102:** System provides metrics on execution duration, success/failure rates, and resource consumption (post-MVP)

### IDE & Tooling Integration

**FR103:** Developers can use syntax highlighting for `.pg` files in VSCode, JetBrains, Vim, Emacs (v1.0+)

**FR104:** System provides Language Server Protocol (LSP) implementation for autocomplete, error checking, go-to-definition (v1.0+)

**FR105:** Developers can use debugger to step through pipeline execution and inspect variables (v1.0+)

**FR106:** Community can contribute IDE plugins and extensions (v1.0+)

### Package Ecosystem

**FR107:** Developers can publish reusable pipeline packages to registry (v1.0)

**FR108:** Developers can discover and install third-party packages (v1.0)

**FR109:** System maintains package versions and dependencies (v1.0)

**FR110:** Community can contribute runtime wrappers for additional languages (v1.0+)

**FR111:** Community can contribute standard library utilities (v1.0+)

### Advanced Features (Post-MVP / Vision)

**FR112:** Developers can define compensation pipelines for automated rollback on failure (v1.0+)

**FR113:** System warns before executing irreversible operations (v1.0+)

**FR114:** System supports code-level checkpoints to pause pipelines mid-execution (v1.0+)

**FR115:** System uses AI to optimize queue priorities and resource allocation (v1.0+)

**FR116:** System supports live code migration to move running pipelines between nodes (v1.0+)

**FR117:** System supports distributed execution across multiple Runner instances (v1.0+)

**FR118:** System can use Kafka as queue backend for massive scale (v1.0+)

**FR119:** Developers can design pipelines visually via GUI designer (v1.0+)

**FR120:** System supports Kubernetes deployment with operators and Helm charts (v1.0+)

---

**Total Functional Requirements: 120 capabilities**

**Completeness Validation:**

✅ **MVP Scope Coverage:** All MVP components captured (compilation, 3 services, database, triggers, Python wrapper, CLI)

✅ **Growth Features Coverage:** Queue management, advanced runtime wrappers, dual-strategy conversion, observability

✅ **Vision Features Coverage:** Compensation pipelines, AI optimization, distributed execution, IDE integration, package ecosystem

✅ **Developer Tool Requirements Coverage:** Installation methods, CLI commands, documentation, examples, IDE integration

✅ **Innovation Requirements Coverage:** Runtime type resolution, dual-strategy conversion, FFI abstraction

✅ **Cross-Reference Check:** Every capability mentioned in Executive Summary, Scope, Innovation, and Developer Tool sections is represented as FR

**Altitude Check:**

✅ All FRs state WHAT capability exists, not HOW it's implemented

✅ No UI/UX details, no performance numbers, no technology choices in FR statements

✅ Each FR is testable and implementation-agnostic

✅ FRs specify WHO (developers, system) and WHAT (capability), not implementation details

---

## Non-Functional Requirements

These quality attributes define how Polyglot must behave to deliver a production-grade developer tool experience.

### Performance

**Why It Matters for Polyglot:** Developer experience depends on responsive compilation, fast pipeline execution, and low-latency queue operations. Serialization overhead is a key architectural concern that affects the FFI value proposition.

**NFR-P1: Compilation Speed**
- `.pg` file compilation to IR completes in <1 second for files up to 1000 lines
- Syntax validation provides feedback within 500ms
- Rationale: Developers expect fast feedback loops during development

**NFR-P2: Pipeline Execution Latency**
- Pipeline instance creation to execution start: <2 seconds for manual triggers
- Trigger detection to instance creation: <5 seconds for time-based triggers
- Rationale: Automation workflows should feel responsive, not sluggish

**NFR-P3: Type Conversion Overhead**
- Serialization-based type conversion (Strategy 1) adds <10ms overhead for typical data sizes (<1MB)
- Direct FFI conversion (Strategy 2, when implemented) matches or exceeds native FFI performance
- Rationale: Overhead must be acceptable for automation use cases (not real-time systems, but not painfully slow)

**NFR-P4: Queue Throughput**
- Queue Manager processes at least 100 pipeline instances per second (single queue)
- Trigger Monitor checks trigger conditions for 1000+ pipelines every 10 seconds
- Rationale: Support production automation scenarios with many concurrent workflows

**NFR-P5: Database Query Performance**
- Pipeline registry queries return results in <100ms
- Instance log retrieval completes in <500ms for up to 10,000 log entries
- Rationale: CLI commands should feel snappy, not database-bound

**NFR-P6: Service Startup Time**
- All 3 backend services start and reach ready state in <10 seconds
- Rationale: Fast iteration during development and deployment

### Security

**Why It Matters for Polyglot:** Pipelines may process sensitive data, execute privileged operations, and interact with databases. Runtime wrappers execute code across language boundaries, requiring isolation.

**NFR-S1: Database Security**
- PostgreSQL connections use TLS encryption in production deployments
- Database credentials never stored in plain text (environment variables or secure config)
- IR and logs may contain sensitive data—access control at database level
- Rationale: Prevent credential leakage and unauthorized access to pipeline data

**NFR-S2: Runtime Isolation**
- Each pipeline instance executes in isolated process (prevents cross-contamination)
- Runtime wrapper failures do not crash Runner service (error boundaries)
- Rationale: One pipeline's errors shouldn't affect others; prevent cascade failures

**NFR-S3: Input Validation**
- All CLI inputs sanitized to prevent command injection
- `.pg` file parsing rejects malicious constructs (e.g., arbitrary code execution attempts)
- IR validation prevents malformed data from reaching execution
- Rationale: Prevent security vulnerabilities in CLI and compilation

**NFR-S4: Secrets Management**
- Configuration supports secure secret injection (not hardcoded in `.pg` files)
- Environment variables for sensitive config (database passwords, API keys)
- Rationale: Enable secure production deployments

**NFR-S5: Least Privilege**
- Runner service executes with minimal required permissions
- Database roles follow principle of least privilege (separate read/write/admin roles)
- Rationale: Limit blast radius of potential security breaches

### Scalability

**Why It Matters for Polyglot:** Success means growing adoption—more pipelines, more instances, more concurrent execution. Architecture must scale from MVP single-instance to distributed production deployments.

**NFR-SC1: Single-Instance Baseline (MVP)**
- MVP deployment handles 100-1000 concurrent pipeline instances on standard hardware (4 CPU, 16GB RAM)
- PostgreSQL database handles 10,000+ registered pipelines without degradation
- Redis queue handles 10,000+ queued instances without performance issues
- Rationale: MVP proves architecture works at small-to-medium scale

**NFR-SC2: Horizontal Scaling Path (Post-MVP)**
- Queue Manager can distribute work across multiple Runner instances
- Runner instances can be added dynamically without downtime
- PostgreSQL can be scaled via read replicas for query-heavy workloads
- Rationale: Enable growth to enterprise scale without architectural rework

**NFR-SC3: Queue Scalability**
- Redis queue backend supports 100,000+ instances in queue (post-MVP)
- Kafka migration path supports millions of queued instances (vision)
- Rationale: Handle massive automation scenarios (large enterprises, cloud-scale deployments)

**NFR-SC4: Database Growth**
- Database schema supports 100,000+ pipelines without query degradation (proper indexing)
- Log storage strategy prevents unbounded growth (archival/retention policies)
- Rationale: Long-term production use generates significant data

**NFR-SC5: Resource Limits**
- Configurable limits on pipeline execution duration (prevent runaway jobs)
- Configurable limits on concurrent instances per queue (prevent resource exhaustion)
- Rationale: Protect system stability under load

### Reliability

**Why It Matters for Polyglot:** Production automation depends on reliability—pipelines must execute predictably, failures must be handled gracefully, and state must be durable.

**NFR-R1: Service Availability**
- Backend services restart automatically on crash (systemd, Docker restart policies)
- Service health checks expose readiness status for monitoring
- Rationale: Minimize downtime, enable automated recovery

**NFR-R2: Data Durability**
- All state persisted to PostgreSQL before acknowledging success
- Pipeline instance state recoverable after service restart
- No data loss on graceful shutdown
- Rationale: Pipelines represent important automation—state must not be lost

**NFR-R3: Error Handling**
- All errors logged with context (stack trace, pipeline instance, timestamp)
- Errors in one pipeline instance do not affect others
- Runtime wrapper errors reported to Polyglot context (cross-language error propagation)
- Rationale: Debugging failures requires good error context; failures should be isolated

**NFR-R4: Idempotency**
- Pipeline registration is idempotent (re-registering same pipeline updates version, doesn't fail)
- Trigger processing handles duplicates gracefully (de-duplication logic)
- Rationale: Prevent accidental duplicate execution, enable safe retries

**NFR-R5: Graceful Degradation**
- If Redis unavailable, queue operations fail with clear error (don't crash service)
- If PostgreSQL unavailable, read-only operations return cached data where possible
- Rationale: Partial functionality better than total failure

### Maintainability

**Why It Matters for Polyglot:** Open-source project success depends on community contributions. Codebase must be understandable, testable, and extensible.

**NFR-M1: Code Quality**
- All Rust code follows standard formatting (rustfmt)
- No compiler warnings in release builds
- All public APIs documented with rustdoc
- Rationale: Consistent code style aids contribution

**NFR-M2: Test Coverage**
- Unit tests for all IR compilation logic (>80% coverage)
- Integration tests for CLI commands (all commands tested)
- End-to-end tests for MVP success scenario (compile → register → trigger → execute)
- Rationale: Tests prevent regressions, document expected behavior

**NFR-M3: Logging & Observability**
- All backend services log to stdout (container-friendly)
- Structured logging with consistent format (JSON or similar)
- Log levels configurable (debug, info, warn, error)
- Rationale: Enable troubleshooting in production

**NFR-M4: Extensibility**
- Runtime wrapper interface well-defined (enable community contributions)
- Trigger system pluggable (new trigger types added without core changes)
- Queue algorithm pluggable (custom priority algorithms)
- Rationale: Enable ecosystem growth without core rewrites

### Portability

**Why It Matters for Polyglot:** Developers use diverse platforms (Linux, macOS, Windows). Deployment targets vary (local dev, cloud, containers).

**NFR-PO1: Platform Support**
- MVP runs on Linux and macOS (primary developer platforms)
- v1.0 adds Windows support (broader developer base)
- Rationale: Meet developers where they work

**NFR-PO2: Dependency Management**
- All dependencies specified in Cargo.toml with version constraints
- PostgreSQL and Redis versions documented (minimum required versions)
- Rationale: Reproducible builds, clear compatibility

**NFR-PO3: Containerization**
- Docker image builds with single command
- Docker Compose provides development environment (PostgreSQL, Redis, all services)
- Rationale: Simplify deployment, enable cloud platforms

**NFR-PO4: Configuration Portability**
- Configuration via environment variables (12-factor app)
- Defaults work for local development (minimal config required)
- Rationale: Same codebase deploys to dev, staging, prod

---

## Implementation Planning

### Epic Breakdown Required

**Next Step:** Run `workflow create-epics-and-stories` to create the implementation breakdown.

---

## References

- **Product Brief:** `docs/product-brief-Polyglot-2025-11-15.md`
- **Polyglot v0.0.2 Documentation:** `docs/user/` (language specification, syntax, examples)
- **Brainstorming Session:** `docs/Agile/brainstorming-session-results-2025-11-15.md`

---

## Next Steps

1. **Epic & Story Breakdown** - Run: `workflow epics-stories`
2. **UX Design** (if UI) - Run: `workflow ux-design`
3. **Architecture** - Run: `workflow create-architecture`

---

_This PRD captures the essence of **Polyglot** - an asynchronous automation language that transforms hours of FFI setup into a single line of code, enabling developers to use the right tool for the right job without the integration tax._

_Created through collaborative discovery between hhj and AI facilitator on 2025-11-15._