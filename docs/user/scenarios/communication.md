---
audience: pg-coder
type: reference
updated: 2026-04-03
---

# Communication Scenarios

<!-- @user/SPEC-INDEX -->
<!-- @user/scenarios/INDEX -->

Automation scenarios for email, social media, messaging platforms, and media production workflows. See [[INDEX]] for the full scenario catalog.

## 3. Email Automation

| # | Scenario | Trigger | Description |
|---|----------|---------|-------------|
| 35 | Auto-reply classifier | Email | Classify incoming emails by intent, send templated responses |
| 36 | Email-to-ticket | Email | Parse support emails, create tickets in Jira or Zendesk |
| 37 | Newsletter builder | Scheduled | Aggregate content from RSS/APIs, compile and send newsletter |
| 38 | Bounce handler | Webhook | Process email bounces, update contact status, alert sender |
| 39 | Meeting summary distributor | Webhook | Receive meeting notes, format digest, email to attendees |
| 40 | Drip campaign engine | Scheduled | Send timed email sequences based on user segment and behavior |
| 41 | Email attachment router | Email | Route attachments to different systems by file type or sender |
| 42 | Unsubscribe processor | Webhook | Handle unsubscribe requests, update lists, confirm removal |
| 43 | Email signature updater | Manual | Batch-update email signatures across organization accounts |
| 44 | Inbox zero automation | Scheduled | Label, archive, and snooze emails by rules and AI classification |
| 45 | Phishing report handler | Email | Analyze reported phishing emails, extract IOCs, alert security |
| 46 | Invoice email extractor | Email | Detect invoices in email, extract amounts and dates, log to sheet |
| 47 | Follow-up reminder | Scheduled | Check sent emails with no reply after N days, send follow-up |
| 48 | Shared mailbox dispatcher | Email | Triage shared inbox, assign to team members by topic |
| 49 | Out-of-office sync | Event | Detect calendar OOO events, enable auto-reply, notify team |
| 50 | Email compliance archiver | Email | Archive all emails matching regulatory keywords to vault |
| 51 | Bulk email validator | Manual | Verify email list for deliverability, remove invalid addresses |

## 10. Social Media

| # | Scenario | Trigger | Description |
|---|----------|---------|-------------|
| 153 | Cross-platform post scheduler | Scheduled | Publish content to Twitter, LinkedIn, Facebook at optimal times |
| 154 | Comment moderator | API Poll | Filter comments for spam and profanity, hide or flag |
| 155 | Follower growth tracker | Scheduled | Record follower counts daily, report weekly trends |
| 156 | Hashtag performance analyzer | Scheduled | Track hashtag reach and engagement, recommend top performers |
| 157 | Social listening pipeline | Stream | Monitor keywords in real-time, classify sentiment, alert on negative |
| 158 | UGC content collector | API Poll | Find user-generated content by hashtag, request permission to share |
| 159 | DM auto-responder | Webhook | Reply to DMs with FAQs or route to support |
| 160 | Social proof collector | Scheduled | Aggregate positive reviews and testimonials for marketing use |
| 161 | Trending topic alerter | Scheduled | Monitor trending topics in industry, suggest content angles |
| 162 | Engagement rate reporter | Scheduled | Calculate engagement metrics per post and platform |
| 163 | Social media bio updater | Manual | Batch-update bios across platforms for campaign launches |
| 164 | Story/reel scheduler | Scheduled | Queue and publish ephemeral content at scheduled times |
| 165 | Social contest manager | Webhook | Track entries, validate eligibility, select and notify winners |
| 166 | Mention reply bot | API Poll | Auto-reply to brand mentions with contextual responses |
| 167 | Social analytics dashboard | Scheduled | Pull data from all social APIs into unified analytics view |
| 168 | Influencer content tracker | Scheduled | Monitor sponsored post performance and contract compliance |

## 15. Communication & Messaging

| # | Scenario | Trigger | Description |
|---|----------|---------|-------------|
| 236 | Slack channel provisioner | Webhook | Create project channels, invite members, post welcome message |
| 237 | Cross-platform message relay | Webhook | Forward messages between Slack, Teams, and Discord |
| 238 | Notification digest | Scheduled | Batch notifications into daily digest instead of real-time spam |
| 239 | Meeting recording transcriber | Webhook | Transcribe recordings, extract action items, share notes |
| 240 | Emergency broadcast | Manual | Send urgent messages across all communication channels |
| 241 | Channel archiver | Scheduled | Archive inactive channels, notify members before archival |
| 242 | Sentiment-based escalation | Stream | Monitor chat sentiment, alert managers on negative trends |
| 243 | Chatbot intent router | Webhook | Route chatbot messages to appropriate handler by intent |
| 244 | SMS appointment reminder | Scheduled | Send SMS reminders 24h and 1h before appointments |
| 245 | Webhook-to-Slack formatter | Webhook | Transform raw webhook payloads into formatted Slack messages |
| 246 | Team availability checker | Scheduled | Check calendars, post team availability summary to channel |
| 247 | Language translator relay | Webhook | Auto-translate messages between multilingual channels |
| 248 | Message compliance scanner | Stream | Scan messages for regulatory keywords, flag for review |
| 249 | Voice message transcriber | Webhook | Transcribe voicemail to text, deliver via email/Slack |
| 250 | On-call rotation notifier | Scheduled | Post current on-call schedule to team channel daily |
| 251 | AMA question collector | Webhook | Collect and deduplicate questions from chat for live sessions |

## 26. Media & Creative

| # | Scenario | Trigger | Description |
|---|----------|---------|-------------|
| 417 | Asset ingestion pipeline | Folder Watch | Process uploaded media files: transcode, tag, store |
| 418 | Thumbnail generator | Webhook | Create thumbnails at multiple sizes for uploaded images/videos |
| 419 | DAM metadata tagger | Webhook | Auto-tag assets with AI-generated keywords and categories |
| 420 | Content approval workflow | Webhook | Route creative assets through review and approval chain |
| 421 | Video transcription pipeline | Webhook | Transcribe uploaded videos, generate SRT subtitles |
| 422 | Rights management tracker | Scheduled | Track usage rights expiry for licensed assets |
| 423 | Brand guideline checker | Webhook | Validate assets against brand colors, fonts, logo rules |
| 424 | Podcast publishing pipeline | Webhook | Process audio, generate show notes, publish to directories |
| 425 | Image watermarker | Folder Watch | Apply watermarks to images before external distribution |
| 426 | Print-ready file verifier | Webhook | Check files meet print specifications (DPI, bleed, color space) |
| 427 | Royalty calculator | Scheduled | Calculate royalties from streaming/sales data, generate statements |
| 428 | Content localization dispatcher | Webhook | Queue content for translation, track progress across locales |
| 429 | Social media asset resizer | Webhook | Resize images/videos to platform-specific dimensions |
| 430 | Archive migration pipeline | Scheduled | Migrate legacy media archives to modern DAM with metadata |
| 431 | Creative brief generator | Webhook | Populate brief templates from project requirements |
| 432 | Usage analytics for assets | Scheduled | Track which assets are used where, report utilization |
