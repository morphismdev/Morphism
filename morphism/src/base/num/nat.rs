//! Natural number syntax: binary representation types (LSB-first).
//!
//! This is a **structural encoding** used by type-level arithmetic in `base::num`.
//! It is intentionally **untagged** (no `IX_*` keys) because it is not dispatched on
//! by key and does not participate in compiler/query pipelines.
//!
//! Reading intuition (informal):
//! - `UTerm` is the terminator (0).
//! - `UInt<B1, UTerm>` is 1.
//! - `UInt<B0, UInt<B1, UTerm>>` is 2 (binary 10, LSB-first).

/// Bit kind: 0.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct B0;

/// Bit kind: 1.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct B1;

/// Zero terminator (canonical zero; no leading zeros beyond this).
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct UTerm;

/// Binary natural (LSB-first), like typenum's `UInt<B, T>`.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct UInt<B, T>(core::marker::PhantomData<(B, T)>);

// ─────────────────────────────────────────────────────────────────────────────
// Normalization operations
// ─────────────────────────────────────────────────────────────────────────────

use crate::base::boolean::IfApply;
use crate::base::num::predicates::IsZeroOp;
use crate::kit::combinators::ConstCOp;
use crate::kit::op::{OpTy, OpTyOut};

/// Op-based normalization for binary naturals.
/// Strips high-order B0 frames (MSB zeros) to guarantee canonical forms.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct NormalizeNatOp;

/// Helper: wrap input in B0 frame.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct WrapB0;

impl<T> OpTy<T> for WrapB0 {
    type OutTy = UInt<B0, T>;
}

/// Helper: given the *already-normalized tail*, decide whether to keep or drop a `B0` frame.
/// - if tail == 0 => drop (so overall becomes 0)
/// - else => keep the `B0` digit
pub type NormalizeB0TailOp = IfApply<IsZeroOp, ConstCOp<UTerm>, WrapB0>;

impl OpTy<UTerm> for NormalizeNatOp {
    type OutTy = UTerm;
}

impl<T> OpTy<UInt<B1, T>> for NormalizeNatOp
where
    NormalizeNatOp: OpTy<T>,
{
    type OutTy = UInt<B1, OpTyOut<NormalizeNatOp, T>>;
}

impl<T> OpTy<UInt<B0, T>> for NormalizeNatOp
where
    NormalizeNatOp: OpTy<T>,
    NormalizeB0TailOp: OpTy<OpTyOut<NormalizeNatOp, T>>,
{
    type OutTy = OpTyOut<NormalizeB0TailOp, OpTyOut<NormalizeNatOp, T>>;
}
