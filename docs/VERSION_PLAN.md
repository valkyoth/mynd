# mynd Release Plan To 1.0

Status: active planning document

This plan is intentionally granular. `mynd` handles hostile binary image
input, so every milestone must remain small enough to implement, test, review,
pentest, and stop cleanly before tagging. Split a milestone or add a patch
release whenever its scope no longer fits one safe review pass.

Tags use:

```text
v0.N.0       milestone release
v0.N.P       patch or security-fix release for milestone N
v1.0.0-rc.N exact 1.0-versioned production candidate
v1.0.0       first serious production-ready mynd crate
```

## Release principles

Every release must have:

- a clear definition of done and one primary outcome;
- explicit `Status`, `Goal`, `Deliverables`, `Verification`, and
  `Exit criteria` blocks;
- current official/original specification evidence for format behavior;
- local verification commands and adversarial tests appropriate to its scope;
- documented limits, unsupported behavior, and compatibility decisions;
- release notes, crate-version metadata, package inspection, and SBOM evidence;
- a completed pentest report for the exact implementation commit;
- green GitHub CI and CodeQL default setup before tag creation;
- no hidden dependency on one maintainer machine.

Each release should prefer one parser state, arithmetic boundary, format
capability, or assurance result at a time. Summary tables are navigation only;
they never replace the detailed release handoff sections below.

## Required milestone format

Every release section must contain:

- `Status`: planned, in implementation, awaiting pentest, or ready to tag;
- `Goal`: the single outcome the release exists to achieve;
- `Deliverables`: bounded code, tests, evidence, and documentation in scope;
- `Verification`: release-specific positive, boundary, malformed, truncation,
  round-trip, differential, fuzz, proof, platform, or packaging evidence;
- `Exit criteria`: observable completion conditions ending with
  `vX.Y.Z implementation stop reached. Run pentest for this exact commit.`

Release-specific verification is additive to repository-wide gates. It never
replaces formatting, linting, the Rust/toolchain/target matrices, dependency
policy, SBOM comparison, documentation checks, exact-commit pentesting, clean
retesting, GitHub CI/CodeQL review, or release metadata validation.

## Pentest before tags

Every milestone and patch version must pass security review and pentest before
it is tagged. Implementation completion is a handoff point, not tag authority.
A version is not tag-ready until:

- `scripts/checks.sh` passes;
- `scripts/check_latest_tools.sh` confirms current Rust, cargo-tool, and
  GitHub Actions pins;
- the applicable Rust compatibility and platform-target gates pass;
- `cargo deny check` and `cargo audit` pass;
- `scripts/generate-sbom.sh --check` matches the committed SPDX inventory;
- specification sources, support claims, limits, and compatibility notes match
  the implemented behavior;
- `release-notes/RELEASE_NOTES_X.Y.Z.md` exists;
- `security/pentest/vX.Y.Z.md` names the exact 40-character reviewed commit,
  tester, date, scope, findings, and residual risk, with `Status: PASS`;
- root `PENTEST.md` scratch findings have been resolved and removed;
- GitHub CI and CodeQL default setup are green for the reviewed commit;
- the version-specific release gate and
  `MYND_RELEASE_REQUIRE_PASS=1 scripts/validate-release-metadata.sh X.Y.Z`
  pass before tag creation;
- the tag does not already exist and will point at the approved report commit.

When implementation reaches its exit criteria, stop and report exactly:

```text
vX.Y.Z implementation stop reached. Run pentest for this exact commit.
```

Do not create a tag at that point.

### Pentest handoff flow

1. Finish only the milestone's deliverables and verification.
2. Freeze and identify the exact implementation commit.
3. Run all local repository, dependency, SBOM, tool, and release-specific gates.
4. Record temporary findings in ignored root `PENTEST.md`.
5. Fix release-scoped findings, update tests/docs/notes, and remove `PENTEST.md`.
6. Re-run every affected gate and request follow-up review.
7. After CI and CodeQL are green, write the permanent exact-commit report under
   `security/pentest/` with `Status: PASS`.
8. Run the version-specific release gate and release metadata validation.
9. Tag only the approved report commit; any later change invalidates approval.

## Crate versioning

The `mynd` facade is the integration train. Changed support crates use their
independent versions; unchanged crates are not republished. Patch releases add
no new format capability. Update `release-crates.toml`,
`docs/CRATE_VERSION_MATRIX.md`, changelog, release notes, SBOM, and pentest
metadata together.

## Milestone summary

| Version | Capability | Primary security review |
| --- | --- | --- |
| 0.1.0 | Virtual workspace, facade/core skeletons, licensing, no_std baseline, policies and plans | Package contents, no unexpected dependency/build script/I/O/unsafe |
| 0.2.0 | Finalize threat model, trust boundaries, attacker capabilities, and non-goals | Memory, CPU, metadata, decompression, animation, parser confusion |
| 0.3.0 | Specification ledger and corpus-provenance schema | Copyright, source integrity, fixture licensing |
| 0.4.0 | Disclosure flow, supported-version policy, advisory template | Private reporting and maintenance clarity |
| 0.5.0 | Release manifest and tag-delta pentest checklist | Simulated audit completeness |
| 0.6.0 | `mynd-math`: checked conversion, add, multiply, align, and range | Extrema, zero, overflow, truncation |
| 0.7.0 | Validated dimensions, stride, rectangle, and output length | Area/stride/rectangle overflow and zero dimensions |
| 0.8.0 | Sample, channel, packing, and endianness domains | Invalid and unsupported combinations |
| 0.9.0 | Color model, alpha mode, pixel format, common constants | Ambiguous alpha, channel/palette invariants |
| 0.10.0 | Checked immutable and mutable image views | Short buffers, strides, last row, overlap |
| 0.11.0 | Frame rectangles, timing, disposal, blend, canvas types | Off-canvas frames, timing and canvas overflow |
| 0.12.0 | Fallible owned image under `alloc`; first core audit | Reservation failure, initialized length, panic audit |
| 0.13.0 | Slice reader/writer, I/O error, exact reads | Partial/empty reads, cursor overflow |
| 0.14.0 | Little/big-endian integer I/O | Every-byte truncation, sign conversion |
| 0.15.0 | Bounded, subrange, and counting readers | Nested bound escape and offset overflow |
| 0.16.0 | Seek/read-at traits with slice implementations | Beyond-source and backward seeks |
| 0.17.0 | Fixed writer and transactional checkpoints | Partial output and rollback |
| 0.18.0 | LSB/MSB bit readers | Shift width, refill, one-bit truncation |
| 0.19.0 | LSB/MSB bit writers | Padding, transitions, exact length |
| 0.20.0 | Optional `std::io` adapters | Partial/interrupted I/O and position agreement |
| 0.21.0 | Format ID, hints, media types, bounded probing | Collisions, spoofing, ambiguous prefixes |
| 0.22.0 | Decode limits, work budget, conservative presets | Cross-budget bypass and conversion |
| 0.23.0 | Structured errors, warning sink, reports | Logging injection, offsets, bounded warnings |
| 0.24.0 | Incremental decoder/encoder contracts; architecture freeze | Stalls and consumed/produced invariants |
| 0.25.0 | BMP probe and file header | Near signatures, short input, pixel offset |
| 0.26.0 | DIB size dispatch and unknown reporting | Header-size confusion and huge declarations |
| 0.27.0 | OS/2 BITMAPCOREHEADER | Narrow conversions and palette sizing |
| 0.28.0 | Windows BITMAPINFOHEADER | Signed height, planes, compression/depth |
| 0.29.0 | OS/2 2.x headers and unsupported subtypes | Header overlap and reserved values |
| 0.30.0 | Windows V4/V5 headers | Profiles, masks, color-space confusion |
| 0.31.0 | Shared validation and row layout | Padding, negative height, source end |
| 0.32.0 | 24-bit BI_RGB decode to caller RGB/BGR | Bottom-up order, padding, stride, truncation |
| 0.33.0 | 32-bit uncompressed decode and alpha policy | Unused byte versus alpha |
| 0.34.0 | 1-bit indexed decode | Bit order, tail bits, palette bounds |
| 0.35.0 | 4-bit indexed decode | Odd width and nibble order |
| 0.36.0 | 8-bit indexed decode | Palette count and short arrays |
| 0.37.0 | 16-bit default RGB decode | Expansion, endian, odd truncation |
| 0.38.0 | 16-bit bitfield decode | Zero/overlap/non-contiguous masks |
| 0.39.0 | 32-bit bitfields and alpha mask | Full-width masks and shifts |
| 0.40.0 | Complete top-down and palette-alpha policy | Signed minimum and reversed rows |
| 0.41.0 | RLE8 decode | Escapes, deltas, rows, expansion |
| 0.42.0 | RLE4 decode | Absolute padding, nibbles, odd count |
| 0.43.0 | Bounded profiles/metadata; embedded payload reporting | Nested offsets and overlap |
| 0.44.0 | Canonical uncompressed true-color/indexed encoder | Lengths, padding, palette count |
| 0.45.0 | RLE encoder and full BMP conformance/fuzz audit | Symmetry and malformed corpus marathon |
| 0.46.0 | Probe and 18-byte header | Weak signatures, type, ID length |
| 0.47.0 | 24-bit true-color decode | BGR, orientation, row truncation |
| 0.48.0 | 32-bit true-color and alpha | Attribute bits and output mismatch |
| 0.49.0 | 15/16-bit true-color | Expansion, attribute, endian |
| 0.50.0 | 8/16-bit grayscale | Depth combinations and luma-alpha |
| 0.51.0 | Color-mapped decode | Map origin, entry width, indices |
| 0.52.0 | Origins and supported interleaving | Coordinate mapping and duplicate rows |
| 0.53.0 | Generic bounded RLE packet parser | Zero progress and packet arithmetic |
| 0.54.0 | True-color RLE decode | Image/row bounds and truncation |
| 0.55.0 | Grayscale/indexed RLE decode | Lookup and raw/run packet bounds |
| 0.56.0 | Bounded image-ID handling | Non-text logging and preservation |
| 0.57.0 | TGA 2.0 footer and extension area | Offsets, sizes, contradictory metadata |
| 0.58.0 | Developer area, correction table, thumbnail | Counts, nested offsets, overlap |
| 0.59.0 | Canonical raw/RLE encoders | Packets, origins, deterministic output |
| 0.60.0 | Legacy compatibility and complete TGA audit | Cross-scanline RLE and differential corpus |
| 0.61.0 | GIF87a/89a signature and version | Near signatures and truncation |
| 0.62.0 | Logical screen descriptor | Canvas area and packed/reserved bits |
| 0.63.0 | Global color table | Table length, truncation, no-allocation path |
| 0.64.0 | Incremental sub-block reader | Missing terminator, total bytes, chunking |
| 0.65.0 | Image descriptor and local table | Rectangle overflow and palette selection |
| 0.66.0 | LZW code extraction and LSB integration | Code size and bit transitions |
| 0.67.0 | Fixed LZW dictionary init/reset | Index bounds, clear behavior, stale state |
| 0.68.0 | Code-width growth and saturation | Exact transition and 12-bit maximum |
| 0.69.0 | Undefined-code special case and rejection | Previous state and forward references |
| 0.70.0 | Pixel accounting and exact frame size | Expansion bombs and no-progress loops |
| 0.71.0 | Four-pass deinterlacing | Tiny heights, duplicate/destination rows |
| 0.72.0 | Complete single-image decode | Palette indices and trailing data |
| 0.73.0 | Graphic Control Extension | Block size, reserved bits, terminator |
| 0.74.0 | Transparency index | Palette bounds and alpha conversion |
| 0.75.0 | Frame clipping and canvas placement | Rectangle addition and partial frames |
| 0.76.0 | Delay representation and timing policy | Zero and cumulative duration |
| 0.77.0 | Keep/do-not-dispose behavior | Retained canvas and ordering |
| 0.78.0 | Restore-to-background | Background selection and affected region |
| 0.79.0 | Restore-to-previous with bounded snapshots | Memory totals and snapshot lifetime |
| 0.80.0 | Raw-frame API | Local versus canvas coordinates |
| 0.81.0 | Composited-frame API | Disposal sequencing and reuse |
| 0.82.0 | Generic bounded extension state machine | Unknown labels and block bombs |
| 0.83.0 | Loops, comments, plain text, unknown extensions | Ordering and metadata limits |
| 0.84.0 | Full GIF decoder conformance/fuzz/differential audit | LZW state space and animation bombs |
| 0.85.0 | Exact palettes and bounded histogram | Indexing, unique colors, determinism |
| 0.86.0 | Deterministic median cut | Empty boxes and split arithmetic |
| 0.87.0 | Palette remap and ordered dither | Distance overflow and work accounting |
| 0.88.0 | GIF LZW encoder | Width transitions and dictionary reset |
| 0.89.0 | Single-frame GIF encoder | Table sizes, transparency, sub-blocks |
| 0.90.0 | Animated GIF encoder | Canvas/frame totals and disposal round trip |
| 0.91.0 | Feature-gated registry and bounded probing | Ambiguity, disabled formats, ordering |
| 0.92.0 | Static `AnyDecoder` dispatch | Variant/error/state confusion |
| 0.93.0 | Unified `decode_into` facade | Limit propagation and partial output |
| 0.94.0 | Fallible owned decode under `alloc` | Reservation and warning collection |
| 0.95.0 | Unified encoder/writer facade | Selection and partial output |
| 0.96.0 | Basic color and alpha conversion | Rounding, division, premultiplication |
| 0.97.0 | Crop, flip, rotate, normalize, nearest resize | Overlap, rectangles, work budget |
| 0.98.0 | CLI inspect/validate/decode/convert/frame/limits | Paths, defaults, terminal injection |
| 0.99.0 | Audit every crate with no default features | Accidental std/alloc/macros/panics |
| 0.100.0 | Complete core/alloc/std and encode/decode matrix | Invalid feature combinations |
| 0.101.0 | Workspace panic/assertion audit | Unwrap, indexing, divide, build-mode differences |
| 0.102.0 | Arithmetic/offset/cast/stride audit | Every input-derived calculation |
| 0.103.0 | Calibrate limits on normal/hostile corpora | Bypass and unusable defaults |
| 0.104.0 | Automated every-byte truncation suite | Stalls, partial state, error consistency |
| 0.105.0 | Corpus provenance, conformance matrix, differential suite | Unsupported claims and disagreements |
| 0.106.0 | Extended fuzz campaign and coverage review | Unreached states and cycles |
| 0.107.0 | Kani: math, views, bits, clipping | Assumptions and unwind bounds |
| 0.108.0 | Kani: BMP/TGA RLE and GIF LZW | Dictionary, packet, output, row invariants |
| 0.109.0 | Miri, sanitizers, unsafe inventory, memory audit | Alias, initialization, SIMD boundaries |
| 0.110.0 | DoS/performance, reproducibility, API docs, RC1 | Release arithmetic and package contents |
| 0.111.0 | External delta pentest, RC2, API freeze decision | Entire attack surface and open findings |

