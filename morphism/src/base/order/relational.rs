//! Type-level ordering for **binary** naturals.
//!
//! Notes:
//! - Nat `LeOp`/`LtOp` implementations are in `cmp.rs` (internal machinery).
//! - This file defines the public wrapper ops (`LeOp`, `LtOp`, `GeOp`, `GtOp`).

use core::marker::PhantomData;

use crate::base::num::nat::{UInt, UTerm};
use crate::kit::op::{OpTy, OpTyOut};

// ─────────────────────────────────────────────────────────────────────────────
// Polymorphic wrappers (nat + int)
// ─────────────────────────────────────────────────────────────────────────────

/// ≤ (nat impl in `cmp.rs`, int impls below)
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct LeOp<Rhs>(PhantomData<fn() -> Rhs>);

/// < (nat impl in `cmp.rs`, int impls below)
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct LtOp<Rhs>(PhantomData<fn() -> Rhs>);

/// ≥ for Naturals (A >= B iff B <= A)
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct GeOp<Rhs>(PhantomData<fn() -> Rhs>);

/// > for Naturals (A > B iff B < A)
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct GtOp<Rhs>(PhantomData<fn() -> Rhs>);

// ─────────────────────────────────────────────────────────────────────────────
// Nat ≥ and > (derive from Le/Lt to avoid duplicating `cmp.rs` internals)
// ─────────────────────────────────────────────────────────────────────────────

impl<Rhs> OpTy<UTerm> for GeOp<Rhs>
where
    LeOp<UTerm>: OpTy<Rhs>,
{
    type OutTy = OpTyOut<LeOp<UTerm>, Rhs>;
}

impl<B, T, Rhs> OpTy<UInt<B, T>> for GeOp<Rhs>
where
    LeOp<UInt<B, T>>: OpTy<Rhs>,
{
    type OutTy = OpTyOut<LeOp<UInt<B, T>>, Rhs>;
}

impl<Rhs> OpTy<UTerm> for GtOp<Rhs>
where
    LtOp<UTerm>: OpTy<Rhs>,
{
    type OutTy = OpTyOut<LtOp<UTerm>, Rhs>;
}

impl<B, T, Rhs> OpTy<UInt<B, T>> for GtOp<Rhs>
where
    LtOp<UInt<B, T>>: OpTy<Rhs>,
{
    type OutTy = OpTyOut<LtOp<UInt<B, T>>, Rhs>;
}
