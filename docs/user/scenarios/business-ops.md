---
audience: automation-builder
type: reference
updated: 2026-04-03
---

# Business Operations Scenarios

<!-- @user/SPEC-INDEX -->
<!-- @user/scenarios/INDEX -->

Automation scenarios for customer support, sales, marketing, HR, and project management workflows. See [[INDEX]] for the full scenario catalog.

## 7. Customer Support

| # | Scenario | Trigger | Description |
|---|----------|---------|-------------|
| 103 | Ticket auto-triage | Webhook | Classify tickets by urgency and topic, route to correct queue |
| 104 | SLA timer tracker | Signal | Start SLA countdown on ticket creation, escalate before breach |
| 105 | Satisfaction survey sender | Webhook | Send CSAT survey after ticket resolution |
| 106 | Knowledge base suggester | Webhook | Match ticket content to KB articles, suggest to agent |
| 107 | Escalation notifier | Signal | Alert managers when tickets hit escalation threshold |
| 108 | Multi-channel ticket merger | Webhook | Merge tickets from same customer across email, chat, phone |
| 109 | Agent workload balancer | Scheduled | Redistribute ticket queues to balance agent load |
| 110 | Feedback aggregator | Scheduled | Collect feedback from surveys, reviews, social; generate report |
| 111 | Canned response recommender | Webhook | Suggest response templates based on ticket classification |
| 112 | Customer health scorer | Scheduled | Calculate health score from tickets, usage, and sentiment |
| 113 | Ticket resolution time reporter | Scheduled | Generate weekly resolution time analytics by team and category |
| 114 | VIP customer flagger | Webhook | Tag high-value customer tickets for priority routing |
| 115 | Refund processor | Webhook | Handle approved refund requests, process payment, send confirmation |
| 116 | Support chatbot fallback | Webhook | Route chatbot failures to live agents with conversation context |
| 117 | Ticket reopen detector | Webhook | Flag tickets reopened within 48 hours for quality review |
| 118 | Localization router | Webhook | Detect ticket language, route to language-matched agent |
| 119 | Auto-close stale tickets | Scheduled | Warn then close tickets with no customer reply for 7+ days |

## 8. Sales & CRM

| # | Scenario | Trigger | Description |
|---|----------|---------|-------------|
| 120 | Lead scoring automation | Webhook | Score inbound leads by firmographic and behavioral data |
| 121 | Deal stage updater | Webhook | Move deals through pipeline stages based on activity triggers |
| 122 | CRM data enrichment | Webhook | Append company and contact data from Clearbit on new lead |
| 123 | Meeting scheduler | Webhook | Send calendar link when lead requests demo, sync to CRM |
| 124 | Proposal generator | Webhook | Populate proposal template from deal data, send for e-sign |
| 125 | Win/loss analysis logger | Webhook | Record deal outcome details, trigger retrospective survey |
| 126 | Territory assignment | Webhook | Auto-assign leads to reps by geography and capacity |
| 127 | Pipeline snapshot | Scheduled | Capture daily pipeline snapshot for trend analysis |
| 128 | Contract renewal reminder | Scheduled | Alert account managers N days before contract expiry |
| 129 | Competitor mention tracker | API Poll | Monitor web for competitor mentions, log to CRM |
| 130 | Quote approval workflow | Webhook | Route discount requests through approval chain |
| 131 | Activity logging | Webhook | Log calls, emails, and meetings to CRM contact timeline |
| 132 | Lead source attribution | Webhook | Tag leads with UTM and referral source data |
| 133 | Stale deal alerter | Scheduled | Flag deals with no activity for 14+ days, notify rep |
| 134 | Commission calculator | Scheduled | Calculate monthly commissions from closed-won deals |
| 135 | Sales forecast aggregator | Scheduled | Roll up rep forecasts, compare to quota, report to leadership |

## 9. Marketing & Content