## Phase A: governance and image model

### v0.1.0 - Virtual workspace, facade/core skeletons, licensing, no_std baseline, policies and plans

Status: implementation complete; awaiting pentest.

Goal:

Establish the release-scoped outcome: Virtual workspace, facade/core skeletons, licensing, no_std baseline, policies and plans. Keep this capability bounded
and independently reviewable. Center security review on package contents, no unexpected dependency/build script/I/O/unsafe.

Deliverables:

- Implement and document only the release-scoped capability: Virtual workspace, facade/core skeletons, licensing, no_std baseline, policies and plans.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for package contents, no unexpected dependency/build script/I/O/unsafe.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  virtual workspace, facade/core skeletons, licensing, no_std baseline, policies and plans.
- Exercise adversarial cases covering package contents, no unexpected dependency/build script/I/O/unsafe.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.1.0 implementation stop reached. Run pentest for this exact commit.`

### v0.2.0 - Finalize threat model, trust boundaries, attacker capabilities, and non-goals

Status: planned.

Goal:

Establish the release-scoped outcome: Finalize threat model, trust boundaries, attacker capabilities, and non-goals. Keep this capability bounded
and independently reviewable. Center security review on memory, CPU, metadata, decompression, animation, parser confusion.

Deliverables:

- Implement and document only the release-scoped capability: Finalize threat model, trust boundaries, attacker capabilities, and non-goals.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for memory, CPU, metadata, decompression, animation, parser confusion.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  finalize threat model, trust boundaries, attacker capabilities, and non-goals.
- Exercise adversarial cases covering memory, CPU, metadata, decompression, animation, parser confusion.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.2.0 implementation stop reached. Run pentest for this exact commit.`

### v0.3.0 - Specification ledger and corpus-provenance schema

Status: planned.

Goal:

Establish the release-scoped outcome: Specification ledger and corpus-provenance schema. Keep this capability bounded
and independently reviewable. Center security review on copyright, source integrity, fixture licensing.

Deliverables:

- Implement and document only the release-scoped capability: Specification ledger and corpus-provenance schema.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for copyright, source integrity, fixture licensing.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  specification ledger and corpus-provenance schema.
- Exercise adversarial cases covering copyright, source integrity, fixture licensing.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.3.0 implementation stop reached. Run pentest for this exact commit.`

### v0.4.0 - Disclosure flow, supported-version policy, advisory template

Status: planned.

Goal:

Establish the release-scoped outcome: Disclosure flow, supported-version policy, advisory template. Keep this capability bounded
and independently reviewable. Center security review on private reporting and maintenance clarity.

Deliverables:

- Implement and document only the release-scoped capability: Disclosure flow, supported-version policy, advisory template.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for private reporting and maintenance clarity.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  disclosure flow, supported-version policy, advisory template.
- Exercise adversarial cases covering private reporting and maintenance clarity.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.4.0 implementation stop reached. Run pentest for this exact commit.`

### v0.5.0 - Release manifest and tag-delta pentest checklist

Status: planned.

Goal:

Establish the release-scoped outcome: Release manifest and tag-delta pentest checklist. Keep this capability bounded
and independently reviewable. Center security review on simulated audit completeness.

Deliverables:

- Implement and document only the release-scoped capability: Release manifest and tag-delta pentest checklist.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for simulated audit completeness.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  release manifest and tag-delta pentest checklist.
- Exercise adversarial cases covering simulated audit completeness.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.5.0 implementation stop reached. Run pentest for this exact commit.`

### v0.6.0 - `mynd-math`: checked conversion, add, multiply, align, and range

Status: planned.

Goal:

Establish the release-scoped outcome: Mynd-math: checked conversion, add, multiply, align, and range. Keep this capability bounded
and independently reviewable. Center security review on extrema, zero, overflow, truncation.

Deliverables:

- Implement and document only the release-scoped capability: `mynd-math`: checked conversion, add, multiply, align, and range.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for extrema, zero, overflow, truncation.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  mynd-math: checked conversion, add, multiply, align, and range.
- Exercise adversarial cases covering extrema, zero, overflow, truncation.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.6.0 implementation stop reached. Run pentest for this exact commit.`

### v0.7.0 - Validated dimensions, stride, rectangle, and output length

Status: planned.

Goal:

Establish the release-scoped outcome: Validated dimensions, stride, rectangle, and output length. Keep this capability bounded
and independently reviewable. Center security review on area/stride/rectangle overflow and zero dimensions.

Deliverables:

- Implement and document only the release-scoped capability: Validated dimensions, stride, rectangle, and output length.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for area/stride/rectangle overflow and zero dimensions.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  validated dimensions, stride, rectangle, and output length.
- Exercise adversarial cases covering area/stride/rectangle overflow and zero dimensions.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.7.0 implementation stop reached. Run pentest for this exact commit.`

### v0.8.0 - Sample, channel, packing, and endianness domains

Status: planned.

Goal:

Establish the release-scoped outcome: Sample, channel, packing, and endianness domains. Keep this capability bounded
and independently reviewable. Center security review on invalid and unsupported combinations.

Deliverables:

- Implement and document only the release-scoped capability: Sample, channel, packing, and endianness domains.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for invalid and unsupported combinations.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  sample, channel, packing, and endianness domains.
- Exercise adversarial cases covering invalid and unsupported combinations.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.8.0 implementation stop reached. Run pentest for this exact commit.`

