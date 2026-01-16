//! Runtime implementation for HList map (DSL layer).
//!
//! **Implementation**: Uses the fold_l pipeline (map elements → segment transformers →
//! balanced compose → apply to accumulator).
//!
//! Semantics:
//! - arity < 12  => map all elements (no tail)
//! - arity == 12 => map first 11 elements, recurse into element 12 (tail)
//!
//! **Public API**: Use `HlistMapProg` / `hlist_map_prog` in this module (AST node),
//! and `HlistMap<Op>` for the runtime op.

use crate::ApplyOp;
use crate::HlistPushBack;
use crate::OpOnce;
use crate::PartialROp;
use crate::{BimapOp, IdOp, ThenOp};
use crate::{HlistComposeBalanced, HlistToSegments};

use crate::{Tagged, IX_HLIST_MAP};

/// Public AST program node: HList map (runtime bridge kernel key).
///
/// Payload is a step program `StepProg` that will be compiled via EvalProg
/// at reify time to produce `HlistMap<EvalProg<StepProg>>`.
pub type HlistMapProg<StepProg> = Tagged<IX_HLIST_MAP, StepProg>;

/// Construct a HList map program node.
#[inline]
pub const fn hlist_map_prog<StepProg>(step: StepProg) -> HlistMapProg<StepProg> {
    HlistMapProg::new(step)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HlistMap<Op>(pub Op);

impl<Op> HlistMap<Op> {
    #[inline]
    pub const fn new(op: Op) -> Self {
        Self(op)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Step function for HlistMap: uses kernel combinators (no bespoke step struct)
// ─────────────────────────────────────────────────────────────────────────────

/// Step function used to derive `HlistMap` from fold_l pipeline.
///
/// Interpreted as:
/// `(acc_tuple, a) ↦ hlist_push_back(acc_tuple, op(a))`
///
/// Implemented purely in terms of kernel combinators:
/// - `BimapOp<IdOp, Op>` transforms `(acc, a)` → `(acc, op(a))`
/// - `ThenOp<..., HlistPushBack>` applies `HlistPushBack` to that result
pub type HlistMapStep<Op> = ThenOp<BimapOp<IdOp, Op>, HlistPushBack>;

/// Construct a map step function from an op.
#[inline]
pub fn hlist_map_step<Op>(op: Op) -> HlistMapStep<Op>
where
    Op: Clone,
{
    ThenOp::new(BimapOp { f: IdOp, g: op }, HlistPushBack)
}

// ─────────────────────────────────────────────────────────────────────────────
// Implementation: delegate to fold_l pipeline
// Pipeline: HlistToSegments(HlistMapStep(op)) |> HlistComposeBalanced |> PartialROp(ApplyOp, ())
// ─────────────────────────────────────────────────────────────────────────────

impl<Op, Args> OpOnce<Args> for HlistMap<Op>
where
    Op: Clone,
    HlistToSegments<HlistMapStep<Op>>: OpOnce<Args>,
    HlistComposeBalanced: OpOnce<<HlistToSegments<HlistMapStep<Op>> as OpOnce<Args>>::OutVal>,
    PartialROp<ApplyOp, ()>: OpOnce<
        <HlistComposeBalanced as OpOnce<
            <HlistToSegments<HlistMapStep<Op>> as OpOnce<Args>>::OutVal,
        >>::OutVal,
    >,
{
    type OutVal = <PartialROp<ApplyOp, ()> as OpOnce<
        <HlistComposeBalanced as OpOnce<
            <HlistToSegments<HlistMapStep<Op>> as OpOnce<Args>>::OutVal,
        >>::OutVal,
    >>::OutVal;

    #[inline]
    fn run(self, args: Args) -> Self::OutVal {
        // Step 1: Map elements to segment transformers
        let segments = HlistToSegments::new(hlist_map_step(self.0)).run(args);
        // Step 2: Compose transformers in balanced fashion
        let composed = HlistComposeBalanced.run(segments);
        // Step 3: Apply to empty accumulator
        PartialROp {
            op: ApplyOp,
            env: (),
        }
        .run(composed)
    }
}