| # | Scenario | Trigger | Description |
|---|----------|---------|-------------|
| 136 | Blog post publisher | Scheduled | Pull approved drafts from CMS, schedule publish, share on social |
| 137 | SEO rank tracker | Scheduled | Check keyword rankings daily, alert on significant changes |
| 138 | UTM link generator | Webhook | Create tracked links for campaigns with consistent UTM params |
| 139 | Content performance reporter | Scheduled | Aggregate page views, shares, conversions per content piece |
| 140 | A/B test result checker | Scheduled | Monitor experiment metrics, alert when statistical significance reached |
| 141 | Webinar registration handler | Webhook | Process registrations, send confirmation, add to CRM |
| 142 | Landing page form processor | Webhook | Capture form submissions, enrich, route to nurture or sales |
| 143 | Email list segmenter | Scheduled | Re-segment contacts by engagement and attributes |
| 144 | Campaign budget tracker | Scheduled | Pull ad spend from platforms, compare against budget |
| 145 | Content calendar sync | Scheduled | Sync editorial calendar with project management tool |
| 146 | Broken link checker | Scheduled | Crawl site for broken links, create tickets for fixes |
| 147 | RSS to social poster | API Poll | Publish new RSS items to social channels with custom format |
| 148 | Influencer outreach tracker | Webhook | Log outreach, track responses, schedule follow-ups |
| 149 | Brand mention aggregator | API Poll | Collect brand mentions from news, blogs, forums into dashboard |
| 150 | Marketing attribution model | Scheduled | Calculate multi-touch attribution across channels |
| 151 | Event invite sender | Scheduled | Send personalized event invitations based on audience segment |
| 152 | Competitor content monitor | Scheduled | Track competitor blog and social posts, summarize weekly |

## 13. HR & People Operations

| # | Scenario | Trigger | Description |
|---|----------|---------|-------------|
| 203 | New hire onboarding checklist | Webhook | Create tasks across IT, facilities, HR on hire trigger |
| 204 | PTO balance tracker | Scheduled | Calculate accrued PTO, update balances, notify near-limit |
| 205 | Performance review reminder | Scheduled | Send review reminders to managers at cycle start |
| 206 | Applicant tracking sync | Webhook | Sync candidate status between ATS and HR system |
| 207 | Employee anniversary notifier | Scheduled | Send congratulations and milestone gifts on work anniversaries |
| 208 | Timesheet compliance checker | Scheduled | Flag missing or incomplete timesheets, send reminders |
| 209 | Benefits enrollment processor | Webhook | Process benefit selections during open enrollment |
| 210 | Org chart updater | Webhook | Reflect reporting changes in org chart on HR updates |
| 211 | Exit interview scheduler | Webhook | Schedule exit interviews on offboarding trigger |
| 212 | Training completion tracker | Webhook | Log completions, check compliance requirements, alert gaps |
| 213 | Headcount report generator | Scheduled | Generate weekly headcount and attrition reports |
| 214 | Referral bonus processor | Webhook | Track referral hires, calculate bonus, trigger payment |
| 215 | Employee survey distributor | Scheduled | Send pulse surveys, collect responses, generate reports |
| 216 | Job posting distributor | Webhook | Publish approved job postings to multiple job boards |
| 217 | Compensation band checker | Scheduled | Flag employees outside compensation bands for review |
| 218 | I-9 verification reminder | Scheduled | Send document verification reminders before compliance deadline |
| 219 | Diversity metrics reporter | Scheduled | Aggregate anonymized diversity data, generate quarterly report |

## 14. Project Management

| # | Scenario | Trigger | Description |
|---|----------|---------|-------------|
| 220 | Task auto-assignment | Webhook | Assign new tasks based on team member skills and capacity |
| 221 | Sprint burndown updater | Scheduled | Calculate remaining story points, update burndown chart |
| 222 | Overdue task notifier | Scheduled | Alert assignees and managers of tasks past due date |
| 223 | Cross-tool status sync | Webhook | Sync task status between Jira, Asana, and Notion |
| 224 | Standup report compiler | Scheduled | Aggregate yesterday's commits, completed tasks for standup |
| 225 | Project health dashboard | Scheduled | Calculate schedule, budget, and scope metrics per project |
| 226 | Milestone completion notifier | Webhook | Announce milestone completions to stakeholders |
| 227 | Dependency blocker alerter | Scheduled | Detect blocked tasks, notify blocking task owners |
| 228 | Resource utilization reporter | Scheduled | Track team allocation across projects, flag over-allocation |
| 229 | Meeting notes to tasks | Webhook | Extract action items from meeting notes, create tasks |
| 230 | Retrospective data collector | Scheduled | Gather sprint metrics for retrospective prep |
| 231 | Time tracking aggregator | Scheduled | Roll up time entries by project, client, and team member |
| 232 | Risk register updater | Scheduled | Review risk items, send updates to risk owners |
| 233 | Project template instantiator | Webhook | Clone task templates for new projects with adjusted dates |
| 234 | Gantt chart refresher | Scheduled | Recalculate timelines from task dependencies and progress |
| 235 | Client status report generator | Scheduled | Compile project updates into client-facing status report |
