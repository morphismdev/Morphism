//! Addition operations for type-level numbers.

use core::marker::PhantomData;

use crate::base::num::nat::{UInt, UTerm, B0, B1};
use crate::kit::op::{OpTy, OpTyOut};

// ─────────────────────────────────────────────────────────────────────────────
// Public API
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Add<Rhs>(PhantomData<fn() -> Rhs>);

// ─────────────────────────────────────────────────────────────────────────────
// Implementation details
// ─────────────────────────────────────────────────────────────────────────────

// Polymorphic morphism: AddCarry<Rhs, C> (carry-aware, implementation detail)
//
// Note: This is technically `pub` because it appears in `Add<Rhs>::OutTy`, but it is an
// implementation detail and should not be used directly outside the `base::arithmetic` modules.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AddCarry<Rhs, C>(PhantomData<fn() -> (Rhs, C)>);

impl<Rhs, C> Default for AddCarry<Rhs, C> {
    #[inline]
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<Rhs, C> AddCarry<Rhs, C> {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}


// ─────────────────────────────────────────────────────────────────────────────
// Carry markers (shared infrastructure used by other arithmetic modules)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct C0;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct C1;

// ─────────────────────────────────────────────────────────────────────────────
// Type-level addition for **binary** naturals (UTerm/UInt only).
// ─────────────────────────────────────────────────────────────────────────────

// Type-level helper alias (readability): output of `AddCarry<Rhs, C>` applied to `Lhs`.
type AddCarryOut<Rhs, C, Lhs> = OpTyOut<AddCarry<Rhs, C>, Lhs>;

// AddTy delegates to AddCTy with C0 (no carry-in).
impl<Lhs, Rhs> OpTy<Lhs> for Add<Rhs>
where
    AddCarry<Rhs, C0>: OpTy<Lhs>,
{
    type OutTy = AddCarryOut<Rhs, C0, Lhs>;
}

// ─────────────────────────────────────────────────────────────────────────────
// Carry-aware addition implementation
// ─────────────────────────────────────────────────────────────────────────────

// ----------------------------------------------------------------------------
// Base cases: 0 + 0 (+ carry)
// ----------------------------------------------------------------------------

impl OpTy<UTerm> for AddCarry<UTerm, C0> {
    type OutTy = UTerm;
}

impl OpTy<UTerm> for AddCarry<UTerm, C1> {
    type OutTy = UInt<B1, UTerm>;
}

// ----------------------------------------------------------------------------
// Base cases: 0 + nonzero (+ carry)
// ----------------------------------------------------------------------------

impl<R> OpTy<UTerm> for AddCarry<UInt<B0, R>, C0> {
    type OutTy = UInt<B0, R>;
}

impl<R> OpTy<UTerm> for AddCarry<UInt<B1, R>, C0> {
    type OutTy = UInt<B1, R>;
}

impl<R> OpTy<UTerm> for AddCarry<UInt<B0, R>, C1> {
    type OutTy = UInt<B1, R>;
}

impl<R> OpTy<UTerm> for AddCarry<UInt<B1, R>, C1>
where
    AddCarry<UTerm, C1>: OpTy<R>,
{
    type OutTy = UInt<B0, AddCarryOut<UTerm, C1, R>>;
}

// ----------------------------------------------------------------------------
// Base cases: nonzero + 0 (+ carry)
// ----------------------------------------------------------------------------

impl<T> OpTy<UInt<B0, T>> for AddCarry<UTerm, C0> {
    type OutTy = UInt<B0, T>;
}

impl<T> OpTy<UInt<B1, T>> for AddCarry<UTerm, C0> {
    type OutTy = UInt<B1, T>;
}

impl<T> OpTy<UInt<B0, T>> for AddCarry<UTerm, C1> {
    type OutTy = UInt<B1, T>;
}

impl<T> OpTy<UInt<B1, T>> for AddCarry<UTerm, C1>
where
    Self: OpTy<T>,
{
    type OutTy = UInt<B0, OpTyOut<Self, T>>;
}

// ----------------------------------------------------------------------------
// Recursive step: fold with carry C0
// ----------------------------------------------------------------------------

impl<T, R> OpTy<UInt<B0, T>> for AddCarry<UInt<B0, R>, C0>
where
    AddCarry<R, C0>: OpTy<T>,
{
    type OutTy = UInt<B0, AddCarryOut<R, C0, T>>;
}

impl<T, R> OpTy<UInt<B0, T>> for AddCarry<UInt<B1, R>, C0>
where
    AddCarry<R, C0>: OpTy<T>,
{
    type OutTy = UInt<B1, AddCarryOut<R, C0, T>>;
}

impl<T, R> OpTy<UInt<B1, T>> for AddCarry<UInt<B0, R>, C0>
where
    AddCarry<R, C0>: OpTy<T>,
{
    type OutTy = UInt<B1, AddCarryOut<R, C0, T>>;
}

impl<T, R> OpTy<UInt<B1, T>> for AddCarry<UInt<B1, R>, C0>
where
    AddCarry<R, C1>: OpTy<T>,
{
    type OutTy = UInt<B0, AddCarryOut<R, C1, T>>;
}

// ----------------------------------------------------------------------------
// Recursive step: fold with carry C1
// ----------------------------------------------------------------------------

impl<T, R> OpTy<UInt<B0, T>> for AddCarry<UInt<B0, R>, C1>
where
    AddCarry<R, C0>: OpTy<T>,
{
    // 0 + 0 + 1 = 1, carry 0
    type OutTy = UInt<B1, AddCarryOut<R, C0, T>>;
}

impl<T, R> OpTy<UInt<B0, T>> for AddCarry<UInt<B1, R>, C1>
where
    AddCarry<R, C1>: OpTy<T>,
{
    // 0 + 1 + 1 = 0, carry 1
    type OutTy = UInt<B0, AddCarryOut<R, C1, T>>;
}

impl<T, R> OpTy<UInt<B1, T>> for AddCarry<UInt<B0, R>, C1>
where
    AddCarry<R, C1>: OpTy<T>,
{
    // 1 + 0 + 1 = 0, carry 1
    type OutTy = UInt<B0, AddCarryOut<R, C1, T>>;
}

impl<T, R> OpTy<UInt<B1, T>> for AddCarry<UInt<B1, R>, C1>
where
    AddCarry<R, C1>: OpTy<T>,
{
    // 1 + 1 + 1 = 1, carry 1
    type OutTy = UInt<B1, AddCarryOut<R, C1, T>>;
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::{AddCarry, C1};
    use crate::assert_type_eq;
    use crate::base::num::nat::{UInt, UTerm, B1};
    use crate::base::num::U2;
    use crate::kit::op::OpTyOut;

    #[test]
    fn add_carry_aware_sanity() {
        // AddCarry with carry-in
        assert_type_eq::<OpTyOut<AddCarry<UTerm, C1>, UTerm>, UInt<B1, UTerm>>(); // 0 + 0 + 1 = 1

        // Carry into a 1-bit: 0 + 1 + 1 = 2
        assert_type_eq::<OpTyOut<AddCarry<UInt<B1, UTerm>, C1>, UTerm>, U2>();
    }
}
