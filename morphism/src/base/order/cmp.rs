//! Binary comparison internals for natural numbers.
//!
//! This module provides type-level-only comparison logic for `Le` and `Lt` morphisms.

use crate::base::boolean::IfApply;
use crate::base::boolean::{False, True};
use crate::base::num::nat::{UInt, UTerm, B0, B1};
use crate::base::order::equality::NumEqOp;
use crate::base::order::relational::{LeOp, LtOp};
use crate::kit::combinators::ConstCOp;
use crate::kit::combinators::IdOp;
use crate::kit::combinators::{FstOp, SndOp};
use crate::kit::op::{OpTy, OpTyOut};

// ─────────────────────────────────────────────────────────────────────────────
// Internal type-level-only comparison machinery
// ─────────────────────────────────────────────────────────────────────────────

/// Compares two bits and returns (LeBool, LtBool) as a tuple.
///
/// Input: (B0|B1, B0|B1)
/// Output: (True|False, True|False) representing (Le, Lt)
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct CmpBit;

// (B0, B0) → (True, False)  // 0 <= 0, 0 !< 0
impl OpTy<(B0, B0)> for CmpBit {
    type OutTy = (True, False);
}

// (B0, B1) → (True, True)   // 0 <= 1, 0 < 1
impl OpTy<(B0, B1)> for CmpBit {
    type OutTy = (True, True);
}

// (B1, B0) → (False, False) // 1 !<= 0, 1 !< 0
impl OpTy<(B1, B0)> for CmpBit {
    type OutTy = (False, False);
}

// (B1, B1) → (True, False)  // 1 <= 1, 1 !< 1
impl OpTy<(B1, B1)> for CmpBit {
    type OutTy = (True, False);
}

/// Compares two naturals and returns (LeBool, LtBool) as a tuple.
///
/// Input: (Lhs, Rhs) where both are UTerm or UInt
/// Output: (True|False, True|False) representing (Le, Lt)
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct CmpNat;

// Base case: (UTerm, UTerm) → (True, False)  // 0 <= 0, 0 !< 0
impl OpTy<(UTerm, UTerm)> for CmpNat {
    type OutTy = (True, False);
}

// (UTerm, UInt) → (True, True)  // 0 <= n, 0 < n (for n > 0)
impl<B, T> OpTy<(UTerm, UInt<B, T>)> for CmpNat {
    type OutTy = (True, True);
}

// (UInt, UTerm) → (False, False)  // n !<= 0, n !< 0 (for n > 0)
impl<B, T> OpTy<(UInt<B, T>, UTerm)> for CmpNat {
    type OutTy = (False, False);
}

// Recursive case: (UInt<BA, TA>, UInt<BB, TB>)
// Strategy:
// 1. Compare tails recursively: TailCmp = CmpNat(TA, TB)
// 2. Check if tails are equal: TailEq = NumEqOp<TB>(TA)
// 3. Compare current bits: BitCmp = CmpBit(BA, BB)
// 4. If tails equal: use BitCmp, else use TailCmp

// Local type alias for readability
type CmpBranch<BA, TA, BB, TB> =
    IfApply<IdOp, ConstCOp<OpTyOut<CmpBit, (BA, BB)>>, ConstCOp<OpTyOut<CmpNat, (TA, TB)>>>;

impl<BA, TA, BB, TB> OpTy<(UInt<BA, TA>, UInt<BB, TB>)> for CmpNat
where
    // Compare tails recursively
    CmpNat: OpTy<(TA, TB)>,
    // Check tail equality
    NumEqOp<TB>: OpTy<TA>,
    // Compare current bits
    CmpBit: OpTy<(BA, BB)>,
    // If tails equal, use bit comparison; else use tail comparison
    CmpBranch<BA, TA, BB, TB>: OpTy<OpTyOut<NumEqOp<TB>, TA>>,
{
    type OutTy = OpTyOut<CmpBranch<BA, TA, BB, TB>, OpTyOut<NumEqOp<TB>, TA>>;
}

// ─────────────────────────────────────────────────────────────────────────────
// Morphism implementations for Le and Lt (natural numbers only)
// ─────────────────────────────────────────────────────────────────────────────

// LeOp<Rhs> for UTerm (zero)
impl<Rhs> OpTy<UTerm> for LeOp<Rhs>
where
    CmpNat: OpTy<(UTerm, Rhs)>,
    FstOp: OpTy<OpTyOut<CmpNat, (UTerm, Rhs)>>,
{
    type OutTy = OpTyOut<FstOp, OpTyOut<CmpNat, (UTerm, Rhs)>>;
}

// LeOp<Rhs> for UInt (non-zero naturals)
impl<B, T, Rhs> OpTy<UInt<B, T>> for LeOp<Rhs>
where
    CmpNat: OpTy<(UInt<B, T>, Rhs)>,
    FstOp: OpTy<OpTyOut<CmpNat, (UInt<B, T>, Rhs)>>,
{
    type OutTy = OpTyOut<FstOp, OpTyOut<CmpNat, (UInt<B, T>, Rhs)>>;
}

// LtOp<Rhs> for UTerm (zero)
impl<Rhs> OpTy<UTerm> for LtOp<Rhs>
where
    CmpNat: OpTy<(UTerm, Rhs)>,
    SndOp: OpTy<OpTyOut<CmpNat, (UTerm, Rhs)>>,
{
    type OutTy = OpTyOut<SndOp, OpTyOut<CmpNat, (UTerm, Rhs)>>;
}

// LtOp<Rhs> for UInt (non-zero naturals)
impl<B, T, Rhs> OpTy<UInt<B, T>> for LtOp<Rhs>
where
    CmpNat: OpTy<(UInt<B, T>, Rhs)>,
    SndOp: OpTy<OpTyOut<CmpNat, (UInt<B, T>, Rhs)>>,
{
    type OutTy = OpTyOut<SndOp, OpTyOut<CmpNat, (UInt<B, T>, Rhs)>>;
}