### v0.9.0 - Color model, alpha mode, pixel format, common constants

Status: planned.

Goal:

Establish the release-scoped outcome: Color model, alpha mode, pixel format, common constants. Keep this capability bounded
and independently reviewable. Center security review on ambiguous alpha, channel/palette invariants.

Deliverables:

- Implement and document only the release-scoped capability: Color model, alpha mode, pixel format, common constants.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for ambiguous alpha, channel/palette invariants.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  color model, alpha mode, pixel format, common constants.
- Exercise adversarial cases covering ambiguous alpha, channel/palette invariants.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.9.0 implementation stop reached. Run pentest for this exact commit.`

### v0.10.0 - Checked immutable and mutable image views

Status: planned.

Goal:

Establish the release-scoped outcome: Checked immutable and mutable image views. Keep this capability bounded
and independently reviewable. Center security review on short buffers, strides, last row, overlap.

Deliverables:

- Implement and document only the release-scoped capability: Checked immutable and mutable image views.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for short buffers, strides, last row, overlap.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  checked immutable and mutable image views.
- Exercise adversarial cases covering short buffers, strides, last row, overlap.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.10.0 implementation stop reached. Run pentest for this exact commit.`

### v0.11.0 - Frame rectangles, timing, disposal, blend, canvas types

Status: planned.

Goal:

Establish the release-scoped outcome: Frame rectangles, timing, disposal, blend, canvas types. Keep this capability bounded
and independently reviewable. Center security review on off-canvas frames, timing and canvas overflow.

Deliverables:

- Implement and document only the release-scoped capability: Frame rectangles, timing, disposal, blend, canvas types.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for off-canvas frames, timing and canvas overflow.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  frame rectangles, timing, disposal, blend, canvas types.
- Exercise adversarial cases covering off-canvas frames, timing and canvas overflow.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.11.0 implementation stop reached. Run pentest for this exact commit.`

### v0.12.0 - Fallible owned image under `alloc`; first core audit

Status: planned.

Goal:

Establish the release-scoped outcome: Fallible owned image under alloc; first core audit. Keep this capability bounded
and independently reviewable. Center security review on reservation failure, initialized length, panic audit.

Deliverables:

- Implement and document only the release-scoped capability: Fallible owned image under `alloc`; first core audit.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for reservation failure, initialized length, panic audit.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  fallible owned image under alloc; first core audit.
- Exercise adversarial cases covering reservation failure, initialized length, panic audit.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.12.0 implementation stop reached. Run pentest for this exact commit.`

## Phase B: I/O and codec contracts

### v0.13.0 - Slice reader/writer, I/O error, exact reads

Status: planned.

Goal:

Establish the release-scoped outcome: Slice reader/writer, I/O error, exact reads. Keep this capability bounded
and independently reviewable. Center security review on partial/empty reads, cursor overflow.

Deliverables:

- Implement and document only the release-scoped capability: Slice reader/writer, I/O error, exact reads.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for partial/empty reads, cursor overflow.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  slice reader/writer, I/O error, exact reads.
- Exercise adversarial cases covering partial/empty reads, cursor overflow.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.13.0 implementation stop reached. Run pentest for this exact commit.`

### v0.14.0 - Little/big-endian integer I/O

Status: planned.

Goal:

Establish the release-scoped outcome: Little/big-endian integer I/O. Keep this capability bounded
and independently reviewable. Center security review on every-byte truncation, sign conversion.

Deliverables:

- Implement and document only the release-scoped capability: Little/big-endian integer I/O.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for every-byte truncation, sign conversion.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  little/big-endian integer I/O.
- Exercise adversarial cases covering every-byte truncation, sign conversion.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.14.0 implementation stop reached. Run pentest for this exact commit.`

### v0.15.0 - Bounded, subrange, and counting readers

Status: planned.

Goal:

Establish the release-scoped outcome: Bounded, subrange, and counting readers. Keep this capability bounded
and independently reviewable. Center security review on nested bound escape and offset overflow.

Deliverables:

- Implement and document only the release-scoped capability: Bounded, subrange, and counting readers.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for nested bound escape and offset overflow.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  bounded, subrange, and counting readers.
- Exercise adversarial cases covering nested bound escape and offset overflow.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.15.0 implementation stop reached. Run pentest for this exact commit.`

### v0.16.0 - Seek/read-at traits with slice implementations

Status: planned.

Goal:

Establish the release-scoped outcome: Seek/read-at traits with slice implementations. Keep this capability bounded
and independently reviewable. Center security review on beyond-source and backward seeks.

Deliverables:

- Implement and document only the release-scoped capability: Seek/read-at traits with slice implementations.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for beyond-source and backward seeks.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  seek/read-at traits with slice implementations.
- Exercise adversarial cases covering beyond-source and backward seeks.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.16.0 implementation stop reached. Run pentest for this exact commit.`

### v0.17.0 - Fixed writer and transactional checkpoints

Status: planned.

Goal:

Establish the release-scoped outcome: Fixed writer and transactional checkpoints. Keep this capability bounded
and independently reviewable. Center security review on partial output and rollback.

Deliverables:

- Implement and document only the release-scoped capability: Fixed writer and transactional checkpoints.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for partial output and rollback.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  fixed writer and transactional checkpoints.
- Exercise adversarial cases covering partial output and rollback.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.17.0 implementation stop reached. Run pentest for this exact commit.`

### v0.18.0 - LSB/MSB bit readers

Status: planned.

Goal:

Establish the release-scoped outcome: LSB/MSB bit readers. Keep this capability bounded
and independently reviewable. Center security review on shift width, refill, one-bit truncation.

Deliverables:

- Implement and document only the release-scoped capability: LSB/MSB bit readers.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for shift width, refill, one-bit truncation.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  lSB/MSB bit readers.
- Exercise adversarial cases covering shift width, refill, one-bit truncation.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.18.0 implementation stop reached. Run pentest for this exact commit.`

### v0.19.0 - LSB/MSB bit writers

Status: planned.

Goal:

Establish the release-scoped outcome: LSB/MSB bit writers. Keep this capability bounded
and independently reviewable. Center security review on padding, transitions, exact length.

Deliverables:

- Implement and document only the release-scoped capability: LSB/MSB bit writers.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for padding, transitions, exact length.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  lSB/MSB bit writers.
- Exercise adversarial cases covering padding, transitions, exact length.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.19.0 implementation stop reached. Run pentest for this exact commit.`

### v0.20.0 - Optional `std::io` adapters

Status: planned.

Goal:

Establish the release-scoped outcome: Optional std::io adapters. Keep this capability bounded
and independently reviewable. Center security review on partial/interrupted I/O and position agreement.

Deliverables:

- Implement and document only the release-scoped capability: Optional `std::io` adapters.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for partial/interrupted I/O and position agreement.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  optional std::io adapters.
- Exercise adversarial cases covering partial/interrupted I/O and position agreement.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.20.0 implementation stop reached. Run pentest for this exact commit.`

### v0.21.0 - Format ID, hints, media types, bounded probing

Status: planned.

Goal:

Establish the release-scoped outcome: Format ID, hints, media types, bounded probing. Keep this capability bounded
and independently reviewable. Center security review on collisions, spoofing, ambiguous prefixes.

Deliverables:

- Implement and document only the release-scoped capability: Format ID, hints, media types, bounded probing.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for collisions, spoofing, ambiguous prefixes.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  format ID, hints, media types, bounded probing.
- Exercise adversarial cases covering collisions, spoofing, ambiguous prefixes.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.21.0 implementation stop reached. Run pentest for this exact commit.`

### v0.22.0 - Decode limits, work budget, conservative presets

Status: planned.

Goal:

Establish the release-scoped outcome: Decode limits, work budget, conservative presets. Keep this capability bounded
and independently reviewable. Center security review on cross-budget bypass and conversion.

Deliverables:

- Implement and document only the release-scoped capability: Decode limits, work budget, conservative presets.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for cross-budget bypass and conversion.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  decode limits, work budget, conservative presets.
- Exercise adversarial cases covering cross-budget bypass and conversion.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.22.0 implementation stop reached. Run pentest for this exact commit.`

### v0.23.0 - Structured errors, warning sink, reports

Status: planned.

Goal:

Establish the release-scoped outcome: Structured errors, warning sink, reports. Keep this capability bounded
and independently reviewable. Center security review on logging injection, offsets, bounded warnings.

Deliverables:

- Implement and document only the release-scoped capability: Structured errors, warning sink, reports.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for logging injection, offsets, bounded warnings.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  structured errors, warning sink, reports.
- Exercise adversarial cases covering logging injection, offsets, bounded warnings.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.23.0 implementation stop reached. Run pentest for this exact commit.`

### v0.24.0 - Incremental decoder/encoder contracts; architecture freeze

Status: planned.

Goal:

Establish the release-scoped outcome: Incremental decoder/encoder contracts; architecture freeze. Keep this capability bounded
and independently reviewable. Center security review on stalls and consumed/produced invariants.

Deliverables:

- Implement and document only the release-scoped capability: Incremental decoder/encoder contracts; architecture freeze.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for stalls and consumed/produced invariants.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  incremental decoder/encoder contracts; architecture freeze.
- Exercise adversarial cases covering stalls and consumed/produced invariants.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.24.0 implementation stop reached. Run pentest for this exact commit.`

## Phase C: BMP

### v0.25.0 - BMP probe and file header

Status: planned.

Goal:

Establish the release-scoped outcome: BMP probe and file header. Keep this capability bounded
and independently reviewable. Center security review on near signatures, short input, pixel offset.

Deliverables:

- Implement and document only the release-scoped capability: BMP probe and file header.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for near signatures, short input, pixel offset.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  bMP probe and file header.
- Exercise adversarial cases covering near signatures, short input, pixel offset.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.25.0 implementation stop reached. Run pentest for this exact commit.`

### v0.26.0 - DIB size dispatch and unknown reporting

Status: planned.

Goal:

Establish the release-scoped outcome: DIB size dispatch and unknown reporting. Keep this capability bounded
and independently reviewable. Center security review on header-size confusion and huge declarations.

Deliverables:

- Implement and document only the release-scoped capability: DIB size dispatch and unknown reporting.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for header-size confusion and huge declarations.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  dIB size dispatch and unknown reporting.
- Exercise adversarial cases covering header-size confusion and huge declarations.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.26.0 implementation stop reached. Run pentest for this exact commit.`

