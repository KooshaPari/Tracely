# Changelog

All notable changes to this project will be documented in this file.

## 🐛 Bug Fixes
- Fix: remove duplicate workspace declaration from tracely-sentinel

- Removed [workspace] declaration from tracely-sentinel/Cargo.toml
- Fixed Cargo.toml dependencies formatting
- Tracely now compiles and all 11 tests pass (`eb922c8`)
## 📚 Documentation
- Docs(readme): expand README.md with purpose, stack, quick-start, related projects (`6114bca`)
- Docs: add PLAN.md (`9578e16`)
- Docs: add SPEC.md (`b305206`)
- Docs: add real spec docs (PRD, FR, ADR) (#2)

* docs: add PRD.md

* docs: add FUNCTIONAL_REQUIREMENTS.md

* docs: add ADR.md

* docs: update CLAUDE.md with full spec context (`3691945`)
- Docs: add real spec docs (PRD, FR, ADR, CLAUDE.md) (#1)

* docs: add PRD.md

* docs: add FUNCTIONAL_REQUIREMENTS.md

* docs: add ADR.md

* docs: update CLAUDE.md with full spec context (`06c1824`)
- Docs: add CLAUDE.md with development guidelines (`9feea75`)
## ✨ Features
- Feat: absorb helix-logging and helix-tracing into tracely (#3)

Consolidates three separate observability repos into one unified crate:
- src/logging.rs: structured logging with LoggerConfig, LogContext, log_json! macro
  (absorbed from KooshaPari/helix-logging, archived 2026-03-26)
- src/tracing.rs: distributed tracing with TracingConfig, TraceContext, span/trace IDs
  (absorbed from KooshaPari/helix-tracing, archived 2026-03-26)
- src/lib.rs: re-exports both modules; bumps version to 0.2.0

Deps updated: adds env_logger, log, uuid, chrono; makes otel/prometheus/metrics
optional features; drops unused direct opentelemetry/prometheus/metrics defaults.

Co-authored-by: Claude Code <claude@anthropic.com>
Co-authored-by: Claude Sonnet 4.6 <noreply@anthropic.com> (`8542609`)
## 🔨 Other
- Chore(governance): adopt standard CLAUDE.md + AGENTS.md + worklog (wave-2) (`e634fb8`)
- Test(smoke): seed minimal smoke test — proves harness works (`eee6a63`)
- Chore(ci): adopt phenotype-tooling quality-gate + fr-coverage (`61ec5f3`)
- Chore(tracely): annotate 5 dead_code suppressions with kept reasons

All 5 suppressions in tracely-sentinel/src/validation.rs mark public
builder API surface (Validator::integer/min/max/validate, validate_field
helper) kept as intentional no-ops for downstream API compatibility.
Added `// kept: ...` rationale above each.

cargo build -p phenotype-sentinel: clean in 9s.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com> (`70c7efb`)
- Chore: add AgilePlus scaffolding (`27d6ffe`)
- Ci(legacy-enforcement): add legacy tooling anti-pattern gate (WARN mode)

Adds legacy-tooling-gate.yml monitoring for anti-patterns per CLAUDE.md.

Refs: CLAUDE.md Technology Adoption Philosophy (`654a26a`)
- Merge: add Zerokit and helix-tracing crates (`847b39b`)
- Merge: add phenoLoggingZig crate (`5d2d932`)
- Ci: migrate to reusable workflows from template-commons

- Use reusable-rust-ci.yml, reusable-python-ci.yml, reusable-typescript-ci.yml
- Add security scanning with reusable-security-scan.yml
- Add governance validation with validate-governance.yml (`9c66306`)
- Add phenoSentinel as tracely-sentinel crate (`e1d7477`)
- Convert to workspace: Add phenoSentinel as tracely-sentinel crate (`afc4eaa`)
## 🔨 Other
- Chore: add unit tests, CI/CD with publishing, and package config (`8f717c5`)
- Chore: add package config for registry publishing (`cd6a655`)
- Chore: add CI/CD, package manifest, and initial structure (`c769e05`)
- Initial tracely - unified observability (`a28e365`)