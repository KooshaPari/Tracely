# Friction Log

Date observations of friction, confusion, or wasted time while working on Tracely.

| Date | Area | Friction | Resolution | Status |
|------|------|----------|------------|--------|
| 2026-07-08 | CI | CI only runs on ubuntu — macOS users can't validate locally without docker | Added macOS+Windows to CI matrix | ✗ Not yet resolved |
| 2026-07-08 | Testing | No criterion bench harness despite criterion being a dev-dep | Need to add benches/ with benchmark targets | ✗ Not yet resolved |
| 2026-07-08 | Release | No automated cargo publish on tag — manual process error-prone | Need to add release workflow | ✗ Not yet resolved |

## How to Log

When you encounter friction:
1. Add a row to the table above with date, area, description, and proposed resolution
2. If resolved quickly, note that
3. If unresolved, leave status as ✗
