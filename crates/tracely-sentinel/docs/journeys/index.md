# phenotype-sentinel — User Journeys

This index lists the primary operator journeys for the sentinel resilience library.
Each journey links to the story file and traceability matrix for that scenario.

## Journey Catalog

| ID | Title | Entry Point | Stories |
|----|-------|-------------|---------|
| J-SENT-001 | Rate-limit a high-traffic API service | `TokenBucket::new` | [stories/J-SENT-001.md](../stories/J-SENT-001.md) |
| J-SENT-002 | Protect a flaky downstream with circuit breaking | `CircuitBreaker::new` | [stories/J-SENT-002.md](../stories/J-SENT-002.md) |
| J-SENT-003 | Isolate concurrent workloads with bulkhead partitions | `Bulkhead::new` | [stories/J-SENT-003.md](../stories/J-SENT-003.md) |
| J-SENT-004 | Observe and alert on resilience state | `state()` / metrics hooks | [stories/J-SENT-004.md](../stories/J-SENT-004.md) |

## Traceability

All journeys trace to `FUNCTIONAL_REQUIREMENTS.md` via the matrix in
[traceability/index.md](../traceability/index.md).