### v0.27.0 - OS/2 BITMAPCOREHEADER

Status: planned.

Goal:

Establish the release-scoped outcome: OS/2 BITMAPCOREHEADER. Keep this capability bounded
and independently reviewable. Center security review on narrow conversions and palette sizing.

Deliverables:

- Implement and document only the release-scoped capability: OS/2 BITMAPCOREHEADER.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for narrow conversions and palette sizing.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  oS/2 BITMAPCOREHEADER.
- Exercise adversarial cases covering narrow conversions and palette sizing.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.27.0 implementation stop reached. Run pentest for this exact commit.`

### v0.28.0 - Windows BITMAPINFOHEADER

Status: planned.

Goal:

Establish the release-scoped outcome: Windows BITMAPINFOHEADER. Keep this capability bounded
and independently reviewable. Center security review on signed height, planes, compression/depth.

Deliverables:

- Implement and document only the release-scoped capability: Windows BITMAPINFOHEADER.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for signed height, planes, compression/depth.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  windows BITMAPINFOHEADER.
- Exercise adversarial cases covering signed height, planes, compression/depth.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.28.0 implementation stop reached. Run pentest for this exact commit.`

### v0.29.0 - OS/2 2.x headers and unsupported subtypes

Status: planned.

Goal:

Establish the release-scoped outcome: OS/2 2.x headers and unsupported subtypes. Keep this capability bounded
and independently reviewable. Center security review on header overlap and reserved values.

Deliverables:

- Implement and document only the release-scoped capability: OS/2 2.x headers and unsupported subtypes.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for header overlap and reserved values.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  oS/2 2.x headers and unsupported subtypes.
- Exercise adversarial cases covering header overlap and reserved values.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.29.0 implementation stop reached. Run pentest for this exact commit.`

### v0.30.0 - Windows V4/V5 headers

Status: planned.

Goal:

Establish the release-scoped outcome: Windows V4/V5 headers. Keep this capability bounded
and independently reviewable. Center security review on profiles, masks, color-space confusion.

Deliverables:

- Implement and document only the release-scoped capability: Windows V4/V5 headers.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for profiles, masks, color-space confusion.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  windows V4/V5 headers.
- Exercise adversarial cases covering profiles, masks, color-space confusion.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.30.0 implementation stop reached. Run pentest for this exact commit.`

### v0.31.0 - Shared validation and row layout

Status: planned.

Goal:

Establish the release-scoped outcome: Shared validation and row layout. Keep this capability bounded
and independently reviewable. Center security review on padding, negative height, source end.

Deliverables:

- Implement and document only the release-scoped capability: Shared validation and row layout.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for padding, negative height, source end.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  shared validation and row layout.
- Exercise adversarial cases covering padding, negative height, source end.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.31.0 implementation stop reached. Run pentest for this exact commit.`

### v0.32.0 - 24-bit BI_RGB decode to caller RGB/BGR

Status: planned.

Goal:

Establish the release-scoped outcome: 24-bit BI_RGB decode to caller RGB/BGR. Keep this capability bounded
and independently reviewable. Center security review on bottom-up order, padding, stride, truncation.

Deliverables:

- Implement and document only the release-scoped capability: 24-bit BI_RGB decode to caller RGB/BGR.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for bottom-up order, padding, stride, truncation.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  24-bit BI_RGB decode to caller RGB/BGR.
- Exercise adversarial cases covering bottom-up order, padding, stride, truncation.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.32.0 implementation stop reached. Run pentest for this exact commit.`

### v0.33.0 - 32-bit uncompressed decode and alpha policy

Status: planned.

Goal:

Establish the release-scoped outcome: 32-bit uncompressed decode and alpha policy. Keep this capability bounded
and independently reviewable. Center security review on unused byte versus alpha.

Deliverables:

- Implement and document only the release-scoped capability: 32-bit uncompressed decode and alpha policy.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for unused byte versus alpha.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  32-bit uncompressed decode and alpha policy.
- Exercise adversarial cases covering unused byte versus alpha.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.33.0 implementation stop reached. Run pentest for this exact commit.`

### v0.34.0 - 1-bit indexed decode

Status: planned.

Goal:

Establish the release-scoped outcome: 1-bit indexed decode. Keep this capability bounded
and independently reviewable. Center security review on bit order, tail bits, palette bounds.

Deliverables:

- Implement and document only the release-scoped capability: 1-bit indexed decode.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for bit order, tail bits, palette bounds.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  1-bit indexed decode.
- Exercise adversarial cases covering bit order, tail bits, palette bounds.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.34.0 implementation stop reached. Run pentest for this exact commit.`

### v0.35.0 - 4-bit indexed decode

Status: planned.

Goal:

Establish the release-scoped outcome: 4-bit indexed decode. Keep this capability bounded
and independently reviewable. Center security review on odd width and nibble order.

Deliverables:

- Implement and document only the release-scoped capability: 4-bit indexed decode.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for odd width and nibble order.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  4-bit indexed decode.
- Exercise adversarial cases covering odd width and nibble order.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.35.0 implementation stop reached. Run pentest for this exact commit.`

### v0.36.0 - 8-bit indexed decode

Status: planned.

Goal:

Establish the release-scoped outcome: 8-bit indexed decode. Keep this capability bounded
and independently reviewable. Center security review on palette count and short arrays.

Deliverables:

- Implement and document only the release-scoped capability: 8-bit indexed decode.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for palette count and short arrays.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  8-bit indexed decode.
- Exercise adversarial cases covering palette count and short arrays.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.36.0 implementation stop reached. Run pentest for this exact commit.`

### v0.37.0 - 16-bit default RGB decode

Status: planned.

Goal:

Establish the release-scoped outcome: 16-bit default RGB decode. Keep this capability bounded
and independently reviewable. Center security review on expansion, endian, odd truncation.

Deliverables:

- Implement and document only the release-scoped capability: 16-bit default RGB decode.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for expansion, endian, odd truncation.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  16-bit default RGB decode.
- Exercise adversarial cases covering expansion, endian, odd truncation.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.37.0 implementation stop reached. Run pentest for this exact commit.`

### v0.38.0 - 16-bit bitfield decode

Status: planned.

Goal:

Establish the release-scoped outcome: 16-bit bitfield decode. Keep this capability bounded
and independently reviewable. Center security review on zero/overlap/non-contiguous masks.

Deliverables:

- Implement and document only the release-scoped capability: 16-bit bitfield decode.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for zero/overlap/non-contiguous masks.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  16-bit bitfield decode.
- Exercise adversarial cases covering zero/overlap/non-contiguous masks.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.38.0 implementation stop reached. Run pentest for this exact commit.`

### v0.39.0 - 32-bit bitfields and alpha mask

Status: planned.

Goal:

Establish the release-scoped outcome: 32-bit bitfields and alpha mask. Keep this capability bounded
and independently reviewable. Center security review on full-width masks and shifts.

Deliverables:

- Implement and document only the release-scoped capability: 32-bit bitfields and alpha mask.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for full-width masks and shifts.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  32-bit bitfields and alpha mask.
- Exercise adversarial cases covering full-width masks and shifts.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.39.0 implementation stop reached. Run pentest for this exact commit.`

### v0.40.0 - Complete top-down and palette-alpha policy

Status: planned.

Goal:

Establish the release-scoped outcome: Complete top-down and palette-alpha policy. Keep this capability bounded
and independently reviewable. Center security review on signed minimum and reversed rows.

Deliverables:

- Implement and document only the release-scoped capability: Complete top-down and palette-alpha policy.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for signed minimum and reversed rows.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  complete top-down and palette-alpha policy.
- Exercise adversarial cases covering signed minimum and reversed rows.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.40.0 implementation stop reached. Run pentest for this exact commit.`

### v0.41.0 - RLE8 decode

Status: planned.

Goal:

Establish the release-scoped outcome: RLE8 decode. Keep this capability bounded
and independently reviewable. Center security review on escapes, deltas, rows, expansion.

Deliverables:

- Implement and document only the release-scoped capability: RLE8 decode.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for escapes, deltas, rows, expansion.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  rLE8 decode.
- Exercise adversarial cases covering escapes, deltas, rows, expansion.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.41.0 implementation stop reached. Run pentest for this exact commit.`

### v0.42.0 - RLE4 decode

Status: planned.

Goal:

Establish the release-scoped outcome: RLE4 decode. Keep this capability bounded
and independently reviewable. Center security review on absolute padding, nibbles, odd count.

Deliverables:

- Implement and document only the release-scoped capability: RLE4 decode.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for absolute padding, nibbles, odd count.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  rLE4 decode.
- Exercise adversarial cases covering absolute padding, nibbles, odd count.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.42.0 implementation stop reached. Run pentest for this exact commit.`

### v0.43.0 - Bounded profiles/metadata; embedded payload reporting

Status: planned.

Goal:

Establish the release-scoped outcome: Bounded profiles/metadata; embedded payload reporting. Keep this capability bounded
and independently reviewable. Center security review on nested offsets and overlap.

Deliverables:

- Implement and document only the release-scoped capability: Bounded profiles/metadata; embedded payload reporting.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for nested offsets and overlap.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  bounded profiles/metadata; embedded payload reporting.
- Exercise adversarial cases covering nested offsets and overlap.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.43.0 implementation stop reached. Run pentest for this exact commit.`

### v0.44.0 - Canonical uncompressed true-color/indexed encoder

Status: planned.

Goal:

Establish the release-scoped outcome: Canonical uncompressed true-color/indexed encoder. Keep this capability bounded
and independently reviewable. Center security review on lengths, padding, palette count.

Deliverables:

- Implement and document only the release-scoped capability: Canonical uncompressed true-color/indexed encoder.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for lengths, padding, palette count.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  canonical uncompressed true-color/indexed encoder.
- Exercise adversarial cases covering lengths, padding, palette count.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.44.0 implementation stop reached. Run pentest for this exact commit.`

