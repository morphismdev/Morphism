//! Closed-table lowering: HList fold pipeline domain (kernel-only).
//!
//! Lowering rules for kernel keys:
//! - Most keys are identity (pass-through) since they are kernel primitives.
//! - `IX_HLIST_TO_SEGMENTS_L` and `IX_HLIST_TO_SEGMENTS_R` recursively lower their `StepProg` payload to ensure
//!   nested frontend keys don't leak into reify.

use crate::compiler::lower::LowerTable;
use crate::compiler::table::LowerByTable;
use crate::{Domain, Key, OpOnce, Tagged};
use crate::{D_HLIST_FOLD, R_SEMANTICS};

// ─────────────────────────────────────────────────────────────────────────────
// Routing: LowerTable dispatches to this domain's table
// ─────────────────────────────────────────────────────────────────────────────

// HList fold semantics lowering: dispatch using the hlist-fold-domain table.
impl<Input, Ix, Payload> OpOnce<Tagged<Key<Domain<D_HLIST_FOLD>, R_SEMANTICS, Ix>, Payload>>
    for LowerTable<Input>
where
    LowerByTable<Input, LowerTableHlistFold<Input>>:
        OpOnce<Tagged<Key<Domain<D_HLIST_FOLD>, R_SEMANTICS, Ix>, Payload>>,
{
    type OutVal = <LowerByTable<Input, LowerTableHlistFold<Input>> as OpOnce<
        Tagged<Key<Domain<D_HLIST_FOLD>, R_SEMANTICS, Ix>, Payload>,
    >>::OutVal;

    #[inline]
    fn run(
        self,
        prog: Tagged<Key<Domain<D_HLIST_FOLD>, R_SEMANTICS, Ix>, Payload>,
    ) -> Self::OutVal {
        LowerByTable::<Input, _>::new(lower_table_hlist_fold::<Input>()).run(prog)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Table definition and handlers
// ─────────────────────────────────────────────────────────────────────────────

use core::marker::PhantomData;

use crate::NullaryToken;
use crate::{
    // constructors
    hlist_compose_balanced_l_prog,
    hlist_map_prog,
    hlist_to_segments_l_prog,
    // semantic aliases
    HlistComposeBalancedLProg,
    HlistMapProg,
    HlistToSegmentsLProg,
};

/// Lowering table for HlistFoldDomain semantics keys (indices `U0..U2`).
///
/// Dense encoding: kernel keys that survive lowering.
pub type LowerTableHlistFold<Input> = (
    LowerHlistToSegmentsL<Input>, // U0: IX_HLIST_TO_SEGMENTS_L
    LowerHlistComposeBalancedL,   // U1: IX_HLIST_COMPOSE_BALANCED_L
    LowerHlistMapRt,              // U2: IX_HLIST_MAP (runtime bridge key)
);

#[inline]
pub const fn lower_table_hlist_fold<Input>() -> LowerTableHlistFold<Input> {
    (
        LowerHlistToSegmentsL::new(),
        LowerHlistComposeBalancedL,
        LowerHlistMapRt,
    )
}

// U0: IX_HLIST_TO_SEGMENTS_L (recursively lowers payload to ensure nested frontend keys are eliminated)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LowerHlistToSegmentsL<Input>(PhantomData<fn() -> Input>);

impl<Input> LowerHlistToSegmentsL<Input> {
    #[inline]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Input, StepProg> OpOnce<StepProg> for LowerHlistToSegmentsL<Input>
where
    crate::compiler::lower::LowerTable<Input>: OpOnce<StepProg>,
{
    type OutVal = HlistToSegmentsLProg<
        <crate::compiler::lower::LowerTable<Input> as OpOnce<StepProg>>::OutVal,
    >;

    #[inline]
    fn run(self, step_prog: StepProg) -> Self::OutVal {
        let step_lowered = crate::compiler::lower::LowerTable::<Input>::new().run(step_prog);
        hlist_to_segments_l_prog(step_lowered)
    }
}

// U1: IX_HLIST_COMPOSE_BALANCED_L (nullary, identity)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LowerHlistComposeBalancedL;

impl OpOnce<NullaryToken> for LowerHlistComposeBalancedL {
    type OutVal = HlistComposeBalancedLProg;
    #[inline]
    fn run(self, _: NullaryToken) -> Self::OutVal {
        hlist_compose_balanced_l_prog()
    }
}

// U2: IX_HLIST_MAP (runtime bridge key - identity pass-through)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LowerHlistMapRt;

impl<StepProg> OpOnce<StepProg> for LowerHlistMapRt {
    type OutVal = HlistMapProg<StepProg>;

    #[inline]
    fn run(self, step: StepProg) -> Self::OutVal {
        hlist_map_prog(step)
    }
}
