//! Equality comparison operations for type-level numbers.
//!
//! Defines `NumEqOp<Rhs>` for naturals (`UTerm`/`UInt`).

use core::marker::PhantomData;

use crate::base::boolean::{False, True};
use crate::base::num::nat::{UInt, UTerm, B0, B1};
use crate::kit::op::{OpTy, OpTyOut};

// ─────────────────────────────────────────────────────────────────────────────
// Polymorphic morphism: NumEqOp<Rhs>
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct NumEqOp<Rhs>(PhantomData<fn() -> Rhs>);

// ─────────────────────────────────────────────────────────────────────────────
// Natural number equality implementations
// ─────────────────────────────────────────────────────────────────────────────

// 0 == 0
impl OpTy<UTerm> for NumEqOp<UTerm> {
    type OutTy = True;
}

// 0 == non-zero → False
impl<T> OpTy<UTerm> for NumEqOp<UInt<B0, T>> {
    type OutTy = False;
}

impl<T> OpTy<UTerm> for NumEqOp<UInt<B1, T>> {
    type OutTy = False;
}

// non-zero == 0 → False (reciprocal)
impl<T> OpTy<UInt<B0, T>> for NumEqOp<UTerm> {
    type OutTy = False;
}

impl<T> OpTy<UInt<B1, T>> for NumEqOp<UTerm> {
    type OutTy = False;
}

// Structural equality for canonical (no leading zeros) UInt
impl<TA, TB> OpTy<UInt<B0, TA>> for NumEqOp<UInt<B0, TB>>
where
    NumEqOp<TB>: OpTy<TA>,
{
    type OutTy = OpTyOut<NumEqOp<TB>, TA>;
}

impl<TA, TB> OpTy<UInt<B1, TA>> for NumEqOp<UInt<B1, TB>>
where
    NumEqOp<TB>: OpTy<TA>,
{
    type OutTy = OpTyOut<NumEqOp<TB>, TA>;
}

// mismatched LSB → False
impl<TA, TB> OpTy<UInt<B0, TA>> for NumEqOp<UInt<B1, TB>> {
    type OutTy = False;
}

impl<TA, TB> OpTy<UInt<B1, TA>> for NumEqOp<UInt<B0, TB>> {
    type OutTy = False;
}
