//! AST program nodes for HList fold pipeline primitives.
//!
//! These are kernel primitives that enable expressing HList operations
//! as pure AST fold pipelines.

use crate::{NullaryToken, Tagged, IX_HLIST_COMPOSE_BALANCED_L, IX_HLIST_TO_SEGMENTS_L};

/// Program node: to-segments transformer (fold-left).
///
/// Payload is a step program `StepProg`.
/// This reifies to the runtime transformer `HlistToSegmentsL<StepOp>`
/// where `StepOp` is the reified step program.
pub type HlistToSegmentsLProg<StepProg> = Tagged<IX_HLIST_TO_SEGMENTS_L, StepProg>;

/// Construct a to-segments program node.
#[inline]
pub const fn hlist_to_segments_l_prog<StepProg>(
    step_prog: StepProg,
) -> HlistToSegmentsLProg<StepProg> {
    Tagged::new(step_prog)
}

/// Program node: balanced composer (fold-left).
///
/// Nullary (no payload).
/// This reifies to the runtime composer `HlistComposeBalancedL`.
pub type HlistComposeBalancedLProg = Tagged<IX_HLIST_COMPOSE_BALANCED_L, NullaryToken>;

/// Construct a balanced composer program node.
#[inline]
pub const fn hlist_compose_balanced_l_prog() -> HlistComposeBalancedLProg {
    Tagged::new(NullaryToken)
}
