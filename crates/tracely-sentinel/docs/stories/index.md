# phenotype-sentinel — Stories

Stories expand each journey into concrete actor/action/outcome sequences.
They are the primary input for acceptance tests.

## Story Files

| Story | Journey | Summary |
|-------|---------|---------|
| [J-SENT-001.md](J-SENT-001.md) | J-SENT-001 | Operator limits API traffic using token-bucket; excess requests receive a fast `Exhausted` error within the SLO budget. |
| [J-SENT-002.md](J-SENT-002.md) | J-SENT-002 | Operator wraps a flaky downstream; after 5 consecutive failures the circuit opens and callers receive `CircuitOpen` without waiting for upstream timeouts. |
| [J-SENT-003.md](J-SENT-003.md) | J-SENT-003 | Operator partitions a shared thread pool; a noisy partition cannot starve other partitions beyond its configured capacity. |
| [J-SENT-004.md](J-SENT-004.md) | J-SENT-004 | Operator exposes sentinel state via metrics endpoint; burn-rate alert fires within one SLO window of threshold breach. |

## Format Convention

Each story uses:

```
Actor: <who is acting>
Precondition: <system state before>
Steps: numbered action list
Expected outcome: observable, verifiable result
FR references: FR-SENT-xxx IDs from FUNCTIONAL_REQUIREMENTS.md
```