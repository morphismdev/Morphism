## `base::num` Module

### Purpose

`base::num` is the numeric foundation for the **showcase crate**.
It defines the core type-level natural-number representation and a curated set of small naturals (`U0..U64`).
Arithmetic operations live in `base::arithmetic` and build on these representations.

This module is intended to be stable and widely reused in the showcase.

### Scope

#### Owns

- **Natural number representation** (binary, LSB-first):
  - `B0`, `B1`, `UTerm`, `UInt<B, T>`
- **Common naturals aliases**:
  - `U0..U64` (as `UInt`/`UTerm` aliases)
- **Numeric predicates**:
  - `IsZeroOp`, `IsNonZeroOp`, `IsOddOp`, `IsEvenOp`

#### Does not contain

- Compiler logic.
- Registry allocation or domain code definitions.
- Application-specific DSLs or container/structure utilities.
- Value-level numeric semantics (`OpOnce`) unless required.

### Contents

- `mod.rs`
  - Re-exports `aliases::*`, `predicates::*`, and the public surface of `nat`.
- `aliases.rs`
  - Convenience aliases `U0..U64` for common type-level naturals.
- `nat.rs`
  - Natural number representation and normalization (`NormalizeNatOp`).
- `predicates.rs`
  - Small, pure numeric predicates used by arithmetic/normalization.

### Reading guide (syntax + notation)

#### Nat syntax (`UTerm` / `UInt<B, T>`; LSB-first)

Nats are a **structural binary encoding**, least-significant bit first:

- `UTerm` is the terminator / canonical 0.
- `UInt<B0, T>` means "prepend a 0 bit to tail `T`".
- `UInt<B1, T>` means "prepend a 1 bit to tail `T`".

Examples (informal):
- `UTerm` = 0
- `UInt<B1, UTerm>` = 1
- `UInt<B0, UInt<B1, UTerm>>` = 2  (binary 10, LSB-first)
- `UInt<B1, UInt<B1, UTerm>>` = 3  (binary 11, LSB-first)

Canonical form rule of thumb: avoid "extra leading zeros" beyond `UTerm`.
When needed, `NormalizeNatOp` strips high-order (MSB) `B0` frames.

#### Reading `OpTy`-style arithmetic

Arithmetic is expressed as `OpTy` morphisms:
- Fix the RHS as a type parameter, apply to LHS as the `OpTy` input.

Example (addition):

```rust
use crate::kit::op::OpTyOut;
use crate::base::Add;

// Interpret as: Out = Lhs + Rhs (at the type level)
type Out<Lhs, Rhs> = OpTyOut<Add<Rhs>, Lhs>;
```

### Invariants

- **Canonical nat zero**: `UTerm` is the canonical terminator/zero.
- **Representation is structural**: `UInt<B, T>` is a pure type-level encoding.
- **Stable public surface**: `U0..U64`, `UInt`, `UTerm`, `B0`, `B1` are foundational.
- **No speculative utilities**: add helpers only when there is a concrete consumer.

### Module-specific rules

- Keep algorithm/implementation details documented in the relevant inner module
  (e.g. normalization in `nat.rs`).
- Favor discoverability while arithmetic APIs are still shifting.
