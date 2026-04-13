---
audience: automation-builder
type: reference
updated: 2026-04-03
---

# Commerce & Finance Scenarios

<!-- @user/SPEC-INDEX -->
<!-- @user/scenarios/INDEX -->

Automation scenarios for e-commerce, financial operations, legal workflows, real estate, and travel/hospitality. See [[INDEX]] for the full scenario catalog.

## 11. E-Commerce & Payments

| # | Scenario | Trigger | Description |
|---|----------|---------|-------------|
| 169 | Order confirmation sender | Webhook | Send order confirmation with details and tracking link |
| 170 | Inventory low-stock alert | Scheduled | Check stock levels, alert purchasing and update listing status |
| 171 | Abandoned cart recovery | Scheduled | Send reminder emails for carts abandoned 1h, 24h, 72h ago |
| 172 | Price comparison monitor | Scheduled | Track competitor prices, suggest adjustments |
| 173 | Payment failure retry | Webhook | Retry failed payments with exponential backoff, notify customer |
| 174 | Product review moderator | Webhook | Screen reviews for spam, flag for approval, thank reviewer |
| 175 | Shipping label generator | Webhook | Create shipping labels via carrier API on order fulfillment |
| 176 | Return and refund processor | Webhook | Handle return requests, generate labels, process refunds |
| 177 | Product catalog sync | Scheduled | Sync product data between ERP and e-commerce platform |
| 178 | Dynamic pricing updater | Scheduled | Adjust prices based on demand, inventory, and competition |
| 179 | Subscription renewal handler | Scheduled | Process recurring payments, handle failures, send receipts |
| 180 | Fraud detection flagger | Webhook | Score orders for fraud risk, hold suspicious for review |
| 181 | Cross-sell recommender | Webhook | Suggest related products post-purchase via email |
| 182 | Coupon code generator | Webhook | Generate unique codes for campaigns, track redemptions |
| 183 | Order fulfillment router | Webhook | Route orders to nearest warehouse by customer location |
| 184 | Wishlist price drop notifier | Scheduled | Alert customers when wishlisted items go on sale |
| 185 | Tax calculation updater | Scheduled | Refresh tax rates from jurisdiction APIs |

## 12. Finance & Accounting

| # | Scenario | Trigger | Description |
|---|----------|---------|-------------|
| 186 | Invoice generator and sender | Scheduled | Generate invoices from billable time/items, email to clients |
| 187 | Expense report processor | Webhook | Validate receipts, categorize expenses, route for approval |
| 188 | Bank reconciliation | Scheduled | Match bank transactions against ledger entries, flag mismatches |
| 189 | Accounts payable automation | Email | Extract invoice data from emails, create payment entries |
| 190 | Revenue recognition logger | Scheduled | Calculate and post revenue recognition entries monthly |
| 191 | Budget variance reporter | Scheduled | Compare actuals vs budget by department, flag overruns |
| 192 | Tax filing preparer | Scheduled | Aggregate financial data, populate tax form templates |
| 193 | Payment reminder sender | Scheduled | Send reminders for overdue invoices at 7, 14, 30 days |
| 194 | Multi-currency converter | Scheduled | Fetch daily rates, convert and consolidate multi-currency books |
| 195 | Payroll processor | Scheduled | Calculate pay, deductions, taxes; generate pay stubs |
| 196 | Financial close checklist | Scheduled | Track month-end close tasks, notify owners, report status |
| 197 | Vendor payment scheduler | Scheduled | Batch vendor payments by due date, optimize cash flow |
| 198 | Audit trail logger | Event | Log all financial transactions with user, timestamp, details |
| 199 | Cash flow forecaster | Scheduled | Project cash flow from AR, AP, and recurring expenses |
| 200 | Credit note issuer | Webhook | Generate and send credit notes for approved adjustments |
| 201 | Intercompany reconciler | Scheduled | Match and reconcile transactions between business entities |
| 202 | Fixed asset depreciation | Scheduled | Calculate monthly depreciation, post journal entries |

## 23. Legal & Contracts

