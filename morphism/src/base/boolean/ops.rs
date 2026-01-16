//! Boolean operations (type-level only).
//!
//! ## Purpose
//! Provide a minimal set of boolean ops as `OpTy` morphisms: `Not`, `And`, `Or`, `Xor`, `AndNot`.
//!
//! ## Invariants
//! - `OpTy` only (no `OpOnce`).
//! - No tagging / registry keys (these ops are unreflected).
//!
//! ## Non-goals
//! - No compiler behavior.
//! - No value-level boolean evaluation.

use core::marker::PhantomData;

use super::{False, True};
use crate::kit::op::{OpTy, OpTyOut};

// ─────────────────────────────────────────────────────────────────────────────
// NOT
// ─────────────────────────────────────────────────────────────────────────────

/// Type-level boolean NOT as an op: `B -> !B`.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Not;

impl OpTy<True> for Not {
    type OutTy = False;
}

impl OpTy<False> for Not {
    type OutTy = True;
}

// ─────────────────────────────────────────────────────────────────────────────
// AND
// ─────────────────────────────────────────────────────────────────────────────

/// Type-level boolean AND as an op: `A -> (A & Rhs)`.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct And<Rhs>(PhantomData<fn() -> Rhs>);

impl<Rhs> OpTy<True> for And<Rhs> {
    type OutTy = Rhs;
}

impl<Rhs> OpTy<False> for And<Rhs> {
    type OutTy = False;
}

// ─────────────────────────────────────────────────────────────────────────────
// OR
// ─────────────────────────────────────────────────────────────────────────────

/// Type-level boolean OR as an op: `A -> (A | Rhs)`.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Or<Rhs>(PhantomData<fn() -> Rhs>);

impl<Rhs> OpTy<True> for Or<Rhs> {
    type OutTy = True;
}

impl<Rhs> OpTy<False> for Or<Rhs> {
    type OutTy = Rhs;
}

// ─────────────────────────────────────────────────────────────────────────────
// XOR
// ─────────────────────────────────────────────────────────────────────────────

/// Type-level boolean XOR as an op: `A -> (A ^ Rhs)`.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Xor<Rhs>(PhantomData<fn() -> Rhs>);

impl<Rhs> OpTy<True> for Xor<Rhs>
where
    Not: OpTy<Rhs>,
{
    type OutTy = OpTyOut<Not, Rhs>;
}

impl<Rhs> OpTy<False> for Xor<Rhs> {
    type OutTy = Rhs;
}

// ─────────────────────────────────────────────────────────────────────────────
// AND-NOT
// ─────────────────────────────────────────────────────────────────────────────

/// Type-level boolean AND-NOT as an op: `A -> (A & !Rhs)`.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct AndNot<Rhs>(PhantomData<fn() -> Rhs>);

impl<Rhs> OpTy<True> for AndNot<Rhs>
where
    Not: OpTy<Rhs>,
{
    type OutTy = OpTyOut<Not, Rhs>;
}

impl<Rhs> OpTy<False> for AndNot<Rhs> {
    type OutTy = False;
}