### v0.45.0 - RLE encoder and full BMP conformance/fuzz audit

Status: planned.

Goal:

Establish the release-scoped outcome: RLE encoder and full BMP conformance/fuzz audit. Keep this capability bounded
and independently reviewable. Center security review on symmetry and malformed corpus marathon.

Deliverables:

- Implement and document only the release-scoped capability: RLE encoder and full BMP conformance/fuzz audit.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for symmetry and malformed corpus marathon.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  rLE encoder and full BMP conformance/fuzz audit.
- Exercise adversarial cases covering symmetry and malformed corpus marathon.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.45.0 implementation stop reached. Run pentest for this exact commit.`

## Phase D: TGA

### v0.46.0 - Probe and 18-byte header

Status: planned.

Goal:

Establish the release-scoped outcome: Probe and 18-byte header. Keep this capability bounded
and independently reviewable. Center security review on weak signatures, type, ID length.

Deliverables:

- Implement and document only the release-scoped capability: Probe and 18-byte header.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for weak signatures, type, ID length.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  probe and 18-byte header.
- Exercise adversarial cases covering weak signatures, type, ID length.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.46.0 implementation stop reached. Run pentest for this exact commit.`

### v0.47.0 - 24-bit true-color decode

Status: planned.

Goal:

Establish the release-scoped outcome: 24-bit true-color decode. Keep this capability bounded
and independently reviewable. Center security review on bGR, orientation, row truncation.

Deliverables:

- Implement and document only the release-scoped capability: 24-bit true-color decode.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for bGR, orientation, row truncation.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  24-bit true-color decode.
- Exercise adversarial cases covering bGR, orientation, row truncation.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.47.0 implementation stop reached. Run pentest for this exact commit.`

### v0.48.0 - 32-bit true-color and alpha

Status: planned.

Goal:

Establish the release-scoped outcome: 32-bit true-color and alpha. Keep this capability bounded
and independently reviewable. Center security review on attribute bits and output mismatch.

Deliverables:

- Implement and document only the release-scoped capability: 32-bit true-color and alpha.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for attribute bits and output mismatch.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  32-bit true-color and alpha.
- Exercise adversarial cases covering attribute bits and output mismatch.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.48.0 implementation stop reached. Run pentest for this exact commit.`

### v0.49.0 - 15/16-bit true-color

Status: planned.

Goal:

Establish the release-scoped outcome: 15/16-bit true-color. Keep this capability bounded
and independently reviewable. Center security review on expansion, attribute, endian.

Deliverables:

- Implement and document only the release-scoped capability: 15/16-bit true-color.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for expansion, attribute, endian.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  15/16-bit true-color.
- Exercise adversarial cases covering expansion, attribute, endian.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.49.0 implementation stop reached. Run pentest for this exact commit.`

### v0.50.0 - 8/16-bit grayscale

Status: planned.

Goal:

Establish the release-scoped outcome: 8/16-bit grayscale. Keep this capability bounded
and independently reviewable. Center security review on depth combinations and luma-alpha.

Deliverables:

- Implement and document only the release-scoped capability: 8/16-bit grayscale.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for depth combinations and luma-alpha.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  8/16-bit grayscale.
- Exercise adversarial cases covering depth combinations and luma-alpha.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.50.0 implementation stop reached. Run pentest for this exact commit.`

### v0.51.0 - Color-mapped decode

Status: planned.

Goal:

Establish the release-scoped outcome: Color-mapped decode. Keep this capability bounded
and independently reviewable. Center security review on map origin, entry width, indices.

Deliverables:

- Implement and document only the release-scoped capability: Color-mapped decode.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for map origin, entry width, indices.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  color-mapped decode.
- Exercise adversarial cases covering map origin, entry width, indices.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.51.0 implementation stop reached. Run pentest for this exact commit.`

### v0.52.0 - Origins and supported interleaving

Status: planned.

Goal:

Establish the release-scoped outcome: Origins and supported interleaving. Keep this capability bounded
and independently reviewable. Center security review on coordinate mapping and duplicate rows.

Deliverables:

- Implement and document only the release-scoped capability: Origins and supported interleaving.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for coordinate mapping and duplicate rows.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  origins and supported interleaving.
- Exercise adversarial cases covering coordinate mapping and duplicate rows.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.52.0 implementation stop reached. Run pentest for this exact commit.`

### v0.53.0 - Generic bounded RLE packet parser

Status: planned.

Goal:

Establish the release-scoped outcome: Generic bounded RLE packet parser. Keep this capability bounded
and independently reviewable. Center security review on zero progress and packet arithmetic.

Deliverables:

- Implement and document only the release-scoped capability: Generic bounded RLE packet parser.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for zero progress and packet arithmetic.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  generic bounded RLE packet parser.
- Exercise adversarial cases covering zero progress and packet arithmetic.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.53.0 implementation stop reached. Run pentest for this exact commit.`

### v0.54.0 - True-color RLE decode

Status: planned.

Goal:

Establish the release-scoped outcome: True-color RLE decode. Keep this capability bounded
and independently reviewable. Center security review on image/row bounds and truncation.

Deliverables:

- Implement and document only the release-scoped capability: True-color RLE decode.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for image/row bounds and truncation.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  true-color RLE decode.
- Exercise adversarial cases covering image/row bounds and truncation.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.54.0 implementation stop reached. Run pentest for this exact commit.`

### v0.55.0 - Grayscale/indexed RLE decode

Status: planned.

Goal:

Establish the release-scoped outcome: Grayscale/indexed RLE decode. Keep this capability bounded
and independently reviewable. Center security review on lookup and raw/run packet bounds.

Deliverables:

- Implement and document only the release-scoped capability: Grayscale/indexed RLE decode.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for lookup and raw/run packet bounds.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  grayscale/indexed RLE decode.
- Exercise adversarial cases covering lookup and raw/run packet bounds.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.55.0 implementation stop reached. Run pentest for this exact commit.`

### v0.56.0 - Bounded image-ID handling

Status: planned.

Goal:

Establish the release-scoped outcome: Bounded image-ID handling. Keep this capability bounded
and independently reviewable. Center security review on non-text logging and preservation.

Deliverables:

- Implement and document only the release-scoped capability: Bounded image-ID handling.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for non-text logging and preservation.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  bounded image-ID handling.
- Exercise adversarial cases covering non-text logging and preservation.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.56.0 implementation stop reached. Run pentest for this exact commit.`

### v0.57.0 - TGA 2.0 footer and extension area

Status: planned.

Goal:

Establish the release-scoped outcome: TGA 2.0 footer and extension area. Keep this capability bounded
and independently reviewable. Center security review on offsets, sizes, contradictory metadata.

Deliverables:

- Implement and document only the release-scoped capability: TGA 2.0 footer and extension area.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for offsets, sizes, contradictory metadata.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  tGA 2.0 footer and extension area.
- Exercise adversarial cases covering offsets, sizes, contradictory metadata.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.57.0 implementation stop reached. Run pentest for this exact commit.`

### v0.58.0 - Developer area, correction table, thumbnail

Status: planned.

Goal:

Establish the release-scoped outcome: Developer area, correction table, thumbnail. Keep this capability bounded
and independently reviewable. Center security review on counts, nested offsets, overlap.

Deliverables:

- Implement and document only the release-scoped capability: Developer area, correction table, thumbnail.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for counts, nested offsets, overlap.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  developer area, correction table, thumbnail.
- Exercise adversarial cases covering counts, nested offsets, overlap.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.58.0 implementation stop reached. Run pentest for this exact commit.`

### v0.59.0 - Canonical raw/RLE encoders

Status: planned.

Goal:

Establish the release-scoped outcome: Canonical raw/RLE encoders. Keep this capability bounded
and independently reviewable. Center security review on packets, origins, deterministic output.

Deliverables:

- Implement and document only the release-scoped capability: Canonical raw/RLE encoders.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for packets, origins, deterministic output.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  canonical raw/RLE encoders.
- Exercise adversarial cases covering packets, origins, deterministic output.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.59.0 implementation stop reached. Run pentest for this exact commit.`

### v0.60.0 - Legacy compatibility and complete TGA audit

Status: planned.

Goal:

Establish the release-scoped outcome: Legacy compatibility and complete TGA audit. Keep this capability bounded
and independently reviewable. Center security review on cross-scanline RLE and differential corpus.

Deliverables:

- Implement and document only the release-scoped capability: Legacy compatibility and complete TGA audit.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for cross-scanline RLE and differential corpus.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  legacy compatibility and complete TGA audit.
- Exercise adversarial cases covering cross-scanline RLE and differential corpus.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.60.0 implementation stop reached. Run pentest for this exact commit.`

## Phase E: GIF decoding

### v0.61.0 - GIF87a/89a signature and version

Status: planned.

Goal:

Establish the release-scoped outcome: GIF87a/89a signature and version. Keep this capability bounded
and independently reviewable. Center security review on near signatures and truncation.

Deliverables:

- Implement and document only the release-scoped capability: GIF87a/89a signature and version.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for near signatures and truncation.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  gIF87a/89a signature and version.
- Exercise adversarial cases covering near signatures and truncation.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.61.0 implementation stop reached. Run pentest for this exact commit.`

### v0.62.0 - Logical screen descriptor

Status: planned.

Goal:

Establish the release-scoped outcome: Logical screen descriptor. Keep this capability bounded
and independently reviewable. Center security review on canvas area and packed/reserved bits.

Deliverables:

- Implement and document only the release-scoped capability: Logical screen descriptor.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for canvas area and packed/reserved bits.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  logical screen descriptor.
- Exercise adversarial cases covering canvas area and packed/reserved bits.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.62.0 implementation stop reached. Run pentest for this exact commit.`

### v0.63.0 - Global color table

Status: planned.

Goal:

Establish the release-scoped outcome: Global color table. Keep this capability bounded
and independently reviewable. Center security review on table length, truncation, no-allocation path.

Deliverables:

- Implement and document only the release-scoped capability: Global color table.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for table length, truncation, no-allocation path.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  global color table.
- Exercise adversarial cases covering table length, truncation, no-allocation path.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.63.0 implementation stop reached. Run pentest for this exact commit.`

### v0.64.0 - Incremental sub-block reader

Status: planned.

Goal:

Establish the release-scoped outcome: Incremental sub-block reader. Keep this capability bounded
and independently reviewable. Center security review on missing terminator, total bytes, chunking.

Deliverables:

- Implement and document only the release-scoped capability: Incremental sub-block reader.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for missing terminator, total bytes, chunking.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  incremental sub-block reader.
- Exercise adversarial cases covering missing terminator, total bytes, chunking.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.64.0 implementation stop reached. Run pentest for this exact commit.`

### v0.65.0 - Image descriptor and local table

Status: planned.

Goal:

Establish the release-scoped outcome: Image descriptor and local table. Keep this capability bounded
and independently reviewable. Center security review on rectangle overflow and palette selection.

Deliverables:

- Implement and document only the release-scoped capability: Image descriptor and local table.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for rectangle overflow and palette selection.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  image descriptor and local table.
- Exercise adversarial cases covering rectangle overflow and palette selection.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.65.0 implementation stop reached. Run pentest for this exact commit.`

### v0.66.0 - LZW code extraction and LSB integration

Status: planned.

Goal:

Establish the release-scoped outcome: LZW code extraction and LSB integration. Keep this capability bounded
and independently reviewable. Center security review on code size and bit transitions.

Deliverables:

- Implement and document only the release-scoped capability: LZW code extraction and LSB integration.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for code size and bit transitions.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  lZW code extraction and LSB integration.
- Exercise adversarial cases covering code size and bit transitions.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.66.0 implementation stop reached. Run pentest for this exact commit.`

### v0.67.0 - Fixed LZW dictionary init/reset

Status: planned.

Goal:

Establish the release-scoped outcome: Fixed LZW dictionary init/reset. Keep this capability bounded
and independently reviewable. Center security review on index bounds, clear behavior, stale state.

Deliverables:

- Implement and document only the release-scoped capability: Fixed LZW dictionary init/reset.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for index bounds, clear behavior, stale state.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  fixed LZW dictionary init/reset.
- Exercise adversarial cases covering index bounds, clear behavior, stale state.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.67.0 implementation stop reached. Run pentest for this exact commit.`

### v0.68.0 - Code-width growth and saturation

Status: planned.

Goal:

Establish the release-scoped outcome: Code-width growth and saturation. Keep this capability bounded
and independently reviewable. Center security review on exact transition and 12-bit maximum.

Deliverables:

- Implement and document only the release-scoped capability: Code-width growth and saturation.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for exact transition and 12-bit maximum.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  code-width growth and saturation.
- Exercise adversarial cases covering exact transition and 12-bit maximum.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.68.0 implementation stop reached. Run pentest for this exact commit.`

### v0.69.0 - Undefined-code special case and rejection

Status: planned.

Goal:

Establish the release-scoped outcome: Undefined-code special case and rejection. Keep this capability bounded
and independently reviewable. Center security review on previous state and forward references.

Deliverables:

- Implement and document only the release-scoped capability: Undefined-code special case and rejection.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for previous state and forward references.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  undefined-code special case and rejection.
- Exercise adversarial cases covering previous state and forward references.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.69.0 implementation stop reached. Run pentest for this exact commit.`

### v0.70.0 - Pixel accounting and exact frame size

Status: planned.

Goal:

Establish the release-scoped outcome: Pixel accounting and exact frame size. Keep this capability bounded
and independently reviewable. Center security review on expansion bombs and no-progress loops.

Deliverables:

- Implement and document only the release-scoped capability: Pixel accounting and exact frame size.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for expansion bombs and no-progress loops.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  pixel accounting and exact frame size.
- Exercise adversarial cases covering expansion bombs and no-progress loops.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.70.0 implementation stop reached. Run pentest for this exact commit.`

### v0.71.0 - Four-pass deinterlacing

Status: planned.

Goal:

Establish the release-scoped outcome: Four-pass deinterlacing. Keep this capability bounded
and independently reviewable. Center security review on tiny heights, duplicate/destination rows.

Deliverables:

- Implement and document only the release-scoped capability: Four-pass deinterlacing.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for tiny heights, duplicate/destination rows.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  four-pass deinterlacing.
- Exercise adversarial cases covering tiny heights, duplicate/destination rows.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.71.0 implementation stop reached. Run pentest for this exact commit.`

### v0.72.0 - Complete single-image decode

Status: planned.

Goal:

Establish the release-scoped outcome: Complete single-image decode. Keep this capability bounded
and independently reviewable. Center security review on palette indices and trailing data.

Deliverables:

- Implement and document only the release-scoped capability: Complete single-image decode.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for palette indices and trailing data.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  complete single-image decode.
- Exercise adversarial cases covering palette indices and trailing data.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.72.0 implementation stop reached. Run pentest for this exact commit.`

### v0.73.0 - Graphic Control Extension

Status: planned.

Goal:

Establish the release-scoped outcome: Graphic Control Extension. Keep this capability bounded
and independently reviewable. Center security review on block size, reserved bits, terminator.

Deliverables:

- Implement and document only the release-scoped capability: Graphic Control Extension.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for block size, reserved bits, terminator.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  graphic Control Extension.
- Exercise adversarial cases covering block size, reserved bits, terminator.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.73.0 implementation stop reached. Run pentest for this exact commit.`

### v0.74.0 - Transparency index

Status: planned.

Goal:

Establish the release-scoped outcome: Transparency index. Keep this capability bounded
and independently reviewable. Center security review on palette bounds and alpha conversion.

Deliverables:

- Implement and document only the release-scoped capability: Transparency index.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for palette bounds and alpha conversion.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  transparency index.
- Exercise adversarial cases covering palette bounds and alpha conversion.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.74.0 implementation stop reached. Run pentest for this exact commit.`

### v0.75.0 - Frame clipping and canvas placement

Status: planned.

Goal:

Establish the release-scoped outcome: Frame clipping and canvas placement. Keep this capability bounded
and independently reviewable. Center security review on rectangle addition and partial frames.

Deliverables:

- Implement and document only the release-scoped capability: Frame clipping and canvas placement.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for rectangle addition and partial frames.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  frame clipping and canvas placement.
- Exercise adversarial cases covering rectangle addition and partial frames.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.75.0 implementation stop reached. Run pentest for this exact commit.`

### v0.76.0 - Delay representation and timing policy

Status: planned.

Goal:

Establish the release-scoped outcome: Delay representation and timing policy. Keep this capability bounded
and independently reviewable. Center security review on zero and cumulative duration.

Deliverables:

- Implement and document only the release-scoped capability: Delay representation and timing policy.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for zero and cumulative duration.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  delay representation and timing policy.
- Exercise adversarial cases covering zero and cumulative duration.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.76.0 implementation stop reached. Run pentest for this exact commit.`

### v0.77.0 - Keep/do-not-dispose behavior

Status: planned.

Goal:

Establish the release-scoped outcome: Keep/do-not-dispose behavior. Keep this capability bounded
and independently reviewable. Center security review on retained canvas and ordering.

Deliverables:

- Implement and document only the release-scoped capability: Keep/do-not-dispose behavior.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for retained canvas and ordering.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  keep/do-not-dispose behavior.
- Exercise adversarial cases covering retained canvas and ordering.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.77.0 implementation stop reached. Run pentest for this exact commit.`

### v0.78.0 - Restore-to-background

Status: planned.

Goal:

Establish the release-scoped outcome: Restore-to-background. Keep this capability bounded
and independently reviewable. Center security review on background selection and affected region.

Deliverables:

- Implement and document only the release-scoped capability: Restore-to-background.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for background selection and affected region.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  restore-to-background.
- Exercise adversarial cases covering background selection and affected region.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.78.0 implementation stop reached. Run pentest for this exact commit.`

### v0.79.0 - Restore-to-previous with bounded snapshots

Status: planned.

Goal:

Establish the release-scoped outcome: Restore-to-previous with bounded snapshots. Keep this capability bounded
and independently reviewable. Center security review on memory totals and snapshot lifetime.

Deliverables:

- Implement and document only the release-scoped capability: Restore-to-previous with bounded snapshots.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for memory totals and snapshot lifetime.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  restore-to-previous with bounded snapshots.
- Exercise adversarial cases covering memory totals and snapshot lifetime.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.79.0 implementation stop reached. Run pentest for this exact commit.`

### v0.80.0 - Raw-frame API

Status: planned.

Goal:

Establish the release-scoped outcome: Raw-frame API. Keep this capability bounded
and independently reviewable. Center security review on local versus canvas coordinates.

Deliverables:

- Implement and document only the release-scoped capability: Raw-frame API.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for local versus canvas coordinates.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  raw-frame API.
- Exercise adversarial cases covering local versus canvas coordinates.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.80.0 implementation stop reached. Run pentest for this exact commit.`

### v0.81.0 - Composited-frame API

Status: planned.

Goal:

Establish the release-scoped outcome: Composited-frame API. Keep this capability bounded
and independently reviewable. Center security review on disposal sequencing and reuse.

Deliverables:

- Implement and document only the release-scoped capability: Composited-frame API.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for disposal sequencing and reuse.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  composited-frame API.
- Exercise adversarial cases covering disposal sequencing and reuse.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.81.0 implementation stop reached. Run pentest for this exact commit.`

### v0.82.0 - Generic bounded extension state machine

Status: planned.

Goal:

Establish the release-scoped outcome: Generic bounded extension state machine. Keep this capability bounded
and independently reviewable. Center security review on unknown labels and block bombs.

Deliverables:

- Implement and document only the release-scoped capability: Generic bounded extension state machine.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for unknown labels and block bombs.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  generic bounded extension state machine.
- Exercise adversarial cases covering unknown labels and block bombs.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.82.0 implementation stop reached. Run pentest for this exact commit.`

### v0.83.0 - Loops, comments, plain text, unknown extensions

Status: planned.

Goal:

Establish the release-scoped outcome: Loops, comments, plain text, unknown extensions. Keep this capability bounded
and independently reviewable. Center security review on ordering and metadata limits.

Deliverables:

