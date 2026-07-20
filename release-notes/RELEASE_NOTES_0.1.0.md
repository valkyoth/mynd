# mynd 0.1.0 Release Notes

Status: planned; not yet released.

This release establishes the virtual workspace, facade/core package boundaries,
dual license, `no_std` and unsafe-forbidden baseline, toolchain/platform CI,
security policies, source ledger, and staged release plans. It contains no
image decoder or encoder and makes no format-support claim.

Security evidence required before release:

- all repository checks pass on Rust 1.90.0 and 1.97.1;
- the full supported Rust matrix compiles and tests;
- every initial target build passes;
- package contents and zero third-party runtime dependencies are verified;
- `security/pentest/v0.1.0.md` reaches `Status: PASS`.
