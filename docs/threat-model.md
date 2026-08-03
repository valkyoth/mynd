# mynd Threat Model

Status: v0.5.0 pixel-storage-foundation threat model. Each codec adds a focused
threat model before parsing code ships.

## Assets

- memory safety and process availability;
- decoded pixel, frame, color, and metadata correctness;
- caller memory, CPU, stack, and output budgets;
- deterministic, interoperable output;
- release, dependency, corpus, and specification integrity.

## Adversaries and inputs

- remote attackers supplying malformed, truncated, ambiguous, or polyglot files;
- decompression, animation, frame-count, metadata, seek, and CPU bombs;
- inputs targeting integer overflow, parser confusion, stale state, or loops;
- malicious metadata intended for logs, terminals, or downstream interpreters;
- compromised dependencies, CI actions, tools, corpora, or reference decoders.

## Trust boundaries

- bytes to structural parser state;
- declared dimensions/offsets/lengths to arithmetic and buffer ranges;
- compressed codes to bounded output;
- metadata to warnings, preservation, and caller-visible values;
- codec-native samples to color conversion and frame composition;
- caller readers/writers, allocators, warning sinks, and output buffers;
- external specifications, corpora, and reference implementations.

## Required properties

- no panic, abort, hang, recursion growth, or unchecked input-derived arithmetic;
- bounded input, output, metadata, frames, chunks, seeks, allocations, and work;
- no writes outside validated output views and no partial success ambiguity;
- strict distinction between malformed, truncated, unsupported, and limited;
- compatibility repairs are explicit warnings and never weaken safety checks;
- safe scalar code and no hidden I/O or concurrency.

## Residual risk

`no_std`, safe Rust, fuzzing, and proofs reduce different risks but cannot prove
complete format correctness. Specifications contain ambiguities, reference
implementations can disagree, and caller-selected high limits can permit high
resource consumption. Support statements must therefore name exact scope and
evidence rather than promise perfect security.

The v0.5.0 types validate sample storage and pixel-to-plane relationships but
allocate no storage and bind no byte slice. A valid layout proves its declared
sample widths, channel/alpha/chroma organization, logical plane dimensions,
exact used row bytes, and geometry relationship. It does not prove format
conformance, a backing buffer, color or floating arithmetic, chroma siting,
resource-limit admission, or safe typed access. Those boundaries remain later
milestones.