- Implement and document only the release-scoped capability: Loops, comments, plain text, unknown extensions.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for ordering and metadata limits.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  loops, comments, plain text, unknown extensions.
- Exercise adversarial cases covering ordering and metadata limits.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.83.0 implementation stop reached. Run pentest for this exact commit.`

### v0.84.0 - Full GIF decoder conformance/fuzz/differential audit

Status: planned.

Goal:

Establish the release-scoped outcome: Full GIF decoder conformance/fuzz/differential audit. Keep this capability bounded
and independently reviewable. Center security review on lZW state space and animation bombs.

Deliverables:

- Implement and document only the release-scoped capability: Full GIF decoder conformance/fuzz/differential audit.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for lZW state space and animation bombs.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  full GIF decoder conformance/fuzz/differential audit.
- Exercise adversarial cases covering lZW state space and animation bombs.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.84.0 implementation stop reached. Run pentest for this exact commit.`

## Phase F: quantization, encoding, facade, processing, and CLI

### v0.85.0 - Exact palettes and bounded histogram

Status: planned.

Goal:

Establish the release-scoped outcome: Exact palettes and bounded histogram. Keep this capability bounded
and independently reviewable. Center security review on indexing, unique colors, determinism.

Deliverables:

- Implement and document only the release-scoped capability: Exact palettes and bounded histogram.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for indexing, unique colors, determinism.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  exact palettes and bounded histogram.
- Exercise adversarial cases covering indexing, unique colors, determinism.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.85.0 implementation stop reached. Run pentest for this exact commit.`

### v0.86.0 - Deterministic median cut

Status: planned.

Goal:

Establish the release-scoped outcome: Deterministic median cut. Keep this capability bounded
and independently reviewable. Center security review on empty boxes and split arithmetic.

Deliverables:

- Implement and document only the release-scoped capability: Deterministic median cut.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for empty boxes and split arithmetic.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  deterministic median cut.
- Exercise adversarial cases covering empty boxes and split arithmetic.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.86.0 implementation stop reached. Run pentest for this exact commit.`

### v0.87.0 - Palette remap and ordered dither

Status: planned.

Goal:

Establish the release-scoped outcome: Palette remap and ordered dither. Keep this capability bounded
and independently reviewable. Center security review on distance overflow and work accounting.

Deliverables:

- Implement and document only the release-scoped capability: Palette remap and ordered dither.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for distance overflow and work accounting.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  palette remap and ordered dither.
- Exercise adversarial cases covering distance overflow and work accounting.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.87.0 implementation stop reached. Run pentest for this exact commit.`

### v0.88.0 - GIF LZW encoder

Status: planned.

Goal:

Establish the release-scoped outcome: GIF LZW encoder. Keep this capability bounded
and independently reviewable. Center security review on width transitions and dictionary reset.

Deliverables:

- Implement and document only the release-scoped capability: GIF LZW encoder.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for width transitions and dictionary reset.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  gIF LZW encoder.
- Exercise adversarial cases covering width transitions and dictionary reset.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.88.0 implementation stop reached. Run pentest for this exact commit.`

### v0.89.0 - Single-frame GIF encoder

Status: planned.

Goal:

Establish the release-scoped outcome: Single-frame GIF encoder. Keep this capability bounded
and independently reviewable. Center security review on table sizes, transparency, sub-blocks.

Deliverables:

- Implement and document only the release-scoped capability: Single-frame GIF encoder.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for table sizes, transparency, sub-blocks.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  single-frame GIF encoder.
- Exercise adversarial cases covering table sizes, transparency, sub-blocks.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.89.0 implementation stop reached. Run pentest for this exact commit.`

### v0.90.0 - Animated GIF encoder

Status: planned.

Goal:

Establish the release-scoped outcome: Animated GIF encoder. Keep this capability bounded
and independently reviewable. Center security review on canvas/frame totals and disposal round trip.

Deliverables:

- Implement and document only the release-scoped capability: Animated GIF encoder.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for canvas/frame totals and disposal round trip.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  animated GIF encoder.
- Exercise adversarial cases covering canvas/frame totals and disposal round trip.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.90.0 implementation stop reached. Run pentest for this exact commit.`

### v0.91.0 - Feature-gated registry and bounded probing

Status: planned.

Goal:

Establish the release-scoped outcome: Feature-gated registry and bounded probing. Keep this capability bounded
and independently reviewable. Center security review on ambiguity, disabled formats, ordering.

Deliverables:

- Implement and document only the release-scoped capability: Feature-gated registry and bounded probing.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for ambiguity, disabled formats, ordering.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  feature-gated registry and bounded probing.
- Exercise adversarial cases covering ambiguity, disabled formats, ordering.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.91.0 implementation stop reached. Run pentest for this exact commit.`

### v0.92.0 - Static `AnyDecoder` dispatch

Status: planned.

Goal:

Establish the release-scoped outcome: Static AnyDecoder dispatch. Keep this capability bounded
and independently reviewable. Center security review on variant/error/state confusion.

Deliverables:

- Implement and document only the release-scoped capability: Static `AnyDecoder` dispatch.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for variant/error/state confusion.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  static AnyDecoder dispatch.
- Exercise adversarial cases covering variant/error/state confusion.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.92.0 implementation stop reached. Run pentest for this exact commit.`

### v0.93.0 - Unified `decode_into` facade

Status: planned.

Goal:

Establish the release-scoped outcome: Unified decode_into facade. Keep this capability bounded
and independently reviewable. Center security review on limit propagation and partial output.

Deliverables:

- Implement and document only the release-scoped capability: Unified `decode_into` facade.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for limit propagation and partial output.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  unified decode_into facade.
- Exercise adversarial cases covering limit propagation and partial output.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.93.0 implementation stop reached. Run pentest for this exact commit.`

### v0.94.0 - Fallible owned decode under `alloc`

Status: planned.

Goal:

Establish the release-scoped outcome: Fallible owned decode under alloc. Keep this capability bounded
and independently reviewable. Center security review on reservation and warning collection.

Deliverables:

- Implement and document only the release-scoped capability: Fallible owned decode under `alloc`.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for reservation and warning collection.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  fallible owned decode under alloc.
- Exercise adversarial cases covering reservation and warning collection.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.94.0 implementation stop reached. Run pentest for this exact commit.`

### v0.95.0 - Unified encoder/writer facade

Status: planned.

Goal:

Establish the release-scoped outcome: Unified encoder/writer facade. Keep this capability bounded
and independently reviewable. Center security review on selection and partial output.

Deliverables:

- Implement and document only the release-scoped capability: Unified encoder/writer facade.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for selection and partial output.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  unified encoder/writer facade.
- Exercise adversarial cases covering selection and partial output.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.95.0 implementation stop reached. Run pentest for this exact commit.`

### v0.96.0 - Basic color and alpha conversion

Status: planned.

Goal:

Establish the release-scoped outcome: Basic color and alpha conversion. Keep this capability bounded
and independently reviewable. Center security review on rounding, division, premultiplication.

Deliverables:

- Implement and document only the release-scoped capability: Basic color and alpha conversion.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for rounding, division, premultiplication.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  basic color and alpha conversion.
- Exercise adversarial cases covering rounding, division, premultiplication.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.96.0 implementation stop reached. Run pentest for this exact commit.`

### v0.97.0 - Crop, flip, rotate, normalize, nearest resize

Status: planned.

Goal:

Establish the release-scoped outcome: Crop, flip, rotate, normalize, nearest resize. Keep this capability bounded
and independently reviewable. Center security review on overlap, rectangles, work budget.

Deliverables:

- Implement and document only the release-scoped capability: Crop, flip, rotate, normalize, nearest resize.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for overlap, rectangles, work budget.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  crop, flip, rotate, normalize, nearest resize.
- Exercise adversarial cases covering overlap, rectangles, work budget.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.97.0 implementation stop reached. Run pentest for this exact commit.`

### v0.98.0 - CLI inspect/validate/decode/convert/frame/limits

Status: planned.

Goal:

Establish the release-scoped outcome: CLI inspect/validate/decode/convert/frame/limits. Keep this capability bounded
and independently reviewable. Center security review on paths, defaults, terminal injection.

Deliverables:

- Implement and document only the release-scoped capability: CLI inspect/validate/decode/convert/frame/limits.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for paths, defaults, terminal injection.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  cLI inspect/validate/decode/convert/frame/limits.
- Exercise adversarial cases covering paths, defaults, terminal injection.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.98.0 implementation stop reached. Run pentest for this exact commit.`

## Phase G: stabilization and 1.0 admission

### v0.99.0 - Audit every crate with no default features

Status: planned.

Goal:

Establish the release-scoped outcome: Audit every crate with no default features. Keep this capability bounded
and independently reviewable. Center security review on accidental std/alloc/macros/panics.

Deliverables:

- Implement and document only the release-scoped capability: Audit every crate with no default features.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for accidental std/alloc/macros/panics.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  audit every crate with no default features.
- Exercise adversarial cases covering accidental std/alloc/macros/panics.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.99.0 implementation stop reached. Run pentest for this exact commit.`

### v0.100.0 - Complete core/alloc/std and encode/decode matrix

Status: planned.

Goal:

Establish the release-scoped outcome: Complete core/alloc/std and encode/decode matrix. Keep this capability bounded
and independently reviewable. Center security review on invalid feature combinations.

Deliverables:

- Implement and document only the release-scoped capability: Complete core/alloc/std and encode/decode matrix.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for invalid feature combinations.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  complete core/alloc/std and encode/decode matrix.
- Exercise adversarial cases covering invalid feature combinations.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.100.0 implementation stop reached. Run pentest for this exact commit.`

### v0.101.0 - Workspace panic/assertion audit

Status: planned.

Goal:

Establish the release-scoped outcome: Workspace panic/assertion audit. Keep this capability bounded
and independently reviewable. Center security review on unwrap, indexing, divide, build-mode differences.

Deliverables:

