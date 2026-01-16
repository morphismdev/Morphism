## `base::arithmetic` Module

### Purpose

`base::arithmetic` provides a minimal set of **type-level arithmetic** operations for the **showcase crate**.

It is intentionally small: it builds on `base::num`'s natural-number syntax (`UTerm` / `UInt<B, T>`) and exposes
core operations used throughout the rest of the DSL.

For the nat syntax and how to read `OpTy`-style arithmetic, see `base/num/README.md` (Reading guide).

### Public surface

- `Add<Rhs>`: type-level addition (apply to `Lhs`).
- `Sub<Rhs>`: type-level subtraction (apply to `Lhs`, **saturating** for naturals).

Usage pattern:

```rust
use crate::kit::op::OpTyOut;
use crate::base::{Add, Sub};

type Sum<Lhs, Rhs> = OpTyOut<Add<Rhs>, Lhs>;
type Diff<Lhs, Rhs> = OpTyOut<Sub<Rhs>, Lhs>;
```

### Scope

#### Owns

- Core arithmetic operations over **binary naturals** (`UTerm` / `UInt`):
  - addition (`Add`)
  - subtraction (`Sub`, saturating)

#### Does not contain

- Numeric representations (those live in `base::num`).
- Comparisons (those live in `base::order`).
- Higher-level numeric algorithms (keep this module small in the showcase).
- Runtime/value-level evaluation (`OpOnce`) unless needed.

### Contents

- `mod.rs`: re-exports arithmetic ops.
- `addition.rs`: `Add<Rhs>` plus carry-aware implementation details.
- `subtraction.rs`: `Sub<Rhs>` plus borrow-aware implementation details.

### Notes on implementation details

Some helper ops/types are `pub` only because they appear in associated output types, but they are intended as
**implementation details**:

- Addition:
  - `AddCarry<Rhs, C>`: carry-aware adder
  - `C0`, `C1`: carry markers (also reused as borrow markers by subtraction)
- Subtraction:
  - `SubBorrow<Rhs, Borrow>`: borrow-aware subtractor core
  - `SubB0Ty`, `SubB1Ty`: internal convenience aliases

### Semantics summary

#### Addition (`Add<Rhs>`)

`Add<Rhs>` delegates to `AddCarry<Rhs, C0>` (no carry-in). `AddCarry` is a recursive full-adder over the LSB-first
binary representation, consuming one bit from each side and threading carry.

#### Subtraction (`Sub<Rhs>`)

`Sub<Rhs>` for naturals is **saturating**:

- if `Lhs < Rhs`, result is `0` (`UTerm`)
- otherwise it computes `SubBorrow<Rhs, C0>(Lhs)` and then normalizes via `NormalizeNatOp`

This uses:
- `base::order::relational::LtOp` for the `<` check
- `base::boolean::elim::IfConst` for branching
- `base::num::nat::NormalizeNatOp` to canonicalize the result

### Invariants

- Type-level only (`OpTy`), zero-sized ops.
- Defined for `base::num` naturals (`UTerm` / `UInt<_, _>`).
- Keep the surface area small and directly useful to the showcase.

