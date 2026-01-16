//! Boolean elimination primitives: case analysis on `True` / `False`.
//!
//! ## Purpose
//!
//! Provides foundational boolean case analysis primitives used throughout the **showcase crate**:
//! - `Select`: direct pattern match on boolean constructors
//! - `If`: type-level conditional (selects between types)
//! - `IfApply`: operation-level conditional (selects between operations)
//!
//! These are **foundational control-flow primitives** that higher layers (lowering, dispatch, compiler, etc.)
//! build on, so they belong in `base` to keep the dependency DAG clean.
//!
//! ## Contents
//!
//! - `Select<ThenBranch, ElseBranch>`: boolean case analysis primitive
//! - `If<Pred, ThenBranch, ElseBranch>`: type-level conditional
//! - `IfApply<Pred, ThenBranch, ElseBranch>`: operation-level conditional
//! - Convenience aliases: `IfOut`, `IfApplyOut`
//!
//! ## Invariants
//!
//! - These are type-level only (`OpTy` implementations, no `OpOnce`).
//! - `Select` is the primitive; `If` and `IfApply` are built on top of it.
//! - All operations are zero-sized and must remain so.
//!
//! ## Non-goals
//!
//! - No higher-level pattern matching frameworks in this crate.
//! - No runtime dispatch or value-level elimination.

use core::marker::PhantomData;

use super::{False, True};
use crate::kit::combinators::ConstCOp;
use crate::kit::op::{OpTy, OpTyOut};

// ─────────────────────────────────────────────────────────────────────────────
// Select: primitive boolean case analysis
// ─────────────────────────────────────────────────────────────────────────────

/// Boolean case analysis (primitive): match on Bool constructors (`True` / `False`).
///
/// Type-level only: selects between two types based on the boolean constructor.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Select<ThenBranch, ElseBranch>(PhantomData<fn() -> (ThenBranch, ElseBranch)>);

impl<ThenBranch, ElseBranch> OpTy<True> for Select<ThenBranch, ElseBranch> {
    type OutTy = ThenBranch;
}

impl<ThenBranch, ElseBranch> OpTy<False> for Select<ThenBranch, ElseBranch> {
    type OutTy = ElseBranch;
}

// ─────────────────────────────────────────────────────────────────────────────
// If: type-level conditional
// ─────────────────────────────────────────────────────────────────────────────

/// Type-level `if`: **branch result is a _type_**, not an op.
///
/// This is the canonical "pattern match on Bool ctor" eliminator when you want to
/// pick between two output *types* based on a predicate evaluated on the input.
///
/// Conceptually:
/// - compute `Pred(Arg) : Bool`
/// - select `ThenBranch` if `True`, else `ElseBranch`
///
/// Notes:
/// - `ThenBranch` / `ElseBranch` are **types**, not ops.
/// - Internally this uses `Select` (`True`/`False` constructors).
///
/// Use this when the branches are "already computed" types (e.g. tuples, tags, etc).
///
/// If you instead want to choose between two **operations** and then apply the chosen
/// operation to the same `Arg`, use [`IfApply`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(clippy::type_complexity)] // PhantomData with tuple is intentionally complex for type-level encoding
pub struct If<Pred, ThenBranch, ElseBranch>(PhantomData<fn() -> (Pred, ThenBranch, ElseBranch)>);

impl<Pred, ThenBranch, ElseBranch, Arg> OpTy<Arg> for If<Pred, ThenBranch, ElseBranch>
where
    Pred: OpTy<Arg>,
    Select<ThenBranch, ElseBranch>: OpTy<OpTyOut<Pred, Arg>>,
{
    type OutTy = OpTyOut<Select<ThenBranch, ElseBranch>, OpTyOut<Pred, Arg>>;
}

/// Convenience alias for the output of [`If`] when applied to an `Arg`.
///
/// `IfOut<Pred, Then, Else, Arg>` = `OpTyOut<If<Pred, Then, Else>, Arg>`
pub type IfOut<Pred, ThenBranch, ElseBranch, Arg> = OpTyOut<If<Pred, ThenBranch, ElseBranch>, Arg>;

// ─────────────────────────────────────────────────────────────────────────────
// IfApply: operation-level conditional
// ─────────────────────────────────────────────────────────────────────────────

