//! Fold-left over hlists (balanced spine approach).
//!
//! Pipeline: map elements → segment transformers → balanced compose → apply to accumulator
//!
//! Note: The HList-domain program constructors (FoldLProg) have been removed.
//! Use the balanced pipeline expansion directly in lowering instead.

// Runtime: executable fold operations
pub mod runtime;
pub use runtime::*;

// Pipeline: composition primitives
pub mod pipeline;
pub use pipeline::*;