| # | Scenario | Trigger | Description |
|---|----------|---------|-------------|
| 369 | Contract expiry tracker | Scheduled | Alert stakeholders N days before contract renewal deadlines |
| 370 | NDA generator | Webhook | Populate NDA template with party details, send for e-sign |
| 371 | Clause extraction pipeline | Webhook | Extract key clauses from uploaded contracts using AI |
| 372 | Regulatory change monitor | Scheduled | Track legal databases for regulatory updates, summarize |
| 373 | E-signature completion handler | Webhook | Process signed documents, file, notify parties |
| 374 | Legal hold processor | Webhook | Preserve relevant documents when litigation hold issued |
| 375 | Patent filing tracker | Scheduled | Track patent application statuses, alert on deadlines |
| 376 | Compliance checklist automator | Scheduled | Generate compliance checklists per jurisdiction |
| 377 | Legal spend tracker | Scheduled | Aggregate outside counsel invoices, report by matter |
| 378 | Contract comparison tool | Webhook | Diff contract versions, highlight changed clauses |
| 379 | Trademark monitoring | Scheduled | Search trademark databases for potential conflicts |
| 380 | Terms of service updater | Webhook | Distribute updated ToS, track user acceptance |
| 381 | Legal intake form processor | Webhook | Route legal requests to appropriate counsel by type |
| 382 | Board resolution tracker | Scheduled | Track required resolutions, chase signatures, file completed |
| 383 | Data processing agreement handler | Webhook | Generate DPAs for new vendors, track execution status |
| 384 | Litigation deadline tracker | Scheduled | Monitor case deadlines, send escalating reminders |

## 28. Real Estate

| # | Scenario | Trigger | Description |
|---|----------|---------|-------------|
| 449 | New listing distributor | Webhook | Push listings to MLS, Zillow, Realtor.com simultaneously |
| 450 | Open house scheduler | Webhook | Create events, send invites, track RSVPs |
| 451 | Lead follow-up sequencer | Webhook | Send timed follow-ups to property inquiry leads |
| 452 | Rent payment processor | Scheduled | Process recurring rent, handle failures, send receipts |
| 453 | Maintenance request handler | Webhook | Log requests, assign vendors, track completion |
| 454 | Lease expiry notifier | Scheduled | Alert property managers N days before lease expiration |
| 455 | Property valuation tracker | Scheduled | Pull comparable sales data, estimate current property values |
| 456 | Tenant screening automator | Webhook | Submit applications to screening services, collect results |
| 457 | Commission splitter | Webhook | Calculate and distribute commissions on deal close |
| 458 | Property inspection scheduler | Scheduled | Schedule periodic inspections, assign inspectors |
| 459 | HOA violation notifier | Webhook | Process reported violations, send notices to owners |
| 460 | Mortgage rate monitor | Scheduled | Track rate changes, alert clients with active searches |
| 461 | Vacancy listing refresher | Scheduled | Re-post and boost stale vacancy listings |
| 462 | Move-in/move-out checklist | Webhook | Generate checklists, schedule walkthroughs, track completion |
| 463 | Property tax payment tracker | Scheduled | Monitor tax due dates, verify payments, alert on delinquency |
| 464 | Market analysis reporter | Scheduled | Compile neighborhood trends from MLS and census data |

## 29. Travel & Hospitality

| # | Scenario | Trigger | Description |
|---|----------|---------|-------------|
| 465 | Booking confirmation sender | Webhook | Send confirmation with itinerary details on reservation |
| 466 | Price drop alerter | Scheduled | Monitor flight/hotel prices, alert subscribers on drops |
| 467 | Guest pre-arrival preparer | Scheduled | Send check-in instructions, preferences form 24h before |
| 468 | Review response automator | Webhook | Draft responses to guest reviews, queue for approval |
| 469 | Channel rate synchronizer | Webhook | Sync room rates across OTAs and direct booking |
| 470 | Loyalty points calculator | Webhook | Calculate and credit loyalty points on checkout |
| 471 | Flight delay notifier | API Poll | Monitor flight status, alert travelers and arrange alternatives |
| 472 | Room assignment optimizer | Scheduled | Assign rooms based on preferences, maintenance status |
| 473 | Travel policy compliance checker | Webhook | Validate bookings against corporate travel policy |
| 474 | Concierge request handler | Webhook | Route guest requests to appropriate department |
| 475 | Revenue management updater | Scheduled | Adjust dynamic pricing based on occupancy and demand |
| 476 | Group booking coordinator | Webhook | Manage room blocks, track pickup, adjust allocations |
| 477 | Housekeeping scheduler | Event | Generate cleaning assignments from checkout and check-in times |
| 478 | Travel expense reconciler | Scheduled | Match credit card charges to bookings, flag discrepancies |
| 479 | Guest feedback aggregator | Scheduled | Collect reviews from all platforms, generate sentiment report |
| 480 | Lost and found tracker | Webhook | Log found items, match to guest inquiries, arrange shipping |
