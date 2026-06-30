# Journey Manifests

Journey manifests capture the observable, verifiable flows through tracely and
phenotype-sentinel for use in CI verification and regression guards.

## Manifest Catalog

| Manifest | Crate | Journey ID | Description |
|----------|-------|------------|-------------|
| `sentinel-rate-limit.yaml` | tracely-sentinel | J-SENT-001 | Token-bucket rate-limit an HTTP handler |
| `sentinel-circuit-breaker.yaml` | tracely-sentinel | J-SENT-002 | Circuit breaker trip + recovery cycle |
| `sentinel-bulkhead.yaml` | tracely-sentinel | J-SENT-003 | Bulkhead partition isolation |
| `tracely-init.yaml` | tracely-core | J-CORE-001 | Library init + first trace emission |

## Format

Each manifest is a YAML file following the
[phenotype-infra journey-traceability standard](https://github.com/kooshapari/phenotype-infra/blob/main/docs/governance/journey-traceability-standard.md).

```yaml
id: J-SENT-001
title: "Rate-limit a high-traffic API service"
crate: phenotype-sentinel
actor: "API service operator"
steps:
  - action: "Construct TokenBucket with capacity=100, refill_rate=10"
    expected: "Bucket created with 100 tokens available"
  - action: "Acquire 100 tokens sequentially"
    expected: "All 100 succeed; remaining = 0"
  - action: "Attempt 101st acquisition"
    expected: "try_acquire returns false"
fr_refs:
  - FR-SENT-001
  - FR-SENT-002
```

## Verification

Run `just journey-verify` (wired to `phenotype-journey verify` when available) or
execute the integration tests in `tests/` which cover each journey step.
