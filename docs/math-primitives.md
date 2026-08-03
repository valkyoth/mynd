# Checked Arithmetic Contract

Status: v0.3.0 released contract.

`mynd-math` is the only implementation owner for the checked integer
primitives introduced by v0.3.0. It is `no_std`, dependency-free,
allocation-free, safe Rust, deterministic, and constant-work. The crate does
not define image dimensions, rectangles, strides, planes, budgets, buffers, or
parser state; those remain later handoffs.

## Failure model

Every operation returns `MathResult<T>`. Failure is classified without storing
attacker-controlled operands:

| Error | Meaning |
| --- | --- |
| `ConversionOutOfRange` | The source integer cannot be represented exactly by the destination type. |
| `Overflow { operation }` | Addition, multiplication, or upward alignment has no representable result. |
| `ZeroAlignment` | The requested alignment multiple is zero. |
| `RangeOutOfBounds` | A representable half-open range ends above its exclusive bound. |

No operation wraps, saturates, truncates, clamps, allocates, logs, or returns a
partial value. Formatting an error is allocation-free and contains no input
data.

## Primitive invariants

### Conversion

The explicit signed, `u32`, `u64`, and `usize` conversion functions succeed if
and only if the source value is representable by the destination. The same
source therefore fails closed on a 32-bit target when it would succeed on a
64-bit target. Negative signed inputs never become unsigned values.

### Addition and multiplication

`checked_add_*` and `checked_mul_*` return the exact mathematical result if it
is representable. Otherwise they return `Overflow` with the corresponding
operation. They never rely on Cargo overflow-check or panic-profile settings.

### Alignment

`checked_align_up_*` returns the smallest representable multiple of
`alignment` that is greater than or equal to `value`. Any nonzero alignment is
accepted, including non-powers of two. Zero fails even when `value` is zero.
An already aligned value is unchanged. A required result above the type's
maximum returns alignment overflow.

This is numeric multiple alignment, not a Rust memory-layout claim. A caller
that needs `core::alloc::Layout` alignment must separately enforce its
power-of-two contract.

### Ranges

`checked_range_*` interprets `(start, length, upper_bound)` as the half-open
range `start..start + length`, with `upper_bound` exclusive. It first rejects
addition overflow, then rejects an end above the bound. Empty ranges are valid
at any `start <= upper_bound`, including exactly at the bound. An empty range
whose start is above the bound fails.

## Requirement-to-evidence mapping

| ID | Requirement | Implementation | Tests | Proof |
| --- | --- | --- | --- | --- |
| MATH-CONV-01 | Conversions are exact or fail. | `convert.rs` | extrema and signed-boundary matrices | arbitrary `u64`/`i64` conversion harnesses |
| MATH-ADD-01 | Addition equals `checked_add`. | `ops.rs` | `u64`/`usize` extrema cross-products | arbitrary operand equivalence |
| MATH-MUL-01 | Multiplication equals `checked_mul`. | `ops.rs` | `u64`/`usize` extrema cross-products | complete `u8` operand domains plus maximum-by-`u8` overflow domains |
| MATH-ALIGN-01 | Alignment is minimal, divisible, nondecreasing, and explicit about zero/overflow. | `ops.rs` | exact, zero, non-power-of-two, maximum, and exhaustive reduced-domain cases | complete `u8` operand-domain equivalence; full-width extrema remain test evidence |
| MATH-RANGE-01 | Ranges cannot overflow or exceed their exclusive bound. | `range.rs` | empty, exact-bound, overflow, and exhaustive reduced-domain cases | arbitrary operand equivalence |
| MATH-ERROR-01 | Errors are structured, bounded, and contain no operands. | `error.rs` | variant and formatting tests | exhaustive enum implementation review |
| MATH-PORT-01 | Behavior remains `no_std` and target-width aware. | crate boundary | MSRV, feature, 32-bit, WASM, and platform matrices | `usize` arithmetic harnesses on the verifier target |

Proof harnesses follow the official Kani `#[kani::proof]` and `kani::any`
model and avoid assumptions that could make a property vacuous. Rust integer
semantics are grounded in the official `checked_add`, `checked_mul`,
`checked_next_multiple_of`, and `TryFrom` contracts. Living sources are
rechecked before release:

- <https://doc.rust-lang.org/core/primitive.u64.html>
- <https://doc.rust-lang.org/core/primitive.usize.html>
- <https://doc.rust-lang.org/core/convert/trait.TryFrom.html>
- <https://model-checking.github.io/kani/usage.html>
- <https://model-checking.github.io/kani/reference/attributes.html>

## Residual risk and exclusions

These primitives prove local integer relationships, not the semantic validity
of an image field. Callers must still apply format limits, source bounds,
cross-field rules, output budgets, and lifetime/aliasing rules. v0.3.0 makes no
claim about allocation sizes, slice validity, dimensions, layouts, codecs, or
floating-point behavior.
