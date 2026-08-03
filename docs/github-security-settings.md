# GitHub Security Settings

GitHub CodeQL default setup is required for the public repository. This project
intentionally does not add an advanced CodeQL workflow because default and
advanced setups must not duplicate analysis uploads.

Before a release tag:

1. Open repository settings and go to Code security.
2. Confirm CodeQL default setup is active for the default branch.
3. Confirm the latest CodeQL analysis and Rust CI completed successfully for
   the final release commit.
4. Record those checks in the pentest report for a publication checkpoint, or
   in the release notes/check evidence for an interim engineering checkpoint.
