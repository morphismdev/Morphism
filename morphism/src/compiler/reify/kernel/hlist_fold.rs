//! Closed-table reify: HList fold pipeline domain (kernel-only).
//!
//! Routes recursion through `ReifyTable` for closed-world reification.
//!
//! Only kernel keys that survive lowering are present here (dense U0..U2).

use crate::compiler::reify::ReifyTable;
use crate::compiler::table::ReifyByTable;
use crate::{Domain, Key, OpOnce, Tagged};
use crate::{D_HLIST_FOLD, R_SEMANTICS};

// ─────────────────────────────────────────────────────────────────────────────
// Routing: ReifyTable dispatches to this domain's table
// ─────────────────────────────────────────────────────────────────────────────

// HList fold semantics keys: dispatch using the hlist-fold-domain table.
impl<Input, Ix, Payload> OpOnce<Tagged<Key<Domain<D_HLIST_FOLD>, R_SEMANTICS, Ix>, Payload>>
    for ReifyTable<Input>
where
    ReifyByTable<Input, ReifyTableHlistFold<Input>>:
        OpOnce<Tagged<Key<Domain<D_HLIST_FOLD>, R_SEMANTICS, Ix>, Payload>>,
{
    type OutVal = <ReifyByTable<Input, ReifyTableHlistFold<Input>> as OpOnce<
        Tagged<Key<Domain<D_HLIST_FOLD>, R_SEMANTICS, Ix>, Payload>,
    >>::OutVal;

    #[inline]
    fn run(
        self,
        prog: Tagged<Key<Domain<D_HLIST_FOLD>, R_SEMANTICS, Ix>, Payload>,
    ) -> Self::OutVal {
        ReifyByTable::<Input, _>::new(reify_table_hlist_fold::<Input>()).run(prog)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Table definition and handlers
// ─────────────────────────────────────────────────────────────────────────────

use core::marker::PhantomData;

use crate::hlist::map::HlistMap;
use crate::{EvalProg, HlistComposeBalancedL, HlistToSegmentsL, NullaryToken};

/// Reify table for HlistFoldDomain semantics keys (indices `U0..U2`).
///
/// Dense encoding: only kernel keys that survive lowering.
pub type ReifyTableHlistFold<Input> = (
    CompileHlistToSegmentsL<Input>,      // U0: IX_HLIST_TO_SEGMENTS_L
    CompileHlistComposeBalancedL<Input>, // U1: IX_HLIST_COMPOSE_BALANCED_L
    CompileHlistMapRt<Input>,            // U2: IX_HLIST_MAP (runtime bridge key)
);

#[inline]
pub const fn reify_table_hlist_fold<Input>() -> ReifyTableHlistFold<Input> {
    (
        CompileHlistToSegmentsL::new(),
        CompileHlistComposeBalancedL::new(),
        CompileHlistMapRt::new(),
    )
}

// U0: IX_HLIST_TO_SEGMENTS_L
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CompileHlistToSegmentsL<Input>(PhantomData<fn() -> Input>);

impl<Input> CompileHlistToSegmentsL<Input> {
    #[inline]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Input, StepProg, StepOp> OpOnce<StepProg> for CompileHlistToSegmentsL<Input>
where
    crate::compiler::reify::ReifyTable<Input>: OpOnce<StepProg, OutVal = StepOp>,
{
    type OutVal = HlistToSegmentsL<StepOp>;

    #[inline]
    fn run(self, step_prog: StepProg) -> Self::OutVal {
        let step_op = crate::compiler::reify::ReifyTable::<Input>::new().run(step_prog);
        HlistToSegmentsL::new(step_op)
    }
}

// U1: IX_HLIST_COMPOSE_BALANCED_L
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CompileHlistComposeBalancedL<Input>(PhantomData<fn() -> Input>);

impl<Input> CompileHlistComposeBalancedL<Input> {
    #[inline]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Input> OpOnce<NullaryToken> for CompileHlistComposeBalancedL<Input> {
    type OutVal = HlistComposeBalancedL;

    #[inline]
    fn run(self, _: NullaryToken) -> Self::OutVal {
        HlistComposeBalancedL
    }
}

// U2: IX_HLIST_MAP (runtime bridge key - construct runtime op with EvalProg)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CompileHlistMapRt<Input>(PhantomData<fn() -> Input>);

impl<Input> CompileHlistMapRt<Input> {
    #[inline]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Input, StepProg> OpOnce<StepProg> for CompileHlistMapRt<Input> {
    type OutVal = HlistMap<EvalProg<StepProg>>;

    #[inline]
    fn run(self, step: StepProg) -> Self::OutVal {
        HlistMap::new(EvalProg::new(step))
    }
}
