---
audience: designer
type: reference
updated: 2026-03-30
---

<!-- @edge-cases/INDEX -->

## 23. Stress Tests (S23)

### ST-1: Full employee onboarding — imports, trigger, parallel, collect, errors, chain

**What it tests:** Full production-grade pipeline combining package imports, `=T.Call` trigger, parallel execution, `*All` collect-all, chain execution, and per-call error handling.

```polyglot
{@} @Local:001.HR.Onboarding:v2.0.0
   [@] @AD << @Local:001.ActiveDirectory:v2.1.0
   [@] @Mail << @Local:001.EmailSystem:v1.5.0
   [@] @HR << @Local:001.HRSystem:v3.0.0
   [@] @Slack << @Community:polyglot-tools.SlackAdmin:v1.3.0

{#} #NewHire
   [.] .id#string
   [.] .name#string
   [.] .email#string
   [.] .department#string
   [.] .startDate#string

{=} =Onboard.Employee
   [%] .description << "Provisions all accounts for a new hire"
   [%] .version << "2.0.0"
   [=] <hire#NewHire
   [=] >report#string ~> "incomplete"
   [=] >success#bool ~> #Boolean.False
   [T] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot

   [ ] Parallel provisioning — all three fire at once
   [p] @AD=Account.Create
      [=] <name << $hire.name
      [=] <email << $hire.email
      [=] >adId >> $adId
      [!] !AD.CreateFailed
         [r] $adId << "AD_FAILED"
      [!] !AD.Timeout
         [r] $adId << "AD_TIMEOUT"

   [p] @Mail=Mailbox.Provision
      [=] <email << $hire.email
      [=] <displayName << $hire.name
      [=] >mailboxId >> $mailboxId
      [!] !Mail.ProvisionFailed
         [r] $mailboxId << "MAIL_FAILED"

   [p] @Slack=User.Invite
      [=] <email << $hire.email
      [=] <team << $hire.department
      [=] >slackId >> $slackId
      [!] !Slack.InviteFailed
         [r] $slackId << "SLACK_FAILED"

   [ ] Wait for all three to complete
   [*] *All
      [*] << $adId
      [*] << $mailboxId
      [*] << $slackId

   [ ] Record to HR system — chain: build record then save
   [r] @HR=Record.Build=>@HR=Record.Save
      [=] >Build.hireId << $hire.id
      [=] >Build.adAccount << $adId
      [=] >Build.mailbox << $mailboxId
      [=] >Build.slack << $slackId
      [=] <Save.status >> >report
      [!] .0!Build.ValidationError
         [r] >report << "record build failed"
      [!] .1!Save.WriteError
         [r] >report << "record save failed"

   [ ] Mark success only if none of the IDs are failure markers
   [?] $adId =!? "AD_FAILED"
   [&] $adId =!? "AD_TIMEOUT"
   [&] $mailboxId =!? "MAIL_FAILED"
   [&] $slackId =!? "SLACK_FAILED"
      [r] >success << #Boolean.True
   [?] *?
      [r] >success << #Boolean.False
```

### ST-2: Complex conditional branching — range, logical, negation, exhaustive

**What it tests:** Nested range checks, all logical operators, negation operators, XOR, and mandatory `*?`. Every branch path is non-trivial.

```polyglot
{=} =Risk.Classify
   [=] <score#int
   [=] <flags#int
   [=] <verified#bool
   [=] >tier#string ~> "unknown"
   [=] >action#string ~> "review"
   [T] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot

   [ ] High risk: score > 80 AND (not verified OR flags > 2)
   [?] $score >? 80
   [&] $verified =!? #Boolean.True
      [r] >tier << "high"
      [r] >action << "block"

   [ ] Also high risk: score > 80 AND flags > 2 (even if verified)
   [?] $score >? 80
   [&] $flags >? 2
      [r] >tier << "high"
      [r] >action << "escalate"

   [ ] Medium: score in [50,80], flags not excessive
   [?] $score ?[50,80]
   [&] $flags <=? 2
      [r] >tier << "medium"
      [r] >action << "monitor"

   [ ] Low: score strictly in (0,50), verified, no flags
   [?] $score ?(0,50)
   [&] $verified =? #Boolean.True
   [&] $flags =? 0
      [r] >tier << "low"
      [r] >action << "pass"

   [ ] Suspicious: high score XOR high flags (one but not both)
   [?] $score >? 80
   [^] $flags >? 2
      [r] >tier << "suspicious"
      [r] >action << "investigate"

   [ ] Zero or negative score — anomalous
   [?] $score <=? 0
      [r] >tier << "invalid"
      [r] >action << "reject"

   [ ] Catch-all for any uncovered combination
   [?] *?
      [r] >tier << "unknown"
      [r] >action << "review"
```

### ST-3: Race collector feeding a chain — `*First` winner into chain execution

**What it tests:** Three parallel pipelines racing, winner fed directly into a chain via `[*] >>` collect output, then chain processed with explicit IO wiring and error handling.

