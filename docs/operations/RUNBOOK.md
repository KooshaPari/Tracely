# phenotype-sentinel — Operations Runbook

_Addresses audit finding L27 (Failure-mode observability: 0/3)._
_Covers SLO definitions, error budgets, burn-rate alerting, and incident response._

---

## SLO Definitions

| SLO | Metric | Target | Window |
|-----|--------|--------|--------|
| Rate-limiter availability | `sentinel_rate_limiter_allowed_total / (allowed + rejected)` | ≥ 99.5% | 30-day rolling |
| Circuit-breaker closed ratio | `(time in Closed state) / total time` | ≥ 99.0% | 7-day rolling |
| Bulkhead headroom | `1 - (exhausted_acquires / total_acquires)` | ≤ 0.5% exhaustions | 1-day rolling |
| Acquire p99 latency | `sentinel_acquire_duration_seconds_p99` | < 100 µs | 1-hour rolling |

### Error Budget

Each SLO has a 30-day error budget:

| SLO | Budget (minutes/30d) |
|-----|----------------------|
| Rate-limiter availability | 216 min (0.5%) |
| Circuit-breaker closed ratio | 432 min (1.0%) |
| Bulkhead headroom | N/A (rate-based) |

When the budget is more than 50% consumed in the first half of the window, trigger
a "budget at risk" investigation. When 100% consumed, freeze non-critical rollouts.

---

## Burn-Rate Alerts

### Alert: `SentinelCircuitOpenBurnRate`

```yaml
# Prometheus alerting rule example
- alert: SentinelCircuitOpenBurnRate
  expr: |
    (
      rate(sentinel_circuit_breaker_state_transitions_total{to="open"}[1h])
      / rate(sentinel_circuit_breaker_requests_total[1h])
    ) > 0.01
  for: 5m
  labels:
    severity: warning
  annotations:
    summary: "Circuit breaker opening too frequently (>1% of requests trigger open)"
    runbook: "docs/operations/RUNBOOK.md#circuit-breaker-opens-repeatedly"
```

### Alert: `SentinelBulkheadExhaustionSpike`

```yaml
- alert: SentinelBulkheadExhaustionSpike
  expr: |
    rate(sentinel_bulkhead_acquire_failed_total[5m]) > 10
  for: 2m
  labels:
    severity: critical
  annotations:
    summary: "Bulkhead rejecting >10 acquires/sec; partition capacity may be undersized"
    runbook: "docs/operations/RUNBOOK.md#bulkhead-exhaustion"
```

---

## Incident Response Playbooks

### Circuit breaker opens repeatedly

**Symptoms**: `sentinel_circuit_breaker_state` stuck in `Open` or frequently
transitioning `Closed → Open`.

1. Check downstream health: `curl -s <downstream>/health` from the calling service.
2. Inspect failure window: `CircuitBreakerConfig.failure_threshold` and
   `recovery_timeout` in the caller's config.
3. If downstream is healthy but slow: increase `recovery_timeout` and reduce
   `failure_threshold` to avoid premature opens.
4. If downstream is genuinely down: leave the breaker open; it prevents cascading
   load while the downstream recovers.
5. Manual reset (once downstream is confirmed healthy):
   ```rust
   cb.record_success(); // transitions HalfOpen → Closed on next allowed request
   ```
6. Add a postmortem entry in `docs/operations/POSTMORTEMS.md`.

### Bulkhead exhaustion

**Symptoms**: `BulkheadError::PartitionExhausted` or `TotalExhausted` errors
spike; callers receive immediate rejections.

1. Check `bulkhead.usage(partition).await` for each partition.
2. If one partition is starving others: redistribute workload or increase
   `capacity_per_partition` for that partition.
3. If total capacity is exhausted: evaluate whether `num_partitions *
   capacity_per_partition` matches actual concurrency demand.
4. Scale configuration via `BulkheadConfig` — no code change required for
   capacity tuning; config is runtime-injectable.
5. Postmortem: record root cause and config delta in `docs/operations/POSTMORTEMS.md`.

### Rate-limiter false positives

**Symptoms**: Legitimate traffic rejected by `TokenBucket`; clients observe
`RateLimiterError::Exhausted` at below-expected rates.

1. Verify clock skew: `TokenBucket` uses `std::time::Instant`; NTP drift on
   multi-node deploys does not affect per-process bucket state.
2. Check `refill_rate`: must be tuned to steady-state RPS, not peak.
3. For burst workloads, increase `capacity` (burst headroom) without changing
   `refill_rate` (steady-state budget).
4. If false-positive rate is systemic, switch to `LeakyBucket` for strict
   output rate control instead of burst tolerance.

---

## Postmortem Template

Add new postmortems to `docs/operations/POSTMORTEMS.md` using:

```markdown
## YYYY-MM-DD — <title>

**Impact**: <duration>, <scope>, <user-visible effect>
**Root cause**: <concise technical cause>
**Detection**: <how and when discovered>
**Timeline**:
- HH:MM — first alert / symptom observed
- HH:MM — diagnosis confirmed
- HH:MM — mitigation applied
- HH:MM — resolved

**Action items**:
- [ ] <concrete fix with owner>
```

---

## Metrics Instrumentation Guidance

phenotype-sentinel does not yet emit metrics autonomously. Callers should wrap
sentinel API calls with their preferred metrics library:

```rust
// Example: instrument TokenBucket with the `metrics` crate
use metrics::{counter, histogram};
use std::time::Instant;

let start = Instant::now();
if bucket.try_acquire() {
    counter!("sentinel.rate_limiter.allowed").increment(1);
} else {
    counter!("sentinel.rate_limiter.rejected").increment(1);
}
histogram!("sentinel.rate_limiter.acquire_duration_us")
    .record(start.elapsed().as_micros() as f64);
```

A first-party optional `metrics` feature flag for sentinel is tracked in
[GitHub Issues](https://github.com/KooshaPari/Tracely/issues) — search for
"sentinel metrics integration".
