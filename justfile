# Phenotype-org standard justfile

default:
    @just --list

# Build workspace
build:
    cargo build --workspace

# Run tests
test:
    cargo test --workspace

# Lint (clippy + fmt --check)
lint:
    cargo clippy --workspace -- -D warnings
    cargo fmt --check

# Format code
fmt:
    cargo fmt

# Security audits (cargo-deny + cargo-audit)
audit:
    cargo deny check
    cargo audit

# Find unused dependencies
unused:
    cargo machete

# Full local CI sweep
ci: lint test audit unused

# Generate docs
docs:
    cargo doc --no-deps --workspace

# Run benchmarks
bench:
    cargo bench --workspace

# Fuzz (requires nightly + cargo-fuzz)
fuzz:
    cargo fuzz run init_fuzz --fuzz-dir crates/tracely-core/fuzz

fuzz-sentinel:
    cargo fuzz run ingest_fuzz --fuzz-dir crates/tracely-sentinel/fuzz

# Audit report (scorecard)
scorecard:
    @echo "See docs/audit/100-PILLAR-AUDIT-REPORT.md"

# Mutation testing (requires cargo-mutants)
mutation:
    cargo mutants --in-place --timeout 120

# Flake detection — run tests N times (default: 5)
flake-detection runs="5":
    @echo "Running tests {{runs}} times to detect flaky tests..."
    @failed=0; \
    for i in $$(seq 1 {{runs}}); do \
      echo "--- Run $$i / {{runs}} ---"; \
      if cargo test --workspace --quiet 2>/dev/null; then \
        echo "  PASS"; \
      else \
        echo "  FAIL"; \
        failed=$$((failed + 1)); \
      fi; \
    done; \
    echo ""; \
    echo "=== Results: $$(({{runs}} - failed))/{{runs}} passed, $$failed failed ==="; \
    if [ "$$failed" -gt 0 ]; then \
      echo "⚠  Flaky tests detected!"; \
    fi

# Full pre-release check
release-check: lint test audit unused bench mutation