```polyglot
{=} =Search.BestResult
   [=] <query#string
   [=] >result#serial ~> {}
   [=] >source#string ~> "none"
   [T] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot

   [ ] Fire three search engines in parallel
   [p] =Search.Engine.Fast
      [=] <q << $query
      [=] >result >> $fast

   [p] =Search.Engine.Semantic
      [=] <q << $query
      [=] >result >> $semantic

   [p] =Search.Engine.Archive
      [=] <q << $query
      [=] >result >> $archive

   [ ] Take whoever finishes first
   [*] *First
      [*] << $fast
      [*] << $semantic
      [*] << $archive
      [*] >> $winner

   [ ] Enrich and format the winner — chain
   [r] =Result.Enrich=>=Result.Format
      [=] >Enrich.raw << $winner
      [=] >Enrich.query << $query
      [=] <Enrich.enriched >> <Format.input
      [=] <Format.output >> >result
      [=] <Format.source >> >source
      [!] .Enrich!Enrich.Failed
         [r] >result << {}
         [r] >source << "enrich_error"
      [!] .Format!Format.Failed
         [r] >source << "format_error"
```

### ST-4: Multi-wave parallel with macro wrapper and nested expand

**What it tests:** Two parallel waves separated by `*All` barrier, followed by an expand+collect pipeline, all inside a DB transaction wrapper.

```polyglot
{=} =Batch.Process
   [=] <items#array:serial
   [=] >summary#serial ~> {}
   [T] =T.Call
   [Q] =Q.Default
   [W] =W.DB.Transaction
      [=] $connectionString << $dbConnStr
      [=] $dbConn >> $dbConn

   [ ] Wave 1: fetch metadata for the batch in parallel
   [p] =Batch.FetchMetadata
      [=] <conn << $dbConn
      [=] <items << $items
      [=] >meta >> $meta

   [p] =Batch.FetchPermissions
      [=] <conn << $dbConn
      [=] <items << $items
      [=] >perms >> $perms

   [*] *All
      [*] << $meta
      [*] << $perms

   [ ] Wave 2: validate and enrich in parallel using wave 1 results
   [p] =Batch.Validate
      [=] <items << $items
      [=] <meta << $meta
      [=] <perms << $perms
      [=] >valid >> $validItems

   [p] =Batch.EnrichAll
      [=] <items << $items
      [=] <meta << $meta
      [=] >enriched >> $enrichedItems

   [*] *All
      [*] << $validItems
      [*] << $enrichedItems

   [ ] Process each valid+enriched item sequentially, collect results
   [r] ~ForEach.Array
      [~] <Array << $validItems
      [~] >item >> $item

      [r] =Item.Process
         [=] <item << $item
         [=] <conn << $dbConn
         [=] >status >> $itemStatus
         [!] !Item.ProcessFailed
            [r] $itemStatus#string << "failed"

      [r] *Into.Serial
         [*] <key << $item:id
         [*] <value << $itemStatus
         [*] >Serial >> >summary
```

### ST-5: Deep nesting — expand inside conditional inside expand with collectors

**What it tests:** Expand nested inside a conditional branch, which is itself inside another expand. Tests 4+ levels of indentation, per-level collector scoping, and `*?` at each conditional level.

```polyglot
{=} =Tree.Flatten
   [=] <categories#array:serial
   [=] >flat#array:string ~> {}
   [T] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot

   [ ] Outer expand — one mini-pipeline per category
   [p] ~ForEach.Array
      [~] <Array << $categories
      [~] >item >> $category

      [?] $category:enabled =? #Boolean.True
         [ ] Inner expand — each category's items
         [r] ~ForEach.Array
            [~] <Array << $category:items
            [~] >item >> $leaf

            [r] $label#string << "{$category:name}/{$leaf:name}"

            [r] *Into.Array
               [*] <item << $label
               [*] >Array >> >flat
      [?] *?
         [ ] Disabled category — skip silently
```

### ST-6: Wrapper with parallel timer in setup, body, and collect in cleanup

**What it tests:** The `[p]` in `[\]` with no `*All` — timer runs concurrently with body. `[/]` uses `*All` with `[*] <<` wait input to collect timer handle before stopping it. See [[concepts/pipelines/wrappers#Parallel Forking in Setup]].

```polyglot
{W} =W.Traced
   [{] $operationId#string
   [}] $durationMs#int
   [}] $spanId#string

   [\]
      [ ] Sequential: open trace session before body
      [r] =Tracer.Open
         [=] <opId << $operationId
         [=] >session >> $session
         [=] >spanId >> $spanId

      [ ] Parallel: start timer — no *All, so it runs with body
      [p] =Tracer.StartTimer
         [=] <session << $session
         [=] >handle >> $timerHandle

   [ ] ... body executes while timer runs ...

   [/]
      [ ] Collect timer handle — must be Final before we stop it
      [*] *All
         [*] << $timerHandle

      [r] =Tracer.StopTimer
         [=] <handle << $timerHandle
         [=] >elapsed >> $durationMs

      [r] =Tracer.Close
         [=] <session << $session

{=} =Invoice.Parse
   [=] <raw#string
   [=] >invoice#serial ~> {}
   [T] =T.Call
   [Q] =Q.Default
   [W] =W.Traced
      [=] $operationId << "invoice.parse"
      [=] $durationMs >> $parseDuration
      [=] $spanId >> $spanId

   [ ] $spanId and $parseDuration from macro [}] outputs
   [r] =JSON.Parse
      [=] <input << $raw
      [=] >parsed >> >invoice
      [!] !JSON.ParseError
         [r] >invoice << {}

   [b] =Metrics.Record
      [=] <span << $spanId
      [=] <duration << $parseDuration
```
