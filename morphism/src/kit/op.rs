//! Morphism kernel: operation traits (`OpTy`, `OpOnce`).
//!
//! ## Purpose
//!
//! Define the minimal *semantic interface* used across the showcase crate to model morphisms:
//! - `OpTy<Args>`: type-level evaluation (computes an output **type**)
//! - `OpOnce<Args>`: value-level evaluation (computes an output **value**)
//!
//! This module is intentionally tiny and dependency-free so it can serve as the
//! stable “kernel vocabulary” for the rest of the crate.
//!
//! ## Contents
//!
//! - `OpTy<Args>`: type-level morphism trait.
//! - `OpTyOut<F, Args>`: convenience alias for `<F as OpTy<Args>>::OutTy`.
//! - `OpOnce<Args>`: value-level morphism trait.
//! - `OpOnceOut<F, Args>`: convenience alias for `<F as OpOnce<Args>>::OutVal`.
//!
//! ## Invariants
//!
//! - This module defines **interfaces only** (traits + aliases), not concrete operations.
//! - No registry declarations (`D_*`, `R_*`, `IX_*`) live here.
//! - No reflected syntax (`Tagged`, AST nodes) lives here.
//! - Keep this module small: it should remain readable in one sitting.
//!
//! ## Non-goals
//!
//! - No compiler behavior (lowering/reification/dispatch).
//! - No domain-specific operations (numeric/bool/order/etc).

/// A type-level morphism: maps argument types `Args` to an output type.
pub trait OpTy<Args> {
    /// The output type produced by this morphism when given `Args`.
    type OutTy;
}

/// Convenience alias for the output type of applying `F` to `Args`.
pub type OpTyOut<F, Args> = <F as OpTy<Args>>::OutTy;

/// A value-level morphism: maps argument values `Args` to an output value.
pub trait OpOnce<Args> {
    /// The output value type produced by this morphism when run on `Args`.
    type OutVal;

    /// Run this morphism once.
    fn run(self, args: Args) -> Self::OutVal;
}

/// Convenience alias for the output value of applying `F` to `Args`.
pub type OpOnceOut<F, Args> = <F as OpOnce<Args>>::OutVal;
