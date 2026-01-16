//! Numeric predicates: type-level checks for numeric values.
//!
//! ## Purpose
//!
//! Provides foundational numeric predicate operations used throughout `base::num`:
//! - `IsZeroOp`: checks if a natural equals zero
//! - `IsNonZeroOp`: checks if a natural is non-zero (negation of `IsZeroOp`)
//! - `IsOddOp`: checks if a natural is odd
//! - `IsEvenOp`: checks if a natural is even
//!
//! These are **pure semantic operations** (`OpTy` implementations) used internally by
//! arithmetic algorithms and normalization. They are not reflected as program nodes
//! (no `Tagged <IX_EXAMPLE, ...>` program nodes or registry keys).
//!
//! ## Contents
//!
//! - `IsZeroOp`: predicate `N == 0` (treats non-canonical "zero-ish" binaries as zero)
//! - `IsNonZeroOp`: predicate `N != 0` (negation of `IsZeroOp`)
//! - `IsOddOp`: predicate `N % 2 == 1`
//! - `IsEvenOp`: predicate `N % 2 == 0`
//!
//! ## Invariants
//!
//! - These are type-level only (`OpTy` implementations, no `OpOnce`).
//! - All operations are zero-sized and must remain so.
//! - `IsZeroOp` treats non-canonical zero-ish forms (like `UInt<B0, UTerm>`) as zero.
//!
//! ## Non-goals
//!
//! - No reflected program nodes (no `Tagged <IX_EXAMPLE, ...>` program nodes or registry keys).
//! - No compiler behavior (lowering/reification/dispatch).

use crate::base::boolean::ops::Not;
use crate::base::boolean::{False, True};
use crate::base::num::nat::{UInt, UTerm, B0, B1};
use crate::kit::op::{OpTy, OpTyOut};

/// IsZero predicate op: N → True/False
///
/// Predicate: `N == 0`.
/// Important: treats non-canonical "zero-ish" binaries like `UInt<B0, UTerm>` as zero.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct IsZeroOp;

// ---- Nat ----

impl OpTy<UTerm> for IsZeroOp {
    type OutTy = True;
}

impl<T> OpTy<UInt<B1, T>> for IsZeroOp {
    type OutTy = False;
}

impl<T> OpTy<UInt<B0, T>> for IsZeroOp
where
    IsZeroOp: OpTy<T>,
{
    type OutTy = OpTyOut<IsZeroOp, T>;
}

/// IsNonZero predicate op: N → True/False
///
/// Predicate: `N != 0`.
/// This is the negation of `IsZeroOp`: `IsNonZeroOp(N) = Not(IsZeroOp(N))`.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct IsNonZeroOp;

impl<N> OpTy<N> for IsNonZeroOp
where
    IsZeroOp: OpTy<N>,
    Not: OpTy<OpTyOut<IsZeroOp, N>>,
{
    type OutTy = OpTyOut<Not, OpTyOut<IsZeroOp, N>>;
}

/// IsOdd predicate op: N → True/False
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct IsOddOp;

// ---- Nat ----

impl OpTy<UTerm> for IsOddOp {
    type OutTy = False;
}

impl<T> OpTy<UInt<B0, T>> for IsOddOp {
    type OutTy = False;
}

impl<T> OpTy<UInt<B1, T>> for IsOddOp {
    type OutTy = True;
}

/// IsEven predicate op: N → True/False
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct IsEvenOp;

// ---- Nat ----

impl OpTy<UTerm> for IsEvenOp {
    type OutTy = True;
}

impl<T> OpTy<UInt<B0, T>> for IsEvenOp {
    type OutTy = True;
}

impl<T> OpTy<UInt<B1, T>> for IsEvenOp {
    type OutTy = False;
}