- Implement and document only the release-scoped capability: Workspace panic/assertion audit.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for unwrap, indexing, divide, build-mode differences.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  workspace panic/assertion audit.
- Exercise adversarial cases covering unwrap, indexing, divide, build-mode differences.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.101.0 implementation stop reached. Run pentest for this exact commit.`

### v0.102.0 - Arithmetic/offset/cast/stride audit

Status: planned.

Goal:

Establish the release-scoped outcome: Arithmetic/offset/cast/stride audit. Keep this capability bounded
and independently reviewable. Center security review on every input-derived calculation.

Deliverables:

- Implement and document only the release-scoped capability: Arithmetic/offset/cast/stride audit.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for every input-derived calculation.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  arithmetic/offset/cast/stride audit.
- Exercise adversarial cases covering every input-derived calculation.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.102.0 implementation stop reached. Run pentest for this exact commit.`

### v0.103.0 - Calibrate limits on normal/hostile corpora

Status: planned.

Goal:

Establish the release-scoped outcome: Calibrate limits on normal/hostile corpora. Keep this capability bounded
and independently reviewable. Center security review on bypass and unusable defaults.

Deliverables:

- Implement and document only the release-scoped capability: Calibrate limits on normal/hostile corpora.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for bypass and unusable defaults.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  calibrate limits on normal/hostile corpora.
- Exercise adversarial cases covering bypass and unusable defaults.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.103.0 implementation stop reached. Run pentest for this exact commit.`

### v0.104.0 - Automated every-byte truncation suite

Status: planned.

Goal:

Establish the release-scoped outcome: Automated every-byte truncation suite. Keep this capability bounded
and independently reviewable. Center security review on stalls, partial state, error consistency.

Deliverables:

- Implement and document only the release-scoped capability: Automated every-byte truncation suite.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for stalls, partial state, error consistency.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  automated every-byte truncation suite.
- Exercise adversarial cases covering stalls, partial state, error consistency.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.104.0 implementation stop reached. Run pentest for this exact commit.`

### v0.105.0 - Corpus provenance, conformance matrix, differential suite

Status: planned.

Goal:

Establish the release-scoped outcome: Corpus provenance, conformance matrix, differential suite. Keep this capability bounded
and independently reviewable. Center security review on unsupported claims and disagreements.

Deliverables:

- Implement and document only the release-scoped capability: Corpus provenance, conformance matrix, differential suite.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for unsupported claims and disagreements.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  corpus provenance, conformance matrix, differential suite.
- Exercise adversarial cases covering unsupported claims and disagreements.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.105.0 implementation stop reached. Run pentest for this exact commit.`

### v0.106.0 - Extended fuzz campaign and coverage review

Status: planned.

Goal:

Establish the release-scoped outcome: Extended fuzz campaign and coverage review. Keep this capability bounded
and independently reviewable. Center security review on unreached states and cycles.

Deliverables:

- Implement and document only the release-scoped capability: Extended fuzz campaign and coverage review.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for unreached states and cycles.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  extended fuzz campaign and coverage review.
- Exercise adversarial cases covering unreached states and cycles.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.106.0 implementation stop reached. Run pentest for this exact commit.`

### v0.107.0 - Kani: math, views, bits, clipping

Status: planned.

Goal:

Establish the release-scoped outcome: Kani: math, views, bits, clipping. Keep this capability bounded
and independently reviewable. Center security review on assumptions and unwind bounds.

Deliverables:

- Implement and document only the release-scoped capability: Kani: math, views, bits, clipping.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for assumptions and unwind bounds.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  kani: math, views, bits, clipping.
- Exercise adversarial cases covering assumptions and unwind bounds.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.107.0 implementation stop reached. Run pentest for this exact commit.`

### v0.108.0 - Kani: BMP/TGA RLE and GIF LZW

Status: planned.

Goal:

Establish the release-scoped outcome: Kani: BMP/TGA RLE and GIF LZW. Keep this capability bounded
and independently reviewable. Center security review on dictionary, packet, output, row invariants.

Deliverables:

- Implement and document only the release-scoped capability: Kani: BMP/TGA RLE and GIF LZW.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for dictionary, packet, output, row invariants.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  kani: BMP/TGA RLE and GIF LZW.
- Exercise adversarial cases covering dictionary, packet, output, row invariants.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.108.0 implementation stop reached. Run pentest for this exact commit.`

### v0.109.0 - Miri, sanitizers, unsafe inventory, memory audit

Status: planned.

Goal:

Establish the release-scoped outcome: Miri, sanitizers, unsafe inventory, memory audit. Keep this capability bounded
and independently reviewable. Center security review on alias, initialization, SIMD boundaries.

Deliverables:

- Implement and document only the release-scoped capability: Miri, sanitizers, unsafe inventory, memory audit.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for alias, initialization, SIMD boundaries.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  miri, sanitizers, unsafe inventory, memory audit.
- Exercise adversarial cases covering alias, initialization, SIMD boundaries.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.109.0 implementation stop reached. Run pentest for this exact commit.`

### v0.110.0 - DoS/performance, reproducibility, API docs, RC1

Status: planned.

Goal:

Establish the release-scoped outcome: DoS/performance, reproducibility, API docs, RC1. Keep this capability bounded
and independently reviewable. Center security review on release arithmetic and package contents.

Deliverables:

- Implement and document only the release-scoped capability: DoS/performance, reproducibility, API docs, RC1.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for release arithmetic and package contents.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  doS/performance, reproducibility, API docs, RC1.
- Exercise adversarial cases covering release arithmetic and package contents.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.110.0 implementation stop reached. Run pentest for this exact commit.`

### v0.111.0 - External delta pentest, RC2, API freeze decision

Status: planned.

Goal:

Establish the release-scoped outcome: External delta pentest, RC2, API freeze decision. Keep this capability bounded
and independently reviewable. Center security review on entire attack surface and open findings.

Deliverables:

- Implement and document only the release-scoped capability: External delta pentest, RC2, API freeze decision.
- Define or update its public API, errors, limits, feature behavior, specification
  mapping, and support status where applicable.
- Keep later roadmap behavior unavailable or explicitly fail closed.
- Add focused test fixtures and regression cases for entire attack surface and open findings.
- Update changelog, release notes, crate-version metadata, SBOM, and the matching
  pentest-report scaffold.

Verification:

- Run focused positive, boundary, malformed, and regression tests for
  external delta pentest, RC2, API freeze decision.
- Exercise adversarial cases covering entire attack surface and open findings.
- Run every applicable truncation, mutation, round-trip, differential, fuzz,
  Kani, Miri, sanitizer, conformance, performance, or package test introduced by
  this milestone.
- Run `scripts/checks.sh`, `cargo deny check`, `cargo audit`, and
  `scripts/generate-sbom.sh --check`.
- Run the applicable supported-Rust, feature, and platform-target checks.

Exit criteria:

- The stated capability and its release-specific evidence are complete without
  silently including work assigned to a later version.
- Public documentation, support claims, specification mapping, limits, and known
  limitations match the tested implementation.
- The implementation commit is frozen for pentest; any finding-driven change
  requires clean retesting and a new exact commit review.
- `v0.111.0 implementation stop reached. Run pentest for this exact commit.`

## v1.0.0-rc.1 - Exact Production Candidate

Status: planned release candidate.

Goal:

Create the exact `1.0.0`-versioned candidate once, then test, pentest, and
preserve that commit and its package archives unchanged for stable promotion.

Deliverables:

- Apply only the reviewed version-promotion and release-metadata changes
  authorized by v0.111.0.
- Set the facade and deliberately stabilized crates to their approved versions
  while preserving independent support-crate versions.
- Regenerate lockfiles, crate matrix, SBOM, checksums, provenance, package
  archives, release notes, and pentest paths.
- Preserve the exact approved `.crate` archives and checksums; do not publish
  a stable release yet.
- Repeat as `v1.0.0-rc.N` from a newly reviewed commit if anything changes.

Verification:

- Run the complete release, Rust, feature, platform, conformance, fuzz, proof,
  Miri, sanitizer, hostile-input, documentation, and package gates.
- Pentest and clean-retest the exact promoted commit.
- Verify green CI and CodeQL for that commit.
- Prove semantic package differences from v0.111.0 are limited to approved
  version and release metadata.
- Reproduce and verify the preserved package checksums offline.

Exit criteria:

- The exact commit and package archives are approved for stable promotion with
  no source, manifest, lockfile, documentation, SBOM, or packaging changes.
- No critical/high finding or unexplained conformance gap remains for claimed
  support.
- `v1.0.0-rc.1 implementation stop reached. Run pentest for this exact commit.`

## v1.0.0 - First Production-Ready mynd Release

Status: planned.

Goal:

Publish the first serious production-ready `mynd` crate only from an unchanged,
approved `v1.0.0-rc.N` commit and its preserved package archives.

Deliverables:

- Stable core and no-allocation decoding APIs.
- Honest, documented BMP, TGA, and GIF support matrices for every claimed
  decode, encode, metadata, compatibility, and animation capability.
- Resource limits on every accepted input path and no unsafe Rust in default
  first-party implementations.
- Stable MSRV, feature, platform, migration, specification, conformance, and
  security policies.
- Signed release manifest, checksums, SBOM, provenance, audit references, and
  independently versioned support-crate compatibility matrix.
- Exact promotion of the already approved candidate archives; no repackaging.

Verification:

- Verify every gate assigned through v0.111.0 and the selected rc.N still passes.
- Confirm the stable tag resolves to the exact approved candidate commit.
- Verify crates.io checksums against the preserved candidate archive manifest.
- Confirm all support claims map to specifications, tests, fuzzing, proofs, and
  conformance evidence with no unexplained skip.
- Confirm independent security-review and clean-retest evidence for the exact
  candidate commit.

Exit criteria:

- No unresolved critical/high finding and no unsupported behavior represented as
  stable.
- Every public capability is implemented, bounded, tested, documented, and
  accurately represented in the support matrix.
- The final tag and published archives are byte-for-byte the approved candidate.
- `v1.0.0 implementation stop reached. Run pentest for this exact commit.`

## Post-1.0 order

New codecs start independently and become facade defaults only after maturity:
PNM, QOI, PNG/APNG, classic JPEG, TIFF/Exif, WebP, JPEG-LS, JPEG XL, AVIF/HEIF,
then JPEG 2000 and specialist families. Each receives the same milestone format
and pentest-before-tag discipline. See `docs/POST_1_0_CODEC_PLAN.md`.
