//! Closed-table reify: Combinators domain (full table `U0..U9`).
//!
//! Dense encoding: only kernel keys that survive lowering.

use crate::compiler::reify::ReifyTable;
use crate::compiler::table::ReifyByTable;
use crate::{Domain, Key, OpOnce, Tagged};
use crate::{D_COMBINATORS, R_SEMANTICS};

// ─────────────────────────────────────────────────────────────────────────────
// Routing: ReifyTable dispatches to this domain's table
// ─────────────────────────────────────────────────────────────────────────────

// Combinators-domain keys: wired to the combinators table (U0..U9).
impl<Input, Ix, Payload> OpOnce<Tagged<Key<Domain<D_COMBINATORS>, R_SEMANTICS, Ix>, Payload>>
    for ReifyTable<Input>
where
    ReifyByTable<Input, ReifyTableCombinatorsFull<Input>>:
        OpOnce<Tagged<Key<Domain<D_COMBINATORS>, R_SEMANTICS, Ix>, Payload>>,
{
    type OutVal = <ReifyByTable<Input, ReifyTableCombinatorsFull<Input>> as OpOnce<
        Tagged<Key<Domain<D_COMBINATORS>, R_SEMANTICS, Ix>, Payload>,
    >>::OutVal;

    #[inline]
    fn run(
        self,
        prog: Tagged<Key<Domain<D_COMBINATORS>, R_SEMANTICS, Ix>, Payload>,
    ) -> Self::OutVal {
        ReifyByTable::<Input, _>::new(reify_table_combinators_full::<Input>()).run(prog)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Table definition and handlers
// ─────────────────────────────────────────────────────────────────────────────

use core::marker::PhantomData;

use crate::{
    // semantics (runtime)
    ApplyOp,
    BimapOp,
    ConstMOp,
    FanoutOp,
    FstOp,
    IdOp,
    // payload/ast
    NullaryToken,
    PartialLOp,
    PartialROp,
    SndOp,
    ThenOp,
};

/// Full combinators reify table type (`U0..U9`).
///
/// Dense encoding: only kernel keys that survive lowering.
pub type ReifyTableCombinatorsFull<Input> = (
    CompileId,              // U0: IX_ID
    CompileThen<Input>,     // U1: IX_THEN
    CompilePartialL<Input>, // U2: IX_PARTIAL_L
    CompilePartialR<Input>, // U3: IX_PARTIAL_R
    CompileFst,             // U4: IX_FST
    CompileSnd,             // U5: IX_SND
    CompileFanout<Input>,   // U6: IX_FANOUT
    CompileConstMove,       // U7: IX_CONST_MOVE
    CompileBimap<Input>,    // U8: IX_BIMAP
    CompileApply,           // U9: IX_APPLY
);

// ─────────────────────────────────────────────────────────────────────────────
// U0: IX_ID
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CompileId;

impl OpOnce<NullaryToken> for CompileId {
    type OutVal = IdOp;
    #[inline]
    fn run(self, _: NullaryToken) -> Self::OutVal {
        IdOp
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// U1: IX_THEN
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CompileThen<Input>(PhantomData<fn() -> Input>);

impl<Input> CompileThen<Input> {
    #[inline]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Input, KF, PF, KG, PG, F> OpOnce<(Tagged<KF, PF>, Tagged<KG, PG>)> for CompileThen<Input>
where
    crate::compiler::reify::ReifyTable<Input>: OpOnce<Tagged<KF, PF>, OutVal = F>,
    F: OpOnce<Input>,
    crate::compiler::reify::ReifyTable<<F as OpOnce<Input>>::OutVal>: OpOnce<Tagged<KG, PG>>,
{
    type OutVal = ThenOp<
        F,
        <crate::compiler::reify::ReifyTable<<F as OpOnce<Input>>::OutVal> as OpOnce<
            Tagged<KG, PG>,
        >>::OutVal,
    >;

    #[inline]
    fn run(self, (f_prog, g_prog): (Tagged<KF, PF>, Tagged<KG, PG>)) -> Self::OutVal {
        let f = crate::compiler::reify::ReifyTable::<Input>::new().run(f_prog);
        let g =
            crate::compiler::reify::ReifyTable::<<F as OpOnce<Input>>::OutVal>::new().run(g_prog);
        ThenOp::new(f, g)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// U2/U3: IX_PARTIAL_L / IX_PARTIAL_R
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CompilePartialL<Input>(PhantomData<fn() -> Input>);

impl<Input> CompilePartialL<Input> {
    #[inline]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Input, Env, KOp, POp, Op> OpOnce<(Env, Tagged<KOp, POp>)> for CompilePartialL<Input>
where
    crate::compiler::reify::ReifyTable<(Env, Input)>: OpOnce<Tagged<KOp, POp>, OutVal = Op>,
{
    type OutVal = PartialLOp<Env, Op>;

    #[inline]
    fn run(self, (env, op_lift): (Env, Tagged<KOp, POp>)) -> Self::OutVal {
        let op = crate::compiler::reify::ReifyTable::<(Env, Input)>::new().run(op_lift);
        PartialLOp { env, op }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CompilePartialR<Input>(PhantomData<fn() -> Input>);

impl<Input> CompilePartialR<Input> {
    #[inline]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Input, KOp, POp, Env, Op> OpOnce<(Tagged<KOp, POp>, Env)> for CompilePartialR<Input>
where
    crate::compiler::reify::ReifyTable<(Input, Env)>: OpOnce<Tagged<KOp, POp>, OutVal = Op>,
{
    type OutVal = PartialROp<Op, Env>;

    #[inline]
    fn run(self, (op_lift, env): (Tagged<KOp, POp>, Env)) -> Self::OutVal {
        let op = crate::compiler::reify::ReifyTable::<(Input, Env)>::new().run(op_lift);
        PartialROp { op, env }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// U4/U5: IX_FST / IX_SND (nullary)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CompileFst;
impl OpOnce<NullaryToken> for CompileFst {
    type OutVal = FstOp;
    fn run(self, _: NullaryToken) -> Self::OutVal {
        FstOp
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CompileSnd;
impl OpOnce<NullaryToken> for CompileSnd {
    type OutVal = SndOp;
    fn run(self, _: NullaryToken) -> Self::OutVal {
        SndOp
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// U6: IX_FANOUT
// ─────────────────────────────────────────────────────────────────────────────
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CompileFanout<Input>(PhantomData<fn() -> Input>);

impl<Input> CompileFanout<Input> {
    #[inline]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Input, KF, PF, KG, PG, F, G> OpOnce<(Tagged<KF, PF>, Tagged<KG, PG>)> for CompileFanout<Input>
where
    crate::compiler::reify::ReifyTable<Input>: OpOnce<Tagged<KF, PF>, OutVal = F>,
    crate::compiler::reify::ReifyTable<Input>: OpOnce<Tagged<KG, PG>, OutVal = G>,
{
    type OutVal = FanoutOp<F, G>;

    #[inline]
    fn run(self, (f_prog, g_prog): (Tagged<KF, PF>, Tagged<KG, PG>)) -> Self::OutVal {
        let f = crate::compiler::reify::ReifyTable::<Input>::new().run(f_prog);
        let g = crate::compiler::reify::ReifyTable::<Input>::new().run(g_prog);
        FanoutOp { f, g }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// U7: IX_CONST_MOVE
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CompileConstMove;

impl<F> OpOnce<F> for CompileConstMove {
    type OutVal = ConstMOp<F>;
    #[inline]
    fn run(self, f: F) -> Self::OutVal {
        ConstMOp::new(f)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// U8: IX_BIMAP
// ─────────────────────────────────────────────────────────────────────────────
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CompileBimap<Input>(PhantomData<fn() -> Input>);

impl<Input> CompileBimap<Input> {
    #[inline]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<A, B, KF, PF, KG, PG, F, G> OpOnce<(Tagged<KF, PF>, Tagged<KG, PG>)> for CompileBimap<(A, B)>
where
    crate::compiler::reify::ReifyTable<A>: OpOnce<Tagged<KF, PF>, OutVal = F>,
    crate::compiler::reify::ReifyTable<B>: OpOnce<Tagged<KG, PG>, OutVal = G>,
{
    type OutVal = BimapOp<F, G>;

    #[inline]
    fn run(self, (f_prog, g_prog): (Tagged<KF, PF>, Tagged<KG, PG>)) -> Self::OutVal {
        let f = crate::compiler::reify::ReifyTable::<A>::new().run(f_prog);
        let g = crate::compiler::reify::ReifyTable::<B>::new().run(g_prog);
        BimapOp { f, g }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// U9: IX_APPLY
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CompileApply;

impl OpOnce<NullaryToken> for CompileApply {
    type OutVal = ApplyOp;
    #[inline]
    fn run(self, _: NullaryToken) -> Self::OutVal {
        ApplyOp
    }
}

/// Construct the full combinators table (`U0..U9`) for the given input type.
#[inline]
pub const fn reify_table_combinators_full<Input>() -> ReifyTableCombinatorsFull<Input> {
    (
        CompileId,
        CompileThen::new(),
        CompilePartialL::new(),
        CompilePartialR::new(),
        CompileFst,
        CompileSnd,
        CompileFanout::new(),
        CompileConstMove,
        CompileBimap::new(),
        CompileApply,
    )
}

#[cfg(test)]
mod tests {
    use crate::compiler::Compile;
    use crate::{fanout, op_lift, OpOnce};

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    struct Inc;

    impl OpOnce<i32> for Inc {
        type OutVal = i32;
        fn run(self, x: i32) -> Self::OutVal {
            x + 1
        }
    }

    #[test]
    fn compile_closed_v2_fanout_uses_contiguous_u11_selection() {
        // Forces combinators-domain dispatch at IX_FANOUT = U11, which must NOT select the tail pointer slot.
        let prog = fanout(op_lift(Inc), op_lift(Inc));
        let out = Compile::<i32>::new().run(prog).run(41);
        assert_eq!(out, (42, 42));
    }
}