/// "If + Apply": select between two **ops** based on a predicate on the input,
/// then apply the selected op to the *same* input.
///
/// This is the canonical eliminator when branches are operations (ops),
/// not precomputed types.
///
/// Conceptually:
/// - compute `Pred(Arg) : Bool`
/// - choose `ThenBranch` or `ElseBranch` (these are *ops*)
/// - apply the chosen op to `Arg`
///
/// You *can* express this manually using [`If`] by selecting an op type and then
/// applying it, but this type packages that boilerplate (and the required bounds)
/// into one place so call sites don't re-implement "manual pattern matching plumbing".
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(clippy::type_complexity)] // PhantomData with tuple is intentionally complex for type-level encoding
pub struct IfApply<Pred, ThenBranch, ElseBranch>(
    PhantomData<fn() -> (Pred, ThenBranch, ElseBranch)>,
);

impl<Pred, ThenBranch, ElseBranch, Arg> OpTy<Arg> for IfApply<Pred, ThenBranch, ElseBranch>
where
    Pred: OpTy<Arg>,
    Select<ThenBranch, ElseBranch>: OpTy<OpTyOut<Pred, Arg>>,
    // After selecting the branch op, it must be applicable to Arg:
    OpTyOut<Select<ThenBranch, ElseBranch>, OpTyOut<Pred, Arg>>: OpTy<Arg>,
{
    type OutTy = OpTyOut<OpTyOut<Select<ThenBranch, ElseBranch>, OpTyOut<Pred, Arg>>, Arg>;
}

/// Convenience alias: `IfApplyOut<Pred, Then, Else, Arg>` = `OpTyOut<IfApply<Pred, Then, Else>, Arg>`
pub type IfApplyOut<Pred, ThenBranch, ElseBranch, Arg> =
    OpTyOut<IfApply<Pred, ThenBranch, ElseBranch>, Arg>;

// ─────────────────────────────────────────────────────────────────────────────
// Convenience aliases: constant-output conditionals
// ─────────────────────────────────────────────────────────────────────────────

/// If with constant output types: `IfConst<Pred, ThenTy, ElseTy>`
/// = `IfApply<Pred, ConstC<ThenTy>, ConstC<ElseTy>>`
///
/// Use this when both branches are constant output types (not arbitrary ops).
///
/// Note: `ConstCOp` is provided by `kit::combinators` (core combinators).
pub type IfConst<Pred, ThenTy, ElseTy> = IfApply<Pred, ConstCOp<ThenTy>, ConstCOp<ElseTy>>;

/// IfConst with output: `IfConstOut<Pred, ThenTy, ElseTy, Arg>`
/// = `OpTyOut<IfConst<Pred, ThenTy, ElseTy>, Arg>`
pub type IfConstOut<Pred, ThenTy, ElseTy, Arg> = OpTyOut<IfConst<Pred, ThenTy, ElseTy>, Arg>;

// ─────────────────────────────────────────────────────────────────────────────
// AssertTrue: compile-time rejection primitive
// ─────────────────────────────────────────────────────────────────────────────

/// Assert that a predicate evaluates to `True`; acts as identity on valid inputs.
///
/// This is a compile-time rejection primitive: if `Pred(Arg)` does not evaluate to `True`
/// (i.e., evaluates to `False`), then no `OpTy` impl applies, causing a hard type error.
///
/// Use this to enforce constraints that must hold at compile time (e.g., "N must be non-zero").
///
/// Example:
/// ```rust,ignore
/// // This will compile:
/// type Valid = OpTyOut<AssertTrue<IsNonZeroOp>, U5>;
///
/// // This will fail to compile (no impl for False):
/// type Invalid = OpTyOut<AssertTrue<IsNonZeroOp>, U0>;
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AssertTrue<Pred>(PhantomData<fn() -> Pred>);

impl<Pred, Arg> OpTy<Arg> for AssertTrue<Pred>
where
    Pred: OpTy<Arg>,
    // Only apply when Pred(Arg) = True; Select<Arg, ()> only has an impl for True input
    Select<Arg, ()>: OpTy<OpTyOut<Pred, Arg>>,
{
    type OutTy = OpTyOut<Select<Arg, ()>, OpTyOut<Pred, Arg>>;
}
