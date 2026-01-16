//! Subtraction operations for type-level numbers.

use core::marker::PhantomData;

use crate::base::arithmetic::addition::{C0, C1};
use crate::base::boolean::elim::{IfConst, IfConstOut};
use crate::base::num::nat::NormalizeNatOp;
use crate::base::num::nat::{UInt, UTerm, B0, B1};
use crate::base::order::relational::LtOp;
use crate::kit::op::{OpTy, OpTyOut};

// ─────────────────────────────────────────────────────────────────────────────
// Public API
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sub<Rhs>(PhantomData<fn() -> Rhs>);

// ─────────────────────────────────────────────────────────────────────────────
// Implementation details
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct SubBorrow<Rhs, Borrow>(PhantomData<fn() -> (Rhs, Borrow)>);

// ─────────────────────────────────────────────────────────────────────────────
// Type-level subtraction for **binary naturals** (saturating).
// ─────────────────────────────────────────────────────────────────────────────

type SubSatBranch<Rhs, Lhs> = IfConst<LtOp<Rhs>, UTerm, OpTyOut<SubBorrow<Rhs, C0>, Lhs>>;

type SubSatRaw<Rhs, Lhs> = IfConstOut<LtOp<Rhs>, UTerm, OpTyOut<SubBorrow<Rhs, C0>, Lhs>, Lhs>;

// ─────────────────────────────────────────────────────────────────────────────
// Public entrypoint (saturating subtraction)
// ─────────────────────────────────────────────────────────────────────────────

impl<Lhs, Rhs> OpTy<Lhs> for Sub<Rhs>
where
    SubBorrow<Rhs, C0>: OpTy<Lhs>,
    SubSatBranch<Rhs, Lhs>: OpTy<Lhs>,
    NormalizeNatOp: OpTy<SubSatRaw<Rhs, Lhs>>,
{
    type OutTy = OpTyOut<NormalizeNatOp, SubSatRaw<Rhs, Lhs>>;
}

// ─────────────────────────────────────────────────────────────────────────────
// Borrow-aware subtraction core (implementation detail)
// ─────────────────────────────────────────────────────────────────────────────

// Type-level helper alias (readability): output of `SubBorrow<Rhs, Borrow>` applied to `Lhs`.
type SubBorrowOut<Rhs, Borrow, Lhs> = OpTyOut<SubBorrow<Rhs, Borrow>, Lhs>;

// ----------------------------------------------------------------------------
// Base cases: 0 - 0 (- borrow)
// ----------------------------------------------------------------------------

impl OpTy<UTerm> for SubBorrow<UTerm, C0> {
    type OutTy = UTerm;
}

impl OpTy<UTerm> for SubBorrow<UTerm, C1> {
    type OutTy = UTerm; // saturate
}

// ----------------------------------------------------------------------------
// Base cases: 0 - nonzero - borrow → 0 (saturate)
// ----------------------------------------------------------------------------

impl<R> OpTy<UTerm> for SubBorrow<UInt<B0, R>, C0> {
    type OutTy = UTerm;
}

impl<R> OpTy<UTerm> for SubBorrow<UInt<B1, R>, C0> {
    type OutTy = UTerm;
}

impl<R> OpTy<UTerm> for SubBorrow<UInt<B0, R>, C1> {
    type OutTy = UTerm;
}

impl<R> OpTy<UTerm> for SubBorrow<UInt<B1, R>, C1> {
    type OutTy = UTerm;
}

// ----------------------------------------------------------------------------
// Base cases: nonzero - 0 (- borrow)
// ----------------------------------------------------------------------------

impl<T> OpTy<UInt<B0, T>> for SubBorrow<UTerm, C0> {
    type OutTy = UInt<B0, T>;
}

impl<T> OpTy<UInt<B1, T>> for SubBorrow<UTerm, C0> {
    type OutTy = UInt<B1, T>;
}

impl<T> OpTy<UInt<B0, T>> for SubBorrow<UTerm, C1>
where
    Self: OpTy<T>,
{
    // 0 - 1 (borrow) → 1 with new borrow; propagate
    type OutTy = UInt<B1, OpTyOut<Self, T>>;
}

impl<T> OpTy<UInt<B1, T>> for SubBorrow<UTerm, C1>
where
    SubBorrow<UTerm, C0>: OpTy<T>,
{
    // 1 - 1 (borrow) → 0, clear borrow
    type OutTy = UInt<B0, SubBorrowOut<UTerm, C0, T>>;
}

// ----------------------------------------------------------------------------
// Recursive step: fold with borrow = C0
// ----------------------------------------------------------------------------

impl<T, R> OpTy<UInt<B0, T>> for SubBorrow<UInt<B0, R>, C0>
where
    SubBorrow<R, C0>: OpTy<T>,
{
    // 0 - 0 → 0, borrow 0
    type OutTy = UInt<B0, SubBorrowOut<R, C0, T>>;
}

impl<T, R> OpTy<UInt<B0, T>> for SubBorrow<UInt<B1, R>, C0>
where
    SubBorrow<R, C1>: OpTy<T>,
{
    // 0 - 1 → 1, borrow 1
    type OutTy = UInt<B1, SubBorrowOut<R, C1, T>>;
}

impl<T, R> OpTy<UInt<B1, T>> for SubBorrow<UInt<B0, R>, C0>
where
    SubBorrow<R, C0>: OpTy<T>,
{
    // 1 - 0 → 1, borrow 0
    type OutTy = UInt<B1, SubBorrowOut<R, C0, T>>;
}

impl<T, R> OpTy<UInt<B1, T>> for SubBorrow<UInt<B1, R>, C0>
where
    SubBorrow<R, C0>: OpTy<T>,
{
    // 1 - 1 → 0, borrow 0
    type OutTy = UInt<B0, SubBorrowOut<R, C0, T>>;
}

// ----------------------------------------------------------------------------
// Recursive step: fold with borrow = C1
// ----------------------------------------------------------------------------

impl<T, R> OpTy<UInt<B0, T>> for SubBorrow<UInt<B0, R>, C1>
where
    SubBorrow<R, C1>: OpTy<T>,
{
    // 0 - 0 - 1 → 1, borrow 1
    type OutTy = UInt<B1, SubBorrowOut<R, C1, T>>;
}

impl<T, R> OpTy<UInt<B0, T>> for SubBorrow<UInt<B1, R>, C1>
where
    SubBorrow<R, C1>: OpTy<T>,
{
    // 0 - 1 - 1 → 0, borrow 1
    type OutTy = UInt<B0, SubBorrowOut<R, C1, T>>;
}

impl<T, R> OpTy<UInt<B1, T>> for SubBorrow<UInt<B0, R>, C1>
where
    SubBorrow<R, C0>: OpTy<T>,
{
    // 1 - 0 - 1 → 0, borrow 0
    type OutTy = UInt<B0, SubBorrowOut<R, C0, T>>;
}

impl<T, R> OpTy<UInt<B1, T>> for SubBorrow<UInt<B1, R>, C1>
where
    SubBorrow<R, C1>: OpTy<T>,
{
    // 1 - 1 - 1 → 1, borrow 1
    type OutTy = UInt<B1, SubBorrowOut<R, C1, T>>;
}
