# Error Handling Patterns

**Version:** 0.0.2  
**Production-Ready Patterns**
**Last Updated:** 2025-12-02

---

## Pattern 1: Retry with Exponential Backoff

Automatically retry failed operations with increasing delays.

```polyglot
[@] @Local::RetryPattern:1.0.0.0
[X]

[|] FetchWithRetry
[i] .url:pg.string
[i] .max_retries:pg.int <~ 3
[t] |T.Call
[o] .result:pg.string
[o] !MaxRetriesExceeded

[r] .attempt:pg.int << 0
[r] .backoff:pg.int << 1
[r] .success:pg.bool << #Boolean.False

[?] .attempt <? .max_retries
[&] .success =? #Boolean.False
[~][<] .attempt << |U.Int.Add"{.attempt, 1"
[~][r] |U.HTTP.Get
[~][<] <url:pg.string << .url
[~][>] >data:pg.string >> .result
[~][~]
[~][~][<] .success << #Boolean.True
[~][~]
[~][~][!] !NetworkTimeout
[~][~][r] .wait_time:pg.int << |U.Int.Multiply"{.backoff, 1000"
[~][~][<] .backoff << |U.Int.Multiply"{.backoff, 2"
[~][~]
[~]

[?] .success =? #Boolean.True
[~][o] .result:pg.string
[~]

[?] *?
[~][o] !MaxRetriesExceeded
[~]
[X]
```

---

## Pattern 2: Fallback Strategy

Try primary service, fall back to secondary on failure.

```polyglot
[|] FetchWithFallback
[i] .resource_id:pg.string
[t] |T.Call
// |W.Polyglot.Scope is IMPLICIT (manages variable lifecycle

[r] .result:pg.string << ""

[r] |FetchFromPrimary
[<] <id:pg.string << .resource_id
[>] >data:pg.string >> .result
[~]
[~][!] !ServiceUnavailable
[~][r] |FetchFromSecondary
[~][<] <id:pg.string << .resource_id
[~][>] >data:pg.string >> .result
[~]
[~][!] *
[~][r] |FetchFromCache
[~][<] <id:pg.string << .resource_id
[~][>] >data:pg.string >> .result
[~]

[o] .result:pg.string
[X]
```

---

## Pattern 3: Circuit Breaker

Prevent cascading failures by breaking circuit after threshold.

```polyglot
[#] #CircuitState
[<] .Closed
[<] .Open
[<] .HalfOpen
[X]

[|] |CircuitBreaker
[i] .service_call:pg.string
[i] .failure_threshold:pg.int <~ 5
[i] .timeout:pg.dt <~ |DT.Seconds"30"
[t] |T.Call
[o] .result:pg.string
[o] !CircuitOpenError
[o] !ServiceError

[r] .circuit_state: #CircuitState << #CircuitState.Closed
[r] .failure_count:pg.int << 0
[r] .last_failure:pg.dt << |DT.Now""

[?] .circuit_state =? #CircuitState.Open
[~][r] .time_since_failure:pg.dt << |DT.ToNow"{.last_failure"
[~][?] .time_since_failure >? .timeout
[~][~][<] .circuit_state << #CircuitState.HalfOpen
[~][~]
[~][?] *?
[~][~][o] !CircuitOpenError
[~][~]
[~]

[?] .circuit_state =? #CircuitState.Closed
[+] .circuit_state =? #CircuitState.HalfOpen
[~][r] |CallService
[~][<] <call:pg.string << .service_call
[~][>] >response:pg.string >> .result
[~][~]
[~][~][<] .circuit_state << #CircuitState.Closed
[~][~][<] .failure_count << 0
[~][~][o] .result:pg.string
[~][~]
[~][~][!] *
[~][~][<] .failure_count << |U.Int.Add"{.failure_count, 1"
[~][~][<] .last_failure << |DT.Now""
[~][~][?] .failure_count =>? .failure_threshold
[~][~][~][<] .circuit_state << #CircuitState.Open
[~][~][~]
[~][~][o] !ServiceError
[~][~]
[~]
[X]
```

---

## Pattern 4: Graceful Degradation

Reduce functionality instead of complete failure.

```polyglot
[|] GetUserProfile
[i] .user_id:pg.int
[t] |T.Call
// |W.Polyglot.Scope is IMPLICIT (manages variable lifecycle

[r] .profile:pg.serial << {

[r] |GetFullProfile
[<] <user_id:pg.int << .user_id
[>] >data:pg.serial >> .profile
[~]
[~][!] !DatabaseUnavailable
[~][r] |GetBasicProfile
[~][<] <user_id:pg.int << .user_id
[~][>] >data:pg.serial >> .profile
[~][<] .profile.degraded << #Boolean.True
[~]

[r] |GetRecentActivity
[<] <user_id:pg.int << .user_id
[>] >activity:pg.array{pg\serial >> .recent
[<] .profile.recent_activity << .recent
[~]
[~][!] *
[~][<] .profile.recent_activity << {
[~]

[o] .profile:pg.serial
[X]
```

---

## Pattern 5: Timeout Handling

Prevent indefinite waits.

```polyglot
[|] FetchWithTimeout
[i] .url:pg.string
[i] .timeout:pg.dt <~ |DT.Seconds"10"
[t] |T.Call
[o] .result:pg.string
[o] !TimeoutError

[r] .start:pg.dt << |DT.Now""

[r] |U.HTTP.Get
[<] <url:pg.string << .url
[>] >data:pg.string >> .result

[r] .elapsed:pg.dt << |DT.ToNow"{.start"

[?] .elapsed >? .timeout
[~][o] !TimeoutError
[~]

[?] *?
[~][o] .result:pg.string
[~]
[X]
```

---

## Key Takeaways

1. **Always handle errors** - Use [!] blocks for expected failures
2. **Retry intelligently** - Exponential backoff prevents thundering herd
3. **Fail gracefully** - Degrade functionality rather than crash
4. **Add context** - Enrich errors as they propagate
5. **Set timeouts** - Prevent indefinite waits
6. **Circuit breakers** - Protect against cascading failures

---

**End of Examples**
