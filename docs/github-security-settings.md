# GitHub Security Settings

The required repository state is: GitHub CodeQL analysis default setup is active.
This project intentionally does not add an advanced CodeQL workflow,
because default and advanced setups must not duplicate analysis uploads.

Before a release tag:

1. Open repository settings and go to Code security.
2. Confirm CodeQL default setup is active for the default branch.
3. Confirm dependency graph, Dependabot alerts, and secret scanning settings.
4. Confirm the latest analysis succeeded for the release commit.
5. Record the check in the release pentest report.
