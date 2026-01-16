#![allow(ambiguous_glob_reexports)]

#[macro_use]
mod pipeline_macros;

mod fold_l;
#[allow(ambiguous_glob_reexports)]
pub use fold_l::*;
// TupleFoldL are internal implementation details (arity <= 11 backend).
// Note: FoldLProg have been removed. Use the balanced pipeline expansion directly.

// ─────────────────────────────────────────────────────────────────────────────
// Exports (flat naming)
// ─────────────────────────────────────────────────────────────────────────────

pub use fold_l::pipeline::{
    HlistComposeBalanced as HlistComposeBalancedL, HlistToSegments as HlistToSegmentsL,
};

mod prog;
pub use prog::*;